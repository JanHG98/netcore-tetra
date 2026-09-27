"""Exercise the actual NMS binary + preview bridge; optional Playwright UI test."""
import json
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import time
import tomllib
import urllib.request

ROOT = Path(__file__).resolve().parents[1]
REPO = ROOT.parents[1]
sys.path.insert(0, str(ROOT / 'logging'))
sys.path.insert(0, str(REPO / 'system-backend/deployment-core'))
from log_store import Store, atomic_json, forward_once
from common import toml_dump


def main():
    binary = Path(os.environ.get('OBSERVABILITY_BINARY', REPO / 'target/debug/netcore-observability'))
    with tempfile.TemporaryDirectory(prefix='netcore-native-') as temporary:
        base = Path(temporary)
        with socket.socket() as sock:
            sock.bind(('127.0.0.1', 0))
            port = sock.getsockname()[1]
        url = f'http://127.0.0.1:{port}'
        cfg = tomllib.loads((ROOT / 'config/observability.example.toml').read_text())
        cfg['server']['bind'] = f'127.0.0.1:{port}'
        cfg['collection'].update(scrape_on_start=False, scrape_interval_secs=3600)
        cfg['discovery']['enabled'] = False  # never connect to the user's LAN in tests
        cfg['storage'] = {'state_path': str(base / 'state.json'), 'backup_path': str(base / 'backup.json'),
                          'diagnostic_dir': str(base / 'diagnostics')}
        (base / 'config.toml').write_text(toml_dump(cfg))
        logcfg = json.loads((ROOT / 'config/syslog.example.json').read_text())
        logcfg.update(state_dir=str(base / 'logs'), inventory=str(ROOT / 'config/openlab-hosts.json'), nms_url=url)
        store = Store(logcfg)
        out = (base / 'native.out').open('w')
        proc = subprocess.Popen([str(binary), '--config', str(base / 'config.toml')], stdout=out, stderr=out)
        def get(path):
            with urllib.request.urlopen(url + path, timeout=3) as response:
                return json.load(response)
        try:
            for _ in range(100):
                try:
                    get('/api/v1/status')
                    break
                except OSError:
                    time.sleep(.05)
            else:
                raise RuntimeError((base / 'native.out').read_text())
            store.append({'source_ip': '10.0.1.179', 'program': 'call-control', 'message': 'native-preview-test', 'severity': 'err'})
            assert forward_once(store) == 1
            assert store.status()['preview_pending'] == 0
            rows = get('/api/v1/logs?contains=native-preview-test')
            assert len(rows) == 1 and rows[0]['node'] == 'Node-Gateway' and rows[0]['level'] == 'error'
            sd = get('/api/v1/targets/prometheus')
            assert any(row['targets'] == ['10.0.1.179:8080'] for row in sd)
            with store.db:
                store.count('unarchived_segments_dropped', 2)
            atomic_json(base / 'logs/receiver-status.json', store.status())
            atomic_json(base / 'logs/archive-status.json', {'error': 'share test offline', 'last_success': None})
            assert get('/api/v1/syslog')['archive']['error'] == 'share test offline'
            if '--browser' in sys.argv:
                env = os.environ.copy()
                env['OBSERVABILITY_TEST_URL'] = url
                subprocess.run(['node', str(ROOT / 'tests/browser-syslog.cjs')], env=env, check=True)
            print('Native NMS + real SQLite preview bridge + Prometheus discovery API: OK')
        finally:
            proc.terminate()
            proc.wait(timeout=10)
            out.close()
            store.db.close()


if __name__ == '__main__':
    main()
