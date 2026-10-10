# NetCore Media Switch

Der Media Switch ist der zentrale, eigenständig deploybare LXC-Dienst für den Transport bereits codierter TETRA-Sprachframes zwischen mehreren TBS-Call-Legs.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/media-switch/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/media-switch/README.md).

Standardport `8130`; benötigt Node Gateway und Call Control. Verteilt codierte Sprachframes und stellt einen begrenzten Recorder-Replayring bereit.
