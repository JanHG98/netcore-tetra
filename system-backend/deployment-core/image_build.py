#!/usr/bin/env python3
"""Build a flashable ARM64 image inside a private mount/PID namespace on the VM."""
import argparse
from contextlib import contextmanager
import hashlib
import json
import lzma
import os
from pathlib import Path
import platform
import re
import shutil
import signal
import subprocess
import time
import urllib.request

from common import ROOT, REPOSITORY, atomic_write, json_file
from deploy import run
from image_spec import BASE, BUILD_ID, validate_image

GIB = 1024 ** 3
WORK_GIB = 24
MIN_FREE = 32 * GIB


def command(args, **kwargs):
    run([str(x) for x in args], print, **kwargs)


def output(args):
    return subprocess.check_output([str(x) for x in args], text=True, timeout=30).strip()


def guest_dns(root):
    """Check the resolver as APT's download user, before any package work."""
    print('DNS im ARM64-Gast als APT-Benutzer _apt prüfen.', flush=True)
    command(['chroot', root, '/usr/sbin/runuser', '-u', '_apt', '--',
             '/usr/bin/test', '-r', '/etc/resolv.conf'], timeout=10)
    for host in ('deb.debian.org', 'archive.raspberrypi.com'):
        command(['chroot', root, '/usr/sbin/runuser', '-u', '_apt', '--',
                 '/usr/bin/timeout', '--kill-after=2s', '15s',
                 '/usr/bin/getent', 'ahostsv4', host], timeout=20)


def sha256(path):
    digest = hashlib.sha256()
    with Path(path).open('rb') as source:
        for block in iter(lambda: source.read(4 * 1024 * 1024), b''):
            digest.update(block)
    return digest.hexdigest()


def recipe_key(commit):
    digest = hashlib.sha256(json.dumps({'base': BASE, 'commit': commit, 'revision': 1}, sort_keys=True).encode())
    for path in sorted([ROOT / 'image_build.py', ROOT / 'image_spec.py', *list((ROOT / 'image').rglob('*'))]):
        if path.is_file() and '__pycache__' not in path.parts:
            digest.update(str(path.relative_to(ROOT)).encode())
            digest.update(path.read_bytes())
    return digest.hexdigest()


def preflight(state):
    if os.geteuid() != 0:
        raise RuntimeError('Der Imagebuilder benötigt root')
    for tool in ('losetup', 'sfdisk', 'mount', 'umount', 'e2fsck', 'resize2fs', 'dumpe2fs', 'xz', 'chroot', 'udevadm'):
        if not shutil.which(tool):
            raise RuntimeError(tool + ' fehlt; install-vm.sh ausführen')
    if platform.machine() not in ('aarch64', 'arm64'):
        entries = list(Path('/proc/sys/fs/binfmt_misc').glob('*aarch64*'))
        if not any('enabled' in p.read_text() and re.search(r'^flags:.*F', p.read_text(), re.M) for p in entries):
            raise RuntimeError('ARM64 binfmt mit F-Flag fehlt; install/install-vm.sh erneut ausführen '
                               '(Ubuntu 24.04: qemu-user-static/binfmt-support; '
                               'Ubuntu 26.04: qemu-user/qemu-user-binfmt, systemd-binfmt.service)')
    free = shutil.disk_usage(state).free
    if free < MIN_FREE:
        raise RuntimeError(f'Mindestens 32 GiB frei erforderlich; vorhanden: {free / GIB:.1f} GiB')


def download_base(cache):
    target = Path(cache) / (BASE['sha256'] + '.img.xz')
    if target.is_file() and sha256(target) == BASE['sha256']:
        print('Geprüftes Raspberry-Pi-OS-Basisimage aus Cache.', flush=True)
        return target
    partial = target.with_suffix('.part')
    print('Offizielles Raspberry-Pi-OS herunterladen und SHA256 prüfen.', flush=True)
    try:
        request = urllib.request.Request(BASE['url'], headers={'User-Agent': 'NetCore-Imagebuilder/0.2'})
        with urllib.request.urlopen(request, timeout=60) as source, partial.open('wb') as dest:
            size = 0
            while block := source.read(1024 * 1024):
                size += len(block)
                if size > 2 * GIB:
                    raise RuntimeError('Basisimage überschreitet das Download-Limit')
                dest.write(block)
            dest.flush()
            os.fsync(dest.fileno())
        if sha256(partial) != BASE['sha256']:
            raise RuntimeError('SHA256 des Raspberry-Pi-OS-Downloads stimmt nicht überein')
        os.replace(partial, target)
        return target
    finally:
        partial.unlink(missing_ok=True)


