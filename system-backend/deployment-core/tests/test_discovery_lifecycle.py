"""Durable inventory removal, restore, endpoint cleanup and fetch-race coverage."""
from copy import deepcopy
import json
import os
from pathlib import Path
import sys
import tempfile
import threading
import unittest
from unittest.mock import patch

sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
import discovery as module
from common import PROTOCOL
from discovery import Discovery, DiscoveryPersistenceUncertain


def manifest(node='gateway-01', *, role='agent', services=None):
    return dict(protocol=PROTOCOL, environment='fixture', security_mode='open_lab',
                node_id=node, role=role, services=services if services is not None else [
                    dict(name='node-gateway', port=8080, ready=True, available=True, commit='a' * 40)])


def config(state, **changes):
    cfg = dict(state_dir=state, node_id='controller-01', environment='fixture', port=8320,
               seeds=[], bindings={}, lease_seconds=90, interval=15,
               allowed_networks=['10.0.0.0/8', '127.0.0.0/8'], discovery_port=48320,
               multicast_group='239.192.84.82', interface='0.0.0.0')
    cfg.update(changes)
    return cfg


class DiscoveryLifecycleTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.state = Path(self.temp.name)
        if os.name == 'nt':
            # Windows lacks fchmod/directory fsync. Linux runs common.atomic_write
            # unchanged; the portable harness still performs real atomic replaces.
            def write(path, value, mode=0o600):
                path = Path(path)
                path.parent.mkdir(parents=True, exist_ok=True)
                with tempfile.NamedTemporaryFile('w', dir=path.parent, delete=False, encoding='utf-8') as out:
                    temp = Path(out.name)
                    out.write(value)
                os.chmod(temp, mode)
                os.replace(temp, path)
            fixture = patch.object(module, 'atomic_write', side_effect=write)
            fixture.start()
            self.addCleanup(fixture.stop)
        self.write = module.atomic_write

    def instance(self, **changes):
        found = Discovery(config(str(self.state), **changes), lambda: manifest('controller-01', services=[]))
        self.addCleanup(found.close)
        return found

    def test_forget_clears_peer_lastgood_endpoint_and_conflict_and_survives_restart(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        found.accept('http://10.0.1.11:8321', manifest('gateway-02'))
        self.assertEqual(found.snapshot()['conflicts']['node-gateway'], ['gateway-01', 'gateway-02'])
        result = found.forget('gateway-01')
        self.assertEqual(result['removed_endpoints'], ['node-gateway'])
        self.assertEqual(found.snapshot()['endpoints']['node-gateway']['node_id'], 'gateway-02')
        self.assertEqual(found.snapshot()['conflicts'], {})
        self.assertEqual([p['node_id'] for p in found.snapshot()['peers']], ['gateway-02'])
        self.assertFalse(found.accept('http://10.0.1.10:8321', manifest()))
        found._resolve()
        restored = self.instance()
        self.assertFalse(restored.accept('http://10.0.1.10:8321', manifest()))
        self.assertEqual([p['node_id'] for p in restored.snapshot()['forgotten']], ['gateway-01'])
        self.assertEqual(json.loads((self.state / 'peers.json').read_text()), restored.peers)
        self.assertEqual(json.loads((self.state / 'endpoints.json').read_text()), restored.endpoints)

    def test_offline_peer_removal_keeps_explicit_binding_unresolved(self):
        found = self.instance(bindings={'node-gateway': 'gateway-01'})
        found.accept('http://10.0.1.10:8321', manifest())
        found.accept('http://10.0.1.11:8321', manifest('gateway-02'))
        found.peers['gateway-01']['last_seen'] = 0
        found._resolve()
        result = found.forget('gateway-01')
        self.assertEqual(result['retained_bindings'], ['node-gateway'])
        self.assertEqual(found.cfg['bindings'], {'node-gateway': 'gateway-01'})
        self.assertNotIn('node-gateway', found.endpoints)
        found._resolve()
        self.assertNotIn('node-gateway', found.endpoints)
        self.assertEqual(found.snapshot()['forgotten'][0]['retained_bindings'], ['node-gateway'])

    def test_own_node_unknown_node_and_invalid_name_cannot_be_removed(self):
        found = self.instance()
        with self.assertRaises(ValueError):
            found.forget('controller-01')
        with self.assertRaises(KeyError):
            found.forget('missing')
        with self.assertRaises(ValueError):
            found.forget('../elsewhere')
        self.assertFalse((self.state / 'discovery-state.json').exists())

    def test_restore_requests_fresh_manifest_never_restores_old_services(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        found.forget('gateway-01')
        old_generation = found.generation
        with patch.object(found, 'schedule') as schedule:
            result = found.restore('gateway-01')
        self.assertTrue(result['rediscovery_scheduled'])
        schedule.assert_called_once_with('http://10.0.1.10:8321')
        self.assertGreater(found.generation, old_generation)
        self.assertEqual(found.snapshot()['peers'], [])
        self.assertEqual(found.snapshot()['endpoints'], {})
        self.assertEqual(found.snapshot()['forgotten'], [])
        found.accept('http://10.0.1.10:8321', manifest(services=[dict(name='tbs', port=8083, ready=True)]))
        self.assertIn('tbs', found.endpoints)
        self.assertNotIn('node-gateway', found.endpoints)

    def test_inflight_fetch_cannot_cross_forget_and_restore_boundary(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        started, release = threading.Event(), threading.Event()
        generation = found.generation
        def request(url):
            started.set()
            self.assertTrue(release.wait(3))
            return manifest()
        with patch.object(module, 'request_json', side_effect=request):
            pending = found.pool.submit(found._fetch, 'http://10.0.1.10:8321', generation)
            self.assertTrue(started.wait(2))
            found.forget('gateway-01')
            with patch.object(found, 'schedule'):
                found.restore('gateway-01')
            release.set()
            pending.result(timeout=4)
        self.assertEqual(found.snapshot()['peers'], [])
        self.assertEqual(found.snapshot()['endpoints'], {})
        self.assertTrue(found.wake.is_set())

    def test_forget_failure_before_replace_preserves_memory_and_disk(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        before = found.snapshot()
        saved = (self.state / 'discovery-state.json').read_bytes()
        def write(path, value, mode=0o600):
            if Path(path).name == 'discovery-state.json':
                raise OSError('fixture replacement blocked')
            return self.write(path, value, mode)
        with patch.object(module, 'atomic_write', side_effect=write), self.assertRaises(OSError):
            found.forget('gateway-01')
        self.assertEqual(found.snapshot(), before)
        self.assertEqual((self.state / 'discovery-state.json').read_bytes(), saved)

    def test_fsync_failure_after_replace_reconciles_committed_exclusion(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        def write(path, value, mode=0o600):
            self.write(path, value, mode)
            if Path(path).name == 'discovery-state.json':
                raise OSError('fixture directory fsync failed')
        with patch.object(module, 'atomic_write', side_effect=write):
            with self.assertRaises(DiscoveryPersistenceUncertain) as failed:
                found.forget('gateway-01')
        self.assertTrue(failed.exception.result['inventory_change_applied'])
        self.assertTrue(failed.exception.result['durability_uncertain'])
        self.assertEqual(found.snapshot()['peers'], [])
        self.assertFalse(found.accept('http://10.0.1.10:8321', manifest()))
        restarted = self.instance()
        self.assertEqual(restarted.snapshot()['peers'], [])
        self.assertEqual(restarted.snapshot()['endpoints'], {})

    def test_initial_legacy_migration_with_failed_mirror_reconciles_on_restart(self):
        peer = dict(node_id='gateway-01', agent_url='http://10.0.1.10:8321', last_seen=0,
                    role='agent', services=[])
        (self.state / 'peers.json').write_text(json.dumps({'gateway-01': peer}))
        (self.state / 'endpoints.json').write_text(json.dumps({
            'node-gateway': dict(url='http://10.0.1.10:8080', node_id='gateway-01')}))
        found = self.instance()
        def write(path, value, mode=0o600):
            if Path(path).name in ('peers.json', 'endpoints.json'):
                raise OSError('fixture mirror failed')
            return self.write(path, value, mode)
        with patch.object(module, 'atomic_write', side_effect=write):
            with self.assertRaises(DiscoveryPersistenceUncertain) as failed:
                found.forget('gateway-01')
        self.assertFalse(failed.exception.result['durability_uncertain'])
        self.assertTrue(failed.exception.result['cache_sync_pending'])
        self.assertEqual(found.snapshot()['peers'], [])
        restarted = self.instance()
        self.assertEqual(restarted.snapshot()['peers'], [])
        self.assertEqual(restarted.snapshot()['endpoints'], {})
        self.assertEqual(json.loads((self.state / 'peers.json').read_text()), {})
        self.assertEqual(json.loads((self.state / 'endpoints.json').read_text()), {})

    def test_unreadable_canonical_inventory_cannot_fall_back_to_legacy_peer_cache(self):
        (self.state / 'peers.json').write_text(json.dumps({'gateway-01': {}}))
        (self.state / 'discovery-state.json').write_text('{broken')
        with self.assertRaises(ValueError):
            self.instance()

    def test_applied_restore_with_failed_mirror_still_requests_fresh_discovery(self):
        found = self.instance()
        found.accept('http://10.0.1.10:8321', manifest())
        found.forget('gateway-01')
        def write(path, value, mode=0o600):
            if Path(path).name == 'endpoints.json':
                raise OSError('fixture mirror failed')
            return self.write(path, value, mode)
        with patch.object(module, 'atomic_write', side_effect=write), patch.object(found, 'schedule') as schedule:
            with self.assertRaises(DiscoveryPersistenceUncertain) as failed:
                found.restore('gateway-01')
        self.assertTrue(failed.exception.result['inventory_change_applied'])
        self.assertEqual(found.snapshot()['forgotten'], [])
        self.assertEqual(found.snapshot()['peers'], [])
        schedule.assert_called_once_with('http://10.0.1.10:8321')

    def test_forgotten_seed_still_relays_other_nodes_without_reviving_itself(self):
        seed = 'http://10.0.1.10:8321'
        found = self.instance(seeds=[seed])
        found.accept(seed, manifest())
        found.forget('gateway-01')
        responses = [manifest(), {'peers': [
            dict(node_id='gateway-01', agent_url=seed),
            dict(node_id='gateway-02', agent_url='http://10.0.1.11:8321')]}]
        with patch.object(module, 'request_json', side_effect=responses), patch.object(found, 'schedule') as schedule:
            found._fetch(seed, found.generation)
        schedule.assert_called_once_with('http://10.0.1.11:8321')
        self.assertEqual(found.snapshot()['peers'], [])
        self.assertTrue(found.snapshot()['forgotten'][0]['observed_recently'])

    def test_new_identity_at_old_address_is_discoverable_and_returned_snapshot_is_detached(self):
        found = self.instance()
        url = 'http://10.0.1.10:8321'
        found.accept(url, manifest())
        found.forget('gateway-01')
        self.assertTrue(found.accept(url, manifest('replacement-01')))
        snapshot = found.snapshot()
        snapshot['peers'][0]['services'].clear()
        snapshot['forgotten'][0]['node_id'] = 'edited'
        self.assertTrue(found.peers['replacement-01']['services'])
        self.assertIn('gateway-01', found.forgotten)

    def test_all_local_manifest_callbacks_run_outside_discovery_lock(self):
        found = self.instance()
        def local():
            self.assertFalse(found.lock._is_owned(), 'manifest callback would invert App/Discovery lock order')
            return manifest('controller-01', services=[])
        found.manifest = local
        found.accept('http://10.0.1.10:8321', manifest())
        found._resolve()
        found.forget('gateway-01')
        with patch.object(found, 'schedule'):
            found.restore('gateway-01')

    def test_fetch_waiting_on_app_lock_does_not_block_inventory_mutator(self):
        found = self.instance()
        app_lock = threading.RLock()
        callback_entered = threading.Event()
        def local():
            callback_entered.set()
            with app_lock:
                return manifest('controller-01', services=[])
        found.manifest = local
        with app_lock:
            pending = found.pool.submit(found.accept, 'http://10.0.1.10:8321', manifest())
            self.assertTrue(callback_entered.wait(2))
            acquired = found.lock.acquire(timeout=0.5)
            if acquired:
                found.lock.release()
            self.assertTrue(acquired, 'App-lock owner would deadlock waiting for Discovery-lock owner')
        self.assertTrue(pending.result(timeout=3))


if __name__ == '__main__':
    unittest.main()
