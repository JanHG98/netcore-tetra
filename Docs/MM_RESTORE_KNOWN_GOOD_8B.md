# Restore known-good MM implementation from 8b00cbc7

## Evidence

The live TBS log from `v1.3.0-8b00cbc7` contains repeated successful CMCE `U-SETUP`
from ISSI 5102 to GSSI 15201, followed by `D-CALL PROCEEDING`, `D-CONNECT` and traffic
circuit allocation. The current `03b6b180` log contains no `U-SETUP` at all; PTT remains
trapped below CMCE while the terminal emits location updates.

Between `8b00cbc7` and `03b6b180`, the only runtime source file changed is
`crates/tetra-entities/src/mm/mm_bs.rs`. The later experiments changed several air-interface
variables simultaneously (accept type handling, optional SSI/ASSI field, energy-saving IE,
and affiliation replay), which made attribution unreliable.

## Decision

Restore `mm_bs.rs` exactly from `8b00cbc7`, the last live build proven to pass PTT into
CMCE. Keep every other current `mqtt` file and backend change intact. This creates a clean
known-good radio baseline before changing one MM variable at a time.
