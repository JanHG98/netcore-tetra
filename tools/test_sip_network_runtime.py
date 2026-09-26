#!/usr/bin/env python3
"""Regression tests for slow dependencies, terminal calls and exclusive fallback."""
import importlib.util
import json
import subprocess
import sys
import tempfile
import threading
import time
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / 'system-backend/sip-switch'
sys.path.insert(0, str(BASE / 'src'))
from netcore_sip_switch import Config, SipSwitch
from netcore_sip_runtime import available_contact, registration_status

spec = importlib.util.spec_from_file_location('fallback', BASE / 'tbs-fallback/src/netcore_tbs_sip_fallback.py')
fallback = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fallback)


class SipRuntimeTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.path = Path(self.temp.name)
        self.config = Config({
            'storage': {key: str(self.path / key) for key in ('state_file', 'event_log', 'audit_log')},
            'mqtt': {'enabled': False},
            'management': {'side_effect_queue_size': 4, 'route_workers': 2},
            'pbx': {'endpoint_id': 'pbx'},
            'routing': {},
            'mobility_core': {'enabled': True},
        }, self.path / 'config.toml')
        self.app = SipSwitch(self.config)
        self.addCleanup(self.app.io.close)

    def test_registered_is_exact_not_unregistered_or_other_id(self):
        text = 'other-registration/sip:pbx Registered\npbx-registration/sip:pbx Unregistered\n'
        self.assertEqual(registration_status(text, 'pbx-registration'), 'unregistered')
        self.assertEqual(registration_status(text, 'registration'), 'absent')
        self.assertFalse(available_contact('Contact: <Aor/ContactUri> <Status>\nContact: x/sip:x NonQual'))
        self.assertTrue(available_contact('Contact: x/sip:one Unavail\nContact: x/sip:two Avail'))

    def test_slow_mqtt_cannot_block_routing_or_call_end(self):
        entered, release = threading.Event(), threading.Event()
        self.addCleanup(release.set)
        def blocked_publish(*_args, **_kwargs):
            entered.set()
            release.wait(3)
            return False
        self.app.publish_mqtt = blocked_publish
        first = self.app.resolve('outbound', '103')
        self.assertTrue(entered.wait(1))
        started = time.monotonic()
        for _ in range(20):
            self.assertEqual(self.app.resolve('outbound', '104')['action'], 'pbx')
        self.assertTrue(self.app.update_call(first['call_token'], 'ended', {})[0])
        self.assertLess(time.monotonic() - started, .5)
        self.assertGreater(self.app.io.dropped, 0)
        release.set()
        self.app.io.close()
        saved = json.loads(self.app.state_path.read_text())
        self.assertEqual(saved['calls'][first['call_token']]['state'], 'ended')

    def test_delayed_callbacks_do_not_resurrect_calls(self):
        token = self.app.resolve('outbound', '103')['call_token']
        self.app.update_call(token, 'answered', {})
        self.app.update_call(token, 'dialing', {})
        self.assertEqual(self.app.calls[token]['state'], 'answered')
        self.app.update_call(token, 'ended', {'hangup_cause': '16'})
        self.app.update_call(token, 'answered', {})
        self.app.update_call(token, 'ended', {})
        self.assertEqual(self.app.calls[token]['state'], 'ended')
        self.assertEqual(self.app.metrics['calls_ended'], 1)
        self.assertFalse(self.app.update_call(token, 'invented', {})[0])

    def test_disconnected_node_never_routes_even_when_registration_is_confirmed(self):
        route = {'state': 'confirmed', 'registered': True, 'serving_node': 'tbs', 'node_connected': False}
        with patch('netcore_sip_switch.http_json', return_value=(200, route)):
            self.assertFalse(self.app.mobility_route(5102)[0])
        route['node_connected'] = True
        route['node_stale'] = True
        with patch('netcore_sip_switch.http_json', return_value=(200, route)):
            self.assertFalse(self.app.mobility_route(5102)[0])

    def test_overload_rejects_without_waiting_on_dependencies(self):
        self.app.route_slots.acquire()
        self.app.route_slots.acquire()
        self.addCleanup(self.app.route_slots.release)
        self.addCleanup(self.app.route_slots.release)
        with patch.object(self.app, '_resolve', side_effect=AssertionError('must not resolve')):
            self.assertEqual(self.app.resolve('outbound', '103')['reason'], 'routing_overloaded')

    def test_stale_health_is_not_ready(self):
        self.app.health.update(asterisk=True, mobility_core=True, pbx=True)
        self.app.last_probe_monotonic = time.monotonic() - 90
        self.assertFalse(self.app.status()['health']['fresh'])


