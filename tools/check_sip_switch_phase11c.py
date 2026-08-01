#!/usr/bin/env python3
from __future__ import annotations

import importlib.util
import os
import subprocess
import sys
import tempfile
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "system-backend/sip-switch"
FALLBACK = BASE / "tbs-fallback"
REQUIRED = [
    BASE / "README.md",
    BASE / "install/install-tbs-local-fallback.sh",
    BASE / "install/update-tbs-local-fallback.sh",
    BASE / "install/apply-tbs-local-asterisk-config.sh",
    BASE / "install/tbs-fallback-status.sh",
    FALLBACK / "config/tbs-sip-fallback.example.toml",
    FALLBACK / "src/netcore_tbs_sip_fallback.py",
    FALLBACK / "systemd/netcore-tbs-sip-failover.service",
    FALLBACK / "install/install-tbs-local-fallback.sh",
    FALLBACK / "install/update-tbs-local-fallback.sh",
    FALLBACK / "install/migrate-phase11c-config.py",
    FALLBACK / "install/apply-native-tbs-config.py",
    FALLBACK / "install/status.sh",
    FALLBACK / "install/uninstall-tbs-local-fallback.sh",
    FALLBACK / "docs/installation-openlab.md",
    ROOT / "Docs/PHASE_11C_EXCLUSIVE_SIP_REGISTRATION_FAILOVER.md",
]


class FakeCLI:
    def __init__(self, active_file: Path):
        self.active_file = active_file
        self.aor_ok = True
        self.central_registered = True
        self.db_mode = None
        self.unregistered: list[str] = []
        self.reloads = 0

    def set_mode_db(self, mode: str) -> None:
        self.db_mode = mode

    def unregister(self, registration_id: str) -> None:
        self.unregistered.append(registration_id)

    def reload_registrations(self) -> None:
        self.reloads += 1
        text = self.active_file.read_text(encoding="utf-8")
        self.central_registered = "registration-central" in text

    def aor_available(self, _aor_id: str) -> bool:
        return self.aor_ok

    def registration_status(self, registration_id: str) -> str:
        if "central" in registration_id:
            return "registered" if self.central_registered and self.aor_ok else "absent"
        text = self.active_file.read_text(encoding="utf-8") if self.active_file.exists() else ""
        return "registered" if "registration-pbx-direct" in text else "absent"


