# NetCore SDS Router

Der SDS Router ist der zentrale SwMI-Dienst für **SDS und pre-coded Status**. Er nimmt verlustfrei normalisierte SDS-Edge-Ereignisse der TBS entgegen, entscheidet über Individual-, Gruppen- und Anwendungsziele und beauftragt die zuständigen TBS über den Node Gateway mit der Air-Interface-Zustellung.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/sds-router/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/sds-router/README.md).

Standardport `8150`; nutzt den Node Gateway für TBS-Zustellung. Persistente Idempotenz und ein optionaler einmaliger Zustellversuch ergänzen normale Retry-Queues.