class FallbackTests(unittest.TestCase):
    def setUp(self):
        from check_sip_switch_phase11c import FakeCLI
        import tomllib
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        path = Path(self.temp.name)
        text = (BASE / 'tbs-fallback/config/tbs-sip-fallback.example.toml').read_text()
        cfg = tomllib.loads(text)
        for name in ('state_file', 'active_registration_file', 'central_registration_file', 'pbx_registration_file'):
            cfg['failover'][name] = str(path / Path(cfg['failover'][name]).name)
        cfg['failover'].update(unregister_grace_secs=0, central_registration_grace_secs=0)
        self.controller = fallback.FailoverController.__new__(fallback.FailoverController)
        self.controller.cfg = cfg
        self.controller.n = fallback.names(cfg)
        self.controller.state = fallback.default_state(cfg)
        self.controller.sleeper = lambda *_: None
        self.controller.cli = FakeCLI(Path(cfg['failover']['active_registration_file']))
        fallback.atomic_write(self.controller.active_file, fallback.render_active_registration(cfg, 'central'))

    def test_failed_reload_rolls_back_registration_and_db(self):
        controller = self.controller
        original = controller.cli.reload_registrations
        attempts = []
        def reload():
            attempts.append(1)
            if len(attempts) == 1:
                raise RuntimeError('injected reload failure')
            original()
        controller.cli.reload_registrations = reload
        with self.assertRaisesRegex(RuntimeError, 'injected'):
            controller.switch_mode('pbx_direct', 'test', 10)
        self.assertEqual(controller.state['mode'], 'central')
        self.assertEqual(controller.cli.db_mode, 'central')
        self.assertEqual(controller.active_file.read_text(), fallback.render_active_registration(controller.cfg, 'central'))
        self.assertEqual(controller.state['phase'], 'TRANSITION_FAILED')

    def test_switch_checks_routing_readiness_and_hysteresis(self):
        controller = self.controller
        controller.routing_ready = lambda: False
        controller.tick(10)
        controller.tick(12)
        self.assertEqual(controller.state['mode'], 'central')
        controller.tick(14)
        self.assertEqual(controller.state['mode'], 'pbx_direct')
        controller.routing_ready = lambda: True
        controller.tick(20)
        controller.tick(49)
        self.assertEqual(controller.state['mode'], 'pbx_direct')
        controller.tick(50)
        self.assertEqual(controller.state['mode'], 'central')

    def test_asterisk_timeout_counts_as_failed_probe(self):
        controller = self.controller
        controller.cli.aor_available = lambda *_: (_ for _ in ()).throw(subprocess.TimeoutExpired('asterisk', 2))
        controller.tick(10)
        self.assertEqual(controller.state['central_failures'], 1)
        self.assertFalse(controller.state['last_probe_ok'])

    def test_direct_path_precedes_central_dial_and_inbound_is_gated(self):
        dialplan = fallback.render_extensions(self.controller.cfg)
        outgoing, incoming = dialplan.split('[netcore-from-central-switch]')
        self.assertLess(outgoing.index('pbx_direct'), outgoing.index('Dial('))
        self.assertIn('central-blocked', incoming)


if __name__ == '__main__':
    unittest.main()
