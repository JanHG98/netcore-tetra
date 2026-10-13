#!/usr/bin/env python3
"""Root-only image worker; fixed recipes, independent queue, Unix socket API."""
import argparse
import fcntl
import grp
import json
import logging
import os
from pathlib import Path
import re
import shutil
import signal
import socketserver
import threading
import uuid
from http.server import BaseHTTPRequestHandler
from urllib.parse import urlsplit

from common import ROOT, atomic_write, json_file
from deploy import run
from image_spec import BASE, BUILD_ID, NETWORKS, public_job, validate_image
from jobs import Jobs


def byte_range(value, size):
    if not value:
        return 0, size - 1, 200
    match = re.fullmatch(r'bytes=(\d*)-(\d*)', value)
    if not match or not any(match.groups()):
        raise ValueError('Ungültiger Download-Bereich')
    start, end = match.groups()
    if not start:
        start, end = max(0, size - int(end)), size - 1
    else:
        start, end = int(start), min(int(end) if end else size - 1, size - 1)
    if not 0 <= start <= end < size:
        raise ValueError('Download-Bereich außerhalb der Datei')
    return start, end, 206


class Worker:
    def __init__(self, state, networks=None):
        self.state = Path(state).resolve()
        self.state.mkdir(parents=True, exist_ok=True, mode=0o700)
        self.networks = networks or NETWORKS
        self.build_lock = threading.Lock()
        for name in ('requests', 'artifacts', 'cache', 'work'):
            (self.state / name).mkdir(exist_ok=True, mode=0o700)
        # A second process must not mark a running build interrupted or touch its mounts.
        self.lock_file = (self.state / 'worker.lock').open('w')
        fcntl.flock(self.lock_file, fcntl.LOCK_EX | fcntl.LOCK_NB)
        self.jobs = Jobs(self.state, self.execute)

    def submit(self, data):
        request = validate_image(data, self.networks)
        request['build_id'] = uuid.uuid4().hex
        return public_job(self.jobs.submit(request))

    def artifacts(self):
        result = []
        for directory in (self.state / 'artifacts').iterdir():
            if BUILD_ID.fullmatch(directory.name) and (directory / 'image.img.xz').is_file():
                manifest = json_file(directory / 'manifest.json', {})
                if manifest.get('id') == directory.name:
                    result.append(manifest)
        return sorted(result, key=lambda a: a['created'], reverse=True)

    def status(self):
        jobs = [public_job(j) for j in self.jobs.list()[:30]]
        for job in jobs:
            job['log'] = job['log'][-20000:]
            job['result'].pop('versions', None)
        artifacts = [{k: v for k, v in a.items() if k != 'versions'} for a in self.artifacts()]
        return {'available': True, 'base': BASE, 'free_bytes': shutil.disk_usage(self.state).free,
                'jobs': jobs, 'artifacts': artifacts,
                'active_profiles': self.jobs.active_profile_names()}

    def execute(self, request, log):
        key = request['build_id']
        path = self.state / 'requests' / (key + '.json')
        with self.build_lock:
            atomic_write(path, json.dumps(request))
            try:
                log('Image-Auftrag ' + key + ' · ' + request['profile']['name'])
                # A private mount and PID namespace contains all temporary guest mounts/processes.
                run(['unshare', '--mount', '--pid', '--fork', '--kill-child', '--propagation', 'private',
                     'python3', str(ROOT / 'image_build.py'), '--state', str(self.state), '--id', key],
                    log, timeout=24 * 3600)
                manifest = json_file(self.state / 'artifacts' / key / 'manifest.json', {})
                if manifest.get('id') != key:
                    raise RuntimeError('Build ohne vollständiges Artefakt beendet')
                return manifest
            finally:
                path.unlink(missing_ok=True)

    def artifact(self, key, kind):
        if not BUILD_ID.fullmatch(key):
            raise ValueError('Ungültige Image-ID')
        files = {'image': ('image.img.xz', 'application/x-xz'),
                 'sha256': ('image.sha256', 'text/plain'), 'manifest': ('manifest.json', 'application/json')}
        if kind not in files:
            raise KeyError(kind)
        filename, content_type = files[kind]
        directory = self.state / 'artifacts' / key
        manifest = json_file(directory / 'manifest.json', {})
        if manifest.get('id') != key or not (directory / filename).is_file():
            raise KeyError(key)
        download = manifest['filename'] if kind == 'image' else manifest['filename'] + ('.sha256' if kind == 'sha256' else '.json')
        return directory / filename, download, content_type

    def remove(self, key):
        self.artifact(key, 'manifest')
        shutil.rmtree(self.state / 'artifacts' / key)
        return {'deleted': key}


