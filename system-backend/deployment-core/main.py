#!/usr/bin/env python3
"""NetCore OpenLab Deployment Core / per-host Discovery Agent."""
from __future__ import annotations
import argparse
from concurrent.futures import ThreadPoolExecutor
import json
import logging
import os
from pathlib import Path
import shlex
import signal
import subprocess
import threading
import time
import tomllib
from http.client import HTTPException
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
from urllib.error import HTTPError
from urllib.parse import parse_qs, urlsplit

from common import (CATALOG, PROTOCOL, ROOT, VERSION, atomic_write, identifier,
                    json_file, load_config, peer_url, request_json)
from deploy import (Deployer, Repository, installed_service, probe, service_spec, tbs_config,
                    validate_job, validate_profile)
from discovery import Discovery
from jobs import Jobs, RemoteJobUncertain
from image_client import ImageClient
from image_spec import BUILD_ID, validate_image


REMOTE_REQUEST_TIMEOUT = 15
REMOTE_RETRY_SECONDS = 300
REMOTE_POLL_INTERVAL = 2
REMOTE_JOB_SECONDS = 7200


def public_openapi():
    """Read-only management contract shared by controller and host agents."""
    descriptions = {
        '/health/live': 'Controller/agent process liveness',
        '/health/ready': 'Local management API readiness; independent of radio peers',
        '/api/v1/status': 'Discovery state and management status',
        '/api/v1/manifest': 'Local host roles without configuration secrets',
        '/api/v1/peers': 'Known peers and cached dependency endpoints',
        '/api/v1/catalog': 'Supported managed service roles',
        '/api/v1/jobs': 'Persistent deployment jobs and bounded logs',
    }
    paths = {path: {'get': {'summary': summary, 'responses': {
        '200': {'description': 'Success', 'content': {'application/json': {'schema': {'type': 'object' if path not in ('/api/v1/catalog', '/api/v1/jobs') else 'array'}}}}}}}
        for path, summary in descriptions.items()}
    for path in ('/api/v1/catalog', '/api/v1/jobs'):
        paths[path]['get']['responses']['200']['content']['application/json']['schema']['items'] = {'type': 'object'}
    paths['/metrics'] = {'get': {'summary': 'Prometheus metrics from cached management state',
        'responses': {'200': {'description': 'Success', 'content': {'text/plain': {'schema': {'type': 'string'}}}}}}}
    return {'openapi': '3.0.3', 'info': {'title': 'NetCore deployment and discovery', 'version': VERSION},
            'paths': paths}


