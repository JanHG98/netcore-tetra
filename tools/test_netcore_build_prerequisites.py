#!/usr/bin/env python3
"""Run the backend prerequisite helper against isolated Linux command fixtures.

No apt, Rust installation, service action or network request touches the host.
Windows runs source-policy checks; native Linux CI also executes Bash scenarios.
"""
from __future__ import annotations

import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import textwrap
import unittest


SCRIPT = Path(__file__).resolve().parent / 'operator/prepare-netcore-build.sh'
PACKAGES = ['build-essential', 'pkg-config', 'libssl-dev', 'libsqlite3-dev',
            'cmake', 'git', 'curl', 'ca-certificates']


class SourcePolicy(unittest.TestCase):
    def test_backend_helper_has_no_os_upgrade_or_service_controls(self):
        source = SCRIPT.read_text()
        for forbidden in ('systemctl ', 'service ', 'reboot', 'apt-get upgrade',
                          'apt-get dist-upgrade', 'rustup default ', 'curl |',
                          'libSoapySDR', 'libtetra-codec'):
            self.assertNotIn(forbidden, source)
        self.assertIn('--proto-redir', source)
        self.assertIn('--default-toolchain none', source)
        self.assertIn('toolchain install stable --profile minimal', source)

    def test_supported_roles_cover_actual_rust_backends(self):
        root = SCRIPT.parents[2]
        source = SCRIPT.read_text()
        for manifest in (root / 'system-backend').glob('*/Cargo.toml'):
            self.assertIn(manifest.parent.name, source)
        self.assertIn('control-room', source)


@unittest.skipUnless(sys.platform.startswith('linux') and shutil.which('bash'),
                     'Requires native Linux Bash (mocked package/Rust commands)')
