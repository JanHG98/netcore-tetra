#!/usr/bin/env python3
"""Exercise the real gateway's HTTP startup and MQTT replay with a full outbox."""
from __future__ import annotations

import argparse
from contextlib import contextmanager
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import json
import os
from pathlib import Path
import socket
import subprocess
import tempfile
import threading
import time
import unittest
from urllib.error import HTTPError
from urllib.request import build_opener, ProxyHandler, Request
import uuid

ROOT = Path(__file__).resolve().parents[1]
BINARY = ROOT / 'target/debug' / ('netcore-iot-gateway.exe' if os.name == 'nt' else 'netcore-iot-gateway')
HTTP = build_opener(ProxyHandler({}))


def packet(stream: socket.socket) -> tuple[int, bytes]:
    def exact(size: int) -> bytes:
        result = bytearray()
        while len(result) < size:
            data = stream.recv(size - len(result))
            if not data:
                raise EOFError
            result.extend(data)
        return bytes(result)

    header = exact(1)[0]
    size, multiplier = 0, 1
    for _ in range(4):
        digit = exact(1)[0]
        size += (digit & 127) * multiplier
        if digit < 128:
            if size > 1_048_576:
                raise ValueError('unexpectedly large test MQTT packet')
            return header, exact(size)
        multiplier *= 128
    raise ValueError('invalid test MQTT remaining length')


class Broker:
    """Reserve a broker port, then enable the MQTT protocol only on request."""
    def __init__(self):
        self.listener = socket.socket()
        self.listener.bind(('127.0.0.1', 0))
        self.listener.listen(8)
        self.listener.settimeout(0.2)
        self.port = self.listener.getsockname()[1]
        self.stopped = threading.Event()
        self.published = []
        self.thread = None

    def start(self):
        self.thread = threading.Thread(target=self.run, daemon=True)
        self.thread.start()

    def run(self):
        while not self.stopped.is_set():
            try:
                stream, _ = self.listener.accept()
            except socket.timeout:
                continue
            except OSError:
                return
            with stream:
                stream.settimeout(0.5)
                try:
                    header, _ = packet(stream)
                    if header != 0x10:
                        continue
                    stream.sendall(b'\x20\x02\x00\x00')
                    while not self.stopped.is_set():
                        try:
                            header, body = packet(stream)
                        except socket.timeout:
                            continue
                        kind = header >> 4
                        if kind == 3:
                            size = int.from_bytes(body[:2], 'big')
                            topic = body[2:2 + size].decode()
                            offset = 2 + size
                            qos = (header >> 1) & 3
                            packet_id = body[offset:offset + 2] if qos else None
                            if qos:
                                offset += 2
                            self.published.append((topic, body[offset:]))
                            if packet_id is not None:
                                stream.sendall(b'\x40\x02' + packet_id)
                        elif kind == 8:
                            stream.sendall(b'\x90\x03' + body[:2] + b'\x01')
                        elif kind == 12:
                            stream.sendall(b'\xd0\x00')
                except (OSError, EOFError):
                    continue

    def close(self):
        self.stopped.set()
        self.listener.close()
        if self.thread is not None:
            self.thread.join(timeout=2)


def request(base: str, path: str, body=None):
    data = None if body is None else json.dumps(body).encode()
    req = Request(base + path, data=data, headers={'Content-Type': 'application/json'})
    try:
        response = HTTP.open(req, timeout=1)
    except HTTPError as error:
        response = error
    with response:
        raw = response.read()
        return response.status, raw


@contextmanager
def gateway(config: Path, base: str, log: Path):
    with log.open('wb') as output:
        process = subprocess.Popen([str(BINARY), '--config', str(config)],
                                   stdin=subprocess.DEVNULL, stdout=output, stderr=subprocess.STDOUT)
        try:
            deadline = time.monotonic() + 8
            while time.monotonic() < deadline:
                if process.poll() is not None:
                    raise AssertionError(log.read_text(errors='replace'))
                try:
                    if request(base, '/health/live')[0] == 200:
                        break
                except OSError:
                    pass
                time.sleep(0.05)
            else:
                raise AssertionError('HTTP did not start: ' + log.read_text(errors='replace'))
            yield process
        finally:
            if process.poll() is None:
                process.terminate()
                try:
                    process.wait(timeout=3)
                except subprocess.TimeoutExpired:
                    process.kill()
                    process.wait(timeout=3)


