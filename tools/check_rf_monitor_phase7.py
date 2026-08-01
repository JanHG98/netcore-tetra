#!/usr/bin/env python3
from __future__ import annotations
import json, py_compile, subprocess, sys, tomllib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
required=[
 'system-backend/rf-monitor/src/netcore_rf_monitor.py',
 'system-backend/rf-monitor/config/rf-monitor.example.toml',
 'system-backend/rf-monitor/systemd/netcore-rf-monitor.service',
 'system-backend/rf-monitor/install/install.sh',
 'system-backend/rf-monitor/install/update.sh',
 'system-backend/rf-monitor/install/uninstall.sh',
 'system-backend/rf-monitor/install/install-tbs-agent.sh',
 'system-backend/rf-monitor/examples/tbs-agent/netcore-rf-agent.py',
 'system-backend/rf-monitor/examples/tbs-agent/rf-agent.example.toml',
 'system-backend/rf-monitor/examples/probes/mock-rf-probe.py',
 'Docs/PHASE_7_RF_MONITORING.md',
]
for rel in required:
 p=ROOT/rel
 if not p.exists(): raise SystemExit(f'MISSING: {rel}')
for rel in [required[0],required[7],required[9]]:
 py_compile.compile(str(ROOT/rel),doraise=True)
config=tomllib.loads((ROOT/required[1]).read_text())
assert config['server']['bind'].endswith(':8260')
assert config['security']['mode']=='open_lab'
source=(ROOT/required[0]).read_text()
for token in ['netcore-rf-telemetry-v1','rf.alarm_raised','reflected_power_ratio_percent','return_loss_db','/api/v1/telemetry','/metrics']:
 assert token in source, token
rust=(ROOT/'crates/tetra-entities/src/net_dashboard/server.rs').read_text()
for token in ['/api/rf-monitor','netcore-rf-tbs-v1','serve_rf_monitor_snapshot']:
 assert token in rust, token
for script in (ROOT/'system-backend/rf-monitor/install').glob('*.sh'):
 subprocess.run(['bash','-n',str(script)],check=True)
print('OK: Phase 7 RF monitor, TBS export endpoint, agent, probe adapter and OPEN-LAB installers')
