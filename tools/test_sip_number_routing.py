#!/usr/bin/env python3
"""Regressions for PBX T-prefixed TETRA destinations and numeric routing.

The small dialplan dispatcher only handles the generated X!/character-class
entry patterns and Goto aliases. It connects those entries to the real Python
resolver; it is not an Asterisk interpreter or a live SIP/media test.
"""
import importlib.util
import re
import sys
import tempfile
import tomllib
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[1]
BASE = ROOT / "system-backend/sip-switch"
sys.path.insert(0, str(BASE / "src"))
from netcore_sip_switch import Config, SipSwitch

spec = importlib.util.spec_from_file_location(
    "number_routing_fallback", BASE / "tbs-fallback/src/netcore_tbs_sip_fallback.py"
)
fallback = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fallback)


def dialplan_entries(text, context):
    """Return one generated context's entry patterns and instruction lists."""
    entries = {}
    active = False
    current = None
    for line in text.splitlines():
        if line.startswith("["):
            active = line == f"[{context}]"
            current = None
        elif active:
            match = re.match(r"\s*exten\s*=>\s*([^,]+),[^,]+,(.*)", line)
            if match:
                current = match[1]
                entries[current] = [match[2]]
            elif current:
                match = re.match(r"\s*same\s*=>\s*[^,]+,(.*)", line)
                if match:
                    entries[current].append(match[1])
    return entries


def dispatch_number(entries, number):
    """Match the generated entry, following only its number-stripping alias."""
    for _ in range(3):
        matched = None
        for pattern, instructions in entries.items():
            if pattern.startswith("_"):
                regex = pattern[1:].replace("X", "[0-9]").replace("!", ".*")
                accepts = re.fullmatch(regex, number) is not None
            else:
                accepts = pattern == number
            if accepts:
                matched = instructions
                break
        if matched is None:
            return None, []
        if matched[0] == "Goto(${EXTEN:1},1)":
            number = number[1:]
            continue
        return number, matched
    raise AssertionError("generated dialplan alias loop")


class SipNumberRoutingTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        path = Path(self.temp.name)
        self.config = Config({
            "storage": {
                name: str(path / name)
                for name in ("state_file", "event_log", "audit_log")
            },
            "mqtt": {"enabled": False},
            "mobility_core": {"enabled": True, "base_url": "http://mobility.test:8090"},
            "pbx": {"endpoint_id": "pbx"},
            "routing": {},
            "tbs": [{
                "node_id": "SRV-M-TBS-01", "endpoint_id": "tbs-srv-m-tbs-01",
                "username": "tbs-registration", "enabled": True,
            }],
        }, path / "config.toml")
        self.app = SipSwitch(self.config)
        self.addCleanup(self.app.io.close)
        self.mobility_queries = []
        self.cli_queries = []

        def mobility_http(method, url, **kwargs):
            self.mobility_queries.append((method, url))
            return 200, {
                "state": "confirmed", "registered": True,
                "serving_node": "SRV-M-TBS-01", "node_connected": True,
            }

        def asterisk_cli(command, **kwargs):
            self.cli_queries.append(command)
            return True, "Contact: tbs-registration/sip:tbs@10.0.1.20:5060 Avail 1.5"

        http_mock = patch("netcore_sip_switch.http_json", side_effect=mobility_http)
        cli_mock = patch.object(self.app, "run_asterisk", side_effect=asterisk_cli)
        http_mock.start()
        cli_mock.start()
        self.addCleanup(http_mock.stop)
        self.addCleanup(cli_mock.stop)

    def resolve_inbound(self, number):
        return self.app.resolve("inbound", number, caller="101", commit=False)

    def test_raw_upper_and_lower_prefixed_numbers_route_to_numeric_issi(self):
        for number in ("5102", "T5102", "t5102", " T5102 "):
            with self.subTest(number=number):
                result = self.resolve_inbound(number)
                self.assertEqual(result["action"], "tbs")
                self.assertEqual(result["issi"], 5102)
                self.assertEqual(result["destination"], "5102")
                self.assertEqual(result["node_id"], "SRV-M-TBS-01")
                self.assertEqual(self.mobility_queries[-1], (
                    "GET", "http://mobility.test:8090/api/v1/subscribers/5102/route"
                ))
                self.assertEqual(self.cli_queries[-1], "pjsip show aor tbs-registration")

    def test_malformed_destination_does_not_query_mobility_or_contact(self):
        for number in ("TT5102", "foo5102", "T", "T51x02", "51x02", "T51-02",
                       "T 5102", "5102T", "+5102", "T５１０２", "٥١٠٢", ""):
            with self.subTest(number=number):
                self.assertEqual(self.resolve_inbound(number)["action"], "reject")
        self.assertEqual(self.mobility_queries, [])
        self.assertEqual(self.cli_queries, [])

    def test_prefixed_number_preserves_explicit_mapping_priority(self):
        self.config.raw["number_mappings"] = [
            {"number": "5102", "target_type": "issi", "target": 2020001, "enabled": True}
        ]
        # Explicit mappings must still win before the configured numeric prefix.
        self.config.routing["tetra_number_prefix"] = "99"
        for number in ("5102", "T5102", "t5102"):
            with self.subTest(number=number):
                result = self.resolve_inbound(number)
                self.assertEqual(result["action"], "tbs")
                self.assertEqual(result["destination"], "2020001")
                self.assertEqual(result["reason"], "explicit_mapping")
                self.assertTrue(self.mobility_queries[-1][1].endswith("/2020001/route"))

    def test_numeric_prefix_rules_are_identical_with_or_without_marker(self):
        self.config.routing.update(tetra_number_prefix="40", strip_tetra_prefix=True)
        for number in ("405102", "T405102", "t405102"):
            with self.subTest(number=number):
                self.assertEqual(self.resolve_inbound(number)["destination"], "5102")
        self.assertEqual(self.resolve_inbound("T5102")["action"], "reject")
        self.config.routing["strip_tetra_prefix"] = False
        self.assertEqual(self.resolve_inbound("T405102")["destination"], "405102")

    def test_24_bit_range_remains_enforced_after_marker(self):
        for number in ("16777215", "T16777215"):
            self.assertEqual(self.resolve_inbound(number)["destination"], "16777215")
        for number in ("16777216", "T16777216", "T99999999999"):
            self.assertEqual(self.resolve_inbound(number)["action"], "reject")

    def test_outbound_numeric_prefix_is_unchanged(self):
        self.config.routing.update(pbx_outbound_prefix="91", strip_pbx_outbound_prefix=True)
        result = self.app.resolve("outbound", "91103", commit=False)
        self.assertEqual((result["action"], result["destination"], result["endpoint"]),
                         ("pbx", "103", "pbx"))

    def test_rendered_pbx_context_dispatches_marker_to_real_resolver(self):
        entries = dialplan_entries(self.app._render_extensions(), "netcore-from-pbx")
        for dialed in ("5102", "T5102", "t5102"):
            with self.subTest(dialed=dialed):
                number, instructions = dispatch_number(entries, dialed)
                self.assertEqual(number, "5102")
                self.assertTrue(any(
                    command.startswith("AGI(") and ",resolve,inbound,${EXTEN}," in command
                    for command in instructions
                ))
                result = self.resolve_inbound(number)
                self.assertEqual(result["destination"], "5102")
                self.assertEqual(result["action"], "tbs")
        self.assertIsNone(dispatch_number(entries, "T")[0])

    def test_managed_agi_uses_absolute_install_path_for_new_and_legacy_config(self):
        expected = "/var/lib/asterisk/agi-bin/netcore-sip-route.py"
        for settings in ({}, {"agi_script": "netcore-sip-route.py"}, {"agi_script": expected}):
            with self.subTest(settings=settings):
                self.config.raw["asterisk"] = settings
                dialplan = self.app._render_extensions()
                invocations = re.findall(r"\bAGI\(([^,]+),([^,]+),", dialplan)
                self.assertTrue(invocations)
                self.assertEqual({path for path, mode in invocations}, {expected})
                self.assertEqual({mode for path, mode in invocations}, {"resolve", "state", "hangup"})

    def test_explicit_custom_agi_paths_are_preserved(self):
        for path in ("/srv/company/agi/route.py", "company-route.py"):
            with self.subTest(path=path):
                self.config.raw["asterisk"] = {"agi_script": path}
                invocations = re.findall(r"\bAGI\(([^,]+),", self.app._render_extensions())
                self.assertTrue(invocations)
                self.assertEqual(set(invocations), {path})

    def test_direct_fallback_alias_keeps_mode_and_prefix_checks(self):
        cfg = tomllib.loads((BASE / "tbs-fallback/config/tbs-sip-fallback.example.toml").read_text())
        cfg["routing"].update(fallback_tetra_prefix="40", strip_fallback_tetra_prefix=True)
        text = fallback.render_extensions(cfg)
        entries = dialplan_entries(text, "netcore-from-pbx-fallback")
        numeric, original_path = dispatch_number(entries, "405102")
        for dialed in ("T405102", "t405102"):
            with self.subTest(dialed=dialed):
                number, instructions = dispatch_number(entries, dialed)
                self.assertEqual(number, numeric)
                self.assertEqual(instructions, original_path)
                # The alias enters the ordinary path before the exclusivity gate.
                gate = next(i for i, cmd in enumerate(instructions) if "DB(netcore/failover_mode)" in cmd)
                prefix = next(i for i, cmd in enumerate(instructions) if '${EXTEN:0:2}' in cmd)
                contacts = next(i for i, cmd in enumerate(instructions) if "PJSIP_DIAL_CONTACTS" in cmd)
                self.assertLess(gate, prefix)
                self.assertLess(prefix, contacts)
                self.assertIn("Set(NETCORE_TETRA_NUMBER=${EXTEN:2})", instructions)
        central_entries = dialplan_entries(text, "netcore-from-central-switch")
        self.assertEqual(dispatch_number(central_entries, "5102")[0], "5102")
        self.assertIsNone(dispatch_number(central_entries, "T5102")[0])

    def test_disabled_direct_fallback_still_rejects_t_marker_path(self):
        cfg = tomllib.loads((BASE / "tbs-fallback/config/tbs-sip-fallback.example.toml").read_text())
        cfg["routing"]["accept_direct_pbx_fallback"] = False
        entries = dialplan_entries(fallback.render_extensions(cfg), "netcore-from-pbx-fallback")
        number, instructions = dispatch_number(entries, "T5102")
        self.assertEqual(number, "5102")
        self.assertEqual(instructions[-1], "Hangup(21)")
        self.assertFalse(any("PJSIP_DIAL_CONTACTS" in cmd for cmd in instructions))

    def test_direct_fallback_checks_final_digits_before_native_contact(self):
        cfg = tomllib.loads((BASE / "tbs-fallback/config/tbs-sip-fallback.example.toml").read_text())
        cfg["routing"].update(fallback_tetra_prefix="40", strip_fallback_tetra_prefix=True)
        entries = dialplan_entries(fallback.render_extensions(cfg), "netcore-from-pbx-fallback")
        for dialed, accepted in (("T405102", True), ("405102", True),
                                 ("T4051x02", False), ("T40", False), ("4051-02", False)):
            with self.subTest(dialed=dialed):
                number, instructions = dispatch_number(entries, dialed)
                assignment = next(cmd for cmd in instructions if cmd.startswith("Set(NETCORE_TETRA_NUMBER="))
                strip_count = int(re.search(r"EXTEN:(\d+)", assignment)[1])
                destination = number[strip_count:]
                guard_position = next(i for i, cmd in enumerate(instructions) if "${REGEX(" in cmd)
                contacts_position = next(i for i, cmd in enumerate(instructions) if "PJSIP_DIAL_CONTACTS" in cmd)
                guard = instructions[guard_position]
                expression = re.search(r'REGEX\("([^"]+)" \$\{NETCORE_TETRA_NUMBER\}\)', guard)
                self.assertIsNotNone(expression)
                self.assertEqual(re.search(expression[1], destination) is not None, accepted)
                self.assertLess(guard_position, contacts_position)
                self.assertIn("?valid-number:invalid-number)", guard)
                self.assertEqual(instructions[guard_position + 1], "Hangup(28)")


if __name__ == "__main__":
    unittest.main()
