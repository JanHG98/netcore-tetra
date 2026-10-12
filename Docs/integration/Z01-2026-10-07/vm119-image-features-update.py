#!/usr/bin/env python3
"""VM119: update the reviewed Pi image features and builder form while idle."""
import configparser
import hashlib
from html.parser import HTMLParser
import importlib.util
import json
import os
from pathlib import Path
import re
import shutil
import socket
import stat
import subprocess
import sys
import tempfile
import time
import tomllib

GUARD_PATH = Path(__file__).with_name('vm119-jobs-update.py')
GUARD_SHA = '50dfc518d9de6cffc5ec3335b3b94e6716d3c22de6653868f895386e64ca39db'
if GUARD_PATH.is_symlink() or hashlib.sha256(GUARD_PATH.read_bytes()).hexdigest() != GUARD_SHA:
    raise RuntimeError('Ungeprüfter VM119-Guard.')
SPEC = importlib.util.spec_from_file_location('vm119_recipe_guard', GUARD_PATH)
GUARD = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GUARD)

LIBRARY = Path('/usr/local/lib/netcore-deployment')
SOURCE = Path(__file__).resolve().parents[3] / 'system-backend/deployment-core'
INVARIANTS = {
    'image_worker.py': '343c84141cb5e747d9a3c6632ac40f17d82f981518d2b5108c7bd6e92d9c4900',
    'image/gpio-compat.py': '133c4d6d433c67ff5aee0726d487282faf19ad370d38da710fc3b883153d33e0',
}
# Payload bytes are from the separately CI-validated source commit below.
# The helper can be checked out at a later commit; each payload blob is verified by SHA-256.
SOURCE_COMMIT = 'e842197c2ede8337267a913304816da11664f7a8'
PINS = {
    "image_spec.py": {
        "old": [
            "5d5eb6e7cb98b72d728a543c48cf0ade6a000360bb941bdfbffebe33efabc801"
        ],
        "new": "1983b7ac625e9e0c8005907d909e9a565a05a10f5f0550511fe2ce989551c2c6"
    },
    "image_build.py": {
        "old": [
            "ecb33c14e7e26c38d28e48ee3bdfae71e64836b368bed70a9daa24da2ce218cc"
        ],
        "new": "b97db194df99a7a4974d175dbe17d92897f30c0e2aa4f3ad3a797b245e0ea925"
    },
    "image/personalize.py": {
        "old": [
            "f19f0053b3b7d4680cf1e2cc423908fd07674688419ef4d8a4660641bf9bf22f"
        ],
        "new": "ff09f79d2e418f04f4cde11046b5cc1a6757d6c78229105caa7fb3a9ab53ce19"
    },
    "image/build-guest.sh": {
        "old": [
            "e6f992907060f7d20378ff1f1c1ca49618c4fa712c9c2dce3a64065bf62e1347"
        ],
        "new": "7cf89b99082261a612ecb2494489ad98aa7cb6890527a060e9589744409af2e6"
    },
    "image/systemd/tetra.service": {
        "old": [
            "97ba337cb6d1ea6db1f659b8451b3fb6e95f30e24701401e4a3830c6d8170319",
            "54cc9b2e88899621f03e22a7c5506d188ffa4af149bfbe0600284b0c122ce7f0"
        ],
        "new": "54cc9b2e88899621f03e22a7c5506d188ffa4af149bfbe0600284b0c122ce7f0"
    },
    "image/desktop-setup.sh": {
        "old": [
            None
        ],
        "new": "e8ec3eab0fde3bb654a09a9edeefc1ef47e0087014996c5ffab8e45efa6968b7"
    },
    "image/sip_setup.py": {
        "old": [
            None
        ],
        "new": "9d10f4e35e6610fc1d13dcd1b5ef60b63471d7740e4b5d29f88f8f021d2a5782"
    },
    "image/49-netcore-networkmanager.rules": {
        "old": [
            None
        ],
        "new": "d035b25eb4e0111efebaabbb9f428cb040c55a1295de3304ac2d16e726baade2"
    },
    "static/index.html": {
        "old": [
            "c2621415484332ec50b4d9254d500ad53f8e7895245ab3ac051b625ca13e8771"
        ],
        "new": "6d85c64fd9f5a9f05ce3a945526d2b0c8dd87a74fe38d28b973b95e408ca392f"
    },
    "static/app.js": {
        "old": [
            "8c9d0fdd37028d084e0f44d614f886446394c9baba210cd6e46f4d550fda1adf"
        ],
        "new": "28f44e97d311733065e842c7a68c84ac96086809ad03d5bb62cb8744cb9478cb"
    }
}


