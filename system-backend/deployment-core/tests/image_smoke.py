#!/usr/bin/env python3
"""Privileged CI test against the real pinned OS image, without a radio or Rust build.

Run in a disposable Ubuntu VM: sudo unshare --mount --pid --fork --kill-child
    --propagation private python3 system-backend/deployment-core/tests/image_smoke.py
"""
import argparse
import json
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import ROOT, atomic_write
from image_build import (Disk, GIB, clean_identity, command, download_base, expand_base,
                         fsck, install_support, output, partition_table, resize_partition, shrink)
from image_spec import DESKTOPS, validate_image
from test_images import image_request


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--desktop', choices=DESKTOPS, default='none')
    args = parser.parse_args()
    os.environ['LC_ALL'] = 'C'
    with tempfile.TemporaryDirectory(prefix='netcore-image-smoke-') as temp:
        work = Path(temp)
        compressed = download_base(work)
        image, root = work / 'image.img', work / 'root'
        expand_base(compressed, image)
        parts = partition_table(image)
        start = parts[1]['start']
        before_boot = None
        with image.open('r+b') as file:
            file.truncate(8 * GIB)
        resize_partition(image, start, 8 * GIB // 512 - start)
        with Disk(image, root) as disk:
            fsck(disk.loop + 'p2')
            command(['resize2fs', disk.loop + 'p2'])
            disk.rootfs()
            before_boot = (root / 'boot/firmware/cmdline.txt').read_text()
            assert 'root=PARTUUID=' in before_boot
            root_id = output(['blkid', '-s', 'PARTUUID', '-o', 'value', disk.loop + 'p2'])
            assert 'root=PARTUUID=' + root_id in before_boot
            print('Actual stock boot command line: ' + before_boot, flush=True)
            install_support(root)
            # Supply only the base account/config/unit prerequisites for personalization.
            # No fake TBS binary and no successful production artifact are generated.
            source = root / 'opt/netcore-tetra/system-backend/deployment-core/config'
            source.mkdir(parents=True)
            shutil.copy2(ROOT / 'config/agent.example.toml', source / 'agent.example.toml')
            for unit in (ROOT / 'image/systemd').iterdir():
                shutil.copy2(unit, root / 'etc/systemd/system' / unit.name)
            (root / 'var/lib/netcore-discovery').mkdir()
            with disk.guest():
                command(['chroot', root, '/bin/bash', '-c',
                         'test "$(dpkg --print-architecture)" = arm64; '
                         'for g in netcore spi gpio; do getent group "$g" || groupadd --system "$g"; done; '
                         'useradd --system --gid netcore --groups audio,spi,gpio --shell /usr/sbin/nologin netcore'])
                if args.desktop != 'none':
                    # Production sets this before any APT operation. Desktop
                    # dependencies can trigger initramfs generation in the chroot.
                    atomic_write(root / 'etc/initramfs-tools/conf.d/zz-netcore-image.conf',
                                 '# NetCore portable Pi image\nMODULES=most\n', 0o644)
                    command(['chroot', root, '/bin/bash', '-c',
                             'set -Eeuo pipefail; export DEBIAN_FRONTEND=noninteractive; '
                             'NC_APT=(apt-get -o Dpkg::Options::=--force-confdef -o Dpkg::Options::=--force-confold); '
                             '"${NC_APT[@]}" -o APT::Update::Error-Mode=any update; '
                             'source /usr/local/lib/netcore-image/desktop-setup.sh; '
                             'netcore_install_desktop "$1"', 'desktop-smoke', args.desktop], timeout=1800)
                req = validate_image(image_request(password='test-image-not-for-distribution', wifi_ssid='NetCore Test',
                                                   wifi_password='example-password', overlay='sx1255',
                                                   vpn_config='client\ndev tun\nremote vpn.fixture.invalid 1194\nauth-user-pass\n',
                                                   vpn_username='arm64-fixture-user', vpn_password='arm64-fixture-vpn-secret',
                                                   desktop=args.desktop))
                atomic_write(root / 'root/netcore-image-request.json', json.dumps(req))
                command(['chroot', root, '/usr/bin/python3', '/usr/local/lib/netcore-image/personalize.py'])
                (root / 'root/netcore-image-request.json').unlink()
                assert (root / 'etc/hostname').read_text() == 'tbs-02\n'
                assert (root / 'etc/netcore/config.toml').stat().st_mode & 0o777 == 0o640
                assert (root / 'etc/NetworkManager/system-connections/netcore-wifi.nmconnection').stat().st_mode & 0o777 == 0o600
                assert 'ssid=NetCore\\sTest' in (root / 'etc/NetworkManager/system-connections/netcore-wifi.nmconnection').read_text()
                auth = root / 'etc/openvpn/client/netcore.auth'
                assert auth.stat().st_mode & 0o777 == 0o600
                assert auth.read_text() == 'arm64-fixture-user\narm64-fixture-vpn-secret\n'
                vpn = root / 'etc/openvpn/client/netcore.conf'
                assert vpn.stat().st_mode & 0o777 == 0o600
                assert 'auth-user-pass /etc/openvpn/client/netcore.auth\n' in vpn.read_text()
                assert 'arm64-fixture' not in (root / 'usr/share/netcore-image/station.json').read_text()
                assert (root / 'etc/systemd/system/timers.target.wants/netcore-vpn-policy.timer').is_symlink()
                station = json.loads((root / 'usr/share/netcore-image/station.json').read_text())
                assert station['desktop'] == args.desktop
                if args.desktop != 'none':
                    manager = {'kde':'sddm','gnome':'gdm3','xfce':'lightdm','pios':'lightdm'}[args.desktop]
                    target = (root / 'etc/systemd/system/display-manager.service').readlink()
                    assert target.name == manager + '.service', str(target)
                    assert (root / 'usr/share/applications/netcore-vpn.desktop').is_file()
                assert 'dtoverlay=netcore-sx1255' in (root / 'boot/firmware/config.txt').read_text()
                assert before_boot == (root / 'boot/firmware/cmdline.txt').read_text()
                assert '/usr/local/lib/netcore-deployment/launch.py' in (root / 'etc/systemd/system/tetra.service.d/50-discovery.conf').read_text()
                command(['chroot', root, '/usr/sbin/visudo', '-cf', '/etc/sudoers.d/090-netcore-user'])
                command(['chroot', root, '/usr/bin/ssh-keygen', '-A'])
                (root / 'run/sshd').mkdir(exist_ok=True)
                command(['chroot', root, '/usr/sbin/sshd', '-t'])
            clean_identity(root)
            assert (root / 'etc/machine-id').read_text() == ''
            assert not list((root / 'etc/ssh').glob('ssh_host_*'))
        shrink(image, root)
        assert image.stat().st_size < 8 * GIB
        assert partition_table(image)[1]['start'] == start
        with Disk(image, root) as disk:
            fsck(disk.loop + 'p2')
            disk.rootfs()
            assert (root / 'etc/hostname').read_text() == 'tbs-02\n'
            assert before_boot == (root / 'boot/firmware/cmdline.txt').read_text()
            assert output(['blkid', '-s', 'PARTUUID', '-o', 'value', disk.loop + 'p2']) == root_id
        loops = output(['losetup', '--list', '--output', 'BACK-FILE'])
        assert str(image) not in loops
        print('PASS: official SHA256, ARM64 chroot, ' + args.desktop + ' desktop, isolated mounts, personalization, VPN credentials, SSH config, shrink, ext4 and loop cleanup. No Pi boot or full Rust build tested.')


if __name__ == '__main__':
    main()

