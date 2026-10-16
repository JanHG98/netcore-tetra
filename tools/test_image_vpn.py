#!/usr/bin/env python3
"""Portable OpenVPN login validation and private, offline image provisioning."""
import base64
from contextlib import ExitStack
import json
import os
from pathlib import Path
import sys
import tempfile
import types
import unittest
from unittest.mock import Mock, patch

ROOT = Path(__file__).resolve().parents[1]
DEPLOY = ROOT / 'system-backend/deployment-core'
sys.path.insert(0, str(DEPLOY))
sys.path.insert(0, str(DEPLOY / 'image'))
if os.name == 'nt':
    for name in ('fcntl', 'grp', 'pwd'):
        sys.modules.setdefault(name, types.ModuleType(name))
from image_spec import VPN_AUTH_PATH, public_job, validate_image, validate_vpn, validate_vpn_material
import personalize

PROFILE = 'client\ndev tun\nremote vpn.example 1194\n<ca>\nFIXTURE CA\n</ca>\n'
USERNAME = 'fixture-vpn-user'
PASSWORD = 'fixture-vpn-password#$; with spaces'
EXPORTED_PROFILE = '''dev tun
persist-tun
persist-key
client
resolv-retry infinite
reneg-sec 0
remote vpn.example 1194 udp4
lport 0
verify-x509-name "C=DE, ST=Lower Saxony, L=Hannover, O=Example, emailAddress=admin@example.invalid, CN=server" subject
remote-cert-tls server
auth-user-pass
<ca>
-----BEGIN CERTIFICATE-----
XYZ
-----END CERTIFICATE-----
</ca>
<cert>
-----BEGIN CERTIFICATE-----
XYZ
-----END CERTIFICATE-----
</cert>
<key>
-----BEGIN PRIVATE KEY-----
XYZ
-----END PRIVATE KEY-----
</key>
<tls-auth>
#
# 2048 bit OpenVPN static key
#
-----BEGIN OpenVPN Static key V1-----
XYZ
-----END OpenVPN Static key V1-----
</tls-auth>
key-direction 1
'''


def request(**changes):
    key = base64.b64encode(b'\0\0\0\x0bssh-ed25519\0\0\0\x20' + b'\x01' * 32).decode()
    data = dict(profile=dict(name='TBS-02', mcc=901, mnc=1510, issi=4010002, la=2, cc=2),
                commit='a' * 40, config='[net_info]\nmcc=901\n[cell_info]\ncolour_code=2\n[phy_io]\nbackend="SoapySdr"\n',
                controller_url='http://10.0.1.50:8320', ssh_key='ssh-ed25519 ' + key)
    data.update(changes)
    return data


