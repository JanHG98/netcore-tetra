"""Execute the real guest recipe's package phase with isolated command fixtures.

The normal tests simulate APT download/conffile outcomes. The native test also
uses real dpkg packages and a temporary --root, never the host package database.
All runs stop at a fake install command before the software build can mutate /opt.
"""
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest

RECIPE = Path(__file__).resolve().parents[1] / 'image/build-guest.sh'
BASH = shutil.which('bash')
DPKG = shutil.which('dpkg')
DPKG_DEB = shutil.which('dpkg-deb')
PACKAGE = 'netcore-image-conffile-test'
CONFIG = 'etc/netcore-image-conffile-test.conf'

APT_STUB = '''import json, os, pathlib, subprocess, sys
args = sys.argv[1:]
operation = next(a for a in args if a in ('update', 'dist-upgrade', 'install'))
with open(os.environ['NC_TEST_LOG'], 'a') as log:
    log.write(json.dumps({'operation': operation, 'args': args,
                          'frontend': os.environ.get('DEBIAN_FRONTEND')}) + '\\n')
options = [args[i + 1] for i, value in enumerate(args[:-1]) if value == '-o']
failure = os.environ.get('NC_TEST_FAILURE')
if failure == operation:
    if operation != 'update' or 'APT::Update::Error-Mode=any' in options:
        sys.exit(100)
required = ('Dpkg::Options::=--force-confdef', 'Dpkg::Options::=--force-confold')
if operation != 'update' and not all(value in options for value in required):
    print('dpkg: end of file on stdin at conffile prompt', file=sys.stderr)
    sys.exit(100)
if operation == 'dist-upgrade' and os.environ.get('NC_TEST_NATIVE_ROOT'):
    flags = [value.split('=', 1)[1] for value in options
             if value.startswith('Dpkg::Options::=')]
    command = [os.environ['NC_TEST_DPKG'], '--root=' + os.environ['NC_TEST_NATIVE_ROOT'],
               *flags, '--install', os.environ['NC_TEST_UPGRADE']]
    sys.exit(subprocess.run(command).returncode)
'''


