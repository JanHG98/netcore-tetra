#!/usr/bin/env python3
"""Station CRUD, durable Asterisk changes and real Deploy SIP import compatibility."""
import copy
import importlib.util
import json
import os
import re
from pathlib import Path
import shutil
import sys
import tempfile
import threading
import tomllib
import unittest
from unittest.mock import Mock, patch
from urllib.error import HTTPError
from urllib.request import ProxyHandler, Request, build_opener

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'system-backend/sip-switch'
sys.path.insert(0, str(BASE / 'src'))
from netcore_sip_switch import BoundedHTTPServer, Config, Handler, SipSwitch, StationError, config_toml
sys.path.insert(0, str(ROOT / 'system-backend/deployment-core'))
from image.sip_setup import configure_local_sip, validate_sip_config


class StationFixture:
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.path = self.root / 'sip-switch.toml'
        data = tomllib.loads((BASE / 'config/sip-switch.example.toml').read_text(encoding='utf-8'))
        data['storage'] = {k: str(self.root / k) for k in ('state_file', 'event_log', 'audit_log')}
        data['asterisk']['config_dir'] = str(self.root / 'asterisk')
        data['mqtt']['enabled'] = False
        data['pbx'].update(host='10.0.1.160', username='shared-central-trunk', password='shared-trunk-secret')
        data['custom'] = {'keep': True, 'nested': {'markers': ['one', 'two']}}
        self.path.write_text(config_toml(data), encoding='utf-8', newline='\n')
        self.app = SipSwitch(Config(data, self.path))
        self.addCleanup(self.app.io.close)
        def cli(command, **kwargs):
            if command.startswith('pjsip show endpoint '):
                endpoint = command.split()[-1]
                pjsip = (self.root / 'asterisk/netcore-pjsip.conf').read_text(encoding='utf-8')
                if re.search(r'\[' + re.escape(endpoint) + r'\]\ntype=endpoint', pjsip):
                    return True, ' Endpoint: ' + endpoint + '/fixture Not in use'
                return True, 'Unable to find object'
            return True, 'Reloaded'
        self.runner = patch.object(self.app, 'run_asterisk', side_effect=cli)
        self.run = self.runner.start()
        self.addCleanup(self.runner.stop)
        self.app.render_asterisk()

    def payload(self, node='TBS-TEST'):
        return {'revision': self.app.config_revision(), 'node_id': node, 'aliases': ['Test-Alias'],
                'provisioning': {'central_host': '10.0.1.125', 'central_port': 5060},
                'fallback_pbx': {'host': '10.0.1.160', 'port': 5060, 'username': 'dedicated-tbs',
                                 'auth_username': 'dedicated-auth', 'password': 'private-pbx-secret'}}

    def create(self):
        self.app.change_station('create', self.payload())
        return self.app.config.tbs[-1]