def digest(body):
    return hashlib.sha256(body).hexdigest() if body is not None else None


def attributes(metadata):
    return (metadata.st_uid, metadata.st_gid, stat.S_IMODE(metadata.st_mode)) if metadata else (0, 0, 0o644)


def sync_directory(path):
    descriptor = os.open(path, os.O_RDONLY | getattr(os, 'O_DIRECTORY', 0))
    try:
        os.fsync(descriptor)
    finally:
        os.close(descriptor)


def replace(path, body, metadata):
    guarded_parent(path, LIBRARY)
    if body is None:
        if path.exists():
            path.unlink()
            sync_directory(path.parent)
        return
    descriptor, staging = tempfile.mkstemp(prefix='.image-features-', dir=path.parent)
    try:
        with os.fdopen(descriptor, 'wb') as output:
            uid, gid, mode = attributes(metadata)
            os.fchown(output.fileno(), uid, gid)
            os.fchmod(output.fileno(), mode)
            output.write(body)
            output.flush()
            os.fsync(output.fileno())
        os.replace(staging, path)
        sync_directory(path.parent)
    finally:
        if os.path.exists(staging):
            os.unlink(staging)


def idle(state, allow_checks=False):
    GUARD.database_idle(state, allow_checks)
    GUARD.database_idle(GUARD.BUILD_STATE)


class BundleHTML(HTMLParser):
    VOID = {'area', 'base', 'br', 'col', 'embed', 'hr', 'img', 'input', 'link',
            'meta', 'param', 'source', 'track', 'wbr'}

    def __init__(self):
        super().__init__(convert_charrefs=True)
        self.stack, self.ids, self.tags = [], set(), set()
        self.app_script = False

    def handle_starttag(self, tag, attrs):
        self.tags.add(tag)
        attrs = dict(attrs)
        if 'id' in attrs:
            GUARD.require(attrs['id'] not in self.ids, 'Doppelte HTML-ID: ' + attrs['id'])
            self.ids.add(attrs['id'])
        if tag == 'script' and attrs.get('src') in ('app.js', '/app.js'):
            self.app_script = True
        if tag not in self.VOID:
            self.stack.append(tag)

    def handle_startendtag(self, tag, attrs):
        self.handle_starttag(tag, attrs)
        if tag not in self.VOID:
            self.handle_endtag(tag)

    def handle_endtag(self, tag):
        GUARD.require(self.stack and self.stack[-1] == tag, 'Nicht ausgeglichener HTML-Abschluss: ' + tag)
        self.stack.pop()

    def verify(self):
        self.close()
        GUARD.require(not self.stack and {'html', 'head', 'body'} <= self.tags,
                      'Unvollständiges HTML-Dokument.')
        GUARD.require({'image-form', 'image-desktop'} <= self.ids and self.app_script,
                      'Imageformular / Desktopfeld / App-Script fehlt.')