class BashFixture(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory(prefix='netcore-build-prereq-test-')
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        self.bin = self.root / 'bin'
        self.bin.mkdir()
        self.log = self.root / 'commands.jsonl'
        self.env = dict(os.environ, PATH=str(self.bin) + ':' + os.environ['PATH'],
                        NC_TEST_LOG=str(self.log), NC_TEST_MISSING='',
                        NC_TEST_UID='0', NC_TEST_UNAME='Linux', NC_TEST_APT_FAILURE='')
        self.env.pop('CARGO_HOME', None)
        self.env.pop('RUSTUP_HOME', None)
        os_file = self.root / 'os-release'
        os_file.write_text('ID=debian\n')
        # Only fixed system paths are rebased. The original shell flow is executed.
        self.fixture = self.root / 'prepare.sh'
        self.fixture.write_text(SCRIPT.read_text().replace('/root/', str(self.root) + '/')
                               .replace('/etc/os-release', str(os_file)))
        self.fake('id', 'print(os.environ["NC_TEST_UID"])')
        self.fake('uname', 'print(os.environ["NC_TEST_UNAME"])')
        self.fake('dpkg-query', '''
            missing = os.environ['NC_TEST_MISSING'].split()
            print('deinstall ok config-files' if sys.argv[-1] in missing
                  else 'install ok installed', end='')
        ''')
        self.fake('apt-get', '''
            if os.environ['NC_TEST_APT_FAILURE'] == sys.argv[1]:
                sys.exit(13)
        ''')
        self.install_rustup()

    def fake(self, name, code, directory=None):
        path = (directory or self.bin) / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text('#!' + sys.executable + '\n'
                        'import json, os, pathlib, sys\n'
                        "with open(os.environ['NC_TEST_LOG'], 'a') as log:\n"
                        f"    log.write(json.dumps([{name!r}] + sys.argv[1:]) + '\\n')\n"
                        + textwrap.dedent(code).strip() + '\n')
        path.chmod(0o755)
        return path

    def install_rustup(self):
        return self.fake('rustup', '''
            if sys.argv[1:] == ['run', 'stable', 'rustc', '--version']:
                print('rustc 1.99.0 (fixture)')
            if sys.argv[1:] == ['run', 'stable', 'cargo', '--version']:
                print('cargo 1.99.0 (fixture)')
        ''', self.root / '.cargo/bin')

    def execute(self, *args):
        return subprocess.run(['bash', str(self.fixture), *args], env=self.env,
                              text=True, capture_output=True, check=False)

    def calls(self, command=None):
        calls = [json.loads(line) for line in self.log.read_text().splitlines()] \
            if self.log.exists() else []
        return [c for c in calls if c[0] == command] if command else calls

    def test_existing_packages_skip_apt_but_refresh_explicit_stable(self):
        result = self.execute('--role', 'packet-core')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls('apt-get'), [])
        self.assertEqual(self.calls('dpkg-query'),
                         [['dpkg-query', '-W', '-f=${Status}', p] for p in PACKAGES])
        self.assertEqual(self.calls('rustup'), [
            ['rustup', 'toolchain', 'install', 'stable', '--profile', 'minimal'],
            ['rustup', 'run', 'stable', 'rustc', '--version'],
            ['rustup', 'run', 'stable', 'cargo', '--version']])
        self.assertIn('rustc 1.99.0', result.stdout)

    def test_only_missing_packages_installed_before_toolchain(self):
        self.env['NC_TEST_MISSING'] = 'pkg-config libssl-dev'
        result = self.execute('--role', 'control-room', '--role', 'observability')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls('apt-get'), [
            ['apt-get', 'update'],
            ['apt-get', 'install', '--no-install-recommends', '-y', 'pkg-config', 'libssl-dev']])
        commands = [c[0] for c in self.calls()]
        self.assertLess(commands.index('apt-get'), commands.index('rustup'))

    def test_ubuntu_and_existing_system_cargo_use_dedicated_root_toolchain(self):
        (self.root / 'os-release').write_text('ID=ubuntu\n')
        self.fake('cargo', 'sys.exit(99)')
        result = self.execute('--role', 'control-room')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual(self.calls('cargo'), [])
        self.assertIn(['rustup', 'run', 'stable', 'cargo', '--version'], self.calls('rustup'))

    def test_toolchain_failure_aborts_without_success_message(self):
        self.fake('rustup', 'sys.exit(42)', self.root / '.cargo/bin')
        result = self.execute('--role', 'media-library')
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('Build-Vorbereitung abgeschlossen', result.stdout)
        self.assertEqual(self.calls('rustup'), [
            ['rustup', 'toolchain', 'install', 'stable', '--profile', 'minimal']])

    def test_apt_failure_aborts_before_toolchain(self):
        for phase in ('update', 'install'):
            with self.subTest(phase=phase):
                if self.log.exists():
                    self.log.unlink()
                self.env['NC_TEST_MISSING'] = 'cmake'
                self.env['NC_TEST_APT_FAILURE'] = phase
                result = self.execute('--role', 'node-gateway')
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.calls('rustup'), [])

    def test_reject_nonroot_nonlinux_and_unsupported_os(self):
        for name, value in (('NC_TEST_UID', '1000'), ('NC_TEST_UNAME', 'MINGW64')):
            with self.subTest(name=name):
                if self.log.exists():
                    self.log.unlink()
                self.env[name] = value
                result = self.execute('--all-backends')
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.calls('apt-get'), [])
                self.assertEqual(self.calls('rustup'), [])
                self.env[name] = '0' if name == 'NC_TEST_UID' else 'Linux'
        (self.root / 'os-release').write_text('ID=fedora\n')
        result = self.execute('--all-backends')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.calls('apt-get'), [])

    def test_pi_and_external_roles_rejected_before_host_inspection(self):
        for role in ('tbs', 'brew', 'piper', 'tbs-sip-fallback', 'unknown'):
            with self.subTest(role=role):
                result = self.execute('--role', role)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.calls(), [])

    def test_argument_errors_and_help_do_not_prepare_host(self):
        for args in ([], ['--role'], ['--unexpected']):
            with self.subTest(args=args):
                result = self.execute(*args)
                self.assertNotEqual(result.returncode, 0)
                self.assertEqual(self.calls(), [])
        self.assertEqual(self.execute('--help').returncode, 0)
        self.assertEqual(self.calls(), [])

    def test_download_failure_never_executes_installer(self):
        (self.root / '.cargo/bin/rustup').unlink()
        self.fake('curl', 'sys.exit(22)')
        result = self.execute('--role', 'packet-core')
        self.assertNotEqual(result.returncode, 0)
        self.assertEqual(self.calls('rustup'), [])
        curl_args = self.calls('curl')[0]
        self.assertIn('--fail', curl_args)
        self.assertIn('--proto', curl_args)
        self.assertIn('=https', curl_args)

    def test_missing_rustup_uses_https_download_without_default_change(self):
        (self.root / '.cargo/bin/rustup').unlink()
        prepared_rustup = self.root / 'fixture-rustup'
        rustup = self.install_rustup()
        rustup.rename(prepared_rustup)
        self.env['NC_TEST_RUSTUP_FIXTURE'] = str(prepared_rustup)
        self.fake('curl', '''
            destination = pathlib.Path(sys.argv[sys.argv.index('-o') + 1])
            target = pathlib.Path(os.environ['NC_TEST_RUSTUP_FIXTURE']).parent / '.cargo/bin/rustup'
            destination.write_text('#!/bin/sh\\n'
                'printf "%s\\n" "$*" > "' + str(target.parent.parent.parent / 'installer-args') + '"\\n'
                'cp "' + os.environ['NC_TEST_RUSTUP_FIXTURE'] + '" "' + str(target) + '"\\n'
                'chmod 755 "' + str(target) + '"\\n')
        ''')
        result = self.execute('--all-backends')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertEqual((self.root / 'installer-args').read_text().strip(),
                         '-y --no-modify-path --default-toolchain none --profile minimal')
        self.assertEqual(self.calls('curl')[0][-3], 'https://sh.rustup.rs')
        self.assertFalse(Path(self.calls('curl')[0][-1]).exists(), 'Temporary installer removed')
        self.assertNotIn(['rustup', 'default', 'stable'], self.calls('rustup'))


if __name__ == '__main__':
    unittest.main(verbosity=2)