class App:
    def __init__(self, cfg):
        self.cfg = cfg
        self.state = Path(cfg['state_dir'])
        self.state.mkdir(parents=True, exist_ok=True)
        self.lock = threading.RLock()
        self.settings = json_file(self.state / 'settings.json', {})
        for key in ('seeds', 'bindings', 'ref'):
            if key in self.settings:
                cfg[key] = self.settings[key]
        self.local = []
        self.desired = json_file(self.state / 'desired.json', {})
        self.profiles = json_file(self.state / 'profiles.json', {})
        self.discovery = Discovery(cfg, self.manifest)
        self.repo = Repository(cfg)
        self.deployer = Deployer(cfg, self.discovery, self.repo)
        self.jobs = Jobs(self.state, self.execute)
        self.images = ImageClient(cfg.get('image_builder_socket', '/run/netcore-image-builder/api.sock'))
        self.stop = threading.Event()

    def start(self):
        self.refresh_local()
        self.discovery.start()
        threading.Thread(target=self._monitor, daemon=True).start()

    def _monitor(self):
        next_poll = 0
        while not self.stop.is_set():
            self.refresh_local()
            if self.cfg['role'] == 'controller' and time.time() >= next_poll:
                if not any(j['status'] in ('queued', 'running') for j in self.jobs.list()):
                    self.jobs.submit({'kind': 'check', 'ref': self.cfg['ref']})
                next_poll = time.time() + max(60, self.cfg.get('poll_seconds', 3600))
            self.stop.wait(10)

    def refresh_local(self):
        def inspect(name):
            spec = service_spec(self.cfg, name)
            if not installed_service(spec):
                return None
            ready = available = False
            try:
                ready = probe(spec)
            except (OSError, ValueError):
                pass
            if ready:
                available = True
            else:
                try:
                    available = probe(spec, live=True)
                except (OSError, ValueError):
                    pass
            marker = json_file(self.state / ('deployed-' + name + '.json'), {})
            return {'name': name, 'port': spec['port'], 'ready': ready, 'available': available,
                    'commit': marker.get('commit', ''), 'unit': spec['unit']}
        if self.cfg['role'] == 'agent':
            with ThreadPoolExecutor(max_workers=8) as pool:
                found = [v for v in pool.map(inspect, self.cfg.get('managed_services', CATALOG)) if v]
            with self.lock:
                self.local = found

    def manifest(self):
        with self.lock:
            return {'protocol': PROTOCOL, 'environment': self.cfg['environment'],
                    'security_mode': 'open_lab', 'node_id': self.cfg['node_id'],
                    'role': self.cfg['role'], 'version': VERSION, 'services': list(self.local)}

    def status(self):
        return dict(self.discovery.snapshot(), **self.manifest(), desired=self.desired,
                    settings={k: self.cfg[k] for k in ('seeds', 'bindings', 'ref')},
                    advertise_url=self.cfg.get('advertise_url', ''),
                    has_template=(self.state / 'tbs-site-template.toml').is_file())

    def metrics(self):
        # Scrapes never trigger discovery, health probes, Git, or deployments.
        peers = self.discovery.snapshot()['peers']
        jobs = self.jobs.list()
        lines = [
            '# HELP netcore_deployment_up Management API process is running.',
            '# TYPE netcore_deployment_up gauge',
            'netcore_deployment_up 1',
            '# HELP netcore_deployment_peers Cached peer count by reachability.',
            '# TYPE netcore_deployment_peers gauge',
        ]
        for state, online in (('online', True), ('offline', False)):
            count = sum(peer['online'] is online for peer in peers)
            lines.append(f'netcore_deployment_peers{{state="{state}"}} {count}')
        lines.extend(['# HELP netcore_deployment_jobs Job count in the bounded status listing.',
                      '# TYPE netcore_deployment_jobs gauge'])
        for status in ('queued', 'running', 'succeeded', 'failed', 'interrupted'):
            count = sum(job['status'] == status for job in jobs)
            lines.append(f'netcore_deployment_jobs{{status="{status}"}} {count}')
        return '\n'.join(lines) + '\n'

    def execute(self, data, log):
        if data.get('kind') == 'check':
            sha = self.repo.resolve(data['ref'], log)
            tags = subprocess.check_output(['git', 'tag', '--sort=-version:refname'], cwd=self.repo.path,
                                           text=True, timeout=10).splitlines()[:30]
            self.desired = {'ref': data['ref'], 'commit': sha, 'checked_at': time.time(), 'tags': tags}
            atomic_write(self.state / 'desired.json', json.dumps(self.desired), 0o644)
            return self.desired
        if self.cfg['role'] == 'agent':
            result = self.deployer.execute(data, log)
            self.refresh_local()
            return result
        target = self.node(data['node_id'])
        # Resolve ONCE, then dispatch a full immutable commit, even when branch moves.
        remote = {k: data[k] for k in ('service', 'action', 'profile', 'confirm_restart') if k in data}
        if remote['action'] != 'restart':
            remote['commit'] = self.repo.resolve(data['ref'], log)
        if data.get('profile'):
            remote['profile'] = self.profiles[data['profile']]
        remote = validate_job(remote)
        jobs_url = target['agent_url'] + '/api/v1/jobs'
        try:
            # A lost POST response is ambiguous. Never submit the installer twice.
            response = request_json(jobs_url, remote, timeout=REMOTE_REQUEST_TIMEOUT)
            key = response['id']
            if not isinstance(key, str) or len(key) != 32 or any(c not in '0123456789abcdef' for c in key):
                raise ValueError('Ungültige Remote-Auftrags-ID')
        except HTTPError as exc:
            if 400 <= exc.code < 500:
                raise
            raise RemoteJobUncertain(f'Antwort beim Anlegen verloren: {exc}; Agent-Aufträge prüfen. '
                                     'Keine automatische Wiederholung.', jobs_url) from exc
        except (OSError, ValueError, KeyError, TypeError, HTTPException) as exc:
            raise RemoteJobUncertain(f'Antwort beim Anlegen verloren: {exc}; Agent-Aufträge prüfen. '
                                     'Keine automatische Wiederholung.', jobs_url) from exc
        job_url = jobs_url + '/' + key
        log(f'Remote-Auftrag {job_url}')
        deadline, last = time.monotonic() + REMOTE_JOB_SECONDS, ''
        unavailable_since = None
        while time.monotonic() < deadline:
            if self.stop.is_set():
                raise RemoteJobUncertain(f'Controller beendet; Remote-Auftrag {key} separat prüfen', job_url, key)
            try:
                result = request_json(job_url, timeout=REMOTE_REQUEST_TIMEOUT)
                if (not isinstance(result, dict) or result.get('status') not in
                        ('queued', 'running', 'succeeded', 'failed', 'interrupted') or
                        not isinstance(result.get('log'), str) or not isinstance(result.get('result'), dict)):
                    raise ValueError('Ungültiger Remote-Auftragsstatus')
            except (OSError, ValueError, HTTPException) as exc:
                # 4xx (except temporary 408/429) cannot be healed by polling.
                permanent = isinstance(exc, HTTPError) and 400 <= exc.code < 500 and exc.code not in (408, 429)
                now = time.monotonic()
                if unavailable_since is None:
                    unavailable_since = now
                    log(f'Status von {key} vorübergehend nicht erreichbar: {exc}. '
                        f'Erneute GET-Abfragen für bis zu {REMOTE_RETRY_SECONDS}s; kein neuer Installationsauftrag.')
                if permanent or now - unavailable_since >= REMOTE_RETRY_SECONDS:
                    raise RemoteJobUncertain(f'Remote-Auftrag {key} läuft möglicherweise weiter; '
                                             f'Status nicht feststellbar: {exc}', job_url, key) from exc
                self.stop.wait(REMOTE_POLL_INTERVAL)
                continue
            if unavailable_since is not None:
                log(f'Status von {key} wieder erreichbar; bestehenden Auftrag weiter verfolgen.')
                unavailable_since = None
            if result['log'] != last:
                log(result['log'][len(last):] if result['log'].startswith(last) else result['log'])
                last = result['log']
            if result['status'] == 'succeeded':
                if result['result'].get('ready') is False:
                    raise RuntimeError(f'Remote-Auftrag {key}: Dienst läuft, ist aber nicht bereit; Deployment fehlgeschlagen.')
                self.discovery.schedule(target['agent_url'])
                return {'remote_job': key, **result['result']}
            if result['status'] in ('failed', 'interrupted'):
                raise RuntimeError(f"Remote-Auftrag {key}: {result['status']}; {result['result'].get('error', '')}")
            self.stop.wait(REMOTE_POLL_INTERVAL)
        raise RemoteJobUncertain(f'Remote-Auftrag {key} läuft möglicherweise weiter; Zeitlimit erreicht', job_url, key)

    def node(self, node_id):
        for node in self.discovery.snapshot()['peers']:
            if node['node_id'] == node_id and node['online'] and node['role'] == 'agent':
                return node
        raise ValueError('Ziel-Agent nicht erreichbar; zuerst Discovery starten')

    def plan(self, data):
        node = self.node(data.get('node_id'))
        role = data.get('service')
        if role not in CATALOG or data.get('action') not in ('install', 'update', 'restart'):
            raise ValueError('Unbekannter Dienst/Aktion')
        dependencies = CATALOG[role]['depends_on']
        endpoints = self.discovery.snapshot()['endpoints']
        return {'node_id': node['node_id'], 'service': role, 'action': data['action'],
                'ref': data.get('ref', self.cfg['ref']), 'restarts': [role],
                'dependencies': [{'name': x, 'resolved': x in endpoints} for x in dependencies],
                'profile': data.get('profile', ''), 'confirm_restart': True}

    def bootstrap(self, base, profile=None):
        base = peer_url(base, self.cfg['allowed_networks'])
        sha = self.desired.get('commit')
        if not sha:
            raise ValueError('Zuerst Git-Stand prüfen')
        name = validate_profile(self.profiles[profile])['name'] if profile else ''
        args = ['bash', 'system-backend/deployment-core/install/install.sh', 'agent', '--seed', base]
        if name:
            args += ['--node-id', name, '--tbs-template', '/etc/netcore/tbs-site-template.toml']
        lines = ['#!/usr/bin/env bash', 'set -Eeuo pipefail', '[[ $EUID -eq 0 ]] || { echo "Bitte als root"; exit 1; }',
                 'apt-get update', 'apt-get install -y git python3 curl ca-certificates',
                 'if [[ ! -d /opt/netcore-tetra/.git ]]; then',
                 '  git clone ' + shlex.quote(self.cfg['repository']) + ' /opt/netcore-tetra', 'fi',
                 'cd /opt/netcore-tetra', 'git diff --quiet && git diff --cached --quiet || { echo "Lokale Änderungen vorhanden. Abbruch."; exit 1; }',
                 'git fetch origin --tags', 'git checkout --detach ' + sha]
        if name:
            lines += ['install -d -m 0755 /etc/netcore',
                      "curl --fail --show-error " + shlex.quote(base + '/api/v1/profiles/' + name + '/config') +
                      ' -o /etc/netcore/tbs-site-template.toml', 'chmod 600 /etc/netcore/tbs-site-template.toml']
        lines += [shlex.join(args), 'echo "Agent bereit. Installation im Deployment-UI starten."']
        return '\n'.join(lines) + '\n'