class StationTests(StationFixture, unittest.TestCase):
    def test_create_durable_generated_secret_reload_and_no_shared_trunk_reuse(self):
        item = self.create()
        self.assertGreaterEqual(len(item['password']), 32)
        saved = tomllib.loads(self.path.read_text(encoding='utf-8'))
        self.assertEqual(saved, self.app.config.raw)
        self.assertTrue(saved['custom']['keep'])
        self.assertRegex(self.path.read_text(encoding='utf-8'), r'(?m)^\[server\]\nbind = ')
        self.assertTrue(self.path.with_suffix('.toml.bak').exists())
        self.assertIn('core reload', [call.args[0] for call in self.run.call_args_list])
        pjsip = (self.root / 'asterisk/netcore-pjsip.conf').read_text(encoding='utf-8')
        self.assertIn('[tbs-tbs-test]', pjsip)
        self.assertIn('password=' + item['password'], pjsip)
        exported = validate_sip_config(self.app.station_export(item), 'TBS-TEST')
        self.assertEqual(exported['fallback_pbx']['username'], 'dedicated-tbs')
        self.assertEqual(exported['fallback_pbx']['password'], 'private-pbx-secret')
        self.assertNotIn('shared-trunk-secret', self.app.station_export(item))
        if os.name == 'posix':
            self.assertEqual(self.path.stat().st_mode & 0o777, 0o600)
            self.assertEqual(self.path.with_suffix('.toml.bak').stat().st_mode & 0o777, 0o600)

    def test_update_rename_preserves_passwords_settings_and_other_stations(self):
        item = copy.deepcopy(self.create())
        self.app.change_station('update', {'revision': self.app.config_revision(), 'node_id': 'TBS-RENAMED',
            'aliases': ['New-Alias'], 'max_contacts': 2, 'password': '', 'fallback_pbx': {'password': ''}}, 'TBS-TEST')
        changed = self.app.config.tbs[-1]
        self.assertEqual(changed['password'], item['password'])
        self.assertEqual(changed['fallback_pbx'], item['fallback_pbx'])
        self.assertEqual(changed['max_contacts'], 2)
        self.assertEqual(self.app.config.tbs[0]['node_id'], 'SRV-M-TBS-01')
        self.assertEqual(validate_sip_config(self.app.station_export(changed), 'TBS-RENAMED')['service']['node_id'], 'TBS-RENAMED')

    def test_delete_removes_pjsip_endpoint_and_survives_new_instance(self):
        self.create()
        self.app.change_station('delete', {'revision': self.app.config_revision()}, 'TBS-TEST')
        self.assertNotIn('[tbs-tbs-test]', (self.root / 'asterisk/netcore-pjsip.conf').read_text(encoding='utf-8'))
        restarted = SipSwitch(Config(tomllib.loads(self.path.read_text(encoding='utf-8')), self.path))
        self.addCleanup(restarted.io.close)
        self.assertEqual([v['node_id'] for v in restarted.config.tbs], ['SRV-M-TBS-01'])

    def test_duplicates_aliases_user_and_rename_collisions_are_rejected(self):
        self.create()
        for update in ({'node_id': 'TBS-TEST'}, {'aliases': ['Test-Alias']}, {'username': 'tbs-tbs-test'},
                       {'endpoint_id': 'netcore-pbx'}, {'endpoint_id': 'tbs-tbs-test-auth'}):
            data = self.payload('TBS-SECOND')
            data['aliases'] = []
            data.update(update)
            with self.subTest(update=update), self.assertRaises(StationError):
                self.app.change_station('create', data)
        with self.assertRaises(StationError):
            self.app.change_station('update', {'revision': self.app.config_revision(), 'node_id': 'SRV-M-TBS-01'}, 'TBS-TEST')

    def test_stale_revision_and_external_file_edit_do_not_overwrite(self):
        stale = self.payload()
        self.create()
        with self.assertRaisesRegex(StationError, 'inzwischen'):
            self.app.change_station('update', stale, 'TBS-TEST')
        external = tomllib.loads(self.path.read_text(encoding='utf-8'))
        external['routing']['dial_timeout_secs'] = 80
        self.path.write_text(config_toml(external), encoding='utf-8')
        with self.assertRaisesRegex(StationError, 'außerhalb'):
            self.app.change_station('delete', {'revision': self.app.config_revision()}, 'TBS-TEST')
        self.assertEqual(tomllib.loads(self.path.read_text(encoding='utf-8')), external)

    def test_active_calls_block_edit_and_delete_until_terminal(self):
        self.create()
        self.app.calls['fixture'] = {'state': 'answered', 'node_id': 'TBS-TEST'}
        for action, body in [('delete', {'revision': self.app.config_revision()}), ('update', self.payload())]:
            with self.subTest(action=action), self.assertRaisesRegex(StationError, 'aktive Rufe'):
                self.app.change_station(action, body, 'TBS-TEST')
        self.app.calls['fixture']['state'] = 'ended'
        self.app.change_station('delete', {'revision': self.app.config_revision()}, 'TBS-TEST')

    def test_reload_failure_rolls_back_disk_memory_and_reloads_original(self):
        before = {p: p.read_bytes() for p in self.root.rglob('*.conf')}
        before[self.path] = self.path.read_bytes()
        revision = self.app.config_revision()
        self.run.side_effect = [(False, 'fixture failure'), (True, 'restored')]
        with self.assertRaisesRegex(StationError, 'wiederhergestellt'):
            self.app.change_station('create', self.payload())
        self.assertEqual(self.app.config_revision(), revision)
        for path, data in before.items():
            self.assertEqual(path.read_bytes(), data)
        self.assertEqual(self.run.call_args_list[-2].args, ('core reload',))
        self.assertEqual(self.run.call_args_list[-1].args, ('core reload',))

    def test_partial_write_failure_restores_every_include_without_reload(self):
        import netcore_sip_switch as module
        original = module.write_asterisk_config
        before = {p: p.read_bytes() for p in self.root.rglob('*.conf')}
        before[self.path] = self.path.read_bytes()
        calls = 0
        def fail_once(path, content):
            nonlocal calls
            calls += 1
            if calls == 2:
                raise OSError('fixture disk full')
            return original(path, content)
        self.run.reset_mock()
        with patch.object(module, 'write_asterisk_config', side_effect=fail_once), self.assertRaises(StationError):
            self.app.change_station('create', self.payload())
        for path, data in before.items():
            self.assertEqual(path.read_bytes(), data)
        self.run.assert_not_called()

    def test_successful_cli_exit_without_loaded_endpoint_is_rolled_back(self):
        revision = self.app.config_revision()
        self.run.side_effect = None
        self.run.return_value = True, 'Unable to find object'
        with self.assertRaises(StationError):
            self.app.change_station('create', self.payload())
        self.assertEqual(self.app.config_revision(), revision)

    def test_truncated_endpoint_column_is_accepted_after_exact_id_lookup(self):
        data = self.payload()
        data['endpoint_id'] = 'tbs-' + 'long-' * 18
        def cli(command, **kwargs):
            if command.startswith('pjsip show endpoint '):
                return True, ' Endpoint: <Endpoint/CID> <State>\n Endpoint: ' + data['endpoint_id'][:40] + ' Not in use'
            return True, 'Reloaded'
        self.run.side_effect = cli
        self.app.change_station('create', data)
        self.assertIn('pjsip show endpoint ' + data['endpoint_id'], [call.args[0] for call in self.run.call_args_list])

    def test_injection_wrong_types_and_unknown_fields_are_rejected_before_writes(self):
        revision = self.app.config_revision()
        for values in ({'node_id': '../bad'}, {'node_id': ''}, {'password': 'abc;injected'}, {'password': 123},
                       {'max_contacts': True}, {'enabled': 'false'}, {'aliases': ['bad\nline']},
                       {'fallback_pbx': {'host': 'localhost'}}, {'provisioning': {'central_port': True}},
                       {'fallback_pbx': {'password': ['bad']}}, {'server': {'bind': 'elsewhere'}}):
            data = self.payload()
            data.update(values)
            with self.subTest(values=values), self.assertRaises(StationError):
                self.app.change_station('create', data)
            self.assertEqual(self.app.config_revision(), revision)

    def test_incomplete_or_disabled_station_has_actionable_export_error(self):
        self.app.change_station('create', {'revision': self.app.config_revision(), 'node_id': 'TBS-TEST'})
        row = self.app.station_list()[-1]
        self.assertFalse(row['export_ready'])
        self.assertIn('SIP-Switch-Adresse', row['export_error'])
        self.app.change_station('update', self.payload(), 'TBS-TEST')
        self.assertTrue(self.app.station_list()[-1]['export_ready'])
        self.app.change_station('update', {'revision': self.app.config_revision(), 'enabled': False}, 'TBS-TEST')
        self.assertFalse(self.app.station_list()[-1]['export_ready'])

    def test_management_list_and_audit_do_not_expose_secrets(self):
        item = self.create()
        encoded = json.dumps(self.app.station_list())
        self.assertNotIn(item['password'], encoded)
        self.assertNotIn('private-pbx-secret', encoded)
        self.assertTrue(self.app.station_list()[-1]['has_password'])
        self.assertTrue(self.app.station_list()[-1]['fallback_pbx']['has_password'])
        self.app.io.close()
        logs = self.app.audit_path.read_text(encoding='utf-8') + self.app.event_path.read_text(encoding='utf-8')
        self.assertNotIn(item['password'], logs)
        self.assertNotIn('private-pbx-secret', logs)

    @unittest.skipUnless(os.name == 'posix', 'real image renderer requires Linux fcntl and directory fsync')
    def test_export_is_personalized_by_actual_deploy_importer(self):
        item = self.create()
        text = self.app.station_export(item)
        image = self.root / 'image'
        script = image / 'usr/local/bin/netcore-tbs-sip-fallback'
        script.parent.mkdir(parents=True)
        shutil.copyfile(BASE / 'tbs-fallback/src/netcore_tbs_sip_fallback.py', script)
        runner = Mock()
        native = configure_local_sip(text, 'TBS-TEST', root=image, runner=runner)
        self.assertEqual(native['remote_host'], '127.0.0.1')
        self.assertEqual(native['remote_port'], 5060)
        rendered = (image / 'etc/asterisk/netcore-tbs-fallback-pjsip.conf').read_text(encoding='utf-8')
        self.assertIn('password=' + item['password'], rendered)
        self.assertIn('password=private-pbx-secret', rendered)
        active = (image / 'etc/asterisk/netcore-active-registration.conf').read_text(encoding='utf-8')
        self.assertIn('netcore-registration-central.conf', active)
        self.assertNotIn('pbx-direct', active)
        with self.assertRaisesRegex(ValueError, 'Stationskennung passt nicht'):
            validate_sip_config(text, 'ANOTHER-TBS')


