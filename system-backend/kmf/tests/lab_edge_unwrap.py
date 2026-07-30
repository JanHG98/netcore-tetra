#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Schlüsselverwaltung und Schlüsselverteilung.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Reference-only decoder for the KMF lab envelope.

This helper is intentionally not a TETRA OTAR implementation. It verifies that a
TBS with the matching bootstrap secret can open a node-bound test envelope.
"""
from __future__ import annotations

import hashlib
import hmac
import json
import sys
from pathlib import Path


# Was: Führt den Arbeitsschritt `stream` für stream aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def stream(key: bytes, nonce: bytes, context: bytes, length: int) -> bytes:
    out = bytearray()
    counter = 0
    # Was: Wiederholt den folgenden Abschnitt für mehrere Einträge oder solange die Bedingung erfüllt ist.
    # Warum: Gleichartige Daten oder wiederkehrende Prüfungen werden dadurch vollständig und einheitlich abgearbeitet.
    while len(out) < length:
        out.extend(
            hashlib.sha256(
                b"netcore-kmf-lab-stream-v1"
                + key
                + nonce
                + context
                + counter.to_bytes(8, "big")
            ).digest()
        )
        counter += 1
    return bytes(out[:length])


# Was: Führt den Arbeitsschritt `mac` für MAC-Funkzugriffssteuerung aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def mac(key: bytes, nonce: bytes, context: bytes, ciphertext: bytes) -> bytes:
    inner = hashlib.sha256(
        b"netcore-kmf-lab-mac-inner-v1" + key + nonce + context + ciphertext
    ).digest()
    return hashlib.sha256(b"netcore-kmf-lab-mac-outer-v1" + key + inner).digest()


# Was: Führt den Arbeitsschritt `derive` für derive aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def derive(node_secret: bytes, node_id: str) -> bytes:
    return hashlib.sha256(
        b"netcore-kmf-node-transport-v1" + node_secret + node_id.encode()
    ).digest()


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> int:
    if len(sys.argv) != 3:
        print("usage: lab_edge_unwrap.py BOOTSTRAP_JSON CLAIMED_ACTION_JSON", file=sys.stderr)
        return 2
    bootstrap = json.loads(Path(sys.argv[1]).read_text())
    action = json.loads(Path(sys.argv[2]).read_text())
    node_secret = bytes.fromhex(bootstrap["transport_secret_hex"])
    key = derive(node_secret, bootstrap["node_id"])
    envelope = action["envelope"]
    nonce = bytes.fromhex(envelope["nonce_hex"])
    ciphertext = bytes.fromhex(envelope["ciphertext_hex"])
    supplied = bytes.fromhex(envelope["mac_hex"])
    context = action["envelope_context"].encode()
    expected = mac(key, nonce, context, ciphertext)
    if not hmac.compare_digest(supplied, expected):
        raise SystemExit("MAC verification failed")
    plaintext = bytes(a ^ b for a, b in zip(ciphertext, stream(key, nonce, context, len(ciphertext))))
    print(json.dumps({"key_fingerprint": hashlib.sha256(plaintext).hexdigest()[:16], "key_bytes": len(plaintext)}))
    return 0


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    raise SystemExit(main())
