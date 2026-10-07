"""Multicast announcements, unicast peers and a persistent last-good endpoint cache."""
from concurrent.futures import ThreadPoolExecutor
import json
import logging
from pathlib import Path
import socket
import struct
import threading
import time
from urllib.parse import urlsplit

from common import CATALOG, PROTOCOL, allowed_ip, atomic_write, identifier, json_file, peer_url, request_json

LOG = logging.getLogger('discovery')


class Discovery:
    def __init__(self, cfg, manifest):
        self.cfg, self.manifest = cfg, manifest
        self.state = Path(cfg['state_dir'])
        self.lock = threading.RLock()
        self.peers = json_file(self.state / 'peers.json', {})
        self.endpoints = json_file(self.state / 'endpoints.json', {})
        self.conflicts = {}
        self.pending = set()
        self.pool = ThreadPoolExecutor(max_workers=4, thread_name_prefix='discovery')
        self.stop = threading.Event()
        self.wake = threading.Event()
        self.sock = None
        self.error = None
        self.last_scan = None

    def start(self):
        try:
            s = socket.socket(socket.AF_INET, socket.SOCK_DGRAM, socket.IPPROTO_UDP)
            s.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
            s.bind(('', self.cfg['discovery_port']))
            group = socket.inet_aton(self.cfg['multicast_group'])
            interface = socket.inet_aton(self.cfg['interface'])
            s.setsockopt(socket.IPPROTO_IP, socket.IP_ADD_MEMBERSHIP, group + interface)
            s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_TTL, struct.pack('b', 1))
            if self.cfg['interface'] != '0.0.0.0':
                s.setsockopt(socket.IPPROTO_IP, socket.IP_MULTICAST_IF, interface)
            s.settimeout(1)
            self.sock = s
            threading.Thread(target=self._listen, daemon=True).start()
        except OSError as exc:
            self.error = str(exc)
            LOG.warning('Multicast unavailable; unicast remains active: %s', exc)
        threading.Thread(target=self._periodic, daemon=True).start()

    def close(self):
        self.stop.set()
        self.wake.set()
        if self.sock:
            self.sock.close()
        self.pool.shutdown(wait=True, cancel_futures=True)

    def scan(self):
        self.wake.set()

    def _periodic(self):
        while not self.stop.is_set():
            self.last_scan = time.time()
            if self.sock:
                message = {'protocol': PROTOCOL, 'environment': self.cfg['environment'],
                           'node_id': self.cfg['node_id'], 'port': self.cfg['port']}
                try:
                    self.sock.sendto(json.dumps(message).encode(),
                                     (self.cfg['multicast_group'], self.cfg['discovery_port']))
                except OSError as exc:
                    self.error = str(exc)
            with self.lock:
                urls = list(dict.fromkeys(self.cfg['seeds'] + [v['agent_url'] for v in self.peers.values()]))[:128]
            for url in urls:
                self.schedule(url)
            with self.lock:
                self._resolve()
            self.wake.wait(self.cfg['interval'])
            self.wake.clear()

    def _listen(self):
        while not self.stop.is_set():
            try:
                data, address = self.sock.recvfrom(4097)
                if len(data) > 4096 or not allowed_ip(address[0], self.cfg['allowed_networks']):
                    continue
                value = json.loads(data)
                if value.get('protocol') != PROTOCOL or value.get('environment') != self.cfg['environment']:
                    continue
                if value.get('node_id') == self.cfg['node_id']:
                    continue
                port = value.get('port')
                if type(port) is int and 1 <= port <= 65535:
                    # Always use the packet source, never a URL advertised in UDP.
                    self.schedule(f'http://{address[0]}:{port}')
            except socket.timeout:
                continue
            except (OSError, ValueError, AttributeError, TypeError):
                if self.stop.is_set():
                    break

    def schedule(self, url):
        try:
            url = peer_url(url, self.cfg['allowed_networks'])
        except ValueError:
            return
        with self.lock:
            if url in self.pending or len(self.pending) >= 128 or self.stop.is_set():
                return
            self.pending.add(url)
        self.pool.submit(self._fetch, url)

    def _fetch(self, url):
        try:
            result = request_json(url + '/api/v1/manifest')
            self.accept(url, result)
            # A unicast seed also relays addresses across routed VPN links.
            if url in self.cfg['seeds']:
                for peer in request_json(url + '/api/v1/peers').get('peers', [])[:128]:
                    if peer.get('node_id') != self.cfg['node_id']:
                        self.schedule(peer.get('agent_url', ''))
        except (OSError, ValueError, TypeError, KeyError, AttributeError) as exc:
            LOG.debug('Peer %s: %s', url, exc)
        finally:
            with self.lock:
                self.pending.discard(url)

    def accept(self, url, manifest):
        url = peer_url(url, self.cfg['allowed_networks'])
        if manifest.get('protocol') != PROTOCOL or manifest.get('environment') != self.cfg['environment']:
            raise ValueError('Different discovery protocol/environment')
        if manifest.get('security_mode') != 'open_lab':
            raise ValueError('Only OpenLab peers are accepted')
        node = identifier(manifest['node_id'])
        if node == self.cfg['node_id']:
            return
        host = urlsplit(url).hostname
        services = []
        for service in manifest.get('services', [])[:64]:
            role = service.get('name')
            port = service.get('port')
            if role not in CATALOG or type(port) is not int or not 1 <= port <= 65535:
                continue
            sha = service.get('commit', '')
            if not isinstance(sha, str) or len(sha) > 40:
                sha = ''
            services.append({'name': role, 'port': port, 'ready': service.get('ready') is True,
                             'available': service.get('available', service.get('ready')) is True,
                             'commit': sha, 'url': f'http://{host}:{port}',
                             'unit': CATALOG[role]['unit']})
        with self.lock:
            if node not in self.peers and len(self.peers) >= 128:
                raise ValueError('Peer limit reached')
            previous = self.peers.get(node)
            if previous and previous['agent_url'] != url and time.time() - previous['last_seen'] < self.cfg['lease_seconds']:
                raise ValueError('Duplicate node_id at different address')
            self.peers[node] = {'node_id': node, 'agent_url': url, 'last_seen': time.time(),
                                'role': manifest.get('role', 'agent'), 'services': services}
            self._resolve()
            atomic_write(self.state / 'peers.json', json.dumps(self.peers), 0o644)

    def _resolve(self):
        candidates = {}
        now = time.time()
        for node, peer in self.peers.items():
            if now - peer['last_seen'] > self.cfg['lease_seconds']:
                continue
            for service in peer['services']:
                if service.get('available', service['ready']):
                    candidates.setdefault(service['name'], []).append(dict(service, node_id=node))
        # Services sharing one host still need each other's discovered endpoints.
        local = self.manifest()
        for service in local.get('services', []):
            if service.get('available', service.get('ready')):
                candidates.setdefault(service['name'], []).append(dict(service,
                    url=f"http://127.0.0.1:{service['port']}", node_id=self.cfg['node_id']))
        self.conflicts = {}
        for role, options in candidates.items():
            pinned = self.cfg['bindings'].get(role)
            selected = [x for x in options if x['node_id'] == pinned] if pinned else options
            if len(selected) == 1:
                self.endpoints[role] = {k: selected[0][k] for k in ('url', 'node_id')}
            elif len(selected) > 1:
                self.conflicts[role] = [x['node_id'] for x in selected]
            # Keep last-good bindings during outage/conflict, never pick randomly.
        atomic_write(self.state / 'endpoints.json', json.dumps(self.endpoints), 0o644)

    def snapshot(self):
        with self.lock:
            now = time.time()
            return {'peers': [dict(x, online=now - x['last_seen'] < self.cfg['lease_seconds'])
                              for x in self.peers.values()], 'endpoints': dict(self.endpoints),
                    'conflicts': dict(self.conflicts), 'multicast_error': self.error,
                    'last_scan': self.last_scan}