@unittest.skipUnless(os.name == 'posix', 'native HTTP tests run in Linux CI')
class StationHttpTests(StationFixture, unittest.TestCase):
    def setUp(self):
        super().setUp()
        self.server = BoundedHTTPServer(('127.0.0.1', 0), Handler)
        self.server.app = self.app
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        self.addCleanup(self.server.server_close)
        self.addCleanup(self.server.shutdown)
        self.url = 'http://127.0.0.1:' + str(self.server.server_port)
        self.opener = build_opener(ProxyHandler({}))

    def request(self, method, path, body=None, headers=None):
        req = Request(self.url + path, data=None if body is None else json.dumps(body).encode(), method=method,
                      headers={'Content-Type': 'application/json', **(headers or {})})
        try:
            response = self.opener.open(req, timeout=5)
        except HTTPError as error:
            response = error
        with response:
            return response.status, response.headers, response.read()

    def test_http_crud_attachment_import_and_secrets(self):
        status, _, response = self.request('POST', '/api/v1/tbs', self.payload())
        self.assertEqual(status, 201, response)
        status, headers, text = self.request('GET', '/api/v1/tbs/TBS-TEST/config.toml')
        self.assertEqual(status, 200)
        self.assertIn('attachment;', headers['Content-Disposition'])
        self.assertEqual(headers['Cache-Control'], 'no-store')
        self.assertNotIn('Access-Control-Allow-Origin', headers)
        exported = validate_sip_config(text.decode(), 'TBS-TEST')
        self.assertEqual(exported['fallback_pbx']['username'], 'dedicated-tbs')
        for path in ('/api/v1/tbs', '/api/v1/config'):
            status, _, body = self.request('GET', path)
            self.assertEqual(status, 200)
            self.assertNotIn(exported['central']['password'].encode(), body)
            self.assertNotIn(b'private-pbx-secret', body)
            self.assertNotIn(b'shared-trunk-secret', body)
        body = {'revision': self.app.config_revision(), 'aliases': ['Changed']}
        self.assertEqual(self.request('PUT', '/api/v1/tbs/TBS-TEST', body)[0], 200)
        self.assertEqual(self.request('DELETE', '/api/v1/tbs/TBS-TEST', {'revision': self.app.config_revision()})[0], 200)
        self.assertEqual(self.request('GET', '/api/v1/tbs/TBS-TEST/config.toml')[0], 404)

    def test_http_conflict_wrong_origin_and_bad_json_have_clear_status(self):
        self.assertEqual(self.request('POST', '/api/v1/tbs', self.payload(), {'Origin': 'http://elsewhere.test'})[0], 403)
        body = self.payload()
        body['revision'] = 'stale'
        self.assertEqual(self.request('POST', '/api/v1/tbs', body)[0], 409)
        self.assertEqual(self.request('PUT', '/api/v1/tbs/absent', self.payload())[0], 404)
        self.assertEqual(self.request('POST', '/api/v1/tbs', self.payload(), {'Content-Type': 'text/plain'})[0], 415)
        request = Request(self.url + '/api/v1/tbs', data=b'{"password":"must-stay-private',
                          headers={'Content-Type': 'application/json'})
        with self.assertRaises(HTTPError) as error:
            self.opener.open(request, timeout=5)
        with error.exception as response:
            self.assertEqual(response.code, 400)
            self.assertNotIn(b'must-stay-private', response.read())


if __name__ == '__main__':
    if '--serve' in sys.argv:
        fixture = StationTests('test_create_durable_generated_secret_reload_and_no_shared_trunk_reuse')
        fixture.setUp()
        server = BoundedHTTPServer(('127.0.0.1', 0), Handler)
        server.app = fixture.app
        print('FIXTURE http://127.0.0.1:' + str(server.server_port), flush=True)
        try:
            server.serve_forever()
        finally:
            server.server_close()
            fixture.doCleanups()
    else:
        unittest.main(verbosity=2)
