#!/usr/bin/env python3
"""Exercise the real Media Library HTTP server and a basis-station fixture."""
import base64
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import io
import json
import os
from pathlib import Path
import socket
import struct
import subprocess
import tempfile
import threading
import time
import unittest
from urllib.error import HTTPError
from urllib.request import ProxyHandler, Request, build_opener
import wave

ROOT = Path(__file__).resolve().parents[1]
OPENER = build_opener(ProxyHandler({}))


class Station(BaseHTTPRequestHandler):
    def log_message(self, *_):
        pass

    def response(self, data):
        body = json.dumps(data).encode()
        self.send_response(200)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(body)))
        self.end_headers()
        self.wfile.write(body)

    def do_GET(self):
        self.response({"available": True, "state": "idle", "job_id": None,
                       "sent_blocks": 1, "total_blocks": 1})

    def do_POST(self):
        request = json.loads(self.rfile.read(int(self.headers["Content-Length"])))
        with OPENER.open(self.server.library_url + "/api/v1/assets/" +
                         request["path"] + "/preview", timeout=10) as response:
            self.server.received.append(response.read())
        self.response({"job_id": "basisstation-fixture-job"})


class NativeHttpArchive(unittest.TestCase):
    def test_real_http_offload_recall_outage_and_station_playout(self):
        with tempfile.TemporaryDirectory(prefix="netcore-media-http-") as temporary:
            root = Path(temporary)
            nas = root / "nas"
            for name in ("Media-Library", "Recordings", "TTS-Dateien"):
                (nas / name).mkdir(parents=True)
            station = ThreadingHTTPServer(("127.0.0.1", 0), Station)
            station.received = []
            thread = threading.Thread(target=station.serve_forever, daemon=True)
            thread.start()
            probe = socket.socket()
            probe.bind(("127.0.0.1", 0))
            port = probe.getsockname()[1]
            probe.close()
            base = "http://127.0.0.1:" + str(port)
            station.library_url = base
            config = root / "media.toml"
            q = lambda value: json.dumps(str(value))
            config.write_text(
                "[server]\nbind=" + q("127.0.0.1:" + str(port)) +
                "\npublic_base_url=" + q(base) +
                "\n[storage]\nroot=" + q(root / "assets") +
                "\nstate_file=" + q(root / "state.json") +
                "\ntemp_root=" + q(root / "tmp") +
                "\nbackup_root=" + q(root / "backups") +
                "\narchive_root=" + q(nas / "Media-Library") +
                "\nrecording_archive_root=" + q(nas / "Recordings") +
                "\ntts_archive_root=" + q(nas / "TTS-Dateien") +
                "\narchive_require_network_mount=false\nlocal_cache_max_bytes=0\nmin_free_bytes=0" +
                "\n[runtime]\noperating_mode='authoritative'\nworker_interval_ms=100" +
                "\n[tts]\nenabled=false\ntemplate_directory=" + q(root / "templates") +
                "\n[dependencies]\nmedia_switch_base_url='http://127.0.0.1:1'" +
                "\nrecorder_base_url='http://127.0.0.1:1'\napplication_gateway_base_url='http://127.0.0.1:1'" +
                "\n[playout]\nmode='basisstation'\ndefault_station='fixture'\npoll_interval_ms=200" +
                "\n[[playout.stations]]\nid='fixture'\nname='Fixture'\nenabled=true\nbase_url=" +
                q("http://127.0.0.1:" + str(station.server_port)) + "\n")
            with (root / "process.log").open("wb") as output:
                process = subprocess.Popen(
                    [str(ROOT / "target/debug/netcore-media-library"), "--config", str(config)],
                    stdout=output, stderr=subprocess.STDOUT,
                    env=dict(os.environ, RUST_LOG="warn"))
                def request(path, data=None, raw=False):
                    body = None if data is None else json.dumps(data).encode()
                    message = Request(base + path, data=body,
                                      headers={"Content-Type": "application/json"})
                    with OPENER.open(message, timeout=15) as response:
                        payload = response.read()
                    return payload if raw else json.loads(payload)
                def wait_for(callback, label):
                    deadline = time.monotonic() + 20
                    error = None
                    while time.monotonic() < deadline:
                        try:
                            value = callback()
                            if value:
                                return value
                        except Exception as exc:
                            error = exc
                        if process.poll() is not None:
                            raise AssertionError("Media Library exited: " +
                                                 (root / "process.log").read_text()[-4000:])
                        time.sleep(0.1)
                    raise AssertionError(label + ": " + str(error))
                try:
                    wait_for(lambda: request("/health/live"), "HTTP startup")
                    buffer = io.BytesIO()
                    with wave.open(buffer, "wb") as audio:
                        audio.setnchannels(1)
                        audio.setsampwidth(2)
                        audio.setframerate(8000)
                        audio.writeframes(struct.pack("<h", 1000) * 800)
                    wav = buffer.getvalue()
                    asset = request("/api/v1/assets/upload-json", {
                        "name": "Native archive", "filename": "fixture.wav",
                        "kind": "recording", "media_type": "audio/wav", "approve": True,
                        "data_base64": base64.b64encode(wav).decode()})
                    asset_url = "/api/v1/assets/" + asset["asset_id"]
                    wait_for(lambda: request(asset_url).get("state") == "ready", "processing")
                    request(asset_url + "/archive", {})
                    maintenance = request("/api/v1/maintenance/tick", {})
                    self.assertEqual(maintenance["cache"]["offloaded"], 1)
                    self.assertTrue(request(asset_url)["local_offloaded"])
                    self.assertEqual(request("/api/v1/status")["assets_offloaded"], 1)

                    (nas / "Recordings").rename(nas / "Recordings-offline")
                    with self.assertRaises(HTTPError) as unavailable:
                        request(asset_url + "/preview", raw=True)
                    self.assertEqual(unavailable.exception.code, 503)
                    self.assertTrue(request("/api/v1/status")["ready"])
                    (nas / "Recordings-offline").rename(nas / "Recordings")
                    self.assertEqual(request(asset_url + "/preview", raw=True), wav)
                    self.assertTrue(request(asset_url + "/waveform?points=32")["points"])
                    self.assertFalse(request(asset_url)["local_offloaded"])

                    request("/api/v1/maintenance/tick", {})
                    job = request("/api/v1/dispatch", {
                        "asset_id": asset["asset_id"], "destination_kind": "group",
                        "destination_id": 1234, "station_id": "fixture"})
                    wait_for(lambda: request("/api/v1/jobs/" + job["job_id"])["state"] == "completed",
                             "basis-station playout completion")
                    self.assertEqual(station.received, [wav])
                    self.assertEqual(request("/api/v1/jobs/" + job["job_id"])["state"], "completed")
                    print("Native HTTP: archive, offload, 503 outage, recall, waveform and station WAV delivery passed")
                finally:
                    process.terminate()
                    try:
                        process.wait(timeout=10)
                    except subprocess.TimeoutExpired:
                        process.kill()
                        process.wait(timeout=10)
                    station.shutdown()
                    station.server_close()
                    thread.join(timeout=5)



