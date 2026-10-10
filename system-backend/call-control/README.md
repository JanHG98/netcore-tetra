# Call Control

Call Control ist der zentrale, eigenständig deploybare Dienst für netzweite logische Gruppen- und Individualrufe. Die TBS behalten weiterhin die zeitkritischen CMCE-, Funkkanal- und Floor-Prozeduren; Call Control koordiniert die lokalen Call Legs über mehrere Zellen hinweg.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/call-control/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/call-control/README.md).

Standardport `8120`; benötigt Node Gateway und für reguläres Individualrufrouting Mobility Core. Der Media Switch bestätigt die Sprachroute vor Operator-Floor-Freigaben.
