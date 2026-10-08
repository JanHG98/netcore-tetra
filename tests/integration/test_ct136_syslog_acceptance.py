"""Isolated resume tests: real raw/gzip/checkpoints, mocked host and systemd."""
import contextlib
import datetime as dt
import fcntl
import gzip
import importlib.util
import io
import json
import os
from pathlib import Path
import tempfile
from types import SimpleNamespace
import unittest
from unittest import mock
import urllib.error

REPO = Path(__file__).resolve().parents[2]
DRIVER = REPO / 'Docs/integration/Z01-2026-10-07/ct136-syslog-acceptance.py'
LOG_STORE = REPO / 'system-backend/observability/logging/log_store.py'


class ResumeTests(unittest.TestCase):
    def exercise(self, kind):
        spec = importlib.util.spec_from_file_location('ct136_' + kind, DRIVER)
        driver = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(driver)
        with tempfile.TemporaryDirectory() as temporary:
            base = Path(temporary)
            state, share, checkpoints = base / 'state', base / 'share', base / 'checkpoints'
            raw = state / 'raw'
            for path in (raw, share, checkpoints):
                path.mkdir(parents=True)
            directory = checkpoints / 'netcore-ct136-syslog-existing'
            directory.mkdir(mode=0o700)
            obs, config = base / 'observability.toml', base / 'syslog.json'
            obs.write_text('[server]\nbind="10.0.1.143:8210"\n')
            cfg = json.loads((LOG_STORE.parent.parent / 'config/syslog.example.json').read_text())
            cfg.update(state_dir='/var/lib/netcore-observability/logs', archive_mount='/mnt/nfs-share',
                       collector_id='observability-10.0.1.143', allowed_networks=['10.0.1.0/24', '127.0.0.0/8'])
            config.write_text(json.dumps(cfg))
            driver.OBS_CONFIG, driver.LOG_CONFIG, driver.LOG_STORE = obs, config, LOG_STORE
            driver.CHECKPOINT_ROOT = str(checkpoints)
            marker = 'Z014-SYSLOG-' + 'c' * 32
            record = {'id': 'a' * 32, 'message': ' ' + marker, 'source_ip': '127.0.0.1', 'transport': 'imtcp'}
            source = raw / (dt.datetime.now(dt.timezone.utc).strftime('%Y%m%dT%H%M%S-') + 'b' * 32 + '.open')
            source.write_text(json.dumps(record) + '\n')
            initial = {'LoadState': 'loaded', 'ActiveState': 'inactive', 'User': 'netcore-observability',
                       'Group': 'netcore-observability', 'Type': 'oneshot',
                       'ExecStart': f'{{ argv[]=/usr/bin/python3 {LOG_STORE} archive ; }}',
                       'InvocationID': 'old', 'ExecMainStartTimestampMonotonic': '1',
                       'ExecMainCode': '1', 'ExecMainStatus': '0', 'Result': 'success'}
            result = {'phase': 'stopped', 'failed_phase': 'preview_pending', 'tcp_attempts': 1,
                      'marker': marker, 'marker_sent_at': dt.datetime.now(dt.timezone.utc).isoformat(),
                      'config_sha256': {str(p): driver.fingerprint(p) for p in (obs, config)},
                      'archive_before': initial, 'error': 'TimeoutError: original 5s GET'}
            if kind == 'invalid_marker':
                result['marker'] = 'unexpected-marker'
            if kind == 'ambiguous_tcp':
                result.update(failed_phase='tcp_send_started', tcp_completed=False)
            if kind == 'repeated_tcp':
                result['tcp_attempts'] = 2
            if kind == 'ambiguous_archive':
                result['archive_start_attempts'] = 1
            report = directory / 'result.json'
            original = json.dumps(result).encode()
            report.write_bytes(original)
            report.chmod(0o600)
            if kind == 'changed_config':
                obs.write_text(obs.read_text() + '# externally changed\n')
            calls = {'start': 0, 'clock': 0, 'get': 0, 'get_errors': 0, 'timeouts': []}

            def path(value):
                value = str(value)
                for prefix, destination in (('/var/lib/netcore-observability/logs', state), ('/mnt/nfs-share', share)):
                    if value == prefix or value.startswith(prefix + '/'):
                        return destination / value[len(prefix):].lstrip('/')
                return Path(value)

            def unit(name, timeout=10):
                status = initial.copy()
                if name != driver.ARCHIVE_UNIT:
                    status['ActiveState'] = 'active'
                elif calls['start'] or kind == 'changed_archive':
                    status.update(InvocationID='', ExecMainStartTimestampMonotonic='2')
                return status

            class HTTP:
                def open(self, url, timeout):
                    calls['get'] += 1
                    calls['timeouts'].append(timeout)
                    latency = 6 if kind == 'slow_get' else 0.2
                    calls['clock'] += min(latency, timeout)
                    if latency > timeout:
                        raise TimeoutError('response exceeded call timeout')
                    if '/api/v1/logs?' in url:
                        if kind == 'transient_get' and calls['get_errors'] < 2:
                            calls['get_errors'] += 1
                            if calls['get_errors'] == 1:
                                raise TimeoutError('temporary GET timeout')
                            raise urllib.error.HTTPError(url, 503, 'busy', {}, None)
                        body = [] if kind == 'missing_preview' else [{'message': record['message'], 'fields': {
                            'syslog_id': record['id'], 'source_ip': record['source_ip'], 'transport': record['transport']}}]
                    elif url.endswith('/health/ready'):
                        body = {'ready': True}
                    else:
                        body = {'server': {'bind': '10.0.1.143:8210'}}
                    return io.BytesIO(json.dumps(body).encode())

            def start(args, **kwargs):
                self.assertEqual(args, ['systemctl', 'start', '--no-block', driver.ARCHIVE_UNIT])
                calls['start'] += 1
                self.assertEqual(calls['start'], 1)
                sealed = source.with_suffix('.jsonl')
                source.rename(sealed)
                target = share / 'Logs/NetCore/observability-10.0.1.143' / dt.datetime.strptime(source.name[:8], '%Y%m%d').date().isoformat() / (sealed.name + '.gz')
                target.parent.mkdir(parents=True)
                with gzip.open(target, 'wb') as output:
                    output.write(sealed.read_bytes())
                sealed.unlink()
                (state / 'archive-status.json').write_text(json.dumps({
                    'error': None, 'archived_segments': 1, 'last_attempt': dt.datetime.now(dt.timezone.utc).isoformat()}))
                return SimpleNamespace(returncode=0)

            original_spec = importlib.util.spec_from_file_location
            def wrapped_spec(name, file):
                spec = original_spec(name, file)
                loader = spec.loader
                class Loader:
                    def create_module(self, spec):
                        return None
                    def exec_module(self, module):
                        loader.exec_module(module)
                        module.open_share = lambda cfg: os.open(share, os.O_RDONLY)
                spec.loader = Loader()
                return spec

            original_stat = Path.stat
            def owned_stat(p, *args, **kwargs):
                metadata = original_stat(p, *args, **kwargs)
                if p in (directory, report):
                    return SimpleNamespace(st_uid=0, st_mode=metadata.st_mode)
                return metadata

            def sleep(seconds):
                calls['clock'] += seconds

            with contextlib.ExitStack() as stack:
                if kind == 'locked':
                    held = os.open(directory / 'resume.lock', os.O_CREAT | os.O_RDWR, 0o600)
                    stack.callback(os.close, held)
                    fcntl.flock(held, fcntl.LOCK_EX | fcntl.LOCK_NB)
                stack.enter_context(mock.patch.object(driver, 'Path', side_effect=path))
                stack.enter_context(mock.patch.object(driver, 'unit', side_effect=unit))
                stack.enter_context(mock.patch.object(driver, 'HTTP', HTTP()))
                stack.enter_context(mock.patch.object(driver.socket, 'gethostname', return_value='Observability'))
                connection = stack.enter_context(mock.patch.object(driver.socket, 'create_connection', side_effect=AssertionError('Resume sent new TCP')))
                stack.enter_context(mock.patch.object(driver.os, 'geteuid', return_value=0))
                stack.enter_context(mock.patch.object(driver.subprocess, 'run', side_effect=start))
                stack.enter_context(mock.patch.object(driver.time, 'monotonic', side_effect=lambda: calls['clock']))
                stack.enter_context(mock.patch.object(driver.time, 'sleep', side_effect=sleep))
                stack.enter_context(mock.patch.object(driver.importlib.util, 'spec_from_file_location', side_effect=wrapped_spec))
                stack.enter_context(mock.patch.object(Path, 'stat', owned_stat))
                stack.enter_context(contextlib.redirect_stdout(io.StringIO()))
                code = driver.main(['--resume', str(directory)])
                connection.assert_not_called()
            updated = json.loads(report.read_text())
            refused = kind in ('invalid_marker', 'ambiguous_tcp', 'repeated_tcp', 'ambiguous_archive', 'changed_config', 'locked')
            if refused:
                self.assertEqual(code, 1)
                self.assertEqual(report.read_bytes(), original)
                self.assertFalse(list(directory.glob('result.before-resume-*')))
                self.assertEqual(calls['get'], 0)
            else:
                history = list(directory.glob('result.before-resume-*'))
                self.assertEqual(len(history), 1)
                self.assertEqual(history[0].read_bytes(), original)
                self.assertEqual(updated['marker'], marker)
                self.assertEqual(updated['archive_before'], initial)
                self.assertEqual(updated['tcp_attempts'], 1)
                self.assertEqual(updated['config_sha256'], result['config_sha256'])
                if kind in ('missing_preview', 'changed_archive'):
                    self.assertEqual(code, 1)
                    self.assertEqual(updated['phase'], 'stopped')
                    self.assertTrue(source.exists())
                else:
                    self.assertEqual(code, 0, updated)
                    self.assertEqual(updated['phase'], 'passed')
                    self.assertFalse(source.exists())
                    self.assertEqual(calls['start'], 1)
                    self.assertTrue(updated['archive_decompressed_sha256'])
            if kind == 'slow_get':
                self.assertGreater(updated['http_get']['max_seconds'], 5)
                self.assertTrue(all(value == 20 for value in calls['timeouts']))
            if kind == 'transient_get':
                self.assertEqual(len(updated['http_get']['errors']), 2)
            if kind == 'missing_preview':
                self.assertGreaterEqual(calls['clock'], 300)
                self.assertLess(calls['clock'], 301)
            if refused or kind in ('missing_preview', 'changed_archive'):
                self.assertEqual(calls['start'], 0)

    def test_resume_same_marker_without_tcp(self):
        self.exercise('normal')

    def test_slow_get_beyond_old_five_seconds_recovers(self):
        self.exercise('slow_get')

    def test_transient_get_timeout_and_http503_recover(self):
        self.exercise('transient_get')

    def test_missing_preview_budget_stops_without_archive(self):
        self.exercise('missing_preview')

    def test_changed_archive_baseline_stops(self):
        self.exercise('changed_archive')

    def test_other_resume_lock_refuses_without_get_tcp_archive_or_report_changes(self):
        self.exercise('locked')

    def test_invalid_or_ambiguous_saved_evidence_is_unchanged(self):
        for kind in ('invalid_marker', 'ambiguous_tcp', 'repeated_tcp', 'ambiguous_archive', 'changed_config'):
            with self.subTest(kind=kind):
                self.exercise(kind)


if __name__ == '__main__':
    unittest.main()
