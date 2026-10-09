import importlib.util
import base64
import json
from pathlib import Path
import sys
import tempfile
import threading
import time
import unittest
from unittest.mock import patch
from urllib.request import Request, urlopen

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
from common import ROOT, atomic_write, load_config, request_json
from image_client import ImageClient
from image_spec import BASE, public_job, validate_image, validate_vpn, wifi_keyfile
from image_worker import Worker, Server as WorkerServer, handler as worker_handler, byte_range
from main import App, Server, handler

spec = importlib.util.spec_from_file_location('vpn_policy', ROOT / 'image/vpn-policy.py')
vpn_policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(vpn_policy)


def image_request(**changes):
    key = base64.b64encode(b'\0\0\0\x0bssh-ed25519\0\0\0\x20' + b'\x01' * 32).decode()
    return dict(profile=dict(name='TBS-02', mcc=901, mnc=1510, issi=4010002, la=2, cc=2),
                commit='a' * 40, config='[net_info]\nmcc=901\n[cell_info]\ncolour_code=2\n[phy_io]\nbackend="SoapySdr"\n',
                controller_url='http://10.0.1.50:8320', ssh_key='ssh-ed25519 ' + key + ' fixture', **changes)


def simulated_build(worker, req, log):
    """Transport fixture only: these bytes are deliberately NOT an OS image."""
    key = req['build_id']
    directory = worker.state / 'artifacts' / key
    directory.mkdir()
    (directory / 'image.img.xz').write_bytes(b'fixture image transport only')
    manifest = dict(id=key, filename='fixture.img.xz', profile=req['profile'], commit=req['commit'],
                    created=time.time(), size_bytes=28, uncompressed_bytes=1024, boot_tested=False)
    atomic_write(directory / 'manifest.json', json.dumps(manifest))
    atomic_write(directory / 'image.sha256', 'fixture checksum\n')
    log('Browser/API integration: simulated image builder; no OS build performed')
    return manifest


