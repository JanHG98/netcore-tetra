# NetCore Recorder

Der Recorder ist ein eigenständiger LXC-Dienst für die passive, unveränderte Ablage der vom Tap übernommenen TETRA-Sprachframes. Er hängt ausschließlich am replay-fähigen Recorder-Tap des Media Switch und liegt damit außerhalb des zeitkritischen Rufpfads.

Lokale Dateien: [src](src/) · [config](config/) · [install](install/) · [systemd](systemd/).

[Vollständige Dokumentation](../../Docs/services/recorder/README.md) · [Zentraler Dokumentationsindex](../../Docs/README.md)

[Online lesen](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/services/recorder/README.md).

Standardport `8140`; liest den Vollframe-Tap des Media Switch. Die Speicherablage bewahrt angenommene Frames unverändert; Tap-Lücken bleiben möglich und werden ausgewiesen.
