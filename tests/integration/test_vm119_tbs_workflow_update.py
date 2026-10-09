"""Seven real runtime files/SQLite databases; HTTP and systemd are isolated."""
import contextlib
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

REPO = Path(__file__).resolve().parents[2]
DRIVER = REPO / 'Docs/integration/Z01-2026-10-07/vm119-tbs-workflow-update.py'


def load_driver():
    spec = importlib.util.spec_from_file_location('vm119_tbs_workflow', DRIVER)
    driver = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(driver)
    return driver


class WorkflowUpdateTests(unittest.TestCase):
    def test_exact_seven_source_pins_and_both_historical_jobs_baselines(self):
        driver = load_driver()
        self.assertEqual(set(driver.PINS), {'main.py', 'deploy.py', 'jobs.py', 'image_worker.py',
                                          'static/index.html', 'static/app.js', 'static/style.css'})
        self.assertEqual(set(driver.PINS['jobs.py']['old']), {
            '30ce8b0f936383f9c20ddb5ffda244c4065b1de1a567ddf3fbda360058d17e64',
            '905e5fc4d257e5d1ad56542cbaef38d2f42a00af357240c0e5c92ea7facc2c21'})
        for name, pins in driver.PINS.items():
            self.assertEqual(driver.digest((driver.SOURCE / name).read_bytes()), pins['new'])

    def exercise(self, kind):
        driver = load_driver()
        guard = driver.GUARD
        with tempfile.TemporaryDirectory() as temporary:
            root = Path(temporary)
            library, source, state, build = (root / name for name in ('installed', 'source', 'controller', 'builder'))
            for path in (library, source, state, build):
                path.mkdir()
            (library / 'static').mkdir()
            (source / 'static').mkdir()
            driver.LIBRARY, driver.SOURCE, guard.BUILD_STATE = library, source, build
            old, new, metadata, pins = {}, {}, {}, {}
            for index, name in enumerate(driver.PINS):
                old[name] = (f'VALUE = "old {name}"\n' if name.endswith('.py') else f'old {name}\n').encode()
                new[name] = (f'VALUE = "new {name}"\n' if name.endswith('.py') else f'new {name}\n').encode()
                target = library / name
                target.write_bytes(new[name] if kind == 'already_current' else old[name])
                target.chmod(0o640 if index % 2 else 0o644)
                metadata[name] = target.stat()
                (source / name).write_bytes(new[name])
                pins[name] = {'old': (driver.digest(old[name]),), 'new': driver.digest(new[name])}
            if kind == 'second_jobs_baseline':
                first = old['jobs.py']
                old['jobs.py'] = b'VALUE = "previous SQLite fix"\n'
                (library / 'jobs.py').write_bytes(old['jobs.py'])
                pins['jobs.py']['old'] = (driver.digest(first), driver.digest(old['jobs.py']))
            driver.PINS = pins
            for directory in (state, build):
                with sqlite3.connect(directory / 'jobs.sqlite3') as db:
                    db.execute('CREATE TABLE jobs(id TEXT,status TEXT,request TEXT,result TEXT)')
                    # Busy rows are beyond both HTTP list limits; SQL must inspect all rows.
                    for index in range(150):
                        db.execute('INSERT INTO jobs VALUES(?,?,?,?)', (str(index), 'succeeded', '{}', '{}'))
            protected = [root / 'deployment.toml', state / 'settings.json', state / 'profiles.json', state / 'tbs-site-template.toml']
            for path in protected:
                path.write_text('preserve ' + path.name)
            protected_before = {path: path.read_bytes() for path in protected}
            operations, replacements = [], []
            active = {guard.CONTROLLER: True, guard.BUILDER: True}
            injected = {'replace': False, 'busy': False, 'check': False}

            def insert(directory, name, status='running', uncertain=False, request_kind='install'):
                with sqlite3.connect(directory / 'jobs.sqlite3') as db:
                    db.execute('INSERT INTO jobs VALUES(?,?,?,?)',
                               (name, status, json.dumps({'kind': request_kind}), json.dumps({'remote_uncertain': uncertain})))

            if kind in ('controller_busy', 'builder_busy', 'hidden_uncertain'):
                directory = build if kind == 'builder_busy' else state
                insert(directory, 'hidden-old-record', 'failed' if kind == 'hidden_uncertain' else 'queued',
                       uncertain=kind == 'hidden_uncertain')
            if kind == 'wrong_target':
                (library / 'static/app.js').write_bytes(b'unreviewed local edit')
            if kind == 'wrong_source':
                (source / 'static/app.js').write_bytes(b'unreviewed source edit')
            if kind == 'source_syntax':
                body = b'if (\n'
                (source / 'jobs.py').write_bytes(body)
                pins['jobs.py']['new'] = driver.digest(body)

            def command(*args):
                if args[1] == 'is-active':
                    self.assertTrue(all(active[name] for name in args[3:]))
                    return
                action, unit = args[1:]
                operations.append((action, unit))
                active[unit] = action == 'start'
                if kind == 'builder_becomes_busy' and action == 'stop' and unit == guard.CONTROLLER:
                    insert(build, 'race-image-job', request_kind='image')
                    injected['busy'] = True
                if kind in ('startup_check', 'ready_failure_with_check') and action == 'start' and unit == guard.CONTROLLER:
                    if not injected['check'] and (library / 'jobs.py').read_bytes() == new['jobs.py']:
                        insert(state, 'startup-read-only-check', request_kind='check')
                        injected['check'] = True

            def api(path):
                if path == '/health/ready':
                    post_update = (library / 'jobs.py').read_bytes() == new['jobs.py']
                    return {'ready': not (kind in ('ready_failure', 'ready_failure_with_check') and post_update)}
                return []

            def builder_status():
                jobs = [{'id': 'race-image-job', 'status': 'running', 'request': {'kind': 'image'}, 'result': {}}] \
                    if injected['busy'] else []
                return {'available': True, 'jobs': jobs}

            original_replace = driver.replace

            def replace(path, body, info):
                self.assertFalse(any(active.values()), 'Both units must be stopped during every replacement.')
                replacements.append(str(path.relative_to(library)))
                original_replace(path, body, info)
                if kind == 'partial_swap_failure' and len(replacements) == 3 and not injected['replace']:
                    injected['replace'] = True
                    raise OSError('Simulated fsync failure after the third successful rename')

            output, error = io.StringIO(), io.StringIO()
            success = kind in ('success', 'second_jobs_baseline', 'startup_check', 'already_current')
            with mock.patch.object(guard, 'command', command), mock.patch.object(guard, 'api', api), \
                    mock.patch.object(guard, 'builder_status', builder_status), mock.patch.object(driver, 'replace', replace), \
                    mock.patch.object(driver.time, 'monotonic', side_effect=[0, 31]), \
                    contextlib.redirect_stdout(output), contextlib.redirect_stderr(error):
                if success:
                    driver.update(state, protected)
                else:
                    with self.assertRaises((RuntimeError, OSError, SyntaxError)):
                        driver.update(state, protected)
            self.assertEqual({path: path.read_bytes() for path in protected}, protected_before)
            for name in pins:
                expected = new[name] if success else old[name]
                if kind == 'wrong_target' and name == 'static/app.js':
                    expected = b'unreviewed local edit'
                self.assertEqual((library / name).read_bytes(), expected)
                after, before = (library / name).stat(), metadata[name]
                self.assertEqual((after.st_uid, after.st_gid, stat.S_IMODE(after.st_mode)),
                                 (before.st_uid, before.st_gid, stat.S_IMODE(before.st_mode)))
            self.assertTrue(all(active.values()))
            self.assertEqual(list(library.rglob('.tbs-workflow-*')), [])
            if kind in ('controller_busy', 'builder_busy', 'hidden_uncertain', 'wrong_target', 'wrong_source',
                        'source_syntax', 'already_current'):
                self.assertEqual(operations, [])
                self.assertEqual(replacements, [])
            elif kind == 'builder_becomes_busy':
                self.assertEqual(operations, [('stop', guard.CONTROLLER), ('start', guard.BUILDER), ('start', guard.CONTROLLER)])
                self.assertEqual(replacements, [])
            elif not success:
                self.assertEqual(replacements[-7:], list(pins))
                self.assertIn('Rückweg:', error.getvalue())
            if success and kind != 'already_current':
                report = json.loads(output.getvalue())
                self.assertEqual(report['phase'], 'passed')
                self.assertEqual(len(report['files']), 7)
                self.assertEqual([row['sha256_after'] for row in report['files']], [pins[name]['new'] for name in pins])

    def test_seven_file_success_and_second_jobs_baseline(self):
        for kind in ('success', 'second_jobs_baseline'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_partial_swap_and_readiness_failures_restore_all_seven_originals(self):
        for kind in ('partial_swap_failure', 'ready_failure', 'ready_failure_with_check'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_all_sql_rows_reject_hidden_busy_and_remote_uncertain_work(self):
        for kind in ('controller_busy', 'builder_busy', 'hidden_uncertain'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_worker_becoming_busy_is_never_stopped(self):
        self.exercise('builder_becomes_busy')

    def test_unreviewed_targets_sources_and_bad_python_refused_without_stops(self):
        for kind in ('wrong_target', 'wrong_source', 'source_syntax'):
            with self.subTest(kind=kind):
                self.exercise(kind)

    def test_automatic_read_only_check_allowed_after_successful_restart(self):
        self.exercise('startup_check')

    def test_already_current_is_idempotent_without_restarts(self):
        self.exercise('already_current')

    def test_host_guard_refuses_wrong_host_without_reading_runtime(self):
        driver = load_driver()
        with mock.patch.object(driver.sys, 'argv', [str(DRIVER)]), mock.patch.object(driver.os, 'geteuid', return_value=0), \
                mock.patch.object(driver.GUARD.socket, 'gethostname', return_value='wrong-host'), \
                mock.patch.object(driver.GUARD, 'command') as command:
            with self.assertRaises(RuntimeError):
                driver.main()
            command.assert_not_called()


if __name__ == '__main__':
    unittest.main()
