"""Resolve only known dependency fields; never rewrite arbitrary URLs by port.

Port 8080 is deliberately NOT an identity: both the Node Gateway and every TBS
dashboard use it. Media playout URLs and external connectors remain untouched.
"""
from copy import deepcopy
from urllib.parse import urlsplit, urlunsplit

ALIASES = {s: s.replace('_', '-') for s in (
    'node_gateway', 'subscriber_core', 'group_core', 'mobility_core', 'call_control',
    'media_switch', 'sds_router', 'packet_core', 'ip_gateway', 'recorder',
    'application_gateway', 'media_library', 'control_room', 'task_workflow',
    'alarm_workflow', 'security_core', 'kmf', 'transit', 'iot_gateway',
    'hardware_gateway', 'rf_monitor', 'asset_management', 'sip_switch', 'alert_service',
)}


def replace_url(old, endpoint):
    p = urlsplit(old)
    target = urlsplit(endpoint)
    if p.scheme not in ('http', 'ws') or p.username or p.password:
        return old
    return urlunsplit((p.scheme, target.netloc, p.path, p.query, p.fragment))


def resolve_config(service, original, endpoints):
    data = deepcopy(original)
    changes = []

    def apply(obj, key, target, path):
        old = obj[key]
        if target in endpoints and isinstance(old, str):
            new = replace_url(old, endpoints[target]['url'])
            if new != old:
                obj[key] = new
                changes.append('.'.join(path + (key,)))

    def visit(obj, path=()):
        for key, val in list(obj.items()):
            if isinstance(val, dict):
                visit(val, path + (key,))
            elif isinstance(val, list):
                for i, item in enumerate(val):
                    if isinstance(item, dict):
                        # Explicit service identity in monitor/registry tables.
                        if key in ('targets', 'services', 'sources'):
                            role = item.get('service') or item.get('name') or item.get('id', '')
                            role = role.removeprefix('netcore-').replace('_', '-')
                            for field in ('url', 'base_url', 'health_url'):
                                if field in item:
                                    apply(item, field, role, path + (key, str(i)))
                        # Only known NetCore connectors, not arbitrary plugin URLs.
                        if key == 'connectors':
                            role = {'sds-router': 'sds-router', 'sds': 'sds-router',
                                    'media-library': 'media-library'}.get(item.get('connector_id', item.get('id')))
                            for field in ('endpoint', 'health_endpoint'):
                                if field in item:
                                    apply(item, field, role, path + (key, str(i)))
            elif isinstance(val, str) and val.startswith(('http://', 'ws://')):
                role = None
                if path and path[-1] in ALIASES:
                    role = ALIASES[path[-1]]
                elif path and path[-1] in ('dependencies', 'upstream', 'upstreams', 'netcore'):
                    base = key.removesuffix('_base_url').removesuffix('_url')
                    role = ALIASES.get(base)
                # TBS control_room is the Node Gateway transport, not port 9010.
                if service == 'tbs' and path == ('control_room',):
                    role = 'node-gateway'
                apply(obj, key, role, path)
    visit(data)
    if service == 'tbs' and 'node-gateway' in endpoints and 'control_room' in data:
        target = urlsplit(endpoints['node-gateway']['url'])
        c = data['control_room']
        if (c.get('host'), c.get('port')) != (target.hostname, target.port):
            c['host'], c['port'], c['use_tls'] = target.hostname, target.port, False
            changes.append('control_room.host/port')
    return data, changes