class Server(ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True
    request_queue_size = 32

    def get_request(self):
        connection, address = super().get_request()
        connection.settimeout(10)
        return connection, address


def handler(app):
    class Handler(BaseHTTPRequestHandler):
        server_version = 'NetCore-OpenLab/' + VERSION

        def log_message(self, fmt, *args):
            logging.getLogger('http').debug(fmt, *args)

        def send(self, data, code=200, content='application/json'):
            body = json.dumps(data).encode() if content == 'application/json' else data.encode() if isinstance(data, str) else data
            self.send_response(code)
            self.send_header('Content-Type', content + ('; charset=utf-8' if content.startswith('text/') else ''))
            self.send_header('Content-Length', str(len(body)))
            self.send_header('Cache-Control', 'no-store')
            self.send_header('X-Content-Type-Options', 'nosniff')
            self.end_headers()
            self.wfile.write(body)

        def do_GET(self):
            self.dispatch(False)

        def do_POST(self):
            self.dispatch(True)

        def dispatch(self, mutation):
            path = urlsplit(self.path).path
            query = parse_qs(urlsplit(self.path).query)
            try:
                if mutation:
                    # No login/token/TLS. Browser mutations still require same-origin JSON.
                    origin = self.headers.get('Origin')
                    if origin and urlsplit(origin).netloc != self.headers.get('Host'):
                        raise ValueError('Cross-origin write refused')
                    if self.headers.get_content_type() != 'application/json':
                        raise ValueError('Content-Type application/json erforderlich')
                    length = int(self.headers.get('Content-Length', '0'))
                    if not 0 < length <= 262144:
                        raise ValueError('Ungültige Request-Größe')
                    data = json.loads(self.rfile.read(length))
                    if not isinstance(data, dict):
                        raise ValueError('JSON object required')
                    return self.mutate(path, data)
                if path in ('/health/live', '/health/ready'):
                    return self.send({'ok': True, 'ready': True, 'service': 'netcore-deployment', 'security_mode': 'open_lab'})
                if path == '/openapi.json':
                    return self.send(public_openapi())
                if path == '/metrics':
                    return self.send(app.metrics(), content='text/plain; version=0.0.4')
                if path == '/api/v1/manifest':
                    return self.send(app.manifest())
                if path == '/api/v1/peers':
                    return self.send(app.discovery.snapshot())
                if path == '/api/v1/status':
                    return self.send(app.status())
                if path == '/api/v1/catalog':
                    return self.send(list(CATALOG.values()))
                if path == '/api/v1/jobs':
                    return self.send(app.jobs.list())
                if path.startswith('/api/v1/jobs/'):
                    return self.send(app.jobs.get(path.rsplit('/', 1)[-1]))
                if path == '/api/v1/profiles':
                    return self.send(list(app.profiles.values()))
                if path == '/api/v1/images' and app.cfg['role'] == 'controller':
                    return self.send(app.images.status())
                if path.startswith('/api/v1/images/') and app.cfg['role'] == 'controller':
                    parts = path.split('/')
                    if len(parts) != 6 or not BUILD_ID.fullmatch(parts[4]) or parts[5] not in ('image', 'sha256', 'manifest'):
                        raise KeyError(path)
                    return app.images.download('/artifacts/' + parts[4] + '/' + parts[5], self)
                if path.startswith('/api/v1/profiles/') and path.endswith('/config'):
                    profile = app.profiles[path.split('/')[-2]]
                    template = (app.state / 'tbs-site-template.toml').read_text()
                    return self.send(tbs_config(template, profile), content='text/plain')
                if path == '/bootstrap.sh' and app.cfg['role'] == 'controller':
                    base = app.cfg.get('advertise_url') or 'http://' + self.headers['Host']
                    return self.send(app.bootstrap(base, query.get('profile', [None])[0]), content='text/x-shellscript')
                static = {'/': 'index.html', '/app.js': 'app.js', '/style.css': 'style.css'}
                if path in static:
                    content = {'/': 'text/html', '/app.js': 'text/javascript', '/style.css': 'text/css'}[path]
                    return self.send((ROOT / 'static' / static[path]).read_bytes(), content=content)
                self.send({'error': 'Not found'}, 404)
            except KeyError:
                self.send({'error': 'Eintrag nicht gefunden'}, 404)
            except (ValueError, TypeError, OSError) as exc:
                self.send({'error': str(exc)}, 400)
            except Exception:
                logging.exception('Request failed')
                self.send({'error': 'Interner Fehler; Dienstprotokoll prüfen'}, 500)

        def mutate(self, path, data):
            if path == '/api/v1/discovery/scan':
                app.discovery.scan()
                return self.send({'accepted': True}, 202)
            if path == '/api/v1/settings':
                seeds = data.get('seeds', app.cfg['seeds'])
                bindings = data.get('bindings', app.cfg['bindings'])
                if not isinstance(seeds, list) or len(seeds) > 64 or not isinstance(bindings, dict):
                    raise ValueError('Ungültige Einstellungen')
                seeds = [peer_url(x, app.cfg['allowed_networks']) for x in seeds]
                for role, node in bindings.items():
                    if role not in CATALOG:
                        raise ValueError('Unbekannte Dienstrolle')
                    identifier(node)
                ref = data.get('ref', app.cfg['ref'])
                if not isinstance(ref, str) or len(ref) > 128:
                    raise ValueError('Ungültiger Ref')
                with app.lock:
                    app.settings = dict(seeds=seeds, bindings=bindings, ref=ref)
                    atomic_write(app.state / 'settings.json', json.dumps(app.settings))
                    app.cfg.update(app.settings)
                app.discovery.scan()
                return self.send(app.settings)
            if path == '/api/v1/jobs' and app.cfg['role'] == 'agent':
                return self.send(app.jobs.submit(validate_job(data)), 202)
            if app.cfg['role'] != 'controller':
                return self.send({'error': 'Controller endpoint'}, 404)
            if path == '/api/v1/check':
                return self.send(app.jobs.submit({'kind': 'check', 'ref': app.cfg['ref']}), 202)
            if path == '/api/v1/images/build':
                profile = app.profiles[data.get('profile')]
                template = (app.state / 'tbs-site-template.toml').read_text()
                request = {**data, 'profile': profile, 'config': tbs_config(template, profile),
                           'environment': app.cfg['environment']}
                request['controller_url'] = data.get('controller_url') or app.cfg.get('advertise_url') or 'http://' + self.headers['Host']
                # Validate before network work; the final SHA is resolved afresh for this build.
                request['commit'] = '0' * 40
                request = validate_image(request, app.cfg['allowed_networks'])
                request['commit'] = app.repo.resolve(data.get('ref', app.cfg['ref']), lambda _: None)
                return self.send(app.images.request('/build', request), 202)
            if path == '/api/v1/images/remove':
                return self.send(app.images.request('/remove', {'id': data.get('id', '')}))
            if path == '/api/v1/plan':
                return self.send(app.plan(data))
            if path == '/api/v1/deploy':
                if data.get('confirm_restart') is not True:
                    raise ValueError('confirm_restart erforderlich')
                plan = app.plan(data)
                if plan['profile'] and plan['profile'] not in app.profiles:
                    raise ValueError('TBS-Profil fehlt')
                return self.send(app.jobs.submit(plan), 202)
            if path == '/api/v1/profiles':
                profile = validate_profile(data)
                if not (app.state / 'tbs-site-template.toml').exists():
                    raise ValueError('Zuerst ein Standort-Template importieren')
                with app.lock:
                    app.profiles[profile['name']] = profile
                    atomic_write(app.state / 'profiles.json', json.dumps(app.profiles))
                return self.send(profile, 201)
            if path == '/api/v1/template':
                template = data.get('toml', '')
                parsed = tomllib.loads(template)
                for key in ('net_info', 'cell_info', 'phy_io'):
                    if key not in parsed:
                        raise ValueError('TBS-Template benötigt ' + key)
                atomic_write(app.state / 'tbs-site-template.toml', template)
                return self.send({'saved': True})
            self.send({'error': 'Not found'}, 404)
    return Handler


def main():
    p = argparse.ArgumentParser()
    p.add_argument('--config', required=True)
    args = p.parse_args()
    logging.basicConfig(level=logging.INFO, format='%(asctime)s %(levelname)s %(message)s')
    app = App(load_config(args.config))
    server = Server((app.cfg['bind'], app.cfg['port']), handler(app))
    app.start()
    def stop(*_):
        app.stop.set()
        threading.Thread(target=server.shutdown, daemon=True).start()
    signal.signal(signal.SIGTERM, stop)
    signal.signal(signal.SIGINT, stop)
    logging.info('OpenLab %s at http://%s:%s', app.cfg['role'], app.cfg['bind'], app.cfg['port'])
    try:
        server.serve_forever()
    finally:
        app.stop.set()
        server.server_close()
        app.discovery.close()


if __name__ == '__main__':
    main()