def expand_base(source, target):
    size = 0
    with lzma.open(source, 'rb') as incoming, Path(target).open('wb') as dest:
        while block := incoming.read(1024 * 1024):
            size += len(block)
            if size > 8 * GIB:
                raise RuntimeError('Unerwartete Größe des offiziellen Basisimages')
            if block.count(0) == len(block):
                dest.seek(len(block), 1)
            else:
                dest.write(block)
        dest.truncate(size)


def partition_table(image):
    table = json.loads(output(['sfdisk', '--json', image]))['partitiontable']
    parts = table['partitions']
    if (table['label'] != 'dos' or table.get('sectorsize', 512) != 512 or len(parts) != 2 or
            parts[0]['type'].lower() not in ('c', 'b', 'e') or parts[1]['type'] != '83' or
            parts[1]['start'] < parts[0]['start'] + parts[0]['size']):
        raise RuntimeError('Erwartet: Raspberry-Pi-Image mit MBR, FAT-Bootpartition und ext4-Rootpartition')
    return parts


def resize_partition(image, start, size):
    # Only an image file produced in this worker's private work directory is accepted here.
    if not Path(image).is_file() or Path(image).is_symlink():
        raise RuntimeError('Partitionierung ist nur für reguläre Build-Dateien erlaubt')
    subprocess.run(['sfdisk', '--no-reread', '--no-tell-kernel', '-N', '2', str(image)],
                   input=f'{start}, {size}, 83\n', text=True, check=True, timeout=30)


def fsck(device):
    result = subprocess.run(['e2fsck', '-f', '-p', str(device)], timeout=300)
    if result.returncode not in (0, 1):
        raise RuntimeError('ext4-Prüfung fehlgeschlagen: ' + str(result.returncode))


class Disk:
    def __init__(self, image, root):
        self.image, self.root = Path(image), Path(root)
        self.loop = None
        self.mounts = []

    def __enter__(self):
        try:
            self.loop = output(['losetup', '--find', '--show', '--partscan', self.image])
            command(['udevadm', 'settle'], timeout=30)
            for _ in range(50):
                if Path(self.loop + 'p1').exists() and Path(self.loop + 'p2').exists():
                    break
                time.sleep(.1)
            if output(['blkid', '-s', 'TYPE', '-o', 'value', self.loop + 'p1']) != 'vfat' or output(
                    ['blkid', '-s', 'TYPE', '-o', 'value', self.loop + 'p2']) != 'ext4':
                raise RuntimeError('Unerwartete Dateisysteme im Basisimage')
            return self
        except BaseException:
            self.__exit__(None, None, None)
            raise

    def mount(self, source, target, *options):
        path = self.root / target
        path.mkdir(parents=True, exist_ok=True)
        command(['mount', *options, source, path])
        self.mounts.append(path)

    def rootfs(self):
        self.mount(self.loop + 'p2', '', '-o', 'noatime')
        self.mount(self.loop + 'p1', 'boot/firmware')
        if not (self.root / 'boot/firmware/config.txt').is_file():
            raise RuntimeError('Raspberry-Pi-Bootkonfiguration fehlt')

    @contextmanager
    def guest(self):
        before = len(self.mounts)
        resolver = self.root / 'etc/resolv.conf'
        saved = resolver.with_name('resolv.conf.netcore-build')
        if resolver.exists() or resolver.is_symlink():
            resolver.rename(saved)
        resolver.write_text(Path('/etc/resolv.conf').read_text())
        # The worker's UMask=0077 must not hide DNS from APT's _apt sandbox.
        resolver.chmod(0o644)
        policy = self.root / 'usr/sbin/policy-rc.d'
        old_policy = policy.read_bytes() if policy.exists() else None
        policy.write_text('#!/bin/sh\nexit 101\n')
        policy.chmod(0o755)
        try:
            self.mount('proc', 'proc', '-t', 'proc', '-o', 'nosuid,nodev,noexec')
            self.mount('sysfs', 'sys', '-t', 'sysfs', '-o', 'ro,nosuid,nodev,noexec')
            self.mount('tmpfs', 'dev', '-t', 'tmpfs', '-o', 'mode=755,nosuid')
            for name, major, minor in [('null', 1, 3), ('zero', 1, 5), ('random', 1, 8), ('urandom', 1, 9), ('tty', 5, 0)]:
                import stat
                os.mknod(self.root / 'dev' / name, stat.S_IFCHR | 0o666, os.makedev(major, minor))
                os.chmod(self.root / 'dev' / name, 0o666)
            self.mount('devpts', 'dev/pts', '-t', 'devpts', '-o', 'newinstance,ptmxmode=0666,mode=0620,gid=5')
            for name, target in [('ptmx', 'pts/ptmx'), ('fd', '/proc/self/fd'), ('stdin', '/proc/self/fd/0'),
                                 ('stdout', '/proc/self/fd/1'), ('stderr', '/proc/self/fd/2')]:
                (self.root / 'dev' / name).symlink_to(target)
            self.mount('tmpfs', 'dev/shm', '-t', 'tmpfs', '-o', 'mode=1777,nosuid,nodev')
            self.mount('tmpfs', 'run', '-t', 'tmpfs', '-o', 'mode=755,nosuid,nodev')
            yield
        finally:
            self.unmount_after(before)
            resolver.unlink(missing_ok=True)
            if saved.exists() or saved.is_symlink():
                saved.rename(resolver)
            if old_policy is None:
                policy.unlink(missing_ok=True)
            else:
                policy.write_bytes(old_policy)

    def unmount_after(self, index):
        while len(self.mounts) > index:
            path = self.mounts[-1]
            command(['umount', str(path)], timeout=30)
            self.mounts.pop()

    def __exit__(self, *args):
        self.unmount_after(0)
        if self.loop:
            command(['losetup', '--detach', self.loop])
            self.loop = None


