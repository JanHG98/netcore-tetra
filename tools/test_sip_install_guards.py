#!/usr/bin/env python3
"""Exercise host-role preflights without changing host files or services."""
import os
from pathlib import Path
import shutil
import subprocess
import tempfile
import tomllib
import unittest


ROOT = Path(__file__).resolve().parents[1]
SERVICE = ROOT / "system-backend/sip-switch"
HELPER = SERVICE / "install/check-host-role.sh"
CENTRAL_SCRIPTS = ("install/install.sh", "install/update.sh")
TBS_SCRIPTS = (
    "tbs-fallback/install/install-tbs-local-fallback.sh",
    "tbs-fallback/install/update-tbs-local-fallback.sh",
)
ARGS = ["SRV-M-TBS-01", "10.0.1.20", "10.0.1.125", "tbs-srv-m-tbs-01",
        "secret-central", "10.0.1.21", "104", "104", "secret-pbx"]


class SipInstallGuardTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.fixture = Path(self.temp.name)
        self.host = self.fixture / "host"
        self.repo = self.fixture / "repo"
        self.bin = self.fixture / "bin"
        self.bin.mkdir()
        self.log = self.fixture / "events"
        self.env = os.environ.copy()
        self.env.update(PATH=str(self.bin) + ":" + self.env.get("PATH", ""),
                        TEST_ROOT=str(self.host), TEST_LOG=str(self.log), TEST_UNIT_STATE="")
        # Only the fixture helper gets a default root. Production installers have
        # no root environment override; all their instructions run unchanged.
        helper = self.repo / "install/check-host-role.sh"
        helper.parent.mkdir(parents=True)
        helper.write_text(HELPER.read_text().replace('root="${2:-}"', 'root="${2:-$TEST_ROOT}"'))
        for relative in (*CENTRAL_SCRIPTS, *TBS_SCRIPTS):
            dest = self.repo / relative
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(SERVICE / relative, dest)
        for command in ("apt-get", "install", "cp", "chmod", "asterisk"):
            script = self.bin / command
            script.write_text('#!/bin/bash\nprintf "mutation:%s\\n" "${0##*/}" >>"$TEST_LOG"\nexit 90\n')
            script.chmod(0o755)
        systemctl = self.bin / "systemctl"
        systemctl.write_text('''#!/bin/bash
case "$1" in
  is-active|is-enabled) [[ "$1" == "$TEST_UNIT_STATE" ]]; exit $? ;;
  *) printf 'mutation:systemctl\\n' >>"$TEST_LOG"; exit 90 ;;
esac
''')
        systemctl.chmod(0o755)

    def write_host(self, relative, content=""):
        path = self.host / relative
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(content)

    def run_script(self, relative, args=None):
        if args is None:
            args = ARGS if relative.endswith("/install-tbs-local-fallback.sh") else []
        return subprocess.run(["bash", str(self.repo / relative), *args], env=self.env,
                              text=True, capture_output=True, timeout=10)

    def assert_blocked_before_mutation(self, result):
        self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
        self.assertFalse(self.log.exists(), self.log.read_text() if self.log.exists() else "")

    def test_existing_tbs_blocks_central_install_and_update(self):
        self.write_host("etc/netcore/tbs-sip-fallback.toml")
        for script in CENTRAL_SCRIPTS:
            with self.subTest(script=script):
                result = self.run_script(script)
                self.assert_blocked_before_mutation(result)
                self.assertIn("TBS-Fallback", result.stderr)

    def test_existing_central_blocks_tbs_install_and_update(self):
        self.write_host("etc/netcore/sip-switch.toml")
        for script in TBS_SCRIPTS:
            with self.subTest(script=script):
                result = self.run_script(script)
                self.assert_blocked_before_mutation(result)
                self.assertIn("zentraler SIP-Switch", result.stderr)

    def test_active_managed_includes_block_without_config(self):
        cases = [
            (CENTRAL_SCRIPTS, "pjsip.conf", '#include netcore-tbs-fallback-pjsip.conf'),
            (CENTRAL_SCRIPTS, "pjsip.conf", '#include netcore-active-registration.conf'),
            (CENTRAL_SCRIPTS, "extensions.conf", '#include netcore-tbs-fallback-extensions.conf'),
            (CENTRAL_SCRIPTS, "rtp.conf", '#include netcore-tbs-fallback-rtp.conf'),
            (TBS_SCRIPTS, "pjsip.conf", '  #include "netcore-pjsip.conf" ; managed'),
            (TBS_SCRIPTS, "extensions.conf", '#tryinclude /etc/asterisk/netcore-extensions.conf'),
            (TBS_SCRIPTS, "rtp.conf", '#include <netcore-rtp.conf>'),
        ]
        for scripts, filename, line in cases:
            with self.subTest(include=line):
                self.write_host("etc/asterisk/" + filename, line + "\n")
                for script in scripts:
                    self.assert_blocked_before_mutation(self.run_script(script))
                (self.host / "etc/asterisk" / filename).unlink()

    def test_active_or_enabled_opposite_unit_blocks_without_config(self):
        for state in ("is-active", "is-enabled"):
            self.env["TEST_UNIT_STATE"] = state
            for script in (*CENTRAL_SCRIPTS, *TBS_SCRIPTS):
                with self.subTest(state=state, script=script):
                    self.assert_blocked_before_mutation(self.run_script(script))

    def test_disabled_unit_stale_files_and_commented_includes_allow_recovery(self):
        self.write_host("etc/systemd/system/netcore-sip-switch.service")
        self.write_host("etc/netcore/sip-switch.toml.disabled")
        self.write_host("etc/asterisk/netcore-pjsip.conf")
        self.write_host("etc/asterisk/pjsip.conf", '; #include netcore-pjsip.conf\n')
        result = subprocess.run(
            ["bash", "-c", 'set -euo pipefail; source "$1"; netcore_check_sip_host_role tbs "$2"',
             "guard-test", str(HELPER), str(self.host)],
            env=self.env, text=True, capture_output=True, timeout=10,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertFalse(self.log.exists())

    def test_placeholder_in_any_argument_blocks_without_echoing_credentials(self):
        for index in range(len(ARGS)):
            with self.subTest(index=index):
                args = ARGS.copy()
                args[index] = "secret-prefix-<PBX-FALLBACK-ID>"
                result = self.run_script(TBS_SCRIPTS[0], args)
                self.assert_blocked_before_mutation(result)
                self.assertIn("Platzhalter", result.stderr)
                self.assertNotIn("secret", result.stdout + result.stderr)
                self.assertNotIn("<PBX-FALLBACK-ID>", result.stdout + result.stderr)

    def test_config_values_round_trip_quotes_backslashes_and_unicode(self):
        # Execute the real embedded renderer with adversarial but valid TOML
        # string values. No installed files or Asterisk are involved.
        script = (SERVICE / TBS_SCRIPTS[0]).read_text()
        renderer = script.split("<<'PY'\n", 1)[1].split("\nPY\n", 1)[0]
        dest = self.fixture / "rendered.toml"
        password = 'quoted" \\ password\nwith Unicode 🛰'
        self.env["NETCORE_ASTERISK_BINARY"] = "/usr/sbin/asterisk"
        result = subprocess.run(
            ["python3", "-", str(SERVICE / "tbs-fallback/config/tbs-sip-fallback.example.toml"),
             str(dest), ARGS[0], ARGS[1], ARGS[2], ARGS[3], password, ARGS[5], ARGS[6],
             "native-user", "native-password", ARGS[7], password],
            input=renderer, env=self.env, text=True, capture_output=True, timeout=10,
        )
        self.assertEqual(result.returncode, 0, result.stderr)
        config = tomllib.loads(dest.read_text())
        self.assertEqual(config["central"]["password"], password)
        self.assertEqual(config["fallback_pbx"]["password"], password)
        self.assertEqual(config["fallback_pbx"]["username"], "104")
        self.assertEqual(config["fallback_pbx"]["match"], ["10.0.1.21"])


if __name__ == "__main__":
    unittest.main()
