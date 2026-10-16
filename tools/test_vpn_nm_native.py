#!/usr/bin/env python3
"""Disposable Linux VM only: real libnm import and authenticated OpenVPN traffic.

Run: sudo unshare --mount --net --pid --fork --kill-child --mount-proc
     python3 tools/test_vpn_nm_native.py --disposable-runner
"""
import importlib.util
import json
import os
from pathlib import Path
import shutil
import signal
import subprocess
import sys
import tempfile
import time

DEPLOY = Path(__file__).resolve().parents[1] / 'system-backend/deployment-core'
sys.path[:0] = [str(DEPLOY), str(DEPLOY / 'image')]
from image_spec import validate_vpn_material
from vpn_nm import VPN_KEYFILE, render
from gi.repository import GLib

spec = importlib.util.spec_from_file_location('vpn_policy', DEPLOY / 'image/vpn-policy.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)

USER = 'fixture-user'
PASSWORD = 'fixture-password#$; with spaces\\tail'


def run(*args, timeout=30, **kwargs):
    proc = subprocess.run(list(map(str, args)), text=True, capture_output=True,
                          timeout=timeout, **kwargs)
    if proc.returncode:
        raise RuntimeError(str(args[0]) + ': ' + proc.stdout[-3000:] + proc.stderr[-3000:])
    return proc.stdout.strip()


def wait_for(check, seconds=20):
    deadline = time.monotonic() + seconds
    while time.monotonic() < deadline:
        try:
            if check():
                return
        except (subprocess.SubprocessError, RuntimeError, OSError):
            pass
        time.sleep(0.2)
    raise RuntimeError('Fixture did not become ready')


def certificate(work, name, usage):
    run('openssl', 'req', '-new', '-newkey', 'rsa:2048', '-nodes',
        '-keyout', work / (name + '.key'), '-out', work / (name + '.csr'),
        '-subj', '/CN=' + name)
    (work / (name + '.ext')).write_text('extendedKeyUsage=' + usage + '\n')
    run('openssl', 'x509', '-req', '-in', work / (name + '.csr'),
        '-CA', work / 'ca.pem', '-CAkey', work / 'ca.key', '-CAcreateserial',
        '-out', work / (name + '.pem'), '-days', '2',
        '-extfile', work / (name + '.ext'))


def main():
    if sys.argv[1:] != ['--disposable-runner'] or os.geteuid() != 0:
        raise SystemExit('Disposable namespace/root invocation required')
    os.environ['LC_ALL'] = 'C'
    children = []
    logs = []
    with tempfile.TemporaryDirectory(prefix='netcore-vpn-native-') as directory:
        work = Path(directory)
        try:
            def spawn(args, name):
                log = (work / (name + '.log')).open('w+')
                logs.append(log)
                child = subprocess.Popen(args, stdout=log, stderr=subprocess.STDOUT)
                children.append(child)
                return child

            run('ip', 'link', 'set', 'lo', 'up')
            run('openssl', 'req', '-new', '-x509', '-newkey', 'rsa:2048', '-nodes',
                '-keyout', work / 'ca.key', '-out', work / 'ca.pem', '-days', '2',
                '-subj', '/CN=NetCore disposable VPN CA')
            certificate(work, 'server', 'serverAuth')
            certificate(work, 'client', 'clientAuth')
            run('openvpn', '--genkey', 'secret', work / 'ta.key')
            auth = work / 'verify-auth.py'
            auth.write_text('#!/usr/bin/python3\nimport os,sys\n'
                            'sys.exit(0 if (os.environ.get("username"),os.environ.get("password")) == '
                            + repr((USER, PASSWORD)) + ' else 1)\n')
            auth.chmod(0o700)
            server = work / 'server.conf'
            server.write_text(
                'port 1194\nproto udp4\ndev tun-server\ntopology subnet\n'
                'server 10.67.0.0 255.255.255.0\ndh none\n'
                'ca "' + str(work / 'ca.pem') + '"\n'
                'cert "' + str(work / 'server.pem') + '"\n'
                'key "' + str(work / 'server.key') + '"\n'
                'tls-auth "' + str(work / 'ta.key') + '" 0\n'
                'script-security 3\nauth-user-pass-verify "' + str(auth) + '" via-env\n'
                'verb 3\n')
            spawn(['openvpn', '--config', str(server)], 'openvpn-server')
            wait_for(lambda: 'Initialization Sequence Completed' in
                     (work / 'openvpn-server.log').read_text())

            # Same portable directive shapes as the operator's export, with
            # actual short-lived certificates and no external profile files.
            profile = ('dev tun\npersist-tun\npersist-key\nclient\nresolv-retry infinite\n'
                       'reneg-sec 0\nremote 127.0.0.1 1194 udp4\nlport 0\n'
                       'verify-x509-name server name\nremote-cert-tls server\nauth-user-pass\n')
            for tag, filename in [('ca', 'ca.pem'), ('cert', 'client.pem'),
                                  ('key', 'client.key'), ('tls-auth', 'ta.key')]:
                profile += '<' + tag + '>\n' + (work / filename).read_text() + '</' + tag + '>\n'
            profile += 'key-direction 1\n'
            normalized, user, password = validate_vpn_material(profile, USER, PASSWORD)
            key = render(dict(vpn_config=normalized, vpn_username=user, vpn_password=password), root=work)
            source = work / str(VPN_KEYFILE).lstrip('/')
            assert source.stat().st_mode & 0o777 == 0o600
            keyfile = GLib.KeyFile()
            keyfile.load_from_file(str(source), GLib.KeyFileFlags.NONE)
            assert keyfile.get_string('connection', 'id') == 'NetCore VPN'
            assert keyfile.get_string('vpn', 'username') == USER
            assert keyfile.get_string('vpn-secrets', 'password') == PASSWORD
            assert keyfile.get_string('vpn', 'password-flags') == '0'
            assert keyfile.get_string('vpn', 'verify-x509-name') == 'name:server'
            assert keyfile.get_string('vpn', 'ta-dir') == '1'
            assert not (work / 'etc/openvpn/client/netcore.d/nm-import.ovpn').exists()
            assert all(p.stat().st_mode & 0o777 == 0o600
                       for p in (work / 'etc/openvpn/client/netcore.d').glob('*.pem'))

            # NM storage is bind-mounted in this private mount namespace.
            # The runner's own system connections/state are untouched.
            connections = work / 'connections'
            connections.mkdir()
            shutil.copy2(source, connections / 'netcore-vpn.nmconnection')
            for target, name in [('/etc/NetworkManager/system-connections', 'connections'),
                                 ('/run/NetworkManager', 'nm-run'),
                                 ('/var/lib/NetworkManager', 'nm-state')]:
                private = work / name
                private.mkdir(exist_ok=True)
                Path(target).mkdir(parents=True, exist_ok=True)
                run('mount', '--bind', private, target)

            # Ubuntu's vendor conf.d marks non-WiFi devices unmanaged. Keep
            # all three layers private rather than inheriting runner policy.
            for target, name in [('/etc/NetworkManager/conf.d', 'nm-conf-etc'),
                                 ('/usr/lib/NetworkManager/conf.d', 'nm-conf-vendor')]:
                private = work / name
                private.mkdir()
                Path(target).mkdir(parents=True, exist_ok=True)
                run('mount', '--bind', private, target)
            run('ip', 'link', 'add', 'nc-uplink', 'type', 'dummy')
            bus_address = 'unix:path=' + str(work / 'bus')
            bus_config = work / 'bus.conf'
            bus_config.write_text(
                '<busconfig><type>system</type><listen>' + bus_address + '</listen>'
                '<auth>EXTERNAL</auth><policy context="default">'
                '<allow own="*"/><allow send_destination="*"/>'
                '<allow receive_sender="*"/></policy></busconfig>')
            os.environ['DBUS_SYSTEM_BUS_ADDRESS'] = bus_address
            spawn(['dbus-daemon', '--nofork', '--config-file=' + str(bus_config)], 'dbus')
            wait_for(lambda: (work / 'bus').exists())
            nm_config = work / 'NetworkManager.conf'
            nm_config.write_text('[main]\nplugins=keyfile\nauth-polkit=false\nno-auto-default=*\n'
                                 '[keyfile]\nunmanaged-devices=none\n'
                                 '[device-fixture]\nmatch-device=interface-name:nc-uplink\nmanaged=1\n'
                                 '[logging]\nlevel=WARN\n')
            nm_args = ['NetworkManager', '--no-daemon', '--config=' + str(nm_config),
                       '--pid-file=' + str(work / 'nm.pid'), '--state-file=' + str(work / 'nm.state')]
            nm = spawn(nm_args, 'networkmanager')
            wait_for(lambda: key in run('nmcli', '-g', 'UUID', 'connection', 'show'))
            assert run('nmcli', '-g', 'connection.id', 'connection', 'show', 'uuid', key) == 'NetCore VPN'

            wait_for(lambda: 'nc-uplink' in run('nmcli', '-g', 'DEVICE', 'device'))
            run('nmcli', 'device', 'set', 'nc-uplink', 'managed', 'yes')
            # NM manages its own synthetic carrier. A veth created behind
            # udev in another network namespace is deliberately not managed.
            run('nmcli', 'connection', 'add', 'type', 'dummy', 'ifname', 'nc-uplink',
                'con-name', 'fixture-uplink', 'ipv4.method', 'manual', 'ipv4.addresses',
                '192.0.2.1/24', 'ipv4.gateway', '192.0.2.2', 'ipv6.method', 'disabled')
            run('nmcli', '--wait', '15', 'connection', 'up', 'fixture-uplink')
            cfg = work / 'policy.json'
            def home(enabled):
                cfg.write_text(json.dumps(dict(vpn_uuid=key, trusted_ssids=[],
                    trusted_lan='192.0.2.0/24' if enabled else '198.51.100.0/24')))
                active = set(run('nmcli', '-g', 'UUID', 'connection', 'show', '--active').splitlines())
                # The synthetic carrier models a connected wired interface;
                # production's nmcli device/SSID parsing has separate tests.
                policy.apply_policy(json.loads(cfg.read_text()),
                    [dict(type='ethernet', addresses=['192.0.2.1/24'])], active)
            def traffic():
                assert key in run('nmcli', '-g', 'UUID', 'connection', 'show', '--active').splitlines()
                run('ping', '-c', '1', '-W', '3', '10.67.0.1', timeout=10)

            home(False)
            traffic()
            first_path = run('nmcli', '-g', 'GENERAL.DBUS-PATH', 'connection', 'show', 'uuid', key)
            home(False)
            assert first_path == run('nmcli', '-g', 'GENERAL.DBUS-PATH', 'connection', 'show', 'uuid', key)
            home(True)
            assert key not in run('nmcli', '-g', 'UUID', 'connection', 'show', '--active').splitlines()
            assert 'fixture-uplink' in run('nmcli', '-g', 'NAME', 'connection', 'show', '--active')

            # Re-read the persisted profile with a fresh daemon. No GUI agent
            # or login keyring supplies its username/password.
            nm.send_signal(signal.SIGTERM)
            nm.wait(timeout=15)
            nm = spawn(nm_args, 'networkmanager-restarted')
            wait_for(lambda: key in run('nmcli', '-g', 'UUID', 'connection', 'show'))
            run('nmcli', '--wait', '15', 'connection', 'up', 'fixture-uplink')
            home(False)
            traffic()
            print('PASS: offline libnm import, private credentials/material, visible profile, '
                  'real certificate+password+tls-auth VPN, tunnel traffic, home disconnect, '
                  'idempotent policy and daemon restart without login.', flush=True)
        except Exception:
            for log in logs:
                log.flush()
                print(log.name + ':\n' + Path(log.name).read_text()[-6000:], file=sys.stderr)
            raise
        finally:
            for child in reversed(children):
                if child.poll() is None:
                    child.terminate()
                    try:
                        child.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        child.kill()
                        child.wait(timeout=5)
            for log in logs:
                log.close()


if __name__ == '__main__':
    main()
