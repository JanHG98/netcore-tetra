#!/usr/bin/env python3
"""Desktop variants, private SIP requests, and the isolated guest installer."""
import base64
import importlib.util
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import types
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
DEPLOY = ROOT / 'system-backend/deployment-core'
sys.path.insert(0, str(DEPLOY))
if os.name == 'nt':
    sys.modules.setdefault('fcntl', types.ModuleType('fcntl'))
    sys.modules.setdefault('grp', types.ModuleType('grp'))
    sys.modules.setdefault('pwd', types.ModuleType('pwd'))
from image_spec import public_job, sip_setup_state, validate_image
import image_build
from image_build import recipe_key
sys.path.insert(0, str(DEPLOY / 'image'))
import personalize

PASSWORD_HASH = '$6$fixture$' + 'x' * 86


def request(**changes):
    key = base64.b64encode(b'\0\0\0\x0bssh-ed25519\0\0\0\x20' + b'\x01' * 32).decode()
    data = dict(profile=dict(name='TBS-02', mcc=901, mnc=1510, issi=4010002, la=2, cc=2),
                commit='a' * 40, config='[net_info]\nmcc=901\n[cell_info]\ncolour_code=2\n[phy_io]\nbackend="SoapySdr"\n',
                controller_url='http://10.0.1.50:8320', ssh_key='ssh-ed25519 ' + key)
    data.update(changes)
    return data


def sip_text(profile='TBS-02'):
    return ('[service]\nnode_id="' + profile + '"\n[central]\nhost="10.0.1.125"\n'
            'username="fixture-station"\npassword="secret-private-sip-fixture"\n'
            '[fallback_pbx]\nhost="10.0.1.21"\nusername="supplied-pbx-identity"\n')


