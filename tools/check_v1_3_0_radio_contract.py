#!/usr/bin/env python3
"""Static guard for the last-known-good v1.3.0 Sepura common-SCCH contract."""
from pathlib import Path
import sys

ROOT = Path(__file__).resolve().parents[1]
MM = ROOT / "crates/tetra-entities/src/mm/mm_bs.rs"
MAIN = ROOT / "bins/bluestation-bs/src/main.rs"

mm = MM.read_text(encoding="utf-8")
main = MAIN.read_text(encoding="utf-8")

required = {
    "stable tag marker": "Stable v1.3.0 radio behaviour (commit 7834f467)",
    "capability predicate": "class.clch_needed || class.common_scch",
    "frame-18 TS1 value": "Some(0x01u64)",
    "runtime diagnostic": "v1.3.0 radio compatibility assigns frame-18 common-SCCH on TS1",
}

errors: list[str] = []
for name, needle in required.items():
    if needle not in mm:
        errors.append(f"missing {name}: {needle}")

forbidden = [
    "keeping it on the ordinary MCCH instead of assigning frame-18 common-SCCH",
    "scch_capable && granted_esm != EnergySavingMode::StayAlive",
]
for needle in forbidden:
    if needle in mm:
        errors.append(f"forbidden StayAlive SCCH suppression still present: {needle}")

if "Radio profile: v1.3.0 stable common-SCCH contract (commit 7834f467)" not in main:
    errors.append("missing startup radio-profile marker")

if errors:
    print("v1.3.0 radio contract check: FAILED", file=sys.stderr)
    for error in errors:
        print(f"  - {error}", file=sys.stderr)
    raise SystemExit(1)

print("v1.3.0 radio contract check: OK")
print("  clch_needed/common_scch -> Some(0x01) also for StayAlive")
print("  stable baseline commit: 7834f46748f3205ce6d3e6e1480345c6bdf27bca")