class ImageVpnTests(unittest.TestCase):
    def setUp(self):
        if os.name == 'nt':
            timezone = patch('image_spec.ZoneInfo', return_value=object())
            timezone.start()
            self.addCleanup(timezone.stop)

    def authenticated(self, profile=None, **changes):
        values = dict(vpn_config=PROFILE + 'auth-user-pass\n' if profile is None else profile,
                      vpn_username=USERNAME, vpn_password=PASSWORD)
        values.update(changes)
        return validate_image(request(**values))

    def test_certificate_only_and_old_requests_remain_unchanged(self):
        self.assertEqual(validate_vpn(PROFILE), PROFILE)
        req = validate_image(request(vpn_config=PROFILE))
        self.assertEqual((req['vpn_config'], req['vpn_username'], req['vpn_password']), (PROFILE, '', ''))
        self.assertEqual(validate_image(req), req)
        req = validate_image(request())
        self.assertEqual((req['vpn_config'], req['vpn_username'], req['vpn_password']), ('', '', ''))

    def test_bare_auth_profile_requires_both_credentials_for_unattended_boot(self):
        for changes in ({}, {'vpn_username': USERNAME}, {'vpn_password': PASSWORD}):
            with self.subTest(fields=tuple(changes)), self.assertRaisesRegex(ValueError, 'VPN-Benutzername.*VPN-Passwort'):
                validate_image(request(vpn_config=PROFILE + 'auth-user-pass\n', **changes))

    def test_credentials_are_separated_and_request_revalidation_is_idempotent(self):
        req = self.authenticated()
        self.assertEqual((req['vpn_username'], req['vpn_password']), (USERNAME, PASSWORD))
        self.assertIn('auth-user-pass ' + VPN_AUTH_PATH + '\n', req['vpn_config'])
        self.assertIn('auth-nocache\n', req['vpn_config'])
        self.assertNotIn(USERNAME, req['vpn_config'])
        self.assertNotIn(PASSWORD, req['vpn_config'])
        self.assertEqual(validate_image(req), req)
        self.assertEqual(req['vpn_config'].count('auth-user-pass'), 1)

    def test_uploaded_old_auth_paths_are_never_read_and_are_replaced(self):
        for directive in ('auth-user-pass old-login.txt', 'auth-user-pass "/stale/user login.txt" # old PC',
                          'auth-user-pass "C:\\old login.txt"', '--auth-user-pass [inline]'):
            with self.subTest(directive=directive), patch.object(Path, 'read_text', side_effect=AssertionError('must not read supplied paths')):
                profile, user, password = validate_vpn_material(PROFILE + directive + '\n', USERNAME, PASSWORD)
            self.assertEqual((user, password), (USERNAME, PASSWORD))
            self.assertNotIn('old-login', profile)
            self.assertNotIn('stale', profile)
            self.assertEqual(profile.count('auth-user-pass'), 1)
            self.assertIn('auth-user-pass ' + VPN_AUTH_PATH, profile)

    def test_standard_export_with_lport_zero_preserves_subject_and_tls_material(self):
        req = self.authenticated(EXPORTED_PROFILE)
        expected = (EXPORTED_PROFILE.replace('auth-user-pass\n', '')
                    + 'auth-nocache\nauth-user-pass ' + VPN_AUTH_PATH + '\n')
        self.assertEqual(req['vpn_config'], expected)
        self.assertEqual((req['vpn_username'], req['vpn_password']), (USERNAME, PASSWORD))
        self.assertEqual(validate_image(req), req)
        with self.assertRaisesRegex(ValueError, 'VPN-Benutzername.*VPN-Passwort'):
            validate_image(request(vpn_config=EXPORTED_PROFILE))

    def test_lport_accepts_numeric_ports_and_quoted_or_commented_zero(self):
        for directive, port in (('lport 0', 0), ('lport 1', 1), ('lport 1194', 1194),
                                ('lport 65535', 65535), ('lport 00000', 0),
                                ('lport "0"', 0), ('--lport 0', 0),
                                ('lport 0 # exported local port', 0),
                                ('lport 0 ; exported local port', 0)):
            with self.subTest(directive=directive):
                expected = PROFILE + 'lport ' + str(port) + '\n'
                self.assertEqual(validate_vpn(PROFILE + directive + '\n'), expected)
                self.assertEqual(validate_vpn(expected), expected)

    def test_lport_attached_comments_normalize_to_unambiguous_numeric_option(self):
        for directive, port in (('lport 0#exported comment', 0), ('lport 65535;comment', 65535)):
            with self.subTest(directive=directive):
                expected = PROFILE + 'lport ' + str(port) + '\n'
                self.assertEqual(validate_vpn(PROFILE + directive + '\n'), expected)
                self.assertEqual(validate_vpn(expected), expected)

    def test_lport_rejects_invalid_ports_or_extra_arguments_without_echoing_values(self):
        for value in ('', '-1', '65536', '999999', '1.5', '1e3', '+1', '１２',
                      'secret-service-name', '0 misplaced-sensitive-password', '"unterminated-secret'):
            with self.subTest(value=value), self.assertRaises(ValueError) as error:
                validate_vpn(PROFILE + 'lport ' + value + '\n')
            self.assertIn('Zeile 7:', str(error.exception))
            for secret in ('secret-service-name', 'misplaced-sensitive-password', 'unterminated-secret'):
                self.assertNotIn(secret, str(error.exception))

    def test_lport_and_nobind_conflict_in_either_order(self):
        for tail in ('lport 0\nnobind\n', 'nobind\nlport 0\n'):
            with self.subTest(tail=tail), self.assertRaisesRegex(ValueError, 'lport und nobind'):
                validate_vpn(PROFILE + tail)

    def test_inline_login_is_extracted_including_comment_prefixes_in_password(self):
        embedded_password = '#;fixture-inline-password with spaces'
        profile = PROFILE + '<auth-user-pass>\n' + USERNAME + '\n' + embedded_password + '\n</auth-user-pass>\n'
        req = validate_image(request(vpn_config=profile))
        self.assertEqual((req['vpn_username'], req['vpn_password']), (USERNAME, embedded_password))
        self.assertNotIn('<auth-user-pass>', req['vpn_config'])
        self.assertNotIn(embedded_password, req['vpn_config'])
        self.assertEqual(validate_image(req), req)

    def test_inline_marker_and_block_normalize_to_one_auth_directive(self):
        profile = PROFILE + 'auth-user-pass [inline]\n<auth-user-pass>\n' + USERNAME + '\n' + PASSWORD + '\n</auth-user-pass>\n'
        req = self.authenticated(profile)
        self.assertEqual(req['vpn_config'].count('auth-user-pass'), 1)
        self.assertEqual(validate_image(req), req)

    def test_entered_credentials_replace_stale_inline_credentials(self):
        profile = PROFILE + '<auth-user-pass>\nold-vpn-user\nold-vpn-secret\n</auth-user-pass>\n'
        req = self.authenticated(profile)
        self.assertEqual((req['vpn_username'], req['vpn_password']), (USERNAME, PASSWORD))
        for secret in ('old-vpn-user', 'old-vpn-secret', USERNAME, PASSWORD):
            self.assertNotIn(secret, req['vpn_config'])
        self.assertEqual(validate_image(req), req)

    def test_inline_username_only_requires_fields_for_unattended_login(self):
        profile = PROFILE + '<auth-user-pass>\nold-vpn-user\n</auth-user-pass>\n'
        with self.assertRaisesRegex(ValueError, 'VPN-Benutzername.*VPN-Passwort'):
            validate_image(request(vpn_config=profile))
        req = self.authenticated(profile)
        self.assertEqual((req['vpn_username'], req['vpn_password']), (USERNAME, PASSWORD))

    def test_malformed_and_duplicate_auth_options_fail_closed(self):
        tails = ('auth-user-pass one two\n', 'auth-user-pass "unterminated\n',
                 'auth-user-pass\nauth-user-pass login.txt\n',
                 '<auth-user-pass>\n</auth-user-pass>\n',
                 '<auth-user-pass>\nuser\npass\nextra\n</auth-user-pass>\n',
                 '<auth-user-pass>\nuser\npass\n',
                 '<auth-user-pass>\nuser\npass\n</auth-user-pass>\n<auth-user-pass>\nuser\npass\n</auth-user-pass>\n')
        for index, tail in enumerate(tails):
            with self.subTest(index=index), self.assertRaises(ValueError):
                self.authenticated(PROFILE + tail)

    def test_invalid_credentials_reject_controls_and_utf8_overflow(self):
        for key in ('vpn_username', 'vpn_password'):
            for value in ('line\nsecret', 'line\rsecret', 'tab\tsecret', 'nul\x00secret', '\x7fsecret', 'é' * 64, 42):
                with self.subTest(field=key, kind=type(value).__name__), self.assertRaises(ValueError) as error:
                    self.authenticated(**{key: value})
                if isinstance(value, str) and len(value) > 1:
                    self.assertNotIn(value, str(error.exception))

    def test_credentials_without_profile_are_rejected_and_fields_can_enable_auth(self):
        with self.assertRaisesRegex(ValueError, 'Profildatei'):
            self.authenticated('')
        req = self.authenticated(PROFILE)
        self.assertIn('auth-user-pass ' + VPN_AUTH_PATH, req['vpn_config'])

    def test_bom_and_crlf_are_accepted_without_corrupting_certificates(self):
        req = self.authenticated('\ufeff' + PROFILE.replace('\n', '\r\n') + 'auth-user-pass\r\n')
        self.assertTrue(req['vpn_config'].startswith('client\n'))
        self.assertIn('<ca>\nFIXTURE CA\n</ca>', req['vpn_config'])

    def test_unsupported_profile_errors_never_echo_potential_secret_tokens(self):
        for tail in ('misplaced-sensitive-password\n', 'up /tmp/secret-hook\n', 'config /etc/passwd\n'):
            with self.subTest(kind=tail.split()[0]), self.assertRaises(ValueError) as error:
                self.authenticated(PROFILE + tail)
            self.assertIn('Zeile 7: nicht unterstützte Direktive', str(error.exception))
            self.assertNotIn(tail.split()[0], str(error.exception))

    def test_diagnostics_count_original_lines_including_inline_auth_bom_and_crlf(self):
        profile = ('\ufeff# comment\n\n' + PROFILE
                   + '<auth-user-pass>\n' + USERNAME + '\n' + PASSWORD
                   + '\n</auth-user-pass>\nmisplaced-sensitive-password\n')
        for newline in ('\n', '\r\n'):
            with self.subTest(newline=repr(newline)), self.assertRaises(ValueError) as error:
                validate_vpn_material(profile.replace('\n', newline))
            self.assertIn('Zeile 13: nicht unterstützte Direktive', str(error.exception))
            for secret in (USERNAME, PASSWORD, 'misplaced-sensitive-password'):
                self.assertNotIn(secret, str(error.exception))

    def test_unsupported_escape_syntax_has_distinct_private_line_diagnostic(self):
        tail = 'verify-x509-name "fixture\\\\private-name" name\n'
        with self.assertRaises(ValueError) as error:
            self.authenticated(PROFILE + tail)
        message = str(error.exception)
        self.assertIn('Zeile 7: Escape-Syntax mit Backslash', message)
        self.assertNotIn('nicht unterstützte Direktive', message)
        self.assertNotIn('fixture', message)
        self.assertNotIn('private-name', message)

    def test_public_job_does_not_publish_login_or_private_profile(self):
        req = self.authenticated()
        result = json.dumps(public_job(dict(request=req, result={}, log='', status='queued')))
        for secret in (USERNAME, PASSWORD, 'vpn_config', 'vpn_username', 'vpn_password', 'FIXTURE CA'):
            self.assertNotIn(secret, result)

    def provision(self, req, root, runner):
        with ExitStack() as stack:
            if os.name == 'nt':
                # Linux CI runs real atomic_write/fchmod and directory fsync.
                def windows_write(path, value, mode=0o600):
                    Path(path).parent.mkdir(parents=True, exist_ok=True)
                    Path(path).write_text(value, encoding='utf-8')
                    os.chmod(path, mode)
                stack.enter_context(patch('personalize.atomic_write', side_effect=windows_write))
            stack.enter_context(patch('personalize.render_vpn', return_value='102a6a50-84ed-4d06-81e3-e8c27fb6bc30'))
            return personalize.configure_vpn(req, root=root, runner=runner)

    def test_image_uses_private_auth_and_nm_profile_with_one_policy(self):
        with tempfile.TemporaryDirectory() as directory:
            root, runner = Path(directory), Mock()
            req = self.authenticated()
            self.provision(req, root, runner)
            auth = root / VPN_AUTH_PATH.lstrip('/')
            config = root / 'etc/openvpn/client/netcore.conf'
            self.assertEqual(auth.read_text(), USERNAME + '\n' + PASSWORD + '\n')
            self.assertEqual(config.read_text(), req['vpn_config'])
            self.assertNotIn(PASSWORD, config.read_text())
            self.assertEqual(runner.call_args_list, [
                unittest.mock.call('systemctl', 'disable', 'openvpn-client@netcore.service'),
                unittest.mock.call('systemctl', 'enable', 'netcore-vpn-policy.timer')])
            self.assertNotIn(PASSWORD, repr(runner.call_args))
            if os.name == 'posix':
                self.assertEqual(auth.stat().st_mode & 0o777, 0o600)
                self.assertEqual(config.stat().st_mode & 0o777, 0o600)

    def test_certificate_only_profile_removes_stale_login_file(self):
        with tempfile.TemporaryDirectory() as directory:
            root, runner = Path(directory), Mock()
            auth = root / VPN_AUTH_PATH.lstrip('/')
            auth.parent.mkdir(parents=True)
            auth.write_text('stale-login\nstale-secret\n')
            self.provision(validate_image(request(vpn_config=PROFILE)), root, runner)
            self.assertFalse(auth.exists())
            self.assertEqual((auth.parent / 'netcore.conf').read_text(), PROFILE)

    def test_optional_vpn_performs_no_file_or_unit_operations(self):
        runner = Mock()
        with tempfile.TemporaryDirectory() as directory:
            root = Path(directory)
            self.provision(validate_image(request()), root, runner)
            self.assertEqual(list(root.iterdir()), [])
        runner.assert_not_called()


if __name__ == '__main__':
    unittest.main(verbosity=2)

