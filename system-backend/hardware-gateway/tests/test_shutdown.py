"""Exercise real HTTP and SIGTERM, including a quiet MQTT child process."""
import os
from pathlib import Path
import signal
import socket
import subprocess
import sys
import tempfile
import time
import unittest
from urllib.request import urlopen

ROOT = Path(__file__).resolve().parents[1]


class ShutdownTests(unittest.TestCase):
    def test_sigterm_exits_and_reaps_mqtt_child(self):
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            mqtt = root / 'mosquitto_sub'
            mqtt.write_text('#!' + sys.executable + '\nimport os,time\nfrom pathlib import Path\n'
                            'Path(os.environ["NC_MQTT_PIDFILE"]).write_text(str(os.getpid()))\n'
                            'time.sleep(60)\n')
            mqtt.chmod(0o755)
            with socket.socket() as sock:
                sock.bind(('127.0.0.1', 0))
                port = sock.getsockname()[1]
            config = (ROOT / 'config/hardware-gateway.example.toml').read_text()
            config = config.replace('0.0.0.0:8250', f'127.0.0.1:{port}')
            config = config.replace('/var/lib/netcore-hardware-gateway', str(root))
            path = root / 'config.toml'
            path.write_text(config)
            pidfile = root / 'mqtt.pid'
            with (root / 'output.log').open('w+') as output:
                process = subprocess.Popen([sys.executable, str(ROOT / 'src/netcore_hardware_gateway.py'),
                                            '--config', str(path)], stdout=output, stderr=output,
                    env=dict(os.environ, PATH=str(root) + ':' + os.environ['PATH'], NC_MQTT_PIDFILE=str(pidfile)),
                    start_new_session=True)
                try:
                    deadline = time.monotonic() + 8
                    while True:
                        try:
                            with urlopen(f'http://127.0.0.1:{port}/health/live', timeout=.3) as response:
                                self.assertEqual(response.status, 200)
                            if pidfile.exists(): break
                        except OSError: pass
                        if time.monotonic() >= deadline:
                            output.seek(0)
                            self.fail('Gateway not ready: ' + output.read())
                        time.sleep(.03)
                    child = int(pidfile.read_text())
                    process.send_signal(signal.SIGTERM)
                    self.assertEqual(process.wait(timeout=5), 0)
                    with self.assertRaises(ProcessLookupError): os.kill(child, 0)
                finally:
                    if process.poll() is None:
                        os.killpg(process.pid, signal.SIGKILL)
                        process.wait()


if __name__ == '__main__': unittest.main()
