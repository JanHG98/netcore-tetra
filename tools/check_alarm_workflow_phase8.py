#!/usr/bin/env python3
from __future__ import annotations
import ast, json, tomllib
from pathlib import Path
ROOT=Path(__file__).resolve().parents[1]
required=[
 'system-backend/alarm-workflow/src/netcore_alarm_workflow.py',
 'system-backend/alarm-workflow/config/alarm-workflow.example.toml',
 'system-backend/alarm-workflow/systemd/netcore-alarm-workflow.service',
 'system-backend/alarm-workflow/install/install.sh',
 'system-backend/alarm-workflow/install/update.sh',
 'system-backend/alarm-workflow/install/uninstall.sh',
 'system-backend/alarm-workflow/install/configure-openlab.sh',
 'Docs/PHASE_8_SDS_STATUS_ALARM_WORKFLOWS.md',
]
errors=[]
for rel in required:
    if not (ROOT/rel).is_file(): errors.append(f'missing {rel}')
source=(ROOT/required[0]).read_text(encoding='utf-8')
try: ast.parse(source)
except SyntaxError as e: errors.append(f'python syntax: {e}')
with (ROOT/required[1]).open('rb') as f: cfg=tomllib.load(f)
if cfg['server']['bind']!='0.0.0.0:8270': errors.append('wrong alarm workflow port')
if cfg['security']['mode']!='open_lab': errors.append('not open_lab')
for token in ['alarm.created','alarm.acknowledged','alarm.escalated','sds.received','netcore-event-v1','5201','ACK|TAKE|START|RESOLVE|CLOSE']:
    if token not in source and token not in (ROOT/required[1]).read_text(encoding='utf-8'):
        errors.append(f'missing token {token}')
for script in (ROOT/'system-backend/alarm-workflow/install').glob('*.sh'):
    if not (script.stat().st_mode & 0o111): errors.append(f'not executable {script.relative_to(ROOT)}')
contracts=(ROOT/'system-backend/shared/contracts/src/event.rs').read_text(encoding='utf-8')
for token in ['ALARM_CREATED','ALARM_ACKNOWLEDGED','HARDWARE_THRESHOLD_CLEARED','RF_ALARM_RAISED','pub const ALARM:']:
    if token not in contracts: errors.append(f'contract missing {token}')
sds=(ROOT/'system-backend/sds-router/src/state.rs').read_text(encoding='utf-8')
for token in ['"status_code": status_code','"text": event_text','"sds_type": sds_type']:
    if token not in sds: errors.append(f'SDS canonical payload missing {token}')
hw=(ROOT/'system-backend/hardware-gateway/src/netcore_hardware_gateway.py').read_text(encoding='utf-8')
for token in ['hardware.threshold_cleared','hardware.input_activated','hardware.input_cleared','hardware.device_online']:
    if token not in hw: errors.append(f'hardware event missing {token}')
if errors:
    print('\n'.join('ERROR: '+e for e in errors)); raise SystemExit(1)
print('OK: Phase 8 alarm workflow, SDS/status feedback and hardware clear events are wired')
