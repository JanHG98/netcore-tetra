#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für check TETRA-Paketdatentransport Laufzeit.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Dependency-free architecture guard for SWMI Foundation 1 Package D."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


# Was: Führt den Arbeitsschritt `fail` für fail aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def fail(message: str) -> None:
    print(f"TLPD Package D check failed: {message}", file=sys.stderr)
    raise SystemExit(1)


# Was: Diese Funktion liest den vorgesehenen Arbeitsschritt.
# Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
def read(relative: str) -> str:
    path = ROOT / relative
    if not path.is_file():
        fail(f"missing required file: {relative}")
    return path.read_text(encoding="utf-8")


# Was: Führt den Arbeitsschritt `require` für require aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def require(text: str, markers: list[str], label: str) -> None:
    missing = [marker for marker in markers if marker not in text]
    if missing:
        fail(f"{label} misses: {', '.join(missing)}")


# Was: Führt den Arbeitsschritt `function_body` für function body aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def function_body(text: str, name: str) -> str:
    match = re.search(rf"\bfn\s+{re.escape(name)}\b", text)
    if not match:
        fail(f"function {name} not found")
    start = text.find("{", match.end())
    if start < 0:
        fail(f"function {name} has no body")
    depth = 0
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for index in range(start, len(text)):
        if text[index] == "{":
            depth += 1
        elif text[index] == "}":
            depth -= 1
            if depth == 0:
                return text[start : index + 1]
    fail(f"function {name} has an unclosed body")


# Was: Führt den Arbeitsschritt `delimiters` für delimiters aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def delimiters(relative: str, text: str) -> None:
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for opening, closing in (("{", "}"), ("(", ")"), ("[", "]")):
        if text.count(opening) != text.count(closing):
            fail(f"unbalanced {opening}{closing} in {relative}")


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> None:
    runtime_path = "crates/tetra-entities/src/mle/ltpd_runtime.rs"
    mle_path = "crates/tetra-entities/src/mle/mle_bs.rs"
    sndcp_path = "crates/tetra-entities/src/sndcp/sndcp_bs.rs"
    ltpd_path = "crates/tetra-saps/src/ltpd/mod.rs"
    test_path = "crates/tetra-entities/tests/test_ltpd_runtime.rs"

    runtime = read(runtime_path)
    mle = read(mle_path)
    sndcp = read(sndcp_path)
    ltpd = read(ltpd_path)
    tests = read(test_path)

    require(
        runtime,
        [
            "pub struct LtpdRuntime",
            "pub struct LtpdRuntimeSnapshot",
            "pub struct LtpdLinkSnapshot",
            "pub fn observe_inbound",
            "pub fn notify_break",
            "pub fn notify_resume",
            "pub fn set_busy",
            "pub fn set_disabled",
            "pub fn open_network",
            "pub fn close_network",
            "pub fn handle_primitive",
            "fn configure",
            "fn connect",
            "fn disconnect",
            "fn reconnect",
            "fn release",
            "fn unitdata",
            "TransferResult::SuccessBufferEmpty",
            "TransferResult::FailedRemovedFromBuffer",
            "MleProtocolDiscriminator::Sndcp",
            "SapMsgInner::TlaTlDataReqBl",
            "SapMsgInner::TlaTlUnitdataReqBl",
        ],
        "TLPD runtime",
    )
    dispatch = function_body(runtime, "handle_primitive")
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for primitive in (
        "LtpdMleActivityReq",
        "LtpdMleCancelReq",
        "LtpdMleConfigureReq",
        "LtpdMleConnectReq",
        "LtpdMleConnectResp",
        "LtpdMleDisconnectReq",
        "LtpdMleReconnectReq",
        "LtpdMleReleaseReq",
        "LtpdMleUnitdataReq",
    ):
        if primitive not in dispatch:
            fail(f"TLPD dispatcher does not handle {primitive}")
    if "unimplemented!" in dispatch or "unimplemented_log!" in dispatch:
        fail("TLPD dispatcher still contains an unimplemented path")

    require(
        mle,
        [
            "ltpd: LtpdRuntime",
            "pub fn ltpd_snapshot",
            "self.ltpd.observe_inbound",
            "self.ltpd.handle_primitive",
            "self.ltpd.tick",
            "LowerLayerResourceAvailability::Available => self.ltpd.notify_resume",
            "LowerLayerResourceAvailability::Unavailable => self.ltpd.notify_break",
        ],
        "MLE-BS adapter",
    )
    if "rx_tlpd_prim called but TLPD SAP is not implemented" in mle:
        fail("old MLE TLPD stub is still present")

    require(
        sndcp,
        [
            "pub struct SndcpLtpdSnapshot",
            "pub fn ltpd_snapshot",
            "fn allocate_ltpd_handle",
            "Sap::TlpdSap",
            "SapMsgInner::LtpdMleUnitdataReq",
            "SapMsgInner::LtpdMleOpenInd",
            "SapMsgInner::LtpdMleBreakInd",
            "SapMsgInner::LtpdMleResumeInd",
            "SapMsgInner::LtpdMleReportInd",
        ],
        "SNDCP TLPD client",
    )
    queue = function_body(sndcp, "queue_ltpd_to")
    if "TlaTlDataReqBl" in queue or "TlaTlUnitdataReqBl" in queue:
        fail("SNDCP still bypasses MLE from queue_ltpd_to")

    require(
        ltpd,
        [
            "pub address: Option<TetraAddress>",
            "pub chan_alloc: Option<CmceChanAllocReq>",
        ],
        "LTPD UNITDATA route/allocation fields",
    )

    require(
        tests,
        [
            "initial_open_and_info_update_the_sndcp_client_snapshot",
            "inbound_unitdata_registers_route_and_reaches_sndcp",
            "downlink_unitdata_is_wrapped_by_mle_and_reported_to_sndcp",
            "route_hint_rebuilds_context_after_local_restart",
            "unknown_route_without_hint_is_rejected",
            "connect_disconnect_and_reconnect_have_explicit_results",
            "tlmc_resource_edges_drive_break_and_resume",
        ],
        "Package D integration tests",
    )

    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    for relative, text in (
        (runtime_path, runtime),
        (mle_path, mle),
        (sndcp_path, sndcp),
        (ltpd_path, ltpd),
        (test_path, tests),
    ):
        delimiters(relative, text)

    print("SWMI Foundation 1 Package D static checks passed.")
    print("  bidirectional MLE-UNITDATA routing: present")
    print("  local context registry and lifecycle: present")
    print("  break/resume and TLMC coupling: present")
    print("  connect/disconnect/reconnect: present")
    print("  SNDCP no longer bypasses MLE for packet-data replies: present")
    print("  TBS diagnostic snapshots: present")


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    main()