class Server(socketserver.ThreadingMixIn, socketserver.UnixStreamServer):
    daemon_threads = True


def handler(worker):
    class Handler(BaseHTTPRequestHandler):
        def log_message(self, fmt, *args):
            logging.debug(fmt, *args)

        def send_json(self, value, code=200):
            body = json.dumps(value).encode()
            self.send_response(code)
            self.send_header('Content-Type', 'application/json')
            self.send_header('Content-Length', str(len(body)))
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):
            self.dispatch(False)

        def do_POST(self):
            self.dispatch(True)

        def dispatch(self, post):
            self.connection.settimeout(60)
            path = urlsplit(self.path).path
            try:
                if post:
                    size = int(self.headers.get('Content-Length', 0))
                    if not 0 < size <= 262144:
                        raise ValueError('Ungültige Request-Größe')
                    data = json.loads(self.rfile.read(size))
                    if not isinstance(data, dict):
                        raise ValueError('JSON-Objekt erforderlich')
                    if path == '/build':
                        return self.send_json(worker.submit(data), 202)
                    if path == '/remove':
                        return self.send_json(worker.remove(data.get('id', '')))
                elif path == '/status':
                    return self.send_json(worker.status())
                elif path.startswith('/artifacts/'):
                    parts = path.split('/')
                    if len(parts) != 4:
                        raise KeyError(path)
                    file, name, content_type = worker.artifact(parts[2], parts[3])
                    with file.open('rb') as source:
                        size = os.fstat(source.fileno()).st_size
                        try:
                            start, end, code = byte_range(self.headers.get('Range'), size)
                        except ValueError:
                            self.send_response(416)
                            self.send_header('Content-Range', f'bytes */{size}')
                            self.send_header('Content-Length', '0')
                            self.end_headers()
                            return
                        self.send_response(code)
                        self.send_header('Content-Type', content_type)
                        self.send_header('Content-Disposition', f'attachment; filename="{name}"')
                        self.send_header('Accept-Ranges', 'bytes')
                        self.send_header('Content-Length', str(end - start + 1))
                        if code == 206:
                            self.send_header('Content-Range', f'bytes {start}-{end}/{size}')
                        self.end_headers()
                        source.seek(start)
                        remaining = end - start + 1
                        while remaining:
                            block = source.read(min(1024 * 1024, remaining))
                            if not block:
                                break
                            self.wfile.write(block)
                            remaining -= len(block)
                    return
                self.send_json({'error': 'Not found'}, 404)
            except KeyError:
                self.send_json({'error': 'Image nicht vorhanden'}, 404)
            except (ValueError, TypeError, OSError) as exc:
                self.send_json({'error': str(exc)}, 400)
            except Exception:
                logging.exception('Image worker request failed')
                self.send_json({'error': 'Imagebuilder-Fehler; Dienstprotokoll prüfen'}, 500)
    return Handler


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--state', default='/var/lib/netcore-image-builder')
    p.add_argument('--socket', default='/run/netcore-image-builder/api.sock')
    p.add_argument('--config', default='/etc/netcore/deployment.toml')
    args = p.parse_args()
    if os.geteuid() != 0:
        p.error('Imagebuilder benötigt root und läuft ausschließlich über seinen lokalen Unix-Socket')
    from common import load_config
    worker = Worker(args.state, load_config(args.config)['allowed_networks'])
    path = Path(args.socket)
    path.parent.mkdir(parents=True, exist_ok=True)
    path.unlink(missing_ok=True)
    server = Server(str(path), handler(worker))
    os.chown(path, 0, grp.getgrnam('netcore-deploy').gr_gid)
    os.chmod(path, 0o660)
    def stop(*_):
        threading.Thread(target=server.shutdown, daemon=True).start()
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    try:
        server.serve_forever()
    finally:
        server.server_close()
        path.unlink(missing_ok=True)


if __name__ == '__main__':
    main()