class RuntimeUpdateSource(unittest.TestCase):
    def test_operator_sparse_source_builds_media_and_loads_updater(self):
        import sys
        commit = subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT,
                                         text=True).strip()
        old = subprocess.run(["git", "config", "--get", "uploadpack.allowFilter"], cwd=ROOT,
                             capture_output=True, text=True)
        try:
            subprocess.run(["git", "config", "uploadpack.allowFilter", "true"], cwd=ROOT, check=True)
            with tempfile.TemporaryDirectory(prefix="netcore-media-source-") as temporary:
                source = Path(temporary)
                def run(*arguments, **kwargs):
                    completed = subprocess.run(arguments, cwd=source, text=True,
                                               stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                               timeout=180, **kwargs)
                    if completed.returncode:
                        raise AssertionError(completed.stderr[-5000:])
                    return completed.stdout
                run("git", "init", "-q")
                run("git", "remote", "add", "origin", ROOT.as_uri())
                run("git", "config", "http.version", "HTTP/1.1")
                run("git", "config", "remote.origin.promisor", "true")
                run("git", "config", "remote.origin.partialclonefilter", "blob:none")
                run("git", "fetch", "--depth", "1", "--filter=blob:none", "origin", commit)
                run("git", "sparse-checkout", "init", "--no-cone")
                run("git", "sparse-checkout", "set", "--no-cone", "--stdin",
                    input="/*\n!/Docs/\n!/wiki/\n")
                run("git", "checkout", "--detach", commit)
                self.assertEqual(run("git", "rev-parse", "HEAD").strip(), commit)
                self.assertFalse((source / "Docs").exists())
                self.assertTrue((source / "system-backend/media-library/src/archive.rs").is_file())
                self.assertTrue((source / "system-backend/media-library/src/state/cache_tests.rs").is_file())
                run(sys.executable, "-B", str(source / "tools/operator/update-netcore-service.py"),
                    "--help")
                run("cargo", "build", "--locked", "-p", "netcore-media-library",
                    "--target-dir", str(ROOT / "target"))
                print("Operator source: exact commit, Docs excluded, updater and media build passed")
        finally:
            if old.returncode == 0:
                subprocess.run(["git", "config", "uploadpack.allowFilter", old.stdout.strip()], cwd=ROOT, check=True)
            else:
                subprocess.run(["git", "config", "--unset", "uploadpack.allowFilter"], cwd=ROOT, check=True)
if __name__ == "__main__":
    unittest.main()