class ImageDesktopRequestTests(unittest.TestCase):
    def setUp(self):
        if os.name == 'nt':
            # Windows test runner has no IANA tzdata; Linux CI uses real ZoneInfo.
            timezone = patch('image_spec.ZoneInfo', return_value=object())
            timezone.start()
            self.addCleanup(timezone.stop)

    def test_old_requests_remain_lite_and_revalidate(self):
        req = validate_image(request())
        self.assertEqual(req['desktop'], 'none')
        self.assertEqual(validate_image(req), req)

    def test_kde_accepts_os_password_hash_and_revalidates(self):
        req = validate_image(request(desktop='kde', password_hash=PASSWORD_HASH))
        self.assertEqual(req['desktop'], 'kde')
        self.assertEqual(validate_image(req), req)

    def test_kde_rejects_ssh_key_only_with_graphical_login_error(self):
        with self.assertRaisesRegex(ValueError, 'grafische Anmeldung'):
            validate_image(request(desktop='kde'))

    def test_unsupported_or_non_text_desktops_fail_before_build(self):
        for value in ('plasma;id', 'gnome', '', None, []):
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate_image(request(desktop=value, password_hash=PASSWORD_HASH))

    def test_cache_never_aliases_lite_and_kde(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / 'image').mkdir()
            (root / 'image/desktop-setup.sh').write_text('fixture recipe')
            (root / 'image_build.py').write_text('builder')
            (root / 'image_spec.py').write_text('spec')
            with patch('image_build.ROOT', root):
                lite = recipe_key('a' * 40)
                kde = recipe_key('a' * 40, 'kde')
                self.assertNotEqual(lite, kde)
                self.assertEqual(lite, recipe_key('a' * 40, 'none'))
                self.assertNotEqual(kde, recipe_key('b' * 40, 'kde'))
                (root / 'image/desktop-setup.sh').write_text('changed recipe')
                self.assertNotEqual(kde, recipe_key('a' * 40, 'kde'))

    def test_sip_import_validates_profile_and_is_private_in_public_job(self):
        req = validate_image(request(sip_fallback_config=sip_text()))
        self.assertEqual(req['sip_setup'], 'configured')
        result = json.dumps(public_job(dict(request=req, status='queued')))
        self.assertIn('configured', result)
        self.assertNotIn('secret-private-sip-fixture', result)
        self.assertNotIn('sip_fallback_config', result)
        with self.assertRaisesRegex(ValueError, 'Stationskennung passt nicht'):
            validate_image(request(sip_fallback_config=sip_text('TBS-03')))

    def test_sip_malformed_import_is_rejected_without_secret_in_error(self):
        with self.assertRaises(ValueError) as error:
            validate_image(request(sip_fallback_config='[central]\npassword="secret-private-sip-fixture\n'))
        self.assertNotIn('secret-private-sip-fixture', str(error.exception))

    def test_missing_private_sip_disables_only_local_empty_pbx(self):
        config = {'asterisk': dict(enabled=True, remote_host='127.0.0.1', remote_port=5060, outbound_prefix='91')}
        req = validate_image(request())
        with patch('personalize.configure_local_sip', return_value=None) as configure:
            self.assertEqual(personalize.apply_sip(config, req), 'setup_required')
        configure.assert_called_once_with('', 'TBS-02')
        self.assertFalse(config['asterisk']['enabled'])
        self.assertEqual(config['asterisk']['outbound_prefix'], '91')
        remote = {'asterisk': dict(enabled=True, remote_host='10.0.1.125', remote_port=5060)}
        with patch('personalize.configure_local_sip', return_value=None):
            self.assertEqual(personalize.apply_sip(remote, req), 'remote_configured')
        self.assertTrue(remote['asterisk']['enabled'])

    def test_import_patch_preserves_station_routing(self):
        config = {'asterisk': dict(outbound_prefix='91', service_numbers=['38*'])}
        req = validate_image(request(sip_fallback_config=sip_text()))
        native = dict(enabled=True, remote_host='127.0.0.1', password='private-generated-local')
        with patch('personalize.configure_local_sip', return_value=native):
            self.assertEqual(personalize.apply_sip(config, req), 'configured')
        self.assertEqual(config['asterisk']['service_numbers'], ['38*'])
        self.assertEqual(config['asterisk']['password'], native['password'])

    def test_build_passes_desktop_to_guest_and_records_public_artifact(self):
        with tempfile.TemporaryDirectory() as temp:
            state = Path(temp)
            for directory in ('requests', 'cache', 'work', 'artifacts'):
                (state / directory).mkdir()
            key = '1' * 32
            data = request(desktop='kde', password_hash=PASSWORD_HASH, sip_fallback_config=sip_text())
            (state / 'requests' / (key + '.json')).write_text(json.dumps(data))
            commands = []

            class Disk:
                def __init__(self, image, root):
                    self.root = root
                    self.loop = 'fixture-loop'
                def __enter__(self):
                    return self
                def __exit__(self, *args):
                    return None
                def rootfs(self):
                    (self.root / 'usr/share/netcore-image').mkdir(parents=True, exist_ok=True)
                def guest(self):
                    from contextlib import nullcontext
                    return nullcontext()

            def command(args, **kwargs):
                commands.append(args)
                if str(args[0]) == 'chroot' and str(args[3]).endswith('build-guest.sh'):
                    version = dict(commit=data['commit'], desktop=args[-1], packages=['fixture software'])
                    (args[1] / 'usr/share/netcore-image/versions.json').write_text(json.dumps(version))
                if str(args[0]) == 'xz':
                    args[-1].with_suffix('.img.xz').write_bytes(b'fixture transport only, not an OS image')
                if str(args[0]) == 'cp':
                    shutil.copyfile(args[-2], args[-1])

            def write(path, content, mode=0o600):
                Path(path).parent.mkdir(parents=True, exist_ok=True)
                Path(path).write_text(content)

            real_read = Path.read_text
            def read(path, *args, **kwargs):
                return '' if path.as_posix() == '/proc/self/mountinfo' else real_read(path, *args, **kwargs)
            from contextlib import ExitStack
            with ExitStack() as stack:
                for name in ('recover_work', 'preflight', 'resize_partition', 'fsck', 'install_support',
                             'guest_dns', 'guest_sdr', 'clean_identity', 'shrink'):
                    stack.enter_context(patch('image_build.' + name))
                stack.enter_context(patch('image_build.Disk', Disk))
                stack.enter_context(patch('image_build.GIB', 1))
                stack.enter_context(patch('image_build.download_base', return_value=state / 'fixture-base'))
                stack.enter_context(patch('image_build.expand_base', side_effect=lambda src, dest: dest.write_bytes(b'fixture base')))
                stack.enter_context(patch('image_build.partition_table', return_value=[{}, dict(start=1)]))
                stack.enter_context(patch('image_build.atomic_write', side_effect=write))
                stack.enter_context(patch('image_build.command', side_effect=command))
                stack.enter_context(patch.object(Path, 'read_text', read))
                image_build.build(state, key)
            guest = [args for args in commands if str(args[0]) == 'chroot' and str(args[3]).endswith('build-guest.sh')]
            self.assertEqual(len(guest), 1)
            self.assertEqual(guest[0][-1], 'kde')
            manifest = json.loads((state / 'artifacts' / key / 'manifest.json').read_text())
            self.assertEqual((manifest['desktop'], manifest['desktop_label'], manifest['sip_setup']),
                             ('kde', 'KDE Plasma', 'configured'))
            self.assertNotIn('secret-private-sip-fixture', json.dumps(manifest))
            self.assertFalse(manifest['boot_tested'])


BASH = shutil.which('bash')
if os.name == 'nt' and Path('C:/Program Files/Git/bin/bash.exe').exists():
    BASH = 'C:/Program Files/Git/bin/bash.exe'


