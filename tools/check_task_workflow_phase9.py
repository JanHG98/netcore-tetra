#!/usr/bin/env python3
from pathlib import Path
import json
import os
import re
import subprocess
import sys
import tempfile
import time
import tomllib
from urllib.parse import urlencode
from urllib.request import Request, urlopen
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[1]
REQUIRED = [
    "system-backend/task-workflow/src/netcore_task_workflow.py",
    "system-backend/task-workflow/config/task-workflow.example.toml",
    "system-backend/task-workflow/systemd/netcore-task-workflow.service",
    "system-backend/task-workflow/install/install.sh",
    "system-backend/task-workflow/install/update.sh",
    "system-backend/task-workflow/install/uninstall.sh",
    "system-backend/task-workflow/install/configure-openlab.sh",
    "system-backend/shared/contracts/schemas/netcore-task-v1.schema.json",
    "system-backend/shared/contracts/TASK_MODEL_V1.md",
    "Docs/PHASE_9_WAP_FORMS_STRUCTURED_TASKS.md",
]
errors: list[str] = []
for rel in REQUIRED:
    if not (ROOT / rel).is_file():
        errors.append(f"missing {rel}")
with open(ROOT / "system-backend/task-workflow/config/task-workflow.example.toml", "rb") as handle:
    cfg = tomllib.load(handle)
if not cfg["server"]["bind"].endswith(":8280"):
    errors.append("wrong management port")
if cfg["security"]["mode"] != "open_lab":
    errors.append("not open_lab")
if len(cfg["templates"]) < 6:
    errors.append("too few task templates")
schema = json.loads((ROOT / "system-backend/shared/contracts/schemas/netcore-task-v1.schema.json").read_text())
if schema["properties"]["schema"]["const"] != "netcore-task-v1":
    errors.append("wrong task schema")
event_source = (ROOT / "system-backend/shared/contracts/src/event.rs").read_text()
for name in [
    "task.created", "task.assigned", "task.accepted", "task.in_progress",
    "task.blocked", "task.completed", "task.cancelled", "task.expired",
    "task.reopened", "task.comment_added", "task.notification_queued",
    "task.notification_failed",
]:
    if name not in event_source:
        errors.append(f"event missing {name}")
for script in (ROOT / "system-backend/task-workflow/install").glob("*.sh"):
    if not os.access(script, os.X_OK):
        errors.append(f"not executable {script}")
portal_source = (ROOT / "crates/tetra-entities/src/sndcp/wap_portal.rs").read_text()
for marker in ["ALL_PAGES: [WapPage; 21]", "WapPage::Tasks", "WapPage::TaskForms", 'Self::Tasks => "tk"', 'Self::TaskForms => "fm"']:
    if marker not in portal_source:
        errors.append(f"compact WAP portal missing {marker}")
if errors:
    print("\n".join(errors), file=sys.stderr)
    raise SystemExit(1)

# Real HTTP runtime smoke with MQTT and SDS disabled.
with tempfile.TemporaryDirectory() as temp_dir:
    temp = Path(temp_dir)
    port = 18280
    config = f'''[service]
name = "netcore-task-workflow"
phase = 9
mode = "open_lab"
[server]
bind = "127.0.0.1:{port}"
[security]
mode = "open_lab"
[storage]
state_file = "{temp / 'state.json'}"
event_log = "{temp / 'events.ndjson'}"
audit_log = "{temp / 'audit.ndjson'}"
[mqtt]
enabled = false
host = "127.0.0.1"
port = 1883
topic_prefix = "netcore/v1"
client_id = "test"
[sds_router]
enabled = false
base_url = "http://127.0.0.1:1"
source_issi = 9999
protocol_id = 130
ttl_secs = 600
max_text_length = 160
default_destination = 15201
default_is_group = true
[workflow]
event_history_limit = 200
seen_event_limit = 1000
expire_check_interval_secs = 2
notify_on_state_change = false
[wap]
enabled = true
page_size = 6
xhtml_entry = "/x"
wml_entry = "/w"
[[templates]]
id = "technical_fault"
name = "Technische Stoerung"
description = "Test"
default_priority = 7
default_severity = "warning"
requires_ack = true
[[templates.fields]]
id = "asset"
label = "Anlage"
type = "text"
required = true
[[templates.fields]]
id = "fault"
label = "Fehler"
type = "text"
required = true
[[status_actions]]
status_code = 5301
action = "accept"
label = "Annehmen"
'''
    config_path = temp / "config.toml"
    config_path.write_text(config)
    process = subprocess.Popen(
        [sys.executable, str(ROOT / "system-backend/task-workflow/src/netcore_task_workflow.py"), "--config", str(config_path)],
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
    )
    try:
        for _ in range(50):
            try:
                urlopen(f"http://127.0.0.1:{port}/health/live", timeout=0.2)
                break
            except Exception:
                time.sleep(0.1)
        else:
            stderr = process.stderr.read().decode() if process.stderr else ""
            raise RuntimeError(f"service did not start: {stderr}")

        openapi = json.loads(urlopen(f"http://127.0.0.1:{port}/openapi.json").read())
        assert openapi["openapi"] == "3.0.3"

        payload = {
            "template_id": "technical_fault",
            "title": "Testauftrag",
            "assigned_issi": 4010001,
            "form_data": {"asset": "TBS-01", "fault": "Antenne"},
            "notify": False,
        }
        request = Request(
            f"http://127.0.0.1:{port}/api/v1/tasks",
            data=json.dumps(payload).encode(), method="POST",
            headers={"Content-Type": "application/json"},
        )
        task = json.loads(urlopen(request).read())
        assert task["schema"] == "netcore-task-v1" and task["state"] == "assigned"
        for action, state in [("accept", "accepted"), ("start", "in_progress"), ("complete", "completed")]:
            request = Request(
                f"http://127.0.0.1:{port}/api/v1/tasks/{task['task_id']}/{action}",
                data=b"{}", method="POST", headers={"Content-Type": "application/json"},
            )
            task = json.loads(urlopen(request).read())
            assert task["state"] == state

        for path in [
            "/x?issi=4010001", "/w?issi=4010001",
            "/x/new?issi=4010001&template=technical_fault",
            "/w/new?issi=4010001&template=technical_fault",
        ]:
            data = urlopen(f"http://127.0.0.1:{port}{path}").read()
            ET.fromstring(re.sub(br"<!DOCTYPE[^>]*>", b"", data, count=1))

        form = urlencode({
            "template_id": "technical_fault", "title": "WML-Test",
            "assigned_issi": "4010002", "f_asset": "Rack", "f_fault": "Warm",
        }).encode()
        request = Request(
            f"http://127.0.0.1:{port}/w/submit?issi=4010002",
            data=form, method="POST",
            headers={"Content-Type": "application/x-www-form-urlencoded"},
        )
        ET.fromstring(re.sub(br"<!DOCTYPE[^>]*>", b"", urlopen(request).read(), count=1))
    finally:
        process.terminate()
        process.wait(timeout=5)

print("OK: Phase 9 task workflow, REST, state machine, XHTML and WML runtime smoke passed")