class OutboxRecovery(unittest.TestCase):
    def test_full_outbox_defers_source_events_and_retries_after_broker_recovery(self):
        events = [{
            'schema': 'netcore-event-v1', 'event_id': str(uuid.uuid4()),
            'event_type': 'subscriber.route_changed',
            'source': {'service': 'mobility-core', 'instance': 'test'},
            'timestamp': '2026-10-13T23:18:47Z', 'severity': 'info',
            'subject': {'type': 'subscriber', 'id': f'2600000{index}'},
            'payload': {'index': index},
        } for index in range(3)]

        class Source(BaseHTTPRequestHandler):
            def do_GET(self):
                raw = json.dumps(list(reversed(events))).encode()
                self.send_response(200)
                self.send_header('Content-Type', 'application/json')
                self.send_header('Content-Length', str(len(raw)))
                self.end_headers()
                self.wfile.write(raw)

            def log_message(self, *args):
                pass

        source = ThreadingHTTPServer(('127.0.0.1', 0), Source)
        source_thread = threading.Thread(target=lambda: source.serve_forever(poll_interval=0.05), daemon=True)
        source_thread.start()
        broker = Broker()
        try:
            with tempfile.TemporaryDirectory(prefix='netcore-iot-backpressure-') as tmp:
                directory = Path(tmp)
                outbox = directory / 'state/outbox'
                outbox.mkdir(parents=True)
                for index in range(100):
                    message = {
                        'id': str(uuid.uuid4()), 'created_at': '2026-10-13T22:00:00Z',
                        'kind': 'event', 'topic': f'netcore/v1/test/backlog/{index}',
                        'qos': 1, 'retain': False, 'payload': '{}',
                        'source_event_id': None,
                    }
                    (outbox / f'{index:020}.json').write_text(json.dumps(message), encoding='utf-8')
                with socket.socket() as reservation:
                    reservation.bind(('127.0.0.1', 0))
                    http_port = reservation.getsockname()[1]
                base = f'http://127.0.0.1:{http_port}'
                config = directory / 'gateway.toml'
                config.write_text(
                    '[[sources]]\nid = "test-source"\nurl = '
                    + json.dumps(f'http://127.0.0.1:{source.server_port}/api/v1/events')
                    + '\nenabled = true\n[server]\n' + f'bind = "127.0.0.1:{http_port}"\n'
                    + '[mqtt]\nhost = "127.0.0.1"\n' + f'port = {broker.port}\n'
                    + 'reconnect_secs = 1\npublish_timeout_secs = 2\n'
                    + '[polling]\ninterval_ms = 250\n'
                    + '[home_assistant]\nenabled = false\n'
                    + '[storage]\nstate_dir = ' + json.dumps(str(directory / 'state'))
                    + '\noutbox_limit = 100\n', encoding='utf-8')
                with gateway(config, base, directory / 'gateway.log'):
                    deadline = time.monotonic() + 3
                    while time.monotonic() < deadline:
                        status = json.loads(request(base, '/api/v1/status')[1])
                        if status['events_deferred'] >= 3:
                            break
                        time.sleep(0.05)
                    self.assertGreaterEqual(status['events_deferred'], 3)
                    self.assertEqual(status['outbox_pending'], 100)
                    self.assertEqual(status['sources_healthy'], 1)
                    self.assertEqual(status['invalid_events'], 0)
                    self.assertEqual(status['events_enqueued'], 0)
                    source_status = json.loads(request(base, '/api/v1/sources')[1])[0]
                    self.assertEqual(source_status['events_seen'], source_status['events_deferred'])
                    broker.start()
                    deadline = time.monotonic() + 10
                    while time.monotonic() < deadline:
                        status = json.loads(request(base, '/api/v1/status')[1])
                        if status['events_enqueued'] == 3 and status['broker_publish_acks'] >= 109:
                            break
                        time.sleep(0.05)
                    self.assertEqual(status['events_enqueued'], 3)
                    self.assertGreaterEqual(status['broker_publish_acks'], 109)
                    self.assertEqual(status['invalid_events'], 0)
                    published = [json.loads(payload) for topic, payload in broker.published
                                 if topic == 'netcore/v1/events/subscriber/route_changed']
                    self.assertEqual([event['event_id'] for event in published],
                                     [event['event_id'] for event in events])
                    self.assertIn('netcore_iot_gateway_events_deferred ',
                                  request(base, '/metrics')[1].decode())
        finally:
            broker.close()
            source.shutdown()
            source.server_close()
            source_thread.join(timeout=2)

    def test_http_survives_restart_and_mqtt_replays_full_or_over_limit_backlog(self):
        for pending in (100, 105):
            with self.subTest(pending=pending), tempfile.TemporaryDirectory(prefix='netcore-iot-outbox-') as tmp:
                directory = Path(tmp)
                outbox = directory / 'state/outbox'
                outbox.mkdir(parents=True)
                original = {}
                for index in range(pending):
                    message = {
                        'id': str(uuid.uuid4()), 'created_at': '2026-10-13T22:00:00Z',
                        'kind': 'event', 'topic': f'netcore/v1/test/backlog/{index}',
                        'qos': 1, 'retain': False, 'payload': json.dumps({'index': index}),
                        'source_event_id': None,
                    }
                    path = outbox / f'{index:020}.json'
                    path.write_text(json.dumps(message), encoding='utf-8')
                    original[path] = path.read_bytes()
                broker = Broker()
                try:
                    with socket.socket() as reservation:
                        reservation.bind(('127.0.0.1', 0))
                        http_port = reservation.getsockname()[1]
                    base = f'http://127.0.0.1:{http_port}'
                    config = directory / 'gateway.toml'
                    config.write_text(
                        'sources = []\n[server]\n' + f'bind = "127.0.0.1:{http_port}"\n'
                        + '[mqtt]\nhost = "127.0.0.1"\n' + f'port = {broker.port}\n'
                        + 'reconnect_secs = 1\npublish_timeout_secs = 2\n'
                        + '[home_assistant]\nenabled = false\n'
                        + '[storage]\nstate_dir = ' + json.dumps(str(directory / 'state'))
                        + '\noutbox_limit = 100\n', encoding='utf-8')
                    # The first process leaves all persisted messages intact while MQTT is offline.
                    for attempt in range(2):
                        with gateway(config, base, directory / f'gateway-{attempt}.log') as process:
                            self.assertEqual(request(base, '/')[0], 200)
                            status = json.loads(request(base, '/api/v1/status')[1])
                            self.assertEqual(status['outbox_pending'], pending)
                            self.assertFalse(status['mqtt_connected'])
                            self.assertEqual(request(base, '/health/ready')[0], 503)
                            code, _ = request(base, '/api/v1/test/publish', {'payload': {'test': True}})
                            self.assertEqual(code, 409)
                            self.assertEqual({p: p.read_bytes() for p in original}, original)
                            if attempt == 0:
                                continue
                            broker.start()
                            replay_started = time.monotonic()
                            # A socket-polling sender needs about 20 seconds for
                            # this backlog; incoming/outgoing work must wake it.
                            deadline = time.monotonic() + 10
                            while time.monotonic() < deadline:
                                self.assertIsNone(process.poll())
                                status = json.loads(request(base, '/api/v1/status')[1])
                                if status['mqtt_connected'] and status['outbox_pending'] == 0:
                                    break
                                time.sleep(0.05)
                            else:
                                self.fail(f'backlog did not drain: {status}')
                            self.assertEqual(status['broker_publish_acks'], pending + 3)
                            print(f'MQTT replay: {pending} backlog messages + 3 startup states '
                                  f'in {time.monotonic() - replay_started:.3f}s', flush=True)
                            self.assertFalse(list(outbox.glob('*.json')))
                            topics = [topic for topic, _ in broker.published]
                            for index in range(pending):
                                self.assertEqual(topics.count(f'netcore/v1/test/backlog/{index}'), 1)
                            self.assertEqual(request(base, '/')[0], 200)
                            self.assertEqual(request(base, '/api/v1/test/publish', {'payload': {'test': True}})[0], 202)
                finally:
                    broker.close()


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--binary', type=Path, default=BINARY)
    args = parser.parse_args()
    BINARY = args.binary.resolve()
    if not BINARY.is_file():
        parser.error(f'build the gateway first: {BINARY}')
    unittest.main(argv=[__file__], verbosity=2)
