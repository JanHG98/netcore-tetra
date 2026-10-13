"""Multicast discovery with durable inventory exclusions and last-good endpoints."""
from concurrent.futures import ThreadPoolExecutor
from copy import deepcopy
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


class DiscoveryPersistenceUncertain(OSError):
    """An inventory replacement or its compatibility exports need reconciliation."""
    def __init__(self, applied, *, durability_uncertain=True, cache_sync_pending=False):
        message = ('Inventaränderung ist gespeichert; Cache-Abgleich steht aus.'
                   if not durability_uncertain else
                   'Speicherbestätigung der Inventaränderung ist unklar; aktuellen Stand prüfen.')
        super().__init__(message)
        self.result = dict(error=message, inventory_change_applied=applied,
                           durability_uncertain=durability_uncertain,
                           cache_sync_pending=cache_sync_pending)


class Discovery:
    def __init__(self, cfg, manifest):
        self.cfg, self.manifest = cfg, manifest
        self.state = Path(cfg['state_dir'])
        self.lock = threading.RLock()
        self.peers = json_file(self.state / 'peers.json', {})
        self.endpoints = json_file(self.state / 'endpoints.json', {})
        self.forgotten = {}
        self.conflicts = {}
        self.generation = 0
        self.ignored_seen = {}
        self.persistence_error = None
        self.persistence_blocked = False
        self.pending = set()
        self.stop = threading.Event()
        self.wake = threading.Event()
        self.sock = None
        self.error = None
        self.last_scan = None
        canonical = self.state / 'discovery-state.json'
        if canonical.exists():
            # Never fall back to stale mirrors when the authoritative inventory
            # is unreadable: that would discard exclusions and revive peers.
            saved = self._validate_document(json.loads(canonical.read_text()))
            self._publish(saved)
            try:
                self._sync_exports()
            except OSError:
                self.persistence_error = DiscoveryPersistenceUncertain(
                    True, durability_uncertain=False, cache_sync_pending=True).result
                LOG.warning('Discovery inventory loaded; compatibility cache sync pending')
        self.pool = ThreadPoolExecutor(max_workers=4, thread_name_prefix='discovery')

    def _document(self, peers, endpoints, forgotten):
        return dict(version=1, protocol=PROTOCOL, environment=self.cfg['environment'],
                    node_id=self.cfg['node_id'], peers=peers, endpoints=endpoints, forgotten=forgotten)

    def _validate_document(self, saved):
        if (not isinstance(saved, dict) or saved.get('version') != 1
                or saved.get('protocol') != PROTOCOL
                or saved.get('environment') != self.cfg['environment']
                or saved.get('node_id') != self.cfg['node_id']
                or any(not isinstance(saved.get(name), dict) for name in ('peers', 'endpoints', 'forgotten'))):
            raise ValueError('Gespeichertes Discovery-Inventar ist ungültig oder gehört zu einem anderen Knoten')
        for node, record in saved['forgotten'].items():
            identifier(node)
            if (node == self.cfg['node_id'] or not isinstance(record, dict)
                    or record.get('node_id') != node or not isinstance(record.get('agent_url'), str)
                    or not isinstance(record.get('forgotten_at'), (int, float))):
                raise ValueError('Gespeicherte Discovery-Ausnahme ist ungültig')
            if record['agent_url']:
                peer_url(record['agent_url'], self.cfg['allowed_networks'])
        for node, record in saved['peers'].items():
            identifier(node)
            if (not isinstance(record, dict) or record.get('node_id') != node
                    or not isinstance(record.get('services'), list)
                    or not isinstance(record.get('last_seen'), (int, float))):
                raise ValueError('Gespeicherter Discovery-Knoten ist ungültig')
            peer_url(record.get('agent_url', ''), self.cfg['allowed_networks'])
        for target in saved['endpoints'].values():
            if not isinstance(target, dict):
                raise ValueError('Gespeicherter Discovery-Endpunkt ist ungültig')
            if target.get('node_id') is not None:
                identifier(target['node_id'])
            networks = self.cfg['allowed_networks']
            if target.get('node_id') == self.cfg['node_id']:
                networks = [*networks, '127.0.0.0/8']
            peer_url(target.get('url', ''), networks)
        return saved

    def _publish(self, saved):
        forgotten = deepcopy(saved['forgotten'])
        if forgotten != self.forgotten:
            # In-flight responses cannot cross a forget OR restore boundary.
            self.generation += 1
        self.forgotten = forgotten
        self.peers = {key: deepcopy(value) for key, value in saved['peers'].items() if key not in forgotten}
        self.endpoints = {key: deepcopy(value) for key, value in saved['endpoints'].items()
                          if value.get('node_id') not in forgotten}

    def _sync_exports(self):
        # launch.py and older tooling read these flat compatibility files.
        # Both are disposable mirrors; the single canonical document owns state.
        errors = []
        for filename, value in (('peers.json', self.peers), ('endpoints.json', self.endpoints)):
            try:
                atomic_write(self.state / filename, json.dumps(value), 0o644)
            except OSError as exc:
                errors.append(exc)
        if errors:
            raise errors[0]

    def _commit(self, peers, endpoints, forgotten, conflicts):
        if self.persistence_blocked:
            raise DiscoveryPersistenceUncertain(None)
        path = self.state / 'discovery-state.json'
        before = path.read_bytes() if path.exists() else None
        updated = self._document(peers, endpoints, forgotten)
        try:
            atomic_write(path, json.dumps(updated), 0o644)
        except OSError as exc:
            try:
                current = path.read_bytes() if path.exists() else None
            except OSError:
                current = object()
            if current == before:
                # Nothing was replaced: caller's previous memory stays intact.
                raise
            try:
                saved = self._validate_document(json.loads(current))
            except (ValueError, TypeError, OSError):
                self.persistence_blocked = True
                failure = DiscoveryPersistenceUncertain(None)
                self.persistence_error = failure.result
                raise failure from exc
            applied = saved == updated
            self._publish(saved)
            self.conflicts = deepcopy(conflicts) if applied else {}
            sync_pending = False
            try:
                self._sync_exports()
            except OSError:
                sync_pending = True
            failure = DiscoveryPersistenceUncertain(True if applied else None,
                                                    cache_sync_pending=sync_pending)
            self.persistence_error = failure.result
            raise failure from exc
        self._publish(updated)
        self.conflicts = deepcopy(conflicts)
        try:
            self._sync_exports()
        except OSError as exc:
            failure = DiscoveryPersistenceUncertain(True, durability_uncertain=False,
                                                    cache_sync_pending=True)
            self.persistence_error = failure.result
            raise failure from exc
        self.persistence_error = None

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
            try:
                self._resolve()
            except OSError as exc:
                LOG.warning('Discovery inventory persistence: %s', exc)
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
                node = identifier(value.get('node_id'))
                if node == self.cfg['node_id']:
                    continue
                with self.lock:
                    if node in self.forgotten:
                        self.ignored_seen[node] = time.time()
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
            generation = self.generation
        try:
            self.pool.submit(self._fetch, url, generation)
        except RuntimeError:
            with self.lock:
                self.pending.discard(url)

    def _fetch(self, url, generation=None):
        if generation is None:
            with self.lock:
                generation = self.generation
        try:
            result = request_json(url + '/api/v1/manifest')
            self.accept(url, result, generation=generation)
            # A forgotten seed can still relay OTHER nodes across routed VPNs.
            with self.lock:
                relay = generation == self.generation and url in self.cfg['seeds']
            if relay:
                relayed = request_json(url + '/api/v1/peers').get('peers', [])[:128]
                for peer in relayed:
                    with self.lock:
                        if generation != self.generation:
                            break
                        node = peer.get('node_id')
                        include = node != self.cfg['node_id'] and node not in self.forgotten
                    if include:
                        self.schedule(peer.get('agent_url', ''))
        except (OSError, ValueError, TypeError, KeyError, AttributeError) as exc:
            LOG.debug('Peer %s: %s', url, exc)
        finally:
            with self.lock:
                self.pending.discard(url)
                if generation != self.generation:
                    self.wake.set()

    def accept(self, url, manifest, *, generation=None):
        url = peer_url(url, self.cfg['allowed_networks'])
        if manifest.get('protocol') != PROTOCOL or manifest.get('environment') != self.cfg['environment']:
            raise ValueError('Different discovery protocol/environment')
        if manifest.get('security_mode') != 'open_lab':
            raise ValueError('Only OpenLab peers are accepted')
        node = identifier(manifest['node_id'])
        if node == self.cfg['node_id']:
            return False
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
        local = self.manifest()
        with self.lock:
            if generation is not None and generation != self.generation:
                return False
            if node in self.forgotten:
                self.ignored_seen[node] = time.time()
                return False
            if node not in self.peers and len(self.peers) >= 128:
                raise ValueError('Peer limit reached')
            previous = self.peers.get(node)
            if previous and previous['agent_url'] != url and time.time() - previous['last_seen'] < self.cfg['lease_seconds']:
                raise ValueError('Duplicate node_id at different address')
            peers = deepcopy(self.peers)
            peers[node] = {'node_id': node, 'agent_url': url, 'last_seen': time.time(),
                           'role': manifest.get('role', 'agent'), 'services': services}
            endpoints, conflicts = self._resolved(peers, self.endpoints, self.forgotten, local)
            self._commit(peers, endpoints, self.forgotten, conflicts)
            return True

    def _resolved(self, peers, previous_endpoints, forgotten, local):
        candidates = {}
        now = time.time()
        endpoints = {key: deepcopy(value) for key, value in previous_endpoints.items()
                     if value.get('node_id') not in forgotten}
        for node, peer in peers.items():
            if node in forgotten or now - peer['last_seen'] > self.cfg['lease_seconds']:
                continue
            for service in peer['services']:
                if service.get('available', service['ready']):
                    candidates.setdefault(service['name'], []).append(dict(service, node_id=node))
        # Services sharing one host still need each other's discovered endpoints.
        for service in local.get('services', []):
            if service.get('available', service.get('ready')):
                candidates.setdefault(service['name'], []).append(dict(service,
                    url=f"http://127.0.0.1:{service['port']}", node_id=self.cfg['node_id']))
        conflicts = {}
        for role, options in candidates.items():
            pinned = self.cfg['bindings'].get(role)
            selected = [x for x in options if x['node_id'] == pinned] if pinned else options
            if len(selected) == 1:
                endpoints[role] = {k: selected[0][k] for k in ('url', 'node_id')}
            elif len(selected) > 1:
                conflicts[role] = [x['node_id'] for x in selected]
            # Keep last-good during outage/conflict, except explicitly forgotten nodes.
        return endpoints, conflicts

    def _resolve(self):
        local = self.manifest()
        with self.lock:
            endpoints, conflicts = self._resolved(self.peers, self.endpoints, self.forgotten, local)
            self._commit(self.peers, endpoints, self.forgotten, conflicts)

    def forget(self, node_id):
        node = identifier(node_id)
        if node == self.cfg['node_id']:
            raise ValueError('Der eigene Discovery-Knoten kann nicht aus dem Inventar entfernt werden')
        local = self.manifest()
        with self.lock:
            retained = [role for role, target in self.cfg['bindings'].items() if target == node]
            removed = [role for role, target in self.endpoints.items() if target.get('node_id') == node]
            if node in self.forgotten:
                return dict(forgotten=node, removed_endpoints=[], retained_bindings=retained,
                            suppressed_until_restore=True, already_forgotten=True)
            peer = self.peers.get(node)
            if peer is None and not removed:
                raise KeyError(node)
            peers = deepcopy(self.peers)
            peers.pop(node, None)
            forgotten = deepcopy(self.forgotten)
            forgotten[node] = dict(node_id=node, agent_url=(peer or {}).get('agent_url', ''),
                                   role=(peer or {}).get('role', 'agent'), forgotten_at=time.time())
            endpoints, conflicts = self._resolved(peers, self.endpoints, forgotten, local)
            self._commit(peers, endpoints, forgotten, conflicts)
            self.wake.set()
            return dict(forgotten=node, removed_endpoints=removed, retained_bindings=retained,
                        suppressed_until_restore=True)

    def restore(self, node_id):
        node = identifier(node_id)
        local = self.manifest()
        failure = None
        with self.lock:
            if node not in self.forgotten:
                raise KeyError(node)
            forgotten = deepcopy(self.forgotten)
            record = forgotten.pop(node)
            endpoints, conflicts = self._resolved(self.peers, self.endpoints, forgotten, local)
            try:
                self._commit(self.peers, endpoints, forgotten, conflicts)
            except DiscoveryPersistenceUncertain as exc:
                if exc.result['inventory_change_applied'] is not True:
                    raise
                # The restore did happen. Still request a fresh manifest even
                # when its cache/durability acknowledgement needs reconciliation.
                failure = exc
            self.ignored_seen.pop(node, None)
        self.scan()
        if record.get('agent_url'):
            self.schedule(record['agent_url'])
        if failure is not None:
            raise failure
        return dict(restored=node, rediscovery_scheduled=True)

    def snapshot(self):
        with self.lock:
            now = time.time()
            forgotten = [dict(record,
                retained_bindings=[role for role, node in self.cfg['bindings'].items() if node == record['node_id']],
                observed_recently=now - self.ignored_seen.get(record['node_id'], 0) < self.cfg['lease_seconds'])
                for record in self.forgotten.values()]
            return deepcopy({'peers': [dict(x, online=now - x['last_seen'] < self.cfg['lease_seconds'])
                              for x in self.peers.values()], 'endpoints': self.endpoints,
                    'conflicts': self.conflicts, 'forgotten': forgotten,
                    'persistence_error': self.persistence_error, 'multicast_error': self.error,
                    'last_scan': self.last_scan})
