"""Real rsyslog protocol/reconnect test. Set RSYSLOGD and RSYSLOG_MODULE_DIR.

No root, systemd, remote LAN host, journal or share is touched by this test.
"""
import json
import os
from pathlib import Path
import re
import shutil
import socket
import sqlite3
import subprocess
import sys
import tempfile
import time
import unittest

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "logging"))
from log_client import render
RSYSLOGD = os.environ.get("RSYSLOGD") or shutil.which("rsyslogd")


def free_port():
    with socket.socket() as s:
        s.bind(("127.0.0.1", 0))
        return s.getsockname()[1]


@unittest.skipUnless(RSYSLOGD, "rsyslog is not installed")
class WireTests(unittest.TestCase):
    def test_production_journal_sender_configuration(self):
        with tempfile.TemporaryDirectory(prefix="netcore-journal-config-") as tmp:
            path = Path(tmp) / "client.conf"
            path.write_text(render("10.0.1.143").replace("/var/lib/netcore-log-client", tmp))
            command = [RSYSLOGD]
            if os.environ.get("RSYSLOG_MODULE_DIR"):
                command += ["-M", os.environ["RSYSLOG_MODULE_DIR"]]
            result = subprocess.run(command + ["-N1", "-f", str(path)], capture_output=True, text=True)
            self.assertEqual(result.returncode, 0, result.stderr)

    def test_tcp_udp_relp_and_sender_queue_after_receiver_restart(self):
        with tempfile.TemporaryDirectory(prefix="netcore-wire-") as tmp:
            base = Path(tmp)
            (base / "queue").mkdir()
            (base / "sender").mkdir()
            tcp, udp, relp = free_port(), free_port(), free_port()
            cfg = json.loads((ROOT / "config/syslog.example.json").read_text())
            cfg.update(state_dir=str(base / "store"), inventory=str(ROOT / "config/openlab-hosts.json"))
            (base / "syslog.json").write_text(json.dumps(cfg))
            receiver = (ROOT / "logging/receiver.rsyslog.conf").read_text().replace(
                "/var/lib/netcore-observability/logs/queue", str(base / "queue")).replace(
                "/opt/netcore-observability/logging/log_store.py", str(ROOT / "logging/log_store.py")).replace(
                "/etc/netcore/syslog.json", str(base / "syslog.json")).replace(
                'type="imtcp" address="0.0.0.0" port="514"', f'type="imtcp" address="127.0.0.1" port="{tcp}"').replace(
                'type="imudp" address="0.0.0.0" port="514"', f'type="imudp" address="127.0.0.1" port="{udp}"').replace(
                'type="imrelp" address="0.0.0.0" port="20514"', f'type="imrelp" address="127.0.0.1" port="{relp}"')
            (base / "receiver.conf").write_text(receiver)
            source = base / "source.log"
            source.touch()
            sender = render("127.0.0.1").replace("/var/lib/netcore-log-client", str(base / "sender"))
            sender = re.sub(r'module\(load="imjournal".*?\)',
                f'module(load="imfile" PollingInterval="1")\ninput(type="imfile" File="{source}" Tag="netcore-wire" PersistStateInterval="1")', sender, flags=re.S)
            sender = sender.replace('port="20514"', f'port="{relp}"')
            (base / "sender.conf").write_text(sender)
            commands = [RSYSLOGD]
            if os.environ.get("RSYSLOG_MODULE_DIR"):
                commands += ["-M", os.environ["RSYSLOG_MODULE_DIR"]]
            for name in ("receiver", "sender"):
                check = subprocess.run(commands + ["-N1", "-f", str(base / (name + ".conf"))], capture_output=True, text=True)
                self.assertEqual(check.returncode, 0, check.stderr)
            processes = []
            def start(name):
                output = (base / (name + ".out")).open("a")
                proc = subprocess.Popen(commands + ["-n", "-i", str(base / (name + ".pid")), "-f", str(base / (name + ".conf"))],
                                        stdout=output, stderr=output)
                output.close()
                processes.append(proc)
                return proc
            def stop(proc):
                proc.terminate()
                try:
                    proc.wait(timeout=8)
                except subprocess.TimeoutExpired:
                    proc.kill()
                    proc.wait()
            def wait_for(predicate, seconds=25):
                deadline = time.monotonic() + seconds
                while time.monotonic() < deadline:
                    if predicate():
                        return
                    time.sleep(.1)
                self.fail("Timed out: " + (base / "receiver.out").read_text() + (base / "sender.out").read_text())
            def received(text):
                try:
                    with sqlite3.connect(base / "store/preview.sqlite") as db:
                        return any(text in json.loads(r[0])["message"] for r in db.execute("SELECT payload FROM outbox"))
                except sqlite3.Error:
                    return False
            try:
                recv = start("receiver")
                start("sender")
                def listening():
                    try:
                        socket.create_connection(("127.0.0.1", tcp), timeout=.1).close()
                        return True
                    except OSError:
                        return False
                wait_for(listening)
                with socket.create_connection(("127.0.0.1", tcp)) as s:
                    s.sendall(b'<14>1 2026-09-27T13:00:00Z test netcore-test 1 - - tcp-quote-"test"\n')
                with socket.socket(socket.AF_INET, socket.SOCK_DGRAM) as s:
                    s.sendto(b'<11>Sep 27 13:00:00 test netcore-test: udp-test', ("127.0.0.1", udp))
                with source.open("a") as f:
                    f.write("relp-before-restart\n")
                wait_for(lambda: received("tcp-quote") and received("udp-test") and received("relp-before-restart"))
                stop(recv)
                with source.open("a") as f:
                    f.write("relp-during-outage\n")
                wait_for(lambda: any((base / "sender").glob("forward.*")))
                recv = start("receiver")
                wait_for(lambda: received("relp-during-outage"))
                self.assertTrue(received("relp-before-restart"))
            finally:
                for proc in reversed(processes):
                    if proc.poll() is None:
                        stop(proc)


if __name__ == "__main__":
    unittest.main()
