"""Run the installer's actual health probe against local startup conditions."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import threading
import unittest


DEPLOY = Path(__file__).resolve().parents[1] / "install/deploy.sh"
PROBE = DEPLOY.read_text(encoding="utf-8").split("<<'HEALTHCHECK'\n", 1)[1].split("\nHEALTHCHECK", 1)[0]


class InstallHealthTests(unittest.TestCase):
    def probe(self, port):
        with tempfile.TemporaryDirectory() as directory:
            config = Path(directory) / "alert-service.toml"
            config.write_text(f'[server]\nbind = "0.0.0.0"\nport = {port}\n', encoding="utf-8")
            env = dict(os.environ)
            # A bad global proxy must not block a check of the local service.
            env.update(http_proxy="http://127.0.0.1:1", HTTP_PROXY="http://127.0.0.1:1",
                       no_proxy="", NO_PROXY="")
            result = subprocess.run([sys.executable, "-", str(config)], input=PROBE,
                                    text=True, capture_output=True, env=env, timeout=5)
            self.assertEqual(result.stdout, "")
            self.assertEqual(result.stderr, "")
            return result.returncode

    def test_startup_refusal_retries_quietly_then_accepts_running_service(self):
        # Reserve a port without listening: the first probe must fail quietly.
        with socket.socket() as reserved:
            reserved.bind(("127.0.0.1", 0))
            port = reserved.getsockname()[1]
            self.assertEqual(self.probe(port), 1)
        self.assertEqual(self.with_server(port, 200), 0)

    def test_http_failure_does_not_report_success_or_print_traceback(self):
        self.assertEqual(self.with_server(0, 503), 1)

    def with_server(self, port, status):
        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                self.send_response(status if self.path == "/health/live" else 404)
                self.end_headers()

            def log_message(self, *args):
                pass

        with ThreadingHTTPServer(("127.0.0.1", port), Handler) as server:
            worker = threading.Thread(target=server.serve_forever, kwargs={"poll_interval": 0.01})
            worker.start()
            try:
                return self.probe(server.server_port)
            finally:
                server.shutdown()
                worker.join(timeout=3)


if __name__ == "__main__":
    unittest.main()
