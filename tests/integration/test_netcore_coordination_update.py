"""Runtime receipts in the agent API and mutual exclusion with UI deployments."""
import importlib.util
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
import types
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[2]
CORE = ROOT / 'system-backend/deployment-core'
if not hasattr(os, 'geteuid'):
    sys.modules.setdefault('fcntl', types.SimpleNamespace(
        LOCK_EX=2, LOCK_NB=4, LOCK_UN=8, flock=lambda *args: None))
sys.path.insert(0, str(CORE))
try:
    import deploy
    spec = importlib.util.spec_from_file_location('netcore_receipt_agent', CORE / 'main.py')
    agent = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(agent)
finally:
    sys.path.remove(str(CORE))


class AgentReceiptTests(unittest.TestCase):
    def fixture(self, state, role='agent'):
        app = object.__new__(agent.App)
        app.state = state
        app.cfg = dict(role=role, environment='fixture', node_id='fixture-node')
        app.local = []
        app.lock = threading.Lock()
        return app

    def test_agent_advertises_successful_self_source_commit(self):
        with tempfile.TemporaryDirectory() as work:
            state = Path(work)
            (state / 'deployed-discovery.json').write_text(json.dumps(
                dict(state='installed', commit='a' * 40)))
            result = self.fixture(state).manifest()
        self.assertEqual('a' * 40, result['source_commit'])
        self.assertEqual([], result['services'])

    def test_installing_failed_and_missing_receipts_never_advertise_old_commit(self):
        with tempfile.TemporaryDirectory() as work:
            state = Path(work)
            app = self.fixture(state)
            self.assertEqual('', app.manifest()['source_commit'])
            for phase in ('installing', 'failed', 'rolled_back'):
                with self.subTest(phase=phase):
                    (state / 'deployed-discovery.json').write_text(json.dumps(
                        dict(state=phase, commit='a' * 40)))
                    self.assertEqual('', app.manifest()['source_commit'])

    def test_controller_does_not_claim_the_agent_receipt_as_its_own_version(self):
        with tempfile.TemporaryDirectory() as work:
            state = Path(work)
            (state / 'deployed-discovery.json').write_text(json.dumps(
                dict(state='installed', commit='a' * 40)))
            self.assertEqual('', self.fixture(state, 'controller').manifest()['source_commit'])


@unittest.skipUnless(hasattr(os, 'geteuid'), 'Actual flock requires POSIX')
class DeploymentMutexTests(unittest.TestCase):
    def test_manual_runtime_mutex_prevents_a_ui_installer_from_starting(self):
        with tempfile.TemporaryDirectory() as work:
            service = deploy.Deployer(dict(state_dir=work), Mock(), Mock())
            with (Path(work) / 'runtime-update.mutex').open('a') as held:
                deploy.fcntl.flock(held, deploy.fcntl.LOCK_EX | deploy.fcntl.LOCK_NB)
                with patch.object(service, '_execute') as installer:
                    with self.assertRaisesRegex(RuntimeError, 'Runtime-Update'):
                        service.execute(dict(service='packet-core'), Mock())
                    installer.assert_not_called()

    def test_ui_execution_releases_mutex_even_on_failed_install(self):
        with tempfile.TemporaryDirectory() as work:
            service = deploy.Deployer(dict(state_dir=work), Mock(), Mock())
            with patch.object(service, '_execute', side_effect=RuntimeError('failed fixture')):
                with self.assertRaisesRegex(RuntimeError, 'failed fixture'):
                    service.execute(dict(service='packet-core'), Mock())
            with (Path(work) / 'runtime-update.mutex').open('a') as retry:
                deploy.fcntl.flock(retry, deploy.fcntl.LOCK_EX | deploy.fcntl.LOCK_NB)


if __name__ == '__main__':
    unittest.main()