class ImageTests(unittest.TestCase):
    def test_image_request_pins_recipe_hashes_password_and_redacts_secrets(self):
        request = image_request(password='secret-os-password', wifi_ssid='Field\\AP; [new]', wifi_password='wlan-secret')
        req = validate_image(request)
        self.assertEqual(req['hostname'], 'tbs-02')
        self.assertEqual(req['base'], BASE['id'])
        self.assertNotIn('password', req)
        self.assertTrue(req['password_hash'].startswith('$6$'))
        public = public_job(dict(request=req, result={}, log='', status='queued'))
        serialized = json.dumps(public)
        self.assertNotIn('secret', serialized)
        self.assertNotIn('config', serialized)
        self.assertNotIn('ssh_key', serialized)
        self.assertEqual(req['config'], request['config'])
        keyfile = wifi_keyfile(request['wifi_ssid'], request['wifi_password'])
        self.assertIn('ssid=Field\\\\AP;\\s[new]', keyfile)

    def test_rejects_paths_shell_fragments_and_ambiguous_requests(self):
        bad = [dict(hostname='../../etc'), dict(username='root'), dict(username='a;id'),
               dict(base='http://elsewhere/image'), dict(commit='main'), dict(timezone='../etc/passwd'),
               dict(controller_url='http://127.0.0.1:8320'), dict(controller_url='http://public.example:8320'),
               dict(wifi_ssid='name\n[connection]'), dict(wifi_ssid='ssid', wifi_password='short'),
               dict(trusted_ssids='Home'), dict(trusted_lan='invalid'), dict(overlay='$(id)'),
               dict(ssh_key='command="id" ssh-ed25519 AAAA'), dict(ssh_key='ssh-ed25519 AAAA'), dict(ssh_key=''),
               dict(password='x\nroot:pw'), dict(password_hash='$6$bad')]
        for change in bad:
            request = image_request(); request.update(change)
            with self.subTest(change=change), self.assertRaises((ValueError, TypeError)):
                validate_image(request)

    def test_portable_vpn_and_home_policy_does_not_match_vpn_subnet(self):
        valid = 'client\ndev tun\nremote vpn.example 1194\n<ca>\nCERTIFICATE\n</ca>\n'
        self.assertEqual(validate_vpn(valid), valid)
        for tail in ['up /tmp/script', 'config /etc/passwd', 'ca /etc/ca.pem', 'plugin foo', 'dev tap', '<ca>\nnot closed']:
            with self.subTest(tail=tail), self.assertRaises(ValueError):
                validate_vpn(valid + tail)
        self.assertTrue(vpn_policy.trusted([dict(type='wifi', ssid='Home')], ['Home'], '10.0.1.0/24'))
        self.assertTrue(vpn_policy.trusted([dict(type='ethernet', addresses=['10.0.1.20/24'])], [], '10.0.1.0/24'))
        self.assertFalse(vpn_policy.trusted([dict(type='tun', addresses=['10.0.1.30/24'])], [], '10.0.1.0/24'))
        self.assertFalse(vpn_policy.trusted([dict(type='wifi', ssid='Field', addresses=['10.0.1.20/24'])], ['Home'], '10.0.1.0/24'))

    def test_resume_ranges(self):
        self.assertEqual(byte_range(None, 100), (0, 99, 200))
        self.assertEqual(byte_range('bytes=20-29', 100), (20, 29, 206))
        self.assertEqual(byte_range('bytes=20-', 100), (20, 99, 206))
        self.assertEqual(byte_range('bytes=-10', 100), (90, 99, 206))
        for bad in ('bytes=100-', 'bytes=-0', 'bytes=10-5', 'bytes=0-2,4-6', 'bytes=-'):
            with self.assertRaises(ValueError):
                byte_range(bad, 100)

    def test_unavailable_worker_keeps_controller_operational(self):
        with tempfile.TemporaryDirectory() as temp:
            state = ImageClient(temp + '/missing.sock').status()
            self.assertFalse(state['available'])
            self.assertIn('install-vm.sh', state['error'])

    def test_real_unix_worker_api_and_controller_download(self):
        with tempfile.TemporaryDirectory() as temp:
            worker = Worker(Path(temp) / 'builder')
            worker.jobs.execute = lambda req, log: simulated_build(worker, req, log)
            socket = temp + '/worker.sock'
            try:
                server = WorkerServer(socket, worker_handler(worker))
            except PermissionError:
                worker.lock_file.close()
                self.skipTest('Unix sockets are unavailable in this execution environment; exercised in Ubuntu CI')
            threading.Thread(target=server.serve_forever, daemon=True).start()
            cfg = load_config(ROOT / 'config/deployment.example.toml')
            cfg.update(state_dir=temp + '/controller', image_builder_socket=socket)
            app = App(cfg)
            web = Server(('127.0.0.1', 0), handler(app))
            threading.Thread(target=web.serve_forever, daemon=True).start()
            url = 'http://127.0.0.1:' + str(web.server_address[1])
            try:
                req = image_request(password='do-not-publish')
                app.profiles['TBS-02'] = req['profile']
                atomic_write(app.state / 'tbs-site-template.toml', req['config'])
                app.repo.resolve_latest_main = lambda log: 'b' * 40
                payload = {**req, 'profile': 'TBS-02', 'commit': 'c' * 40}
                job = request_json(url + '/api/v1/images/build', payload)
                self.assertNotIn('do-not-publish', json.dumps(job))
                deadline = time.monotonic() + 3
                while worker.jobs.get(job['id'])['status'] in ('queued', 'running'):
                    if time.monotonic() > deadline:
                        self.fail('Build fixture timed out')
                    time.sleep(.02)
                status = request_json(url + '/api/v1/images')
                self.assertEqual(status['jobs'][0]['request']['commit'], 'b' * 40)
                self.assertNotIn('password_hash', json.dumps(status))
                key = status['artifacts'][0]['id']
                response = urlopen(url + '/api/v1/images/' + key + '/image')
                self.assertEqual(response.read(), b'fixture image transport only')
                self.assertIn('fixture.img.xz', response.headers['Content-Disposition'])
                response = urlopen(Request(url + '/api/v1/images/' + key + '/image', headers={'Range': 'bytes=8-12'}))
                self.assertEqual(response.status, 206)
                self.assertEqual(response.read(), b'image')
                self.assertTrue(request_json(url + '/health/live')['ok'])
                with self.assertRaises((ValueError, KeyError)):
                    worker.remove('../../cache')
                request_json(url + '/api/v1/images/remove', dict(id=key))
                self.assertEqual(worker.artifacts(), [])
                self.assertEqual(worker.jobs.get(job['id'])['status'], 'succeeded')
            finally:
                app.stop.set(); app.discovery.close()
                web.shutdown(); web.server_close()
                server.shutdown(); server.server_close()
                worker.lock_file.close()


if __name__ == '__main__':
    unittest.main()
