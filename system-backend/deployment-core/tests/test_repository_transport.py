"""Real smart HTTP Git transport, truncated packs and immutable runtime caches."""
from http.server import BaseHTTPRequestHandler, ThreadingHTTPServer
import os
from pathlib import Path
import socket
import subprocess
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import deploy


def git(path, *args):
    return subprocess.check_output(['git', '-C', str(path), *args], text=True).strip()


class GitHTTP:
    def __init__(self, directory):
        self.directory = directory
        self.posts = []
        self.drop = 0
        self.drop_blob = 0
        self.denied = False
        fixture = self

        class Handler(BaseHTTPRequestHandler):
            protocol_version = 'HTTP/1.1'

            def do_GET(self):
                self.backend()

            def do_POST(self):
                self.backend()

            def backend(self):
                path, _, query = self.path.partition('?')
                body = self.rfile.read(int(self.headers.get('Content-Length', 0)))
                if fixture.denied:
                    self.send_error(403)
                    return
                env = dict(os.environ, GIT_PROJECT_ROOT=str(fixture.directory), GIT_HTTP_EXPORT_ALL='1',
                           PATH_INFO=path, QUERY_STRING=query, REQUEST_METHOD=self.command,
                           CONTENT_TYPE=self.headers.get('Content-Type', ''), CONTENT_LENGTH=str(len(body)),
                           GIT_PROTOCOL=self.headers.get('Git-Protocol', ''), REMOTE_ADDR='127.0.0.1')
                output = subprocess.run(['git', 'http-backend'], env=env, input=body,
                                        capture_output=True, check=True, timeout=30).stdout
                delimiter = b'\r\n\r\n' if b'\r\n\r\n' in output else b'\n\n'
                headers, payload = output.split(delimiter, 1)
                fields = dict(line.split(':', 1) for line in headers.decode().splitlines() if ':' in line)
                if self.command == 'POST':
                    fixture.posts.append(body)
                drop = False
                if self.command == 'POST' and b'want ' in body:
                    if fixture.drop:
                        fixture.drop -= 1
                        drop = True
                    elif fixture.drop_blob and b'filter blob:none' not in body:
                        fixture.drop_blob -= 1
                        drop = True
                self.send_response(200)
                for name, value in fields.items():
                    if name.lower() not in ('status', 'content-length', 'connection'):
                        self.send_header(name, value.strip())
                self.send_header('Content-Length', str(len(payload)))
                self.send_header('Connection', 'close')
                self.end_headers()
                if drop:
                    self.wfile.write(payload[:max(1, len(payload) // 2)])
                    self.wfile.flush()
                    self.connection.shutdown(socket.SHUT_RDWR)
                    self.connection.close()
                else:
                    self.wfile.write(payload)

            def log_message(self, *args):
                pass

        self.server = ThreadingHTTPServer(('127.0.0.1', 0), Handler)
        self.thread = threading.Thread(target=self.server.serve_forever, daemon=True)
        self.thread.start()
        self.url = f'http://127.0.0.1:{self.server.server_port}/origin.git'

    def close(self):
        self.server.shutdown()
        self.server.server_close()
        self.thread.join(3)


class TransportTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        working = self.root / 'working'
        subprocess.run(['git', 'init', '-b', 'main', str(working)], check=True, capture_output=True)
        for name, body in {'Cargo.toml': b'workspace', 'system-backend/test/runtime.py': b'first runtime\n',
                           'Docs/large.pdf': os.urandom(1024 * 1024), 'wiki/large.dat': os.urandom(32768)}.items():
            path = working / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_bytes(body)
        git(working, 'add', '.')
        git(working, '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-m', 'first')
        self.sha = git(working, 'rev-parse', 'HEAD')
        self.doc_blob = git(working, 'rev-parse', 'HEAD:Docs/large.pdf')
        (working / 'system-backend/test/runtime.py').write_text('new main\n')
        git(working, 'add', '.')
        git(working, '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid', 'commit', '-m', 'second')
        self.new_sha = git(working, 'rev-parse', 'HEAD')
        git(working, 'tag', 'unrelated-tag')
        self.origin = self.root / 'origin.git'
        cloned = subprocess.run(['git', 'clone', '--bare', str(working), str(self.origin)], capture_output=True)
        self.assertEqual(cloned.returncode, 0, cloned.stderr.decode(errors='replace'))
        git(self.origin, 'config', 'uploadpack.allowFilter', 'true')
        git(self.origin, 'config', 'uploadpack.allowAnySHA1InWant', 'true')
        self.http = GitHTTP(self.root)
        self.addCleanup(self.http.close)
        self.state = self.root / 'state'
        self.repo = deploy.Repository(dict(repository=self.http.url, state_dir=str(self.state)))
        self.logs = []
        # Preserve real subprocess draining intervals; only backoff sleeps are omitted.
        original_sleep = deploy.time.sleep
        self.pause = patch.object(deploy.time, 'sleep', side_effect=lambda seconds: original_sleep(seconds) if seconds < 1 else None)
        self.pause.start()
        self.addCleanup(self.pause.stop)

    def checkout(self):
        return self.repo.checkout(self.sha, self.logs.append)

    def test_exact_sha_shallow_http11_and_document_blobs_not_downloaded(self):
        source = self.checkout()
        self.assertEqual(git(source, 'rev-parse', 'HEAD'), self.sha)
        self.assertEqual(git(source, 'rev-list', '--count', 'HEAD'), '1')
        self.assertEqual((source / 'system-backend/test/runtime.py').read_text(), 'first runtime\n')
        self.assertFalse((source / 'Docs').exists())
        self.assertFalse((source / 'wiki').exists())
        missing = git(source, 'rev-list', '--objects', '--missing=print', '--all')
        self.assertIn('?' + self.doc_blob, missing)
        self.assertNotIn(self.doc_blob.encode(), b''.join(self.http.posts))
        self.assertEqual(git(source, 'config', 'http.version'), 'HTTP/1.1')
        self.assertEqual(git(source, 'config', 'pack.threads'), '1')
        self.assertEqual(git(source, 'tag'), '')

    def test_truncated_commit_pack_retries_source_without_a_second_install_job(self):
        self.http.drop = 1
        source = self.checkout()
        self.assertEqual(git(source, 'rev-parse', 'HEAD'), self.sha)
        self.assertEqual(sum('Git-Transport unterbrochen' in line for line in self.logs), 1)
        self.assertTrue(all(b'git-receive-pack' not in body for body in self.http.posts))

    def test_truncated_lazy_blob_download_is_retried_in_a_fresh_private_tree(self):
        self.http.drop_blob = 1
        source = self.checkout()
        self.assertEqual((source / 'system-backend/test/runtime.py').read_text(), 'first runtime\n')
        self.assertEqual(sum('Git-Transport unterbrochen' in line for line in self.logs), 1)
        self.assertEqual(list(self.state.glob('source-new-*')), [])

    def test_exhausted_transport_keeps_previous_cache_selector_and_deployment_receipt(self):
        source = self.checkout()
        selector = (self.state / 'source-location.json').read_bytes()
        marker = self.state / 'deployed-test.json'
        marker.write_text('{"commit":"previous"}')
        self.http.drop = 10
        self.logs.clear()
        with self.assertRaises(deploy.CommandFailed):
            self.repo.checkout(self.new_sha, self.logs.append)
        self.assertEqual(sum(line.startswith('Runtime-Quellabruf ') for line in self.logs), 3)
        self.assertEqual(self.repo.path, source)
        self.assertEqual((self.state / 'source-location.json').read_bytes(), selector)
        self.assertEqual(marker.read_text(), '{"commit":"previous"}')
        self.assertEqual((source / 'system-backend/test/runtime.py').read_text(), 'first runtime\n')
        self.assertEqual(list(self.state.glob('source-new-*')), [])

    def test_authentication_error_is_not_retried(self):
        self.http.denied = True
        with self.assertRaises(deploy.CommandFailed):
            self.checkout()
        self.assertEqual(sum(line.startswith('Runtime-Quellabruf ') for line in self.logs), 1)
        self.assertFalse((self.state / 'source-location.json').exists())

    def test_same_pinned_cache_survives_restart_and_offline_origin(self):
        source = self.checkout()
        previous_requests = len(self.http.posts)
        self.http.denied = True
        restarted = deploy.Repository(dict(repository=self.http.url, state_dir=str(self.state)))
        self.assertEqual(restarted.checkout(self.sha, self.logs.append), source)
        self.assertEqual(len(self.http.posts), previous_requests)

    def test_dirty_cache_is_preserved_and_rejected_before_another_fetch(self):
        source = self.checkout()
        (source / 'system-backend/test/runtime.py').write_text('operator edit')
        previous_requests = len(self.http.posts)
        with self.assertRaisesRegex(RuntimeError, 'local changes'):
            self.repo.checkout(self.new_sha, self.logs.append)
        self.assertEqual(len(self.http.posts), previous_requests)
        self.assertEqual((source / 'system-backend/test/runtime.py').read_text(), 'operator edit')


class FailureClassificationTests(unittest.TestCase):
    def test_known_transient_git_errors_and_timeouts(self):
        for text in ('error: RPC failed; curl 56 Recv failure: Connection reset by peer\nfatal: early EOF',
                     'fetch-pack: unexpected disconnect while reading sideband packet',
                     'RPC failed; curl 92 HTTP/2 stream was not closed cleanly'):
            self.assertTrue(deploy.transient_git_failure(deploy.CommandFailed(['git', 'fetch'], 128, text)))
        self.assertTrue(deploy.transient_git_failure(TimeoutError()))

    def test_permanent_errors_override_transport_words_and_installers_never_retry(self):
        for text in ('Permission denied', 'No space left on device', 'Authentication failed',
                     'index.lock exists', 'SSL certificate problem', 'not our ref'):
            self.assertFalse(deploy.transient_git_failure(deploy.CommandFailed(
                ['git', 'fetch'], 128, 'fatal: early EOF\n' + text)))
        self.assertFalse(deploy.transient_git_failure(deploy.CommandFailed(['bash', 'install.sh'], 128, 'early EOF')))
        self.assertFalse(deploy.transient_git_failure(RuntimeError('early EOF')))


if __name__ == '__main__':
    unittest.main()
