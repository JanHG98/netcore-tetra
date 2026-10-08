"""Real atomic file and SQLite fixtures; systemd/HTTP are isolated test doubles."""
import contextlib
import hashlib
import importlib.util
import io
import json
import os
from pathlib import Path
import sqlite3
import stat
import tempfile
import unittest
from unittest import mock

DRIVER = Path(__file__).resolve().parents[2] / 'Docs/integration/Z01-2026-10-07/vm119-jobs-update.py'


class JobsUpdateTests(unittest.TestCase):
    def exercise(self, kind):
        spec = importlib.util.spec_from_file_location('vm119_' + kind, DRIVER)
        driver = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(driver)
        old, new = b'VALUE = "old"\n', b'VALUE = "fixed"\n'
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            state, build = base / 'controller', base / 'builder'
            state.mkdir()
            build.mkdir()
            for directory in (state, build):
                with sqlite3.connect(directory / 'jobs.sqlite3') as db:
                    db.execute('CREATE TABLE jobs(id TEXT,status TEXT,request TEXT,result TEXT)')
            target, source = base / 'installed.py', base / 'source.py'
            target.write_bytes(new if kind == 'already_fixed' else old)
            target.chmod(0o640)
            source.write_bytes(new)
            protected = [base / name for name in ('deployment.toml', 'settings.json', 'profiles.json', 'template.toml')]
            for path in protected:
                path.write_text('unchanged ' + path.name)
            original_protected = {path: path.read_bytes() for path in protected}
            metadata = target.stat()
            driver.TARGET, driver.SOURCE, driver.BUILD_STATE = target, source, build
            driver.OLD_SHA, driver.NEW_SHA = (hashlib.sha256(body).hexdigest() for body in (old, new))
            commands = []
            active = {driver.CONTROLLER: True, driver.BUILDER: True}
            post_failure = kind in ('post_swap_failure', 'post_check', 'post_deploy')

            def command(*args):
                commands.append(args)
                action = args[1]
                if action in ('start', 'stop'):
                    active[args[2]] = action == 'start'
                elif action == 'is-active':
                    self.assertTrue(all(active[name] for name in args[3:]))

            def job(kind='install', status='running', uncertain=False):
                return {'id': 'fixture-job', 'status': status, 'request': {'kind': kind},
                        'result': {'remote_uncertain': uncertain}}

            def api(path):
                if path == '/health/ready':
                    return {'ready': not (kind == 'post_swap_failure' and target.read_bytes() == new)}
                if kind == 'remote_uncertain':
                    return [job(status='failed', uncertain=True)]
                if kind == 'active_controller':
                    return [job()]
                if kind in ('post_check', 'post_deploy') and target.read_bytes() == new:
                    return [job('check' if kind == 'post_check' else 'install')]
                return []

            def builder_status():
                between_stops = not active[driver.CONTROLLER] and active[driver.BUILDER]
                jobs = [job('image')] if kind == 'builder_became_busy' and between_stops else []
                return {'available': True, 'jobs': jobs}

            if kind in ('active_controller_database', 'active_builder_database'):
                directory = state if kind == 'active_controller_database' else build
                with sqlite3.connect(directory / 'jobs.sqlite3') as db:
                    db.execute('INSERT INTO jobs VALUES(?,?,?,?)',
                               ('database-only-job', 'queued', json.dumps({'kind': 'image'}), '{}'))
            if kind == 'wrong_target':
                target.write_bytes(b'VALUE = "unreviewed"\n')
            expect_failure = kind not in ('success', 'already_fixed', 'post_check')
            with mock.patch.object(driver, 'command', command), mock.patch.object(driver, 'api', api), \
                    mock.patch.object(driver, 'builder_status', builder_status), \
                    mock.patch.object(driver.time, 'monotonic', side_effect=[0, 31]), \
                    contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                if expect_failure:
                    with self.assertRaises(RuntimeError):
                        driver.update(state, protected)
                else:
                    driver.update(state, protected)
            self.assertEqual({path: path.read_bytes() for path in protected}, original_protected)
            after = target.stat()
            self.assertEqual((after.st_uid, after.st_gid, stat.S_IMODE(after.st_mode)),
                             (metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)))
            self.assertTrue(all(active.values()))
            self.assertEqual(list(base.glob('.jobs-update-*')), [])
            if kind in ('success', 'post_check', 'already_fixed'):
                self.assertEqual(target.read_bytes(), new)
            elif kind == 'wrong_target':
                self.assertEqual(target.read_bytes(), b'VALUE = "unreviewed"\n')
            else:
                self.assertEqual(target.read_bytes(), old)
            operations = [args[1:] for args in commands if args[1] in ('start', 'stop')]
            if kind in ('success', 'post_check'):
                self.assertEqual(operations, [('stop', driver.CONTROLLER), ('stop', driver.BUILDER),
                                             ('start', driver.BUILDER), ('start', driver.CONTROLLER)])
            elif kind == 'builder_became_busy':
                self.assertEqual(operations, [('stop', driver.CONTROLLER), ('start', driver.BUILDER),
                                             ('start', driver.CONTROLLER)])
            elif post_failure:
                self.assertEqual([op for op in operations if op[0] == 'stop'],
                                 [('stop', driver.CONTROLLER), ('stop', driver.BUILDER)] * 2)
            else:
                self.assertEqual(operations, [])

    def test_atomic_update_preserves_owner_mode_and_protected_bytes(self):
        self.exercise('success')

    def test_readiness_failure_restores_actual_file_and_both_services(self):
        self.exercise('post_swap_failure')

    def test_startup_check_allowed_but_new_deploy_stops_acceptance(self):
        for kind in ('post_check', 'post_deploy'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_preflight_rejects_busy_and_uncertain_work_without_mutation(self):
        for kind in ('active_controller', 'remote_uncertain', 'active_controller_database',
                     'active_builder_database', 'wrong_target'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_busy_builder_before_swap_restores_controller_without_stopping_builder(self):
        self.exercise('builder_became_busy')

    def test_fixed_version_is_idempotent_without_restart(self):
        self.exercise('already_fixed')


if __name__ == '__main__':
    unittest.main()