@unittest.skipUnless(BASH, 'Bash is required to execute the guest recipe')
class GuestPackageTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix='netcore-guest-packages-')
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.bin = self.root / 'bin'
        self.bin.mkdir()
        self.log = self.root / 'apt-calls.jsonl'
        self.write_stub('dpkg', "import sys\nprint('arm64') if sys.argv[1:] == ['--print-architecture'] else sys.exit(2)\n")
        self.write_stub('apt-get', APT_STUB)
        self.write_stub('install', "import sys\nsys.exit(97)\n")
        self.environment = dict(os.environ, PATH=str(self.bin) + os.pathsep + os.environ['PATH'],
                                NC_TEST_LOG=str(self.log))
        for key in ('NC_TEST_FAILURE', 'NC_TEST_NATIVE_ROOT', 'NC_TEST_DPKG', 'NC_TEST_UPGRADE'):
            self.environment.pop(key, None)

    def write_stub(self, name, source):
        path = self.bin / name
        path.write_text('#!' + sys.executable + '\n' + source)
        path.chmod(0o755)

    def run_recipe(self, **environment):
        return subprocess.run([BASH, str(RECIPE), 'https://example.invalid/netcore.git',
                               'a' * 40, 'https://example.invalid/soapy.git', 'b' * 40],
                              env={**self.environment, **environment}, stdin=subprocess.DEVNULL,
                              capture_output=True, text=True, timeout=30)

    def calls(self):
        return [json.loads(line) for line in self.log.read_text().splitlines()]

    def test_all_package_operations_handle_conffiles_without_stdin(self):
        result = self.run_recipe()
        self.assertEqual(result.returncode, 97, result.stderr)
        calls = self.calls()
        self.assertEqual([call['operation'] for call in calls], ['update', 'dist-upgrade', 'install'])
        for call in calls:
            self.assertEqual(call['frontend'], 'noninteractive')
            self.assertIn('Dpkg::Options::=--force-confdef', call['args'])
            self.assertIn('Dpkg::Options::=--force-confold', call['args'])
        self.assertIn('APT::Update::Error-Mode=any', calls[0]['args'])

    def test_update_error_stops_before_upgrade_or_install(self):
        result = self.run_recipe(NC_TEST_FAILURE='update')
        self.assertEqual(result.returncode, 100, result.stderr)
        self.assertEqual([call['operation'] for call in self.calls()], ['update'])

    def test_package_failure_stops_remaining_operations(self):
        for operation, expected in [('dist-upgrade', ['update', 'dist-upgrade']),
                                    ('install', ['update', 'dist-upgrade', 'install'])]:
            with self.subTest(operation=operation):
                self.log.unlink(missing_ok=True)
                result = self.run_recipe(NC_TEST_FAILURE=operation)
                self.assertEqual(result.returncode, 100, result.stderr)
                self.assertEqual([call['operation'] for call in self.calls()], expected)

    @unittest.skipUnless(os.geteuid() == 0 and DPKG and DPKG_DEB,
                         'Native conffile fixture requires root, dpkg and dpkg-deb')
    def test_real_dpkg_conffile_upgrade_keeps_guest_config_without_prompt(self):
        target = self.root / 'guest'
        database = target / 'var/lib/dpkg'
        database.mkdir(parents=True)
        (database / 'status').write_text('')
        packages = []
        for version in ('1', '2'):
            package = self.root / ('package-' + version)
            (package / 'DEBIAN').mkdir(parents=True)
            (package / 'etc').mkdir()
            (package / 'DEBIAN/control').write_text(
                'Package: ' + PACKAGE + '\nVersion: ' + version + '\nArchitecture: all\n'
                'Maintainer: NetCore test <test@example.invalid>\n'
                'Description: Isolated guest conffile regression fixture\n')
            (package / 'DEBIAN/conffiles').write_text('/' + CONFIG + '\n')
            (package / CONFIG).write_text('package-version=' + version + '\n')
            archive = self.root / ('package-' + version + '.deb')
            built = subprocess.run([DPKG_DEB, '--build', str(package), str(archive)],
                                   capture_output=True, text=True, timeout=10)
            self.assertEqual(built.returncode, 0, built.stderr)
            packages.append(archive)

        installed = subprocess.run([DPKG, '--root=' + str(target), '--install', str(packages[0])],
                                   stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=10)
        self.assertEqual(installed.returncode, 0, installed.stderr)
        configured = target / CONFIG
        original = b'guest-custom-setting=yes\n'
        configured.write_bytes(original)

        # Noninteractive debconf alone still fails when dpkg asks about a conffile.
        baseline = subprocess.run([DPKG, '--root=' + str(target), '--install', str(packages[1])],
                                  env={**os.environ, 'DEBIAN_FRONTEND': 'noninteractive'},
                                  stdin=subprocess.DEVNULL, capture_output=True, text=True, timeout=10)
        self.assertNotEqual(baseline.returncode, 0)
        self.assertIn('end of file on stdin at conffile prompt', baseline.stderr)
        self.assertEqual(configured.read_bytes(), original)

        # The recipe's actual Bash array supplies the options to this real dpkg.
        result = self.run_recipe(NC_TEST_NATIVE_ROOT=str(target), NC_TEST_DPKG=DPKG,
                                 NC_TEST_UPGRADE=str(packages[1]))
        self.assertEqual(result.returncode, 97, result.stderr)
        self.assertEqual(configured.read_bytes(), original)
        status = (database / 'status').read_text()
        self.assertIn('Status: install ok installed', status)
        self.assertIn('Version: 2', status)
        self.assertEqual([call['operation'] for call in self.calls()],
                         ['update', 'dist-upgrade', 'install'])


if __name__ == '__main__':
    unittest.main()