def import_module(path: Path):
    spec = importlib.util.spec_from_file_location("phase11c", path)
    assert spec and spec.loader
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main() -> int:
    errors: list[str] = []
    for path in REQUIRED:
        if not path.is_file():
            errors.append(f"missing {path.relative_to(ROOT)}")
    for path in list((BASE / "install").glob("*.sh")) + list((FALLBACK / "install").glob("*.sh")):
        if not os.access(path, os.X_OK):
            errors.append(f"not executable {path.relative_to(ROOT)}")
    if errors:
        print("\n".join(errors), file=sys.stderr)
        return 1

    with (FALLBACK / "config/tbs-sip-fallback.example.toml").open("rb") as handle:
        cfg = tomllib.load(handle)
    assert cfg["service"]["phase"] == "11c"
    assert cfg["fallback_pbx"]["mode"] == "registration"
    assert cfg["failover"]["failure_threshold"] == 3
    assert cfg["failover"]["default_mode"] == "central"

    with tempfile.TemporaryDirectory() as raw_td:
        td = Path(raw_td)
        ast = td / "asterisk"
        config = td / "fallback.toml"
        state = td / "state.json"
        active = ast / "netcore-active-registration.conf"
        central_reg = ast / "netcore-registration-central.conf"
        pbx_reg = ast / "netcore-registration-pbx-direct.conf"
        text = (FALLBACK / "config/tbs-sip-fallback.example.toml").read_text(encoding="utf-8")
        replacements = {
            'config_dir = "/etc/asterisk"': f'config_dir = "{ast}"',
            'native_snippet_file = "/etc/netcore/tbs-asterisk-local-snippet.toml"': f'native_snippet_file = "{td / "native-snippet.toml"}"',
            'state_file = "/var/lib/netcore-tbs-sip-fallback/state.json"': f'state_file = "{state}"',
            'lock_file = "/run/netcore-tbs-sip-fallback.lock"': f'lock_file = "{td / "lock"}"',
            'active_registration_file = "/etc/asterisk/netcore-active-registration.conf"': f'active_registration_file = "{active}"',
            'central_registration_file = "/etc/asterisk/netcore-registration-central.conf"': f'central_registration_file = "{central_reg}"',
            'pbx_registration_file = "/etc/asterisk/netcore-registration-pbx-direct.conf"': f'pbx_registration_file = "{pbx_reg}"',
            'startup_grace_secs = 10': 'startup_grace_secs = 0',
            'failure_threshold = 3': 'failure_threshold = 1',
            'recovery_stable_secs = 30': 'recovery_stable_secs = 0',
            'central_registration_grace_secs = 15': 'central_registration_grace_secs = 0',
            'unregister_grace_secs = 2': 'unregister_grace_secs = 0',
        }
        for old, new in replacements.items():
            assert old in text
            text = text.replace(old, new, 1)
        config.write_text(text, encoding="utf-8")
        renderer = FALLBACK / "src/netcore_tbs_sip_fallback.py"
        run = subprocess.run([sys.executable, str(renderer), "--config", str(config), "--render"], capture_output=True, text=True)
        if run.returncode:
            print(run.stdout, run.stderr, file=sys.stderr)
            return 1

        base = (ast / "netcore-tbs-fallback-pjsip.conf").read_text(encoding="utf-8")
        dialplan = (ast / "netcore-tbs-fallback-extensions.conf").read_text(encoding="utf-8")
        assert "type=registration" not in base
        assert central_reg.read_text(encoding="utf-8").count("type=registration") == 1
        assert pbx_reg.read_text(encoding="utf-8").count("type=registration") == 1
        assert "netcore-registration-central.conf" in active.read_text(encoding="utf-8")
        assert "netcore-registration-pbx-direct.conf" not in active.read_text(encoding="utf-8")
        assert "${DB(netcore/failover_mode)}" in dialplan
        assert "pbx_direct" in dialplan
        assert '"${DIALSTATUS}"="NOANSWER"' not in dialplan

        module = import_module(renderer)
        fake = FakeCLI(active)
        controller = module.FailoverController(config, fake, sleeper=lambda _secs: None)
        fake.aor_ok = False
        state1 = controller.tick(now=100.0)
        assert state1["mode"] == "pbx_direct"
        assert fake.db_mode == "pbx_direct"
        assert "netcore-registration-pbx-direct.conf" in active.read_text(encoding="utf-8")
        assert "netcore-registration-central.conf" not in active.read_text(encoding="utf-8")

        fake.aor_ok = True
        controller.tick(now=200.0)
        state2 = controller.tick(now=201.0)
        assert state2["mode"] == "central"
        assert fake.db_mode == "central"
        assert "netcore-registration-central.conf" in active.read_text(encoding="utf-8")
        assert "netcore-registration-pbx-direct.conf" not in active.read_text(encoding="utf-8")
        assert any("pbx-fallback" in item for item in fake.unregistered)

        tbs_config = td / "tbs.toml"
        tbs_config.write_text('[general]\nname="test"\n[asterisk]\nenabled=false\noutbound_prefix="91*"\nremote_host="10.0.1.160"\n[brew]\nenabled=true\n', encoding="utf-8")
        apply_script = FALLBACK / "install/apply-native-tbs-config.py"
        run = subprocess.run([sys.executable, str(apply_script), "--config", str(tbs_config), "--snippet", str(td / "native-snippet.toml")], capture_output=True, text=True)
        if run.returncode:
            print(run.stdout, run.stderr, file=sys.stderr)
            return 1
        with tbs_config.open("rb") as handle:
            applied = tomllib.load(handle)
        assert applied["asterisk"]["remote_host"] == "127.0.0.1"
        assert applied["asterisk"]["remote_port"] == 5060
        assert applied["asterisk"]["outbound_prefix"] == "91*"

    print("OK: Phase 11c exclusive central/PBX registration failover with hysteresis and dialplan gate")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