def source_valid(path, body):
    suffix = path.suffix
    if suffix == '.py':
        compile(body, str(path), 'exec')
        return 'python-compile'
    if suffix == '.sh':
        GUARD.command('bash', '-n', str(path))
        return 'bash-n'
    if suffix in ('.js', '.rules'):
        node = shutil.which('node')
        if node:
            subprocess.run([node, '--check', '--input-type=commonjs'], input=body,
                           check=True, timeout=40)
            return 'node-check'
        # Both blobs are exact SHA-pinned payloads and must pass native CI
        # before this updater is offered. The VM needs no JavaScript runtime.
        return 'sha256-pinned; JavaScript-Syntax in nativer CI geprüft'
    if suffix == '.html':
        document = BundleHTML()
        document.feed(body.decode('utf-8'))
        document.verify()
        return 'html-parser; IDs und Dokumentstruktur'
    if suffix == '.service':
        unit = configparser.ConfigParser(interpolation=None, strict=False)
        unit.optionxform = str
        unit.read_string(body.decode('utf-8'))
        GUARD.require(unit.has_section('Unit') and unit.has_section('Install')
                      and unit.get('Service', 'User', fallback=None) == 'netcore'
                      and unit.get('Service', 'ExecStart', fallback='').startswith('/usr/local/bin/bluestation-bs '),
                      'Ungültiges Gast-TBS-Unit-Rezept.')
        return 'unit-parser; Gast-TBS'
    raise RuntimeError('Unbekannter Rezeptdateityp: ' + str(path))


def guarded_parent(path, root):
    GUARD.require(path.is_relative_to(root), 'Rezeptpfad außerhalb der Bibliothek.')
    parent = path.parent
    while True:
        GUARD.require(parent.is_dir() and not parent.is_symlink(), 'Ungültiger Rezeptordner: ' + str(parent))
        if parent == root:
            break
        parent = parent.parent


def invariants_current():
    for name, expected in INVARIANTS.items():
        path = LIBRARY / name
        guarded_parent(path, LIBRARY)
        GUARD.require(not path.is_symlink() and path.is_file(), 'Ungültiger unveränderter Rezeptpfad: ' + name)
        GUARD.require(digest(path.read_bytes()) == expected, 'Unveränderter Rezeptteil hat unerwarteten Stand: ' + name)


def verify_files(files, key):
    for name, record in files.items():
        path = LIBRARY / name
        guarded_parent(path, LIBRARY)
        body = record[key]
        GUARD.require(not path.is_symlink(), 'Rezeptdatei ist ein Symlink: ' + name)
        if body is None:
            GUARD.require(not path.exists(), 'Unerwartete neue Rezeptdatei: ' + name)
            continue
        GUARD.require(path.is_file() and path.read_bytes() == body, 'Rezeptbytes geändert: ' + name)
        metadata = path.stat()
        GUARD.require(attributes(metadata) == attributes(record['metadata']), 'Owner/Mode geändert: ' + name)


def wait_ready():
    deadline = time.monotonic() + 30
    while True:
        try:
            GUARD.ready()
            return
        except (OSError, RuntimeError, subprocess.SubprocessError):
            GUARD.require(time.monotonic() < deadline, 'Dienste nach Rezeptupdate nicht bereit.')
            time.sleep(1)


