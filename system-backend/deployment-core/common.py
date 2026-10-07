"""Shared OpenLab primitives. Python 3.11+, standard library only."""
from __future__ import annotations

import ipaddress
import json
import os
from pathlib import Path
import re
import socket
import tempfile
import tomllib
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parent
CATALOG = {s['name']: s for s in json.loads((ROOT / 'catalog.json').read_text())}
PROTOCOL = 'netcore.discovery.v1'
VERSION = '0.3.0'
REPOSITORY = 'https://github.com/JanHG98/netcore-tetra.git'
NAME = re.compile(r'[A-Za-z0-9][A-Za-z0-9_.-]{0,63}\Z')
SHA = re.compile(r'[0-9a-f]{40}\Z')


def identifier(value):
    if not isinstance(value, str) or not NAME.fullmatch(value):
        raise ValueError('Ungültiger Name (1–64 Zeichen: Buchstaben, Ziffern, Punkt, - oder _).')
    return value


def atomic_write(path, text, mode=0o600):
    path = Path(path)
    path.parent.mkdir(parents=True, exist_ok=True)
    fd, temp = tempfile.mkstemp(dir=path.parent, prefix='.' + path.name)
    try:
        with os.fdopen(fd, 'w') as f:
            os.fchmod(f.fileno(), mode)
            f.write(text)
            f.flush()
            os.fsync(f.fileno())
        os.replace(temp, path)
        directory = os.open(path.parent, os.O_RDONLY)
        try:
            os.fsync(directory)
        finally:
            os.close(directory)
    finally:
        if os.path.exists(temp):
            os.unlink(temp)


def json_file(path, default):
    try:
        return json.loads(Path(path).read_text())
    except (OSError, ValueError):
        return default


def toml_dump(data):
    """Serialize parsed TOML, including nested arrays of tables; never interpolate code."""
    def scalar(v):
        if isinstance(v, bool):
            return str(v).lower()
        if isinstance(v, str):
            return json.dumps(v, ensure_ascii=False)
        if isinstance(v, (int, float)):
            return repr(v)
        if isinstance(v, list):
            return '[' + ', '.join(scalar(x) for x in v) + ']'
        if hasattr(v, 'isoformat'):
            return v.isoformat()
        raise ValueError('Unsupported TOML value')

    lines = []
    def table(d, path=(), array=False):
        if path:
            name = '.'.join(json.dumps(k) for k in path)
            lines.append((' [[' if array else ' [').strip() + name + (']]' if array else ']'))
        for k, v in d.items():
            if not isinstance(v, dict) and not (isinstance(v, list) and v and isinstance(v[0], dict)):
                lines.append(json.dumps(k) + ' = ' + scalar(v))
        lines.append('')
        for k, v in d.items():
            if isinstance(v, dict):
                table(v, path + (k,))
            elif isinstance(v, list) and v and isinstance(v[0], dict):
                for item in v:
                    table(item, path + (k,), True)
    table(data)
    result = '\n'.join(lines)
    tomllib.loads(result)
    return result


class NoRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        raise ValueError('HTTP redirects are not accepted for management calls')


def request_json(url, data=None, timeout=3, headers=None):
    request = urllib.request.Request(url, data=None if data is None else json.dumps(data).encode(),
                                     headers={'Content-Type': 'application/json', 'Accept': 'application/json', **(headers or {})})
    # LAN calls must not accidentally use a workstation's HTTP proxy.
    opener = urllib.request.build_opener(urllib.request.ProxyHandler({}), NoRedirect())
    with opener.open(request, timeout=timeout) as response:
        body = response.read(1024 * 1024 + 1)
        if len(body) > 1024 * 1024:
            raise ValueError('Antwort zu groß')
        return json.loads(body)


def allowed_ip(ip, networks):
    addr = ipaddress.IPv4Address(ip)
    return not addr.is_multicast and not addr.is_unspecified and any(addr in ipaddress.ip_network(n) for n in networks)


def peer_url(value, networks):
    p = urllib.parse.urlsplit(value)
    if p.scheme != 'http' or p.username or p.password or p.path not in ('', '/') or p.query or p.fragment:
        raise ValueError('Gegenstelle muss http://IPv4:Port sein.')
    if not p.hostname or not allowed_ip(p.hostname, networks) or not p.port:
        raise ValueError('Gegenstelle liegt außerhalb der erlaubten Netze.')
    return f'http://{p.hostname}:{p.port}'


def load_config(path):
    cfg = tomllib.loads(Path(path).read_text())
    if cfg.get('mode', 'open_lab') != 'open_lab' or cfg.get('tls', False) or cfg.get('token_auth', False):
        raise ValueError('Deployment unterstützt ausschließlich open_lab ohne TLS und Tokens.')
    cfg.setdefault('node_id', socket.gethostname())
    identifier(cfg['node_id'])
    cfg.setdefault('environment', 'netcore-openlab')
    identifier(cfg['environment'])
    cfg.setdefault('role', 'agent')
    if cfg['role'] not in ('agent', 'controller'):
        raise ValueError('role muss agent oder controller sein')
    cfg.setdefault('bind', '0.0.0.0')
    cfg.setdefault('port', 8320 if cfg['role'] == 'controller' else 8321)
    cfg.setdefault('state_dir', '/var/lib/netcore-deployment' if cfg['role'] == 'controller' else '/var/lib/netcore-discovery')
    cfg.setdefault('multicast_group', '239.192.84.82')
    cfg.setdefault('discovery_port', 48320)
    cfg.setdefault('interface', '0.0.0.0')
    cfg.setdefault('interval', 15)
    cfg.setdefault('lease_seconds', 90)
    cfg.setdefault('allowed_networks', ['10.0.0.0/8', '172.16.0.0/12', '192.168.0.0/16', '127.0.0.0/8'])
    cfg.setdefault('seeds', [])
    cfg.setdefault('bindings', {})
    cfg.setdefault('services', [])
    managed = cfg.get('managed_services')
    if managed is not None and (not isinstance(managed, list) or
            any(not isinstance(name, str) or name not in CATALOG for name in managed)):
        raise ValueError('managed_services muss eine Liste bekannter Dienstnamen sein')
    cfg.setdefault('repository', REPOSITORY)
    cfg.setdefault('ref', 'main')
    for url in cfg['seeds']:
        peer_url(url, cfg['allowed_networks'])
    if cfg['interval'] < 2 or cfg['lease_seconds'] < cfg['interval'] * 2:
        raise ValueError('Discovery interval/lease ungültig')
    for service in cfg['services']:
        if service['name'] not in CATALOG:
            raise ValueError('Unbekannter Dienst: ' + service['name'])
    return cfg