def shrink(image, root):
    start = partition_table(image)[1]['start']
    with Disk(image, root) as disk:
        device = disk.loop + 'p2'
        fsck(device)
        command(['resize2fs', '-M', device], timeout=600)
        info = output(['dumpe2fs', '-h', device])
        count = int(re.search(r'^Block count:\s+(\d+)', info, re.M)[1])
        block_size = int(re.search(r'^Block size:\s+(\d+)', info, re.M)[1])
        # Leave at least 1 GiB free before first-boot expansion to the SD size.
        target_bytes = ((count * block_size + GIB + 4 * 1024 * 1024 - 1) // (4 * 1024 * 1024)) * (4 * 1024 * 1024)
        command(['resize2fs', device, str(target_bytes // 1024) + 'K'], timeout=600)
        fsck(device)
    resize_partition(image, start, target_bytes // 512)
    with Path(image).open('r+b') as file:
        file.truncate(start * 512 + target_bytes)
    partition_table(image)


def install_support(root):
    dest = root / 'usr/local/lib/netcore-image'
    shutil.copytree(ROOT / 'image', dest, dirs_exist_ok=True, ignore=shutil.ignore_patterns('__pycache__', '*.pyc'))
    # Reuse pure validation/serialization in the guest, without copying the worker API.
    for name in ('image_spec.py', 'common.py', 'deploy.py', 'bindings.py', 'catalog.json'):
        shutil.copy2(ROOT / name, dest / name)
    return dest


def clean_identity(root):
    (root / 'etc/machine-id').write_text('')
    dbus = root / 'var/lib/dbus/machine-id'
    dbus.unlink(missing_ok=True)
    dbus.parent.mkdir(parents=True, exist_ok=True)
    dbus.symlink_to('/etc/machine-id')
    for path in (root / 'etc/ssh').glob('ssh_host_*'):
        path.unlink()
    for rel in ('var/lib/systemd/random-seed', 'var/lib/NetworkManager/secret_key', 'var/lib/NetworkManager/seen-bssids',
                'var/lib/NetworkManager/timestamps', 'var/lib/dhcpcd/duid', 'var/lib/netcore-firstboot.done'):
        (root / rel).unlink(missing_ok=True)
    logs = root / 'var/log'
    for path in logs.rglob('*'):
        if path.is_file() and not path.is_symlink():
            path.write_bytes(b'')


def recover_work(state):
    """Recover only loops backed by our own abandoned work files, never other disks."""
    work = state / 'work'
    loops = json.loads(output(['losetup', '--json', '--list', '--output', 'NAME,BACK-FILE']))['loopdevices']
    for loop in loops:
        backing = Path(loop['back-file'])
        if backing.parent.parent == work and BUILD_ID.fullmatch(backing.parent.name) and backing.name == 'image.img':
            command(['losetup', '--detach', loop['name']])
    for child in work.iterdir():
        if child.is_dir() and BUILD_ID.fullmatch(child.name):
            # Refuse deletion while a mount remains visible in this namespace.
            mountinfo = Path('/proc/self/mountinfo').read_text()
            if str(child) + '/' in mountinfo:
                raise RuntimeError('Temporärer Mount noch aktiv: ' + str(child))
            shutil.rmtree(child)


def build(state, key):
    request = json_file(state / 'requests' / (key + '.json'), {})
    req = validate_image(request, request.get('allowed_networks'))
    recover_work(state)
    preflight(state)
    work = state / 'work' / key
    work.mkdir(mode=0o700)
    image, root = work / 'image.img', work / 'root'
    cache = state / 'cache'
    digest = recipe_key(req['commit'])
    base = cache / (digest + '.img')
    versions_file = cache / (digest + '.json')
    try:
        if not base.exists() or not versions_file.exists():
            compressed = download_base(cache)
            print('Basisimage entpacken und Build-Dateisystem vorbereiten.', flush=True)
            expand_base(compressed, image)
            parts = partition_table(image)
            with image.open('r+b') as file:
                file.truncate(WORK_GIB * GIB)
            resize_partition(image, parts[1]['start'], WORK_GIB * GIB // 512 - parts[1]['start'])
            with Disk(image, root) as disk:
                fsck(disk.loop + 'p2')
                command(['resize2fs', disk.loop + 'p2'], timeout=300)
                disk.rootfs()
                install_support(root)
                with disk.guest():
                    guest_dns(root)
                    print('ARM64-Build auf der VM: Pakete, SoapySX, Codec, NetCore. Der erste Build kann mehrere Stunden dauern.', flush=True)
                    command(['chroot', root, '/bin/bash', '/usr/local/lib/netcore-image/build-guest.sh',
                             REPOSITORY, req['commit'], BASE['soapy_repository'], BASE['soapy_commit']], timeout=22 * 3600)
                versions = json_file(root / 'usr/share/netcore-image/versions.json', {})
                if versions.get('commit') != req['commit']:
                    raise RuntimeError('Build-Version konnte nicht verifiziert werden')
                clean_identity(root)
            shrink(image, root)
            # Immutable generalized cache contains no station/user/network secrets.
            os.replace(image, base)
            atomic_write(versions_file, json.dumps(versions))
        else:
            print('Vorbereitete Softwarebasis aus Cache: ' + digest[:12], flush=True)
        command(['cp', '--reflink=auto', '--sparse=always', base, image], timeout=300)
        print('TBS-Profil, Netzwerk und Betriebssystem-Zugang einrichten.', flush=True)
        with Disk(image, root) as disk:
            disk.rootfs()
            install_support(root)
            profile_path = root / 'root/netcore-image-request.json'
            atomic_write(profile_path, json.dumps(req))
            try:
                with disk.guest():
                    command(['chroot', root, '/usr/bin/python3', '/usr/local/lib/netcore-image/personalize.py'])
            finally:
                profile_path.unlink(missing_ok=True)
            clean_identity(root)
        with Disk(image, root) as disk:
            fsck(disk.loop + 'p2')
        print('Flashbares Image komprimieren; danach SHA256 und Manifest erzeugen.', flush=True)
        command(['xz', '-T2', '-3', '--keep', image], timeout=3600)
        artifact = image.with_suffix('.img.xz')
        filename = f"netcore-{req['hostname']}-{req['commit'][:12]}-{key[:8]}.img.xz"
        checksum = sha256(artifact)
        manifest = dict(id=key, filename=filename, profile=req['profile'], hostname=req['hostname'],
                        commit=req['commit'], created=time.time(), base=BASE, recipe=digest,
                        versions=json_file(versions_file, {}), sha256=checksum,
                        size_bytes=artifact.stat().st_size, uncompressed_bytes=image.stat().st_size,
                        overlay=req['overlay'], boot_tested=False,
                        note='Auf der VM vorinstalliert. Ein physischer Pi-/SDR-Start ist damit noch nicht geprüft.')
        pending = work / 'artifact'
        pending.mkdir()
        os.replace(artifact, pending / 'image.img.xz')
        atomic_write(pending / 'image.sha256', checksum + '  ' + filename + '\n')
        atomic_write(pending / 'manifest.json', json.dumps(manifest, indent=2))
        os.replace(pending, state / 'artifacts' / key)
        print('Fertig: ' + filename + ' · SHA256 ' + checksum, flush=True)
    finally:
        if not any(str(work) + '/' in line for line in Path('/proc/self/mountinfo').read_text().splitlines()):
            shutil.rmtree(work)


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--state', required=True)
    p.add_argument('--id', required=True)
    args = p.parse_args()
    if not BUILD_ID.fullmatch(args.id):
        p.error('Ungültige Build-ID')
    os.environ['LC_ALL'] = 'C'
    def interrupted(*_):
        raise InterruptedError('Image-Build abgebrochen')
    signal.signal(signal.SIGTERM, interrupted)
    build(Path(args.state).resolve(), args.id)


if __name__ == '__main__':
    main()
