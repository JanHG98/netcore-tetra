#!/usr/bin/env python3
"""Disposable CI only: exercise the production unit's actual systemd namespace."""
import os
from pathlib import Path
import re
import shutil
import stat
import subprocess
import unittest
import uuid


ROOT = Path(__file__).resolve().parents[1]
UNIT = ROOT / "system-backend/sip-switch/tbs-fallback/systemd/netcore-tbs-sip-failover.service"


@unittest.skipUnless(os.name == "posix" and os.environ.get("GITHUB_ACTIONS") == "true", "Disposable Linux CI only")
class ActualFailoverNamespaceTests(unittest.TestCase):
    def test_missing_directory_fails_before_fix_and_is_created_before_namespace_setup(self):
        self.assertEqual(os.geteuid(), 0, "Run this disposable CI test with sudo")
        self.assertEqual(Path("/proc/1/comm").read_text().strip(), "systemd")
        name = "netcore-ci-sip-failover-" + uuid.uuid4().hex
        unit_name = name + ".service"
        unit_path = Path("/run/systemd/system") / unit_name
        unit_path.parent.mkdir(parents=True, exist_ok=True)
        state_dir = Path("/var/lib") / name
        runtime_dir = Path("/run") / name
        config_dir = runtime_dir / "asterisk"
        config_dir.mkdir(parents=True)

        def command(*args, check=True):
            result = subprocess.run(args, capture_output=True, text=True, timeout=20)
            if check and result.returncode:
                self.fail("Command failed: " + " ".join(args) + "\n" + result.stdout + result.stderr)
            return result

        def cleanup():
            command("systemctl", "stop", unit_name, check=False)
            unit_path.unlink(missing_ok=True)
            command("systemctl", "daemon-reload", check=False)
            command("systemctl", "reset-failed", unit_name, check=False)
            for path, parent in ((state_dir, Path("/var/lib")), (runtime_dir, Path("/run"))):
                # Only the two UUID-named trees owned by this test may be removed.
                assert path.parent == parent and path.name == name
                assert re.fullmatch(r"netcore-ci-sip-failover-[0-9a-f]{32}", path.name)
                if path.exists():
                    shutil.rmtree(path)
        self.addCleanup(cleanup)

        source = UNIT.read_text(encoding="utf-8")
        self.assertIn("StateDirectory=netcore-tbs-sip-fallback\n", source)
        # Keep the production sandbox properties. Only the dependency and command
        # are replaced so no Asterisk or real station service is touched.
        source = source.replace("After=network-online.target asterisk.service", "After=network-online.target")
        source = source.replace("Requires=asterisk.service\n", "")
        source = source.replace("Type=simple", "Type=oneshot").replace("Restart=always", "Restart=no")
        source = re.sub(r"^ExecStart=.*$", "ExecStart=/usr/bin/python3 -c \"from pathlib import Path; Path('" + str(state_dir / "namespace-ok") + "').write_text('ok')\"", source, flags=re.MULTILINE)
        source = source.replace("/etc/asterisk", str(config_dir)).replace("netcore-tbs-sip-fallback", name)
        self.assertFalse(state_dir.exists())

        old = "\n".join(line for line in source.splitlines() if not line.startswith(("StateDirectory=", "StateDirectoryMode="))) + "\n"
        unit_path.write_text(old, encoding="utf-8")
        command("systemctl", "daemon-reload")
        self.assertNotEqual(command("systemctl", "start", unit_name, check=False).returncode, 0)
        self.assertEqual(command("systemctl", "show", unit_name, "-p", "ExecMainStatus", "--value").stdout.strip(), "226")
        self.assertFalse(state_dir.exists())

        unit_path.write_text(source, encoding="utf-8")
        command("systemctl", "daemon-reload")
        command("systemctl", "reset-failed", unit_name)
        command("systemctl", "start", unit_name)
        self.assertEqual(command("systemctl", "show", unit_name, "-p", "Result", "--value").stdout.strip(), "success")
        self.assertEqual((state_dir / "namespace-ok").read_text(), "ok")
        self.assertEqual(stat.S_IMODE(state_dir.stat().st_mode), 0o750)
        print("PASS: production sandbox fails with NAMESPACE/226 without its state directory; StateDirectory creates it before namespace setup")


if __name__ == "__main__":
    unittest.main(verbosity=2)
