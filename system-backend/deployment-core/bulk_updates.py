"""Controller-only immutable plans for installed-service update batches."""
from concurrent.futures import ThreadPoolExecutor
from http.client import HTTPException

from common import CATALOG, PROTOCOL, SHA, peer_url
from jobs import JobConflict


def verified_manifest(app, node, request):
    """Recheck the pinned address belongs to the intended agent/environment."""
    url = peer_url(node['agent_url'], app.cfg['allowed_networks'])
    value = request(url + '/api/v1/manifest', timeout=15)
    if (not isinstance(value, dict) or value.get('node_id') != node['node_id']
            or value.get('protocol') != PROTOCOL or value.get('environment') != app.cfg['environment']
            or value.get('role') != 'agent' or value.get('security_mode') != 'open_lab'
            or not isinstance(value.get('services'), list)):
        raise ValueError('Zielidentität oder verwaltetes Dienstinventar stimmt nicht; kein Update angelegt.')
    return value


def service_order(name, seen=None):
    """Prefer foundational roles when scheduling; remote queues serialize execution."""
    seen = set() if seen is None else set(seen)
    if name in seen or name not in CATALOG:
        return 0
    seen.add(name)
    return 1 + max((service_order(dep, seen) for dep in CATALOG[name]['depends_on']), default=0)


class BulkUpdates:
    def __init__(self, app, request):
        self.app, self.request = app, request

    def plan(self, data):
        if data:
            raise ValueError('Sammel-Updates verwenden den aktuellen main-Stand und ausschließlich das ermittelte Inventar.')
        commit = self.app.repo.resolve_latest_main(lambda _: None)
        if not SHA.fullmatch(commit):
            raise ValueError('Geprüfter main-Commit fehlt; kein Sammel-Update angelegt.')
        peers = self.app.discovery.snapshot()['peers']

        def inspect(node):
            skipped, targets = [], []
            if node.get('role') != 'agent':
                return targets, [dict(node_id=node['node_id'], reason='Kein verwalteter Ziel-Agent')]
            if node.get('online') is not True:
                return targets, [dict(node_id=node['node_id'], reason='Agent offline; kein Update angelegt')]
            try:
                manifest = verified_manifest(self.app, node, self.request)
            except (OSError, ValueError, TypeError, KeyError, HTTPException) as exc:
                return targets, [dict(node_id=node['node_id'], reason=str(exc))]
            seen = set()
            for service in manifest['services']:
                if not isinstance(service, dict):
                    skipped.append(dict(node_id=node['node_id'], reason='Ungültiger Dienst im Inventar'))
                    continue
                name = service.get('name')
                if not isinstance(name, str) or name not in CATALOG:
                    skipped.append(dict(node_id=node['node_id'], reason='Nicht verwaltete Dienstrolle'))
                    continue
                if name in seen:
                    continue
                seen.add(name)
                targets.append(dict(node_id=node['node_id'], service=name, agent_url=node['agent_url'],
                                    unit=service.get('unit', CATALOG[name]['unit']), installed_commit=service.get('commit', '')))
            targets.sort(key=lambda row: (service_order(row['service']), row['service']))
            if not targets:
                skipped.append(dict(node_id=node['node_id'], reason='Keine installierten verwalteten Dienste'))
            return targets, skipped

        targets, skipped = [], []
        with ThreadPoolExecutor(max_workers=8) as pool:
            for selected, ignored in pool.map(inspect, sorted(peers, key=lambda row: row['node_id'])):
                targets.extend(selected)
                skipped.extend(ignored)
        stored = self.app.jobs.plan_batch(dict(commit=commit, ref='main', targets=targets, skipped=skipped,
                                              scope='installed_managed_services', max_parallel_hosts=8))
        return {key: stored[key] for key in ('plan_id', 'commit', 'ref', 'created', 'status', 'targets', 'skipped', 'counts', 'scope', 'max_parallel_hosts')}

    def run(self, data):
        if set(data) != {'plan_id', 'confirm_restart'} or data['confirm_restart'] is not True:
            raise ValueError('plan_id und confirm_restart=true erforderlich')
        key = data['plan_id']
        if not isinstance(key, str) or len(key) != 32 or any(c not in '0123456789abcdef' for c in key):
            raise ValueError('Ungültige Update-Vorschau')
        plan = self.app.jobs.batch(key)
        if plan['status'] != 'planned':
            return plan
        nodes = {node['node_id']: node for node in self.app.discovery.snapshot()['peers']}
        hosts = {}
        for target in plan['targets']:
            node = nodes.get(target['node_id'])
            if not node or not node.get('online') or node.get('role') != 'agent' or node['agent_url'] != target['agent_url']:
                raise JobConflict('Zielinventar hat sich geändert; neue Vorschau erstellen.')
            hosts.setdefault(target['node_id'], node)
        with ThreadPoolExecutor(max_workers=8) as pool:
            try:
                values = list(pool.map(lambda node: verified_manifest(self.app, node, self.request), hosts.values()))
            except (OSError, ValueError, KeyError, TypeError, HTTPException) as exc:
                raise JobConflict('Zielidentität oder Erreichbarkeit hat sich geändert; neue Vorschau erstellen.') from exc
        manifests = {node: value for node, value in zip(hosts, values)}
        for target in plan['targets']:
            if target['service'] not in {row['name'] for row in manifests[target['node_id']]['services'] if isinstance(row, dict) and isinstance(row.get('name'), str)}:
                raise JobConflict('Ein geplanter Dienst ist nicht mehr installiert; neue Vorschau erstellen.')
        with self.app.lock:
            # Forget/settings handlers use the same app lock during publication.
            current = {node['node_id']: node for node in self.app.discovery.snapshot()['peers']}
            for name, node in hosts.items():
                if name not in current or not current[name].get('online') or current[name]['agent_url'] != node['agent_url']:
                    raise JobConflict('Zielinventar hat sich geändert; neue Vorschau erstellen.')
            return self.app.jobs.start_batch(key)
