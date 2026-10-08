"""Check the DNS wrapper and real file swaps with isolated systemd/HTTP doubles."""
import contextlib
import hashlib
import importlib.util
import io
import json
from pathlib import Path
import sqlite3
import stat
import tempfile
import unittest
from unittest import mock

REPO = Path(__file__).resolve().parents[2]
WRAPPER = REPO / 'Docs/integration/Z01-2026-10-07/vm119-image-dns-update.py'


def load_wrapper():
    spec = importlib.util.spec_from_file_location('vm119_image_dns', WRAPPER)
    wrapper = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(wrapper)
    return wrapper


class ImageDnsUpdateTests(unittest.TestCase):
    def test_wrapper_selects_only_image_builder_and_reviewed_source(self):
        driver = load_wrapper().UPDATER
        self.assertEqual(driver.TARGET, Path('/usr/local/lib/netcore-deployment/image_build.py'))
        self.assertEqual(driver.SOURCE, REPO / 'system-backend/deployment-core/image_build.py')
        self.assertRegex(driver.OLD_SHA, r'^[0-9a-f]{64}$')
        self.assertEqual(hashlib.sha256(driver.SOURCE.read_bytes()).hexdigest(), driver.NEW_SHA)

    def test_inherited_host_guard_refuses_wrong_machine_without_side_effects(self):
        driver = load_wrapper().UPDATER
        with mock.patch.object(driver.socket, 'gethostname', return_value='wrong-host'), \
                mock.patch.object(driver, 'command') as command:
            with self.assertRaises(RuntimeError):
                driver.main()
            command.assert_not_called()

    def exercise(self, kind):
        driver = load_wrapper().UPDATER
        old = b'VALUE = "old builder"\n'
        new = driver.SOURCE.read_bytes()
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            state, build = base / 'controller', base / 'builder'
            for directory in (state, build):
                directory.mkdir()
                with sqlite3.connect(directory / 'jobs.sqlite3') as db:
                    db.execute('CREATE TABLE jobs(id TEXT,status TEXT,request TEXT,result TEXT)')
            target = base / 'image_build.py'
            target.write_bytes(old)
            target.chmod(0o640)
            metadata = target.stat()
            protected = [base / name for name in ('deployment.toml', 'settings.json', 'profiles.json', 'template.toml')]
            for path in protected:
                path.write_text('preserve ' + path.name)
            expected_protected = {path: path.read_bytes() for path in protected}
            driver.TARGET, driver.BUILD_STATE = target, build
            driver.OLD_SHA = hashlib.sha256(old).hexdigest()
            operations = []

            def command(*args):
                if args[1] in ('start', 'stop'):
                    operations.append(args[1:])

            def api(path):
                if path == '/health/ready':
                    return {'ready': not (kind == 'rollback' and target.read_bytes() == new)}
                return []

            busy = [{'id': 'busy-image', 'status': 'running', 'request': {'kind': 'image'}, 'result': {}}]
            status = {'available': True, 'jobs': busy if kind == 'busy' else []}
            output = io.StringIO()
            with mock.patch.object(driver, 'command', command), mock.patch.object(driver, 'api', api), \
                    mock.patch.object(driver, 'builder_status', return_value=status), \
                    mock.patch.object(driver.time, 'monotonic', side_effect=[0, 31]), \
                    contextlib.redirect_stdout(output), contextlib.redirect_stderr(io.StringIO()):
                if kind in ('rollback', 'busy'):
                    with self.assertRaises(RuntimeError):
                        driver.update(state, protected)
                else:
                    driver.update(state, protected)
            self.assertEqual(target.read_bytes(), new if kind == 'success' else old)
            after = target.stat()
            self.assertEqual((after.st_uid, after.st_gid, stat.S_IMODE(after.st_mode)),
                             (metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)))
            self.assertEqual({path: path.read_bytes() for path in protected}, expected_protected)
            self.assertEqual(list(base.glob('.jobs-update-*')), [])
            if kind == 'busy':
                self.assertEqual(operations, [])
            else:
                self.assertEqual(operations[:4], [('stop', driver.CONTROLLER), ('stop', driver.BUILDER),
                                                 ('start', driver.BUILDER), ('start', driver.CONTROLLER)])
                if kind == 'rollback':
                    self.assertEqual(operations[4:], operations[:4])
                else:
                    report = json.loads(output.getvalue())
                    self.assertEqual(report['file'], str(target))
                    self.assertEqual(report['sha256_after'], driver.NEW_SHA)

    def test_real_source_swap_preserves_files_and_reports_correct_target(self):
        self.exercise('success')

    def test_real_source_rollback_returns_original_builder_bytes(self):
        self.exercise('rollback')

    def test_active_image_job_stops_without_swap_or_service_stop(self):
        self.exercise('busy')


if __name__ == '__main__':
    unittest.main()
