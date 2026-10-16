#!/usr/bin/env python3
"""Disposable CI only: apply station CRUD to a real isolated Asterisk daemon."""
import os
from pathlib import Path
import shutil
import socket
import subprocess
import time
import unittest

import test_sip_station_management as station_tests
from netcore_sip_switch import config_toml


class ActualAsteriskTests(unittest.TestCase):
    def test_actual_endpoint_auth_and_removal(self):
        binary = shutil.which('asterisk') or '/usr/sbin/asterisk'
        self.assertTrue(Path(binary).is_file(), 'Install Asterisk in the disposable CI container')
        fixture = station_tests.StationTests('test_create_durable_generated_secret_reload_and_no_shared_trunk_reuse')
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        fixture.runner.stop()
        app, root = fixture.app, fixture.root
        config = root / 'asterisk'
        with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as listener:
            listener.bind(('127.0.0.1', 0))
            sip_port = listener.getsockname()[1]
        app.config.raw['asterisk']['sip_bind'] = '127.0.0.1:' + str(sip_port)
        app.config.raw['pbx'].update(mode='ip_trunk', password='')
        fixture.path.write_text(config_toml(app.config.raw), encoding='utf-8')
        modules = next(p.parent for p in Path('/usr/lib').glob('**/asterisk/modules/chan_pjsip.so'))
        directories = {'astetcdir': config, 'astmoddir': modules, 'astvarlibdir': root/'var',
            'astdbdir': root/'db', 'astkeydir': root/'keys', 'astdatadir': '/usr/share/asterisk',
            'astagidir': root/'agi', 'astspooldir': root/'spool', 'astrundir': root/'run', 'astlogdir': root/'logs'}
        for name,value in directories.items():
            if name not in {'astmoddir', 'astdatadir'}:
                Path(value).mkdir(parents=True, exist_ok=True)
        main = config / 'asterisk.conf'
        main.write_text('[directories]\n' + ''.join(f'{k} = {v}\n' for k,v in directories.items()) + '\n[options]\nnofork = yes\n', encoding='utf-8')
        for filename, include in [('pjsip.conf','netcore-pjsip.conf'),('extensions.conf','netcore-extensions.conf'),('rtp.conf','netcore-rtp.conf')]:
            (config/filename).write_text('#include '+include+'\n', encoding='utf-8')
        (config/'modules.conf').write_text('[modules]\nautoload=yes\nnoload=chan_sip.so\n', encoding='utf-8')
        app.render_asterisk()
        def cli(command, timeout=5):
            result = subprocess.run([binary, '-C', str(main), '-rx', command], capture_output=True, text=True, timeout=timeout)
            return result.returncode == 0, result.stdout + result.stderr
        app.run_asterisk = cli
        with (root/'startup.log').open('w') as log:
            daemon = subprocess.Popen([binary, '-f', '-g', '-C', str(main)], stdout=log, stderr=log)
            try:
                for _ in range(100):
                    if cli('core waitfullybooted')[0]:
                        break
                    self.assertIsNone(daemon.poll(), 'Asterisk exited; inspect isolated startup log')
                    time.sleep(.1)
                else:
                    self.fail('Asterisk did not boot')
                fixture.create()
                self.assertIn('tbs-tbs-test', cli('pjsip show endpoint tbs-tbs-test')[1])
                first = app.config.tbs[-1]['password']
                self.assertIn(first, cli('pjsip show auth tbs-tbs-test-auth')[1])
                app.change_station('update', {'revision': app.config_revision(), 'password': 'actual-updated-secret', 'max_contacts': 2}, 'TBS-TEST')
                auth = cli('pjsip show auth tbs-tbs-test-auth')[1]
                self.assertIn('actual-updated-secret', auth)
                self.assertNotIn(first, auth)
                self.assertRegex(cli('pjsip show aor tbs-tbs-test')[1], r'max_contacts\s*:\s*2')
                app.change_station('delete', {'revision': app.config_revision()}, 'TBS-TEST')
                self.assertIn('Unable to find object', cli('pjsip show endpoint tbs-tbs-test')[1])
                self.assertIn('Unable to find object', cli('pjsip show auth tbs-tbs-test-auth')[1])
                print('PASS: actual Asterisk loaded station endpoint/auth/AOR, changed secret/contact limit and removed endpoint/auth without restarting the daemon')
            finally:
                cli('core stop now')
                try:
                    daemon.wait(timeout=10)
                except subprocess.TimeoutExpired:
                    daemon.kill()
                    daemon.wait(timeout=5)


if __name__ == '__main__':
    unittest.main(verbosity=2)
