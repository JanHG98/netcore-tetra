#!/usr/bin/env python3
"""Exercise image-directory ownership with real systemd on a disposable CI VM.

Run BEFORE the ARM64 unshare/chroot test, while PID 1 is the native systemd:
sudo /usr/bin/python3 image_runtime_dirs_smoke.py --disposable-runner
The renamed image unit keeps production users and directory directives. Only
dependencies, the hardware precheck and RF command are removed or replaced.
"""
import grp
import hashlib
import json
import os
from pathlib import Path
import pwd
import shutil
import stat
import subprocess
import sys
import tempfile
import uuid

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import ROOT
from deploy import install_dropin

STATE = [Path('/var/lib/netcore-tbs'), Path('/var/lib/netcore/recordings'),
         Path('/var/lib/netcore/audio'), Path('/var/lib/netcore-discovery-tbs')]
CACHE = [Path('/var/cache/netcore/audio')]
LEAVES = STATE + CACHE
PARENTS = [Path('/var/lib/netcore'), Path('/var/cache/netcore')]


def require(condition, message):
    if not condition:
        raise RuntimeError(message)


def call(*args, check=True):
    return subprocess.run(args, check=check, capture_output=True, text=True, timeout=45)


def snapshot(path):
    metadata = path.stat()
    return (metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode),
            hashlib.sha256(path.read_bytes()).hexdigest() if path.is_file() else None)


