#!/usr/bin/env python3
"""Policy and private inline material regressions without NetworkManager."""
import importlib.util
import os
from pathlib import Path
import sys
import tempfile
import unittest
from unittest.mock import Mock, patch

DEPLOY = Path(__file__).resolve().parents[1] / 'system-backend/deployment-core'
sys.path.insert(0, str(DEPLOY))
sys.path.insert(0, str(DEPLOY / 'image'))
if os.name == 'nt':
    import types
    sys.modules.setdefault('fcntl', types.ModuleType('fcntl'))
import vpn_nm

spec = importlib.util.spec_from_file_location('vpn_policy', DEPLOY / 'image/vpn-policy.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


class PolicyTests(unittest.TestCase):
    def setUp(self):
        self.key = '102a6a50-84ed-4d06-81e3-e8c27fb6bc30'
        self.cfg = dict(vpn_uuid=self.key, trusted_ssids=['Home:WiFi'], trusted_lan='10.0.1.0/24')
        self.runner = Mock()
        self.away = [dict(type='ethernet', addresses=['192.0.2.10/24'])]

    def test_away_activates_profile_without_credentials_in_argv(self):
        self.assertEqual(policy.apply_policy(self.cfg, self.away, set(), runner=self.runner), 'connecting')
        self.runner.assert_called_once_with(
            ['nmcli', '--wait', '20', 'connection', 'up', 'uuid', self.key], check=True, timeout=25)

    def test_wired_home_only_deactivates_our_profile(self):
        links = [dict(type='ethernet', addresses=['10.0.1.77/24'])]
        self.assertEqual(policy.apply_policy(self.cfg, links, {self.key, 'foreign'}, runner=self.runner),
                         'disconnected_home')
        self.assertEqual(self.runner.call_args.args[0][-4:], ['connection', 'down', 'uuid', self.key])

    def test_trusted_wifi_including_colons(self):
        links = [dict(type='wifi', ssid='Home:WiFi', addresses=['172.16.0.20/24'])]
        self.assertEqual(policy.apply_policy(self.cfg, links, set(), runner=self.runner), 'home')
        self.runner.assert_not_called()

    def test_foreign_wifi_is_not_trusted_just_because_ip_matches_home(self):
        links = [dict(type='wifi', ssid='Hotel', addresses=['10.0.1.77/24'])]
        self.assertEqual(policy.apply_policy(self.cfg, links, set(), runner=self.runner), 'connecting')

    def test_activating_or_connected_profile_is_not_started_twice(self):
        self.assertEqual(policy.apply_policy(self.cfg, self.away, {self.key}, runner=self.runner), 'active')
        self.runner.assert_not_called()

    def test_offline_waits_for_underlying_network(self):
        self.assertEqual(policy.apply_policy(self.cfg, [], set(), runner=self.runner), 'waiting_network')
        self.runner.assert_not_called()

    def test_other_tunnel_addresses_cannot_identify_trusted_wired_lan(self):
        for kind in ('tun', 'bridge', 'vpn'):
            with self.subTest(kind=kind):
                self.assertFalse(policy.trusted(
                    [dict(type=kind, addresses=['10.0.1.7/24'])], [], '10.0.1.0/24'))

    def test_activation_error_reaches_timer(self):
        self.runner.side_effect = RuntimeError('fixture connection failure')
        with self.assertRaisesRegex(RuntimeError, 'fixture connection failure'):
            policy.apply_policy(self.cfg, self.away, set(), runner=self.runner)

    def test_unmigrated_config_cannot_start_another_hidden_tunnel(self):
        del self.cfg['vpn_uuid']
        with self.assertRaises(KeyError):
            policy.apply_policy(self.cfg, self.away, set(), runner=self.runner)
        self.runner.assert_not_called()

    def test_device_collection_preserves_ssid_and_ignores_tunnel_addresses(self):
        answers = {
            ('-t', '-f', 'DEVICE', 'device'): 'eth0\nwlan0\ntun0',
            ('-g', 'GENERAL.STATE', 'device', 'show', 'eth0'): '100 (connected)',
            ('-g', 'GENERAL.TYPE', 'device', 'show', 'eth0'): 'ethernet',
            ('-g', 'IP4.ADDRESS', 'device', 'show', 'eth0'): '192.0.2.10/24',
            ('-g', 'GENERAL.STATE', 'device', 'show', 'wlan0'): '100 (connected)',
            ('-g', 'GENERAL.TYPE', 'device', 'show', 'wlan0'): 'wifi',
            ('-g', 'IP4.ADDRESS', 'device', 'show', 'wlan0'): '172.16.0.1/24',
            ('-g', 'GENERAL.CON-UUID', 'device', 'show', 'wlan0'): 'wifi-uuid',
            ('-g', '802-11-wireless.ssid', 'connection', 'show', 'uuid', 'wifi-uuid'): 'Home:WiFi',
            ('-g', 'GENERAL.STATE', 'device', 'show', 'tun0'): '100 (connected)',
            ('-g', 'GENERAL.TYPE', 'device', 'show', 'tun0'): 'tun',
            ('-g', 'UUID', 'connection', 'show', '--active'): self.key,
        }
        with tempfile.TemporaryDirectory() as temp:
            import json
            path = Path(temp) / 'network.json'
            path.write_text(json.dumps(self.cfg))
            with patch.object(policy, 'query', side_effect=lambda *args: answers[args]):
                with patch.object(policy, 'apply_policy', return_value='home') as apply:
                    policy.main(path)
            self.assertEqual(apply.call_args.args[1], [
                dict(type='ethernet', addresses=['192.0.2.10/24']),
                dict(type='wifi', addresses=['172.16.0.1/24'], ssid='Home:WiFi')])
            self.assertEqual(apply.call_args.args[2], {self.key})

    def test_malformed_lan_addresses_are_ignored(self):
        self.assertFalse(policy.trusted(
            [dict(type='ethernet', addresses=['', 'invalid'])], [], '10.0.1.0/24'))


class MaterialTests(unittest.TestCase):
    def test_all_inline_material_is_private_and_no_importer_home_extraction_remains(self):
        profile = 'client\ndev tun\nremote vpn.invalid 1194\n'
        for tag in sorted(vpn_nm.INLINE_FILES):
            profile += '<' + tag + '>\n# preserved\nFIXTURE ' + tag + '\n</' + tag + '>\n'
        with patch('vpn_nm.atomic_write') as write:
            result = vpn_nm.materialize_inline(profile, Path('/etc/openvpn/client/netcore.d'))
        self.assertEqual(write.call_count, len(vpn_nm.INLINE_FILES))
        for call in write.call_args_list:
            self.assertEqual(call.args[2], 0o600)
            self.assertTrue(call.args[0].as_posix().startswith('/etc/openvpn/client/netcore.d/'))
            self.assertIn('# preserved\nFIXTURE ', call.args[1])
        self.assertNotIn('<', result)
        self.assertIn('remote vpn.invalid 1194', result)

    def test_unclosed_material_is_rejected(self):
        with self.assertRaisesRegex(RuntimeError, 'Inline'):
            vpn_nm.materialize_inline('<key>\nnot-complete', Path('/private'))

    def test_plugin_error_does_not_publish_secrets_or_issue_a_keyfile(self):
        nm = Mock()
        info = Mock()
        info.get_service.return_value = vpn_nm.SERVICE_TYPE
        info.load_editor_plugin.return_value.import_.side_effect = RuntimeError('PRIVATE_TOKEN')
        nm.VpnPluginInfo.list_load.return_value = [info]
        with tempfile.TemporaryDirectory() as temp:
            with patch('vpn_nm.atomic_write') as write:
                with self.assertRaises(RuntimeError) as context:
                    vpn_nm.render(dict(vpn_config='client\ndev tun\nremote vpn.invalid\n',
                                       vpn_username='', vpn_password=''), root=Path(temp), nm=nm)
            self.assertNotIn('PRIVATE_TOKEN', str(context.exception))
            self.assertEqual(write.call_count, 1)


if __name__ == '__main__':
    unittest.main(verbosity=2)