@unittest.skipUnless(BASH, 'Bash is required for the isolated installer harness')
class ImageDesktopInstallerTests(unittest.TestCase):
    def run_installer(self, desktop, *, session=True, absent='', apt_fails=False):
        temp = tempfile.TemporaryDirectory()
        self.addCleanup(temp.cleanup)
        root = Path(temp.name) / 'guest'
        (root / 'etc/X11').mkdir(parents=True)
        if os.name == 'nt':
            (root / 'etc/sddm.conf.d').mkdir(parents=True)
        (root / 'usr/share/xsessions').mkdir(parents=True)
        if session:
            (root / 'usr/share/xsessions/plasma.desktop').write_text('[Desktop Entry]\nName=Plasma\n')
        logfile = Path(temp.name) / 'events'
        script = Path(temp.name) / 'run.sh'
        # Functions record every package and unit action; real file writes only
        # target the temporary guest. No host package/service operation executes.
        script.write_text('''#!/usr/bin/env bash
set -Eeuo pipefail
export PATH="/usr/bin:/bin:$PATH"
fixture_apt() {
  printf 'apt %s\\n' "$*" >> "$LOGFILE"
  [[ $APT_FAIL == 0 ]]
}
debconf-set-selections() { cat >> "$LOGFILE"; }
dpkg-query() {
  printf 'verify %s\\n' "${@: -1}" >> "$LOGFILE"
  [[ ${@: -1} != "$ABSENT" ]] || { echo 'deinstall ok config-files'; return; }
  echo 'install ok installed'
}
systemctl() { printf 'unit %s\\n' "$*" >> "$LOGFILE"; }
if [[ $WINDOWS_FIXTURE == 1 ]]; then
  # Native POSIX directory mode changes are denied by this Windows sandbox.
  # Linux CI executes GNU install/chmod against the temporary guest directly.
  install() { [[ $* == "-d -m 0755 $GUEST/etc/sddm.conf.d" ]]; }
  chmod() { [[ $* == "0644 $GUEST/etc/sddm.conf.d/90-netcore.conf" ]]; }
fi
NC_APT=(fixture_apt -o Dpkg::Options::=--force-confold)
source "$HELPER"
netcore_install_desktop "$DESKTOP" "$GUEST"
''', encoding='utf-8', newline='\n')
        def bash_path(path):
            value = path.as_posix()
            return '/' + value[0].lower() + value[2:] if os.name == 'nt' else value
        env = dict(os.environ, LOGFILE=bash_path(logfile), DESKTOP=desktop, GUEST=bash_path(root),
                   HELPER=bash_path(DEPLOY / 'image/desktop-setup.sh'), ABSENT=absent,
                   APT_FAIL='1' if apt_fails else '0', WINDOWS_FIXTURE='1' if os.name == 'nt' else '0')
        if os.name == 'nt':
            env['PATH'] = '/usr/bin:/bin:' + env.get('PATH', '')
        result = subprocess.run([BASH, script.as_posix()], env=env, capture_output=True, text=True)
        events = logfile.read_text() if logfile.exists() else ''
        return root, result, events

    def test_kde_installs_packages_writes_login_config_and_enables_graphical_target(self):
        root, result, events = self.run_installer('kde')
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertIn('--no-install-recommends plasma-desktop plasma-workspace kwin-x11 sddm', events)
        for package in ('konsole', 'dolphin', 'plasma-nm', 'systemsettings', 'dbus-x11'):
            self.assertIn('verify ' + package, events)
        self.assertNotIn('kde-full', events)
        self.assertIn('unit enable sddm.service', events)
        self.assertIn('unit set-default graphical.target', events)
        self.assertEqual((root / 'etc/X11/default-display-manager').read_text(), '/usr/bin/sddm\n')
        login = (root / 'etc/sddm.conf.d/90-netcore.conf').read_text()
        self.assertIn('[Autologin]\nUser=\nSession=\nRelogin=false', login)

    def test_lite_and_invalid_desktop_do_not_install_or_enable_gui(self):
        for desktop, returncode in (('none', 0), ('kde;id', 1)):
            with self.subTest(desktop=desktop):
                root, result, events = self.run_installer(desktop)
                self.assertEqual(result.returncode, returncode)
                self.assertEqual(events, '')
                self.assertFalse((root / 'etc/sddm.conf.d/90-netcore.conf').exists())

    def test_missing_session_or_package_prevents_enable(self):
        for options in (dict(session=False), dict(absent='kwin-x11')):
            with self.subTest(options=options):
                root, result, events = self.run_installer('kde', **options)
                self.assertNotEqual(result.returncode, 0)
                self.assertNotIn('unit ', events)
                self.assertFalse((root / 'etc/sddm.conf.d/90-netcore.conf').exists())

    def test_failed_package_install_prevents_login_configuration(self):
        root, result, events = self.run_installer('kde', apt_fails=True)
        self.assertNotEqual(result.returncode, 0)
        self.assertNotIn('verify ', events)
        self.assertNotIn('unit ', events)
        self.assertFalse((root / 'etc/sddm.conf.d/90-netcore.conf').exists())


if __name__ == '__main__':
    unittest.main()
