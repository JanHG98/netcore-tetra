#!/usr/bin/env python3
"""Provide the pinned SoapySX GPIO path on Pi 5 without guessing chip numbers.

Installation writes configuration only and also works inside the image chroot.
Runtime reads GPIO chip metadata; it never requests or changes GPIO lines.
"""
import argparse
import fcntl
import os
from pathlib import Path
import stat
import struct
import sys
import tempfile
import time

GPIO_GET_CHIPINFO_IOCTL = 0x8044B401
MODEL_PATH = Path('/proc/device-tree/model')
HELPER_PATH = Path('usr/local/lib/netcore-image/gpio-compat.py')
SERVICE_NAME = 'netcore-sx-gpio-compat.service'
UNIT = '''[Unit]
Description=Verified RP1 GPIO path for the pinned SoapySX driver
After=systemd-udev-trigger.service
Before=tetra.service

[Service]
Type=oneshot
ExecStart=/usr/bin/python3 /usr/local/lib/netcore-image/gpio-compat.py
TimeoutStartSec=20
RemainAfterExit=yes

[Install]
WantedBy=multi-user.target
'''
DROPIN = '''[Unit]
Requires=netcore-sx-gpio-compat.service
After=netcore-sx-gpio-compat.service
'''


class RP1NotReady(RuntimeError):
    pass


def chip_info(path):
    fd = os.open(path, os.O_RDONLY | os.O_CLOEXEC)
    try:
        data = fcntl.ioctl(fd, GPIO_GET_CHIPINFO_IOCTL, bytes(68))
        _, label, lines = struct.unpack('=32s32sI', data)
        return label.split(b'\0', 1)[0].decode('ascii'), lines
    finally:
        os.close(fd)


def rp1_node(dev=Path('/dev')):
    matches = []
    for path in sorted(dev.glob('gpiochip*')):
        if path.is_symlink() or not stat.S_ISCHR(path.stat().st_mode):
            continue
        if chip_info(path) == ('pinctrl-rp1', 54):
            matches.append(path)
    if not matches:
        raise RP1NotReady('Kein nativer RP1-GPIO-Controller mit 54 Leitungen gefunden.')
    if len(matches) != 1:
        raise RuntimeError('RP1-GPIO-Controller nicht eindeutig; kein Alias erstellt.')
    return matches[0]


def ensure_alias(dev=Path('/dev')):
    target = rp1_node(dev)
    alias = dev / 'gpiochip0'
    if os.path.lexists(alias):
        try:
            matches = os.path.samefile(alias, target)
        except OSError:
            matches = False
        if not matches:
            raise RuntimeError(f'{alias} existiert und verweist nicht auf den geprüften RP1; unverändert.')
    else:
        os.symlink(str(target.absolute()), alias)
    print(f'Geprüfter GPIO-Pfad: {alias} -> {target} (pinctrl-rp1, 54 Leitungen)')
    return target


def install(root=Path('/'), source=None):
    source = Path(source) if source is not None else Path(__file__)
    files = [
        (root / HELPER_PATH, source.read_bytes(), 0o755),
        (root / 'etc/systemd/system' / SERVICE_NAME, UNIT.encode(), 0o644),
        (root / 'etc/systemd/system/tetra.service.d/10-gpio-compat.conf', DROPIN.encode(), 0o644),
    ]
    # Check every destination before writing any file; preserve local changes.
    for path, content, _ in files:
        if os.path.lexists(path) and (path.is_symlink() or not path.is_file() or path.read_bytes() != content):
            raise RuntimeError(f'Abbruch: vorhandene abweichende Konfiguration {path}; unverändert.')
    for path, content, mode in files:
        path.parent.mkdir(parents=True, exist_ok=True, mode=0o755)
        with tempfile.NamedTemporaryFile(dir=path.parent, prefix='.' + path.name + '.', delete=False) as temporary:
            temporary_path = Path(temporary.name)
            try:
                temporary.write(content)
                temporary.flush()
                os.fchmod(temporary.fileno(), mode)
                os.replace(temporary_path, path)
            finally:
                temporary_path.unlink(missing_ok=True)
    print(f'{SERVICE_NAME} und tetra-Abhängigkeit installiert; kein GPIO geöffnet.')


def main(argv=None):
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--install', action='store_true', help='Dienst und Abhängigkeit installieren, ohne Hardwarezugriff')
    args = parser.parse_args(argv)
    if os.geteuid() != 0:
        raise RuntimeError('Root-Rechte für Installation bzw. /dev-Alias erforderlich.')
    if args.install:
        install()
        return
    model = MODEL_PATH.read_bytes().rstrip(b'\0').decode('ascii') if MODEL_PATH.exists() else ''
    if not model.startswith(('Raspberry Pi 5', 'Raspberry Pi Compute Module 5')):
        print('Kein Pi-5/CM5-Modell: bestehende GPIO-Zuordnung bleibt unverändert.')
        return
    deadline = time.monotonic() + 15
    while True:
        try:
            ensure_alias()
            return
        except RP1NotReady:
            if time.monotonic() >= deadline:
                raise
            time.sleep(0.25)


if __name__ == '__main__':
    try:
        main()
    except (OSError, RuntimeError, ValueError) as error:
        print(f'GPIO-Kompatibilität fehlgeschlagen: {error}', file=sys.stderr)
        sys.exit(1)
