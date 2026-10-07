"""Regressions from the OpenLab rollout on 2026-09-27."""
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import Mock, patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import deploy
import main
from common import CATALOG, ROOT, load_config


class RolloutTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.config = self.root / 'config.toml'
        self.config.write_text('service_name="tetra"\n[dashboard]\nport=8082\n')
        self.cfg = load_config(ROOT / 'config/agent.example.toml')
        self.cfg.update(state_dir=str(self.root / 'state'), services=[
            {'name': 'tbs', 'config_target': str(self.config)}])

    def properties(self, unit):
        if unit == 'tetra.service':
            return {'LoadState': 'loaded', 'FragmentPath': '/etc/systemd/system/tetra.service',
                    'ExecStart': '{ path=/opt/tetra/bluestation-bs ; argv[]=/opt/tetra/bluestation-bs '
                                 + str(self.config) + ' ; ignore_errors=no ; }'}
        # A .service.d directory alone must not count as a real unit.
        return {'LoadState': 'not-found', 'FragmentPath': '', 'ExecStart': ''}

    def test_legacy_tbs_unit_and_config_are_resolved(self):
        with patch.object(deploy, 'unit_properties', side_effect=self.properties):
            spec = deploy.service_spec(self.cfg, 'tbs')
            self.assertEqual(spec['unit'], 'tetra.service')
            self.assertEqual(spec['port'], 8082)
            self.assertTrue(deploy.installed_service(spec))

    def test_copied_config_does_not_match_another_unit_config(self):
        with patch.object(deploy, 'unit_properties', return_value={
                'LoadState': 'loaded', 'FragmentPath': '/etc/systemd/system/netcore-tbs.service',
                'ExecStart': '/opt/tetra/bluestation-bs /elsewhere/config.toml'}):
            self.assertFalse(deploy.installed_service(deploy.service_spec(self.cfg, 'tbs')))

    def test_explicit_tbs_override_is_preserved(self):
        self.cfg['services'][0]['unit'] = 'custom-radio.service'
        with patch.object(deploy, 'existing_tbs') as detect:
            self.assertEqual(deploy.service_spec(self.cfg, 'tbs')['unit'], 'custom-radio.service')
            detect.assert_not_called()

    def test_config_without_unit_is_not_advertised_or_updated(self):
        spec = dict(CATALOG['node-gateway'], config_target=str(self.config))
        app = main.App.__new__(main.App)
        app.cfg = dict(self.cfg, managed_services=['node-gateway'])
        app.state, app.lock, app.local = self.root, threading.RLock(), []
        with patch.object(main, 'service_spec', return_value=spec), \
             patch.object(deploy, 'unit_properties', return_value={'LoadState':'not-found','FragmentPath':''}), \
             patch.object(main, 'probe') as probe:
            app.refresh_local()
            self.assertEqual(app.local, [])
            probe.assert_not_called()
            worker = deploy.Deployer(self.cfg, Mock(), Mock())
            with patch.object(deploy, 'service_spec', return_value=spec), self.assertRaises(ValueError):
                worker.execute(dict(service='node-gateway', action='update', commit='a'*40), lambda _: None)
            worker.repo.checkout.assert_not_called()

    def test_host_assignment_filters_even_previously_installed_extras(self):
        app = main.App.__new__(main.App)
        app.cfg = dict(self.cfg, managed_services=['iot-gateway'])
        app.state, app.lock, app.local = self.root, threading.RLock(), []
        with patch.object(main, 'service_spec', side_effect=lambda cfg, name: CATALOG[name]) as specs, \
             patch.object(main, 'installed_service', return_value=True), \
             patch.object(main, 'probe', return_value=True):
            app.refresh_local()
            self.assertEqual([s['name'] for s in app.local], ['iot-gateway'])
            self.assertEqual(specs.call_count, 1)
        app.cfg['managed_services'] = []
        app.refresh_local()
        self.assertEqual(app.local, [])

    def test_tbs_update_preserves_execstart_and_resolves_active_binary(self):
        (self.root / 'install').mkdir()
        (self.root / 'install/update-basisstation.sh').write_text('# test fixture\n')
        repo, discovery = Mock(), Mock()
        repo.checkout.return_value = self.root
        worker = deploy.Deployer(self.cfg, discovery, repo)
        with patch.object(deploy, 'unit_properties', side_effect=self.properties), \
             patch.object(deploy, 'run') as run, \
             patch.object(deploy, 'install_dropin') as dropin, \
             patch.object(worker, 'health', return_value=True), \
             patch.dict(os.environ, {'BINARY_PATH': '/wrong/catalog/binary'}):
            result = worker.execute(dict(service='tbs', action='update', commit='a'*40), lambda _: None)
        dropin.assert_not_called()
        install_calls = [c for c in run.call_args_list if c.args[0][0] == 'bash']
        self.assertTrue(install_calls)
        for call in install_calls:
            self.assertEqual(call.kwargs['env']['UNIT'], 'tetra.service')
            self.assertEqual(call.kwargs['env']['CONFIG_PATH'], str(self.config))
            self.assertNotIn('BINARY_PATH', call.kwargs['env'])
        self.assertFalse(any(c.args[0][:2] == ['systemctl', 'restart'] for c in run.call_args_list))
        self.assertEqual(result['commit'], 'a'*40)

    def test_invalid_managed_services_are_rejected(self):
        file = self.root / 'agent.toml'
        for value in ['"iot-gateway"', '["made-up-service"]', '[1]']:
            file.write_text('managed_services=' + value + '\n')
            with self.assertRaises(ValueError):
                load_config(file)

    def test_live_without_readiness_fails_instead_of_completing_deployment(self):
        worker = deploy.Deployer(self.cfg, Mock(), Mock())
        with patch.object(deploy, 'HEALTH_TIMEOUT_SECONDS', 1), \
             patch.object(deploy.time, 'monotonic', side_effect=[0, 0, 2]), \
             patch.object(deploy.time, 'sleep'), \
             patch.object(deploy, 'probe', side_effect=[False, True]), \
             self.assertRaisesRegex(RuntimeError, 'Readiness'):
            worker.health(CATALOG['node-gateway'], lambda _: None)

    def test_readiness_can_recover_during_the_health_wait(self):
        worker = deploy.Deployer(self.cfg, Mock(), Mock())
        with patch.object(deploy, 'HEALTH_TIMEOUT_SECONDS', 1), \
             patch.object(deploy.time, 'monotonic', side_effect=[0, 0, .5]), \
             patch.object(deploy.time, 'sleep'), \
             patch.object(deploy, 'probe', side_effect=[False, True, True]):
            self.assertTrue(worker.health(CATALOG['node-gateway'], lambda _: None))

    def test_failed_readiness_keeps_commit_marker_unconfirmed_and_job_failed(self):
        from common import atomic_write, json_file
        from jobs import Jobs
        (self.root / 'install').mkdir()
        (self.root / 'install/update-basisstation.sh').write_text('# test fixture\n')
        repo, discovery = Mock(), Mock()
        repo.checkout.return_value = self.root
        worker = deploy.Deployer(self.cfg, discovery, repo)
        marker = worker.state / 'deployed-tbs.json'
        atomic_write(marker, '{"commit":"' + 'b'*40 + '"}')
        with patch.object(deploy, 'unit_properties', side_effect=self.properties), \
             patch.object(deploy, 'run'), \
             patch.object(worker, 'health', side_effect=RuntimeError('Readiness timeout')):
            jobs = Jobs(worker.state, worker.execute)
            key = jobs.submit(dict(service='tbs', action='update', commit='a'*40))['id']
            jobs.queue.join()
        self.assertEqual(jobs.get(key)['status'], 'failed')
        state = json_file(marker, {})
        self.assertEqual(state['commit'], '')
        self.assertEqual(state['previous_commit'], 'b'*40)
        self.assertEqual(state['requested_commit'], 'a'*40)
        self.assertTrue(list((worker.state / 'backups').glob('tbs-*.toml')))
        discovery.scan.assert_not_called()

    def test_iot_migration_runs_without_executable_bit(self):
        repo = ROOT.parents[1]
        installer = repo / 'system-backend/iot-gateway/install/install.sh'
        text = installer.read_text()
        start = text.index('CONFIG="${CONFIG}" EXAMPLE=')
        command = text[start:].split('\n\n', 1)[0]
        self.config.write_text('[mqtt]\nhost="10.0.1.119"\n[storage]\nstate_file="state.json"\n')
        result = subprocess.run(['bash', '-c', command], text=True, capture_output=True,
                                env=dict(os.environ, REPO_ROOT=str(repo), CONFIG=str(self.config)))
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('host="10.0.1.119"', self.config.read_text())
        self.assertIn('[home_assistant]', self.config.read_text())


if __name__ == '__main__':
    unittest.main()
