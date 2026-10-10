# Generierte Prüfdaten

Diese Dateien werden aus dem jeweiligen Arbeitsbaum erzeugt. Sie sind eine statische Quellenbestandsaufnahme; Konfiguration, Installation und tatsächlicher Funkbetrieb werden getrennt geprüft.

## Protokollinventur

Vom Repository-Root aus:

```bash
python3 tools/protocol_inventory.py
python3 tools/protocol_inventory.py --check
```

Der Generator erzeugt die lesbaren Berichte unter [Protokolle](../protocols/README.md) und hier `protocol_inventory.json`, `pdu_inventory.csv`, `sap_inventory.csv`, `gap_inventory.csv` und `state_inventory.csv`. Die JSON-/CSV-Dateien bilden die ausgewerteten Rust-Quellen und Verweise ab; vor einer Verwendung nach Codeänderungen regenerieren.

## Deployment- und Systemaudit

```bash
python3 tools/check_full_system_integration.py
```

[Vollständiger Integrationsaudit](systemintegration-pruefbericht.md) prüft Registry, Inventar, URLs, Abhängigkeiten und Fallback-Verträge. Der Checker schreibt den Bericht neu; ein PASS bestätigt diese Quell-/Konfigurationsverträge und keine erreichbare Gesamtflotte. Die CI-Verknüpfung steht in [deployment-consistency.yml](../../.github/workflows/deployment-consistency.yml).