def main():
    require(sys.argv[1:] == ['--disposable-runner'] and os.geteuid() == 0,
            'Requires root and --disposable-runner on an empty CI VM.')
    require(Path('/proc/1/comm').read_text().strip() == 'systemd',
            'Native systemd PID 1 required; do not run in the ARM64 unshare/chroot.')
    for path in LEAVES:
        require(not path.exists() and not path.is_symlink(),
                'Refusing existing production runtime path: ' + str(path))
    for path in PARENTS:
        require(not path.exists() and not path.is_symlink(),
                'First-start scenario requires absent shared parent: ' + str(path))

    unit = 'netcore-image-runtime-ci-' + uuid.uuid4().hex + '.service'
    runtime = Path('/run/systemd/system')
    fragment, dropins = runtime / unit, runtime / (unit + '.d')
    created_groups, created_parents = [], list(PARENTS)
    created_user = False
    private = None
    old_umask = os.umask(0o077)
    try:
        for name in ('netcore', 'audio', 'spi', 'gpio'):
            try:
                grp.getgrnam(name)
            except KeyError:
                call('groupadd', '--system', name)
                created_groups.append(name)
        try:
            account = pwd.getpwnam('netcore')
        except KeyError:
            call('useradd', '--system', '--gid', 'netcore', '--groups', 'audio,spi,gpio',
                 '--home-dir', '/var/lib/netcore-tbs', '--shell', '/usr/sbin/nologin', 'netcore')
            created_user = True
            account = pwd.getpwnam('netcore')
        require(account.pw_uid != 0, 'netcore must not be root.')
        group_id = grp.getgrnam('netcore').gr_gid
        with tempfile.TemporaryDirectory(prefix='netcore-image-runtime-ci-', dir='/run') as temp:
            work = Path(temp)
            work.chmod(0o755)
            probe = work / 'probe.py'
            marker = STATE[0] / '.image-ci-result.json'
            retained = STATE[1] / '.image-ci-retained-media.bin'
            # This workload executes as the actual production service user.
            # The real systemd must create all directories before it can run.
            probe.write_text('''import json, os
from pathlib import Path
import stat
states = ''' + repr([str(path) for path in STATE]) + '''
caches = ''' + repr([str(path) for path in CACHE]) + '''
uid, gid = ''' + repr((account.pw_uid, group_id)) + '''
assert (os.geteuid(), os.getegid()) == (uid, gid)
for name in states + caches:
    path = Path(name)
    mode = 0o700 if name in states else 0o750
    info = path.stat()
    assert not path.is_symlink()
    assert (info.st_uid, info.st_gid, stat.S_IMODE(info.st_mode)) == (uid, gid, mode), name
    source, renamed = path / '.image-ci-write', path / '.image-ci-renamed'
    source.write_bytes(b'actual service-user write')
    source.rename(renamed)
    assert renamed.read_bytes() == b'actual service-user write'
    renamed.unlink()
retained = Path(''' + repr(str(retained)) + ''')
if not retained.exists():
    retained.write_bytes(b'retained recording survives restart')
assert retained.read_bytes() == b'retained recording survives restart'
Path(''' + repr(str(marker)) + ''').write_text(json.dumps({'uid': uid, 'gid': gid, 'passed': True}))
''', encoding='utf-8')
            probe.chmod(0o644)
            # Dependencies cannot be cleared safely through drop-ins. Remove
            # them from this renamed copy so no real firstboot service runs.
            source = (ROOT / 'image/systemd/tetra.service').read_text()
            fragment.write_text('\n'.join(line for line in source.splitlines()
                                         if not line.startswith(('Requires=', 'After='))) + '\n', encoding='utf-8')
            fragment.chmod(0o644)
            install_dropin(dict(name='tbs', unit=unit, config_target='/etc/netcore/config.toml',
                                command='/usr/local/bin/bluestation-bs /etc/netcore/config.toml'),
                           Path('/var/lib/netcore-discovery/endpoints.json'), root=runtime)
            verifier = dropins / '90-ci-verifier.conf'
            verifier.write_text('[Service]\nExecStartPre=\nExecStart=\nExecStart=/usr/bin/python3 '
                                + json.dumps(str(probe)) + '\n', encoding='utf-8')
            verifier.chmod(0o644)
            call('systemctl', 'daemon-reload')
            directory_metadata = None
            retained_snapshot = None
            protected = None
            for attempt in range(2):
                marker.unlink(missing_ok=True)
                call('systemctl', '--wait', 'start', unit)
                require(marker.is_file() and json.loads(marker.read_text()).get('passed') is True,
                        'Service-user directory roundtrip did not complete.')
                current = {path: snapshot(path) for path in LEAVES}
                if attempt == 0:
                    # The first real start creates missing shared parents.
                    # The second covers existing root-owned 0755 parents and
                    # a private sibling that must never be chowned/broadened.
                    require(all(not path.is_symlink() and snapshot(path)[:3] == (0, 0, 0o755)
                                for path in PARENTS), 'Shared-parent ownership/mode is not root:root 0755.')
                    private = Path('/var/lib/netcore') / ('.image-ci-private-' + uuid.uuid4().hex)
                    private.mkdir(mode=0o700)
                    secret = private / 'unrelated.bin'
                    secret.write_bytes(b'private unrelated state must remain unchanged\n')
                    secret.chmod(0o600)
                    protected = {path: snapshot(path) for path in [*PARENTS, private, secret]}
                    directory_metadata, retained_snapshot = current, snapshot(retained)
                else:
                    require(all(snapshot(path) == before for path, before in protected.items()),
                            'Unrelated shared-parent state was modified.')
                    require(current == directory_metadata and snapshot(retained) == retained_snapshot,
                            'Restart changed existing directory metadata or retained media.')
            print('PASS: real systemd creates image recording/audio state and cache for netcore; '
                  'discovery StateDirectory append/0700, write/rename/delete, repeat start, retained media '
                  'and unrelated private state verified. No RF command or Pi boot tested.', flush=True)
    except BaseException:
        if fragment.exists():
            logs = call('journalctl', '-u', unit, '--no-pager', '-n', '60', check=False)
            print(logs.stdout, file=sys.stderr, flush=True)
        raise
    finally:
        # Exactly the paths proved absent before this test; never delete a
        # shared parent recursively or follow a substituted leaf symlink.
        if fragment.exists():
            call('systemctl', 'stop', unit, check=False)
            call('systemctl', 'reset-failed', unit, check=False)
        for path in reversed(LEAVES):
            if path.exists() or path.is_symlink():
                require(not path.is_symlink() and path.resolve() == path,
                        'Unsafe cleanup target: ' + str(path))
                shutil.rmtree(path)
        if private is not None and private.exists():
            require(not private.is_symlink(), 'Unsafe private fixture cleanup.')
            shutil.rmtree(private)
        if dropins.exists():
            shutil.rmtree(dropins)
        fragment.unlink(missing_ok=True)
        call('systemctl', 'daemon-reload', check=False)
        for path in reversed(created_parents):
            if path.exists():
                require(not path.is_symlink() and path.resolve() == path,
                        'Unsafe shared-parent cleanup target: ' + str(path))
                path.rmdir()
        if created_user:
            call('userdel', 'netcore')
        for name in reversed(created_groups):
            # userdel can remove the matching private primary group itself.
            try:
                grp.getgrnam(name)
            except KeyError:
                continue
            call('groupdel', name)
        os.umask(old_umask)


if __name__ == '__main__':
    main()
