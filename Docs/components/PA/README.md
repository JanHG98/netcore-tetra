# Little-PA-V2: Hardware-Referenz

Quellen: [Schaltplan, Platine und Fertigungsdateien](../../../PA). Dokumentationsprüfung: `main` (`c3ccdb4`, 09.10.2026). Dies beschreibt die abgelegte Hardware-Revision; daraus folgt keine neue elektrische Prüfung, HF-Abnahme oder Leistungsfreigabe.

Die Revision ergänzt die Little-PA-Platine um eine thermisch isolierte TCXO-Insel, größere Vias, einen Bandpassfilter im TX-Pfad und einen Bypass für Betrieb ohne PA. Die Bauteilbezeichnungen gelten ausschließlich für diese Revision und müssen mit Schaltplan und tatsächlicher Bestückung übereinstimmen.

| Bestückungsvariante | Vorgabe der abgelegten Revision |
| --- | --- |
| mit PA | R29 und R32 unbestückt |
| ohne PA / Bypass | U4 und C28 unbestückt; R29 und R32 mit 0-Ohm-Brücken bestückt |

Die ursprüngliche Platinenreferenz stammt von Z32IT: [Little PA V2 SX1255 HAT](https://www.pcbway.com/project/shareproject/Little_PA_V2_SX1255_HAT_2c8ffceb.html). Das ist eine Herkunftsreferenz; vor Fertigung oder Umbau die aktuellen eigenen Dateien, Stückliste, Frequenzbereich und Bestückung prüfen.