def update(state, protected):
    invariants_current()
    files, validation = {}, {}
    for name, pins in PINS.items():
        target, source = LIBRARY / name, SOURCE / name
        GUARD.require(re.fullmatch(r'[0-9a-f]{64}', pins['new']) is not None, 'Quellpin ist noch nicht final: ' + name)
        guarded_parent(target, LIBRARY)
        guarded_parent(source, SOURCE)
        GUARD.require(not target.is_symlink(), 'Rezeptziel ist ein Symlink: ' + name)
        metadata = target.stat() if target.exists() else None
        if metadata is not None:
            GUARD.require(stat.S_ISREG(metadata.st_mode), 'Rezeptziel ist keine Datei: ' + name)
        original = target.read_bytes() if metadata is not None else None
        GUARD.require(digest(original) in (*pins['old'], pins['new']), 'Unerwarteter installierter Rezeptstand: ' + name)
        GUARD.require(source.is_file() and not source.is_symlink(), 'Ungültige Rezeptquelle: ' + name)
        replacement = source.read_bytes()
        GUARD.require(digest(replacement) == pins['new'], 'Ungeprüfter Quellfingerprint: ' + name)
        validation[name] = source_valid(source, replacement)
        files[name] = {'original': original, 'replacement': replacement, 'metadata': metadata}
    baseline = GUARD.snapshot(protected)
    GUARD.ready()
    GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'))
    GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
    idle(state)
    changed_names = [name for name, record in files.items() if record['original'] != record['replacement']]
    if not changed_names:
        return {'phase': 'already_current', 'changed_files': [], 'validation': validation,
                'protected_sha256': baseline}

    stopping, attempted = False, []
    try:
        verify_files(files, 'original')
        invariants_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Stopp geändert.')
        idle(state)
        stopping = True
        GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state)
        GUARD.command('systemctl', 'stop', GUARD.BUILDER)
        verify_files(files, 'original')
        invariants_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien vor Austausch geändert.')
        for name in changed_names:
            attempted.append(name)  # A rename may succeed before a later fsync fails.
            record = files[name]
            replace(LIBRARY / name, record['replacement'], record['metadata'])
        GUARD.command('systemctl', 'start', GUARD.BUILDER)
        GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
        wait_ready()
        verify_files(files, 'replacement')
        invariants_current()
        GUARD.require(GUARD.snapshot(protected) == baseline, 'Standortdateien nach Update geändert.')
        GUARD.require_jobs_idle(GUARD.api('/api/v1/jobs'), allow_checks=True)
        GUARD.require_jobs_idle(GUARD.builder_status()['jobs'])
        idle(state, allow_checks=True)
        return {'phase': 'passed', 'changed_files': changed_names, 'files': [
            {'file': str(LIBRARY / name), 'sha256_before': digest(record['original']),
             'sha256_after': digest(record['replacement'])} for name, record in files.items()],
            'protected_sha256': baseline, 'validation': validation}
    except BaseException:
        if stopping:
            try:
                if attempted:
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.CONTROLLER)
                    idle(state, allow_checks=True)
                    GUARD.command('systemctl', 'stop', GUARD.BUILDER)
                    for name in reversed(attempted):
                        record, target = files[name], LIBRARY / name
                        guarded_parent(target, LIBRARY)
                        GUARD.require(not target.is_symlink(), 'Rückwegziel ist ein Symlink: ' + name)
                        actual = target.read_bytes() if target.exists() else None
                        GUARD.require(actual in (record['original'], record['replacement']), 'Fremde Rezeptänderung vor Rückweg: ' + name)
                        replace(target, record['original'], record['metadata'])
                    verify_files(files, 'original')
                GUARD.command('systemctl', 'start', GUARD.BUILDER)
                GUARD.command('systemctl', 'start', GUARD.CONTROLLER)
                print('Rückweg: vorheriges Rezept wiederhergestellt; Dienste gestartet.', file=sys.stderr)
            except BaseException as rollback_error:
                print('Rückweg nicht vollständig: ' + str(rollback_error), file=sys.stderr)
        raise


def main():
    GUARD.require(len(sys.argv) == 1 and os.geteuid() == 0 and socket.gethostname() == 'VM-H-DEPLOY-01',
                  'Nur per sudo/root auf VM-H-DEPLOY-01; keine Operator-Overrides.')
    cfg = tomllib.loads(GUARD.CONFIG.read_text())
    GUARD.require(cfg['role'] == 'controller' and cfg['node_id'] == 'VM-H-DEPLOY-01', 'Falsche Controlleridentität.')
    state = Path(cfg.get('state_dir', '/var/lib/netcore-deployment'))
    GUARD.require(state == Path('/var/lib/netcore-deployment'), 'Unerwartetes Controller-State-Verzeichnis.')
    protected = [GUARD.CONFIG, state / 'settings.json', state / 'profiles.json', state / 'tbs-site-template.toml']
    print(json.dumps(update(state, protected), indent=2))


if __name__ == '__main__':
    main()
