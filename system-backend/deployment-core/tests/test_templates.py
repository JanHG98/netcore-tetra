"""New site templates: RF consistency, durable create and the real controller image path."""
from concurrent.futures import ThreadPoolExecutor
import json
from pathlib import Path
import sys
import tempfile
import threading
import tomllib
import unittest
from unittest.mock import Mock
from urllib.error import HTTPError
from urllib.request import Request, urlopen

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import ROOT, load_config, toml_dump
from deploy import new_tbs_template, tbs_config
from main import App, Server, handler
from test_images import image_request

PROFILE = dict(name='TBS-CREATE', mcc=901, mnc=1510, issi=4010007, la=3, cc=7)


class TemplateTests(unittest.TestCase):
    def test_dual_carrier_matches_cell_and_contains_no_connector_credentials(self):
        config = new_tbs_template({})
        radio = config['phy_io']['soapysdr']
        self.assertEqual((radio['tx_freq'], radio['rx_freq']), (418000000, 408000000))
        self.assertEqual((radio['tx_center_freq'], radio['rx_center_freq']), (418012500, 408012500))
        self.assertEqual(config['cell_info']['secondary_carrier'], 721)
        self.assertEqual(tomllib.loads(toml_dump(config)), config)
        self.assertNotIn('brew', config)
        self.assertNotIn('asterisk', config)
        self.assertNotIn('password', toml_dump(config))

    def test_single_carrier_reverse_offset_custom_spacing_and_device(self):
        config = new_tbs_template(dict(secondary_carrier=None, reverse_operation=True,
            freq_offset=6250, custom_duplex_spacing=7600000, device='driver=fixture,label="test"'))
        radio = config['phy_io']['soapysdr']
        self.assertNotIn('secondary_carrier', config['cell_info'])
        self.assertEqual(radio['tx_freq'], 418006250)
        self.assertEqual(radio['rx_freq'], 425606250)
        self.assertEqual(radio['tx_center_freq'], radio['tx_freq'])
        self.assertEqual(tomllib.loads(toml_dump(config))['phy_io']['soapysdr']['device'], 'driver=fixture,label="test"')

    def test_invalid_settings_rejected_before_storage(self):
        for value in ([], {'unknown': 1}, {'main_carrier': True}, {'main_carrier': 4000},
                {'secondary_carrier': 720}, {'secondary_carrier': False}, {'sample_rate': 48000},
                {'sample_rate': float('nan')}, {'freq_band': 9}, {'freq_offset': 1},
                {'duplex_spacing': 7}, {'custom_duplex_spacing': -1},
                {'reverse_operation': 'false'}, {'device': 'driver=test\n[dashboard]'}):
            with self.subTest(value=value), self.assertRaises(ValueError):
                new_tbs_template(value)

    def test_profile_overrides_identifiers_and_discovery_resolves_gateway(self):
        config = new_tbs_template({}, {'node-gateway': {'url': 'http://10.0.1.179:8080'}})
        rendered = tomllib.loads(tbs_config(toml_dump(config), PROFILE))
        self.assertEqual(rendered['control_room']['host'], '10.0.1.179')
        self.assertEqual(rendered['control_room']['node_id'], PROFILE['name'])
        self.assertEqual(rendered['net_info'], dict(mcc=901, mnc=1510))
        self.assertEqual(rendered['cell_info']['location_area'], 3)
        self.assertEqual(rendered['cell_info']['colour_code'], 7)
        self.assertEqual(rendered['phy_io'], config['phy_io'])


class TemplateHttpTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        cfg = load_config(ROOT / 'config/deployment.example.toml')
        cfg.update(state_dir=self.temp.name, advertise_url='http://10.0.1.50:8320')
        self.app = App(cfg)
        self.addCleanup(self.app.discovery.close)
        self.addCleanup(self.app.stop.set)
        self.server = Server(('127.0.0.1', 0), handler(self.app))
        self.addCleanup(self.server.server_close)
        self.addCleanup(self.server.shutdown)
        threading.Thread(target=self.server.serve_forever, daemon=True).start()
        self.url = 'http://127.0.0.1:' + str(self.server.server_address[1])
        self.path = self.app.state / 'tbs-site-template.toml'

    def request(self, path, data=None, **headers):
        req = Request(self.url + path, data=None if data is None else json.dumps(data).encode(),
                      headers={'Content-Type': 'application/json', **headers})
        try:
            response = urlopen(req, timeout=5)
        except HTTPError as error:
            response = error
        with response:
            body = response.read().decode()
            try:
                body = json.loads(body)
            except ValueError:
                pass
            return response.status, body

    def test_preview_cancel_create_profile_config_and_persistence(self):
        self.assertFalse(self.request('/api/v1/status')[1]['has_template'])
        status, preview = self.request('/api/v1/template/preview', {})
        self.assertEqual(status, 200)
        self.assertEqual(preview['rf']['tx_freq'], 418000000)
        self.assertFalse(self.path.exists(), 'Preview must not persist a template')
        self.assertEqual(self.request('/api/v1/template/create', {})[0], 201)
        saved = self.path.read_text()
        self.assertEqual(tomllib.loads(saved)['phy_io']['soapysdr']['tx_freq'], 418000000)
        self.assertEqual(self.path.stat().st_mode & 0o777, 0o600)
        self.assertTrue(self.request('/api/v1/status')[1]['has_template'])
        self.assertEqual(self.request('/api/v1/profiles', PROFILE)[0], 201)
        status, text = self.request('/api/v1/profiles/TBS-CREATE/config')
        self.assertEqual(status, 200)
        self.assertEqual(tomllib.loads(text)['cell_info']['location_area'], 3)
        self.assertEqual(self.path.read_text(), saved)
        self.assertNotIn('config', self.request('/api/v1/status')[1])

    def test_existing_template_and_concurrent_create_are_not_overwritten(self):
        with ThreadPoolExecutor(max_workers=2) as pool:
            results = list(pool.map(lambda main: self.request('/api/v1/template/create',
                {'main_carrier': main, 'secondary_carrier': None}), (720, 722)))
        self.assertEqual(sorted(result[0] for result in results), [201, 409])
        saved = self.path.read_text()
        self.assertEqual(self.request('/api/v1/template/create', {})[0], 409)
        self.assertEqual(self.path.read_text(), saved)
        # The existing explicit import is still available for a replacement.
        replacement = toml_dump(new_tbs_template({'main_carrier': 724, 'secondary_carrier': None}))
        self.assertEqual(self.request('/api/v1/template', {'toml': replacement})[0], 200)
        self.assertEqual(self.path.read_text(), replacement)

    def test_invalid_origin_or_agent_request_cannot_create_template(self):
        for action in ('preview', 'create'):
            self.assertEqual(self.request('/api/v1/template/' + action, {'secondary_carrier': 720})[0], 400)
        self.assertFalse(self.path.exists())
        self.assertEqual(self.request('/api/v1/template/create', {}, Origin='http://elsewhere.invalid')[0], 400)
        self.assertFalse(self.path.exists())
        self.app.cfg['role'] = 'agent'
        self.assertEqual(self.request('/api/v1/template/create', {})[0], 404)
        self.assertFalse(self.path.exists())

    def test_created_template_reaches_validated_pinned_image_request_without_import(self):
        self.assertEqual(self.request('/api/v1/template/create', {})[0], 201)
        self.assertEqual(self.request('/api/v1/profiles', PROFILE)[0], 201)
        self.app.repo.resolve_latest_main = lambda log: 'b' * 40
        self.app.images = Mock()
        self.app.images.request.return_value = {'id': 'template-image-fixture'}
        request = image_request()
        request.pop('config')
        request['profile'] = PROFILE['name']
        status, _ = self.request('/api/v1/images/build', request)
        self.assertEqual(status, 202)
        path, validated = self.app.images.request.call_args.args
        self.assertEqual(path, '/build')
        self.assertEqual(validated['commit'], 'b' * 40)
        config = tomllib.loads(validated['config'])
        self.assertEqual(config['phy_io']['soapysdr']['rx_freq'], 408000000)
        self.assertEqual(config['control_room']['node_id'], PROFILE['name'])
        self.assertEqual(config['cell_info']['colour_code'], PROFILE['cc'])


if __name__ == '__main__':
    unittest.main(verbosity=2)
