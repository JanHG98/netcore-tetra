# Basisstationen im Directory beschreiben

Der Directory-Bereich „Basisstationen“ hält Metadaten zu einzelnen RF-Standorten oder NetCore-Nodes.

## Felder

| Feld | Bedeutung |
|---|---|
| `issi` | System-/Basisstations-ISSI |
| `name` | vollständiger Standortname |
| `short` | Kurzbezeichnung |
| `location` | Standortbeschreibung |
| `mcc` / `mnc` | Netzkennung |
| `color` | Darstellungsfarbe |
| `visible` | Sichtbarkeit |
| `notes` | interne Hinweise |

## Abgrenzung

Der Directory-Eintrag konfiguriert nicht automatisch RF-Parameter. Carrier, Duplex, Colour Code und Location Area stehen weiterhin in der lokalen `config.toml` der jeweiligen Basisstation.

## Benennung

Eine praktikable Struktur ist:

| Feld | Beispiel |
|---|---|
| Name | Hannover – Rack 01 |
| Kurzname | H-R01 |
| Standort | Technikraum Nord |

Hostnamen können zusätzlich in `node_id` oder `station_name` der Control-Room-Konfiguration geführt werden. Directory-Name und Node-ID sollten stabil und eindeutig sein.

## Weiterführend

[Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) · [Konfiguration der TBS](basisstation-konfigurieren.md) · [Mehrzellenbetrieb, Mobility und Edge-Fallback](mehrzellenbetrieb-und-ausfallverhalten.md)

## Quellen zur Pflege dieser Seite

[Directory-Standortschema](../../system-backend/directory/netcore-directory.py) · [TBS-Node-Metadaten](../../crates/tetra-config/src/bluestation/sec_control_room.rs).
