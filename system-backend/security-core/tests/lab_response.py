#!/usr/bin/env python3
# NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Sicherheitsrichtlinien und Authentifizierungsabläufe.
# NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

"""Berechnet ausschließlich für den Open-Lab-Provider eine Testantwort."""
import argparse
import hashlib
import hmac
from pathlib import Path


# Was: Führt den Arbeitsschritt `u32` für u32 aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def u32(value: int) -> bytes:
    return value.to_bytes(4, "big")


# Was: Führt den Arbeitsschritt `part` für part aus.
# Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
def part(value: bytes) -> bytes:
    return u32(len(value)) + value


# Was: Startet das Programm, lädt die benötigten Einstellungen und übergibt an den eigentlichen Dienstablauf.
# Warum: Ein klarer Einstiegspunkt hält Startreihenfolge, Fehlerausgabe und geordnetes Beenden zusammen.
def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--seed", type=Path, required=True)
    parser.add_argument("--issi", type=int, required=True)
    parser.add_argument("--node", required=True)
    parser.add_argument("--context", required=True)
    parser.add_argument("--challenge", required=True, help="hex")
    parser.add_argument("--bytes", type=int, default=16)
    args = parser.parse_args()
    seed = args.seed.read_bytes()
    subscriber = hmac.new(
        seed,
        b"netcore-security-core/lab-subscriber/v1" + u32(args.issi),
        hashlib.sha256,
    ).digest()
    challenge = bytes.fromhex(args.challenge)
    payload = (
        b"netcore-security-core/lab-response/v1"
        + u32(args.issi)
        + part(args.node.encode())
        + part(args.context.encode())
        + part(challenge)
    )
    print(hmac.new(subscriber, payload, hashlib.sha256).digest()[: args.bytes].hex())


# Was: Startet den Programmablauf nur dann, wenn diese Datei direkt ausgeführt wird.
# Warum: Beim Import als Modul sollen nur Funktionen bereitstehen und keine Nebenwirkungen automatisch starten.
if __name__ == "__main__":
    main()
