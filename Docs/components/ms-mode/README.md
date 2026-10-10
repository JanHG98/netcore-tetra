# MS-Modus: eigener Radio-Workspace

**Quellstand:** `main`, `c3ccdb4`, 09.10.2026. Quellen: [ms-mode](../../../ms-mode). Dieser importierte Workspace ist getrennt vom NetCore-Basisstations-Workspace im Repository-Root; beide besitzen eine Binary namens `bluestation-bs`.

Alle hier beschriebenen MS-Build- und Testbefehle im Verzeichnis **`ms-mode/`** ausführen. Der Workspace verwendet Rust Edition 2024 und Paketversion `0.5.9`; die Beispielkonfiguration verwendet `config_version = "0.7"`. Der externe Managementvertrag meldet aktuell **`bluestation-ms-interface-6`**.

- [Erste Schritte im MS-Modus](ms-erste-inbetriebnahme.md)
- [MS-Konfiguration](ms-konfiguration.md)
- [MS-Architektur und Funktionsstand](ms-architektur-und-funktionsstand.md)
- [Externe Schnittstelle und Nachrichtenkatalog](examples/ms-interface/README.md)
- [Kommentierte Beispielkonfiguration](../../../ms-mode/example_config/config-ms.toml)

Der SDR-Prozess synchronisiert und registriert sich als TETRA-Teilnehmer; Bedienoberfläche und ACELP-Vocoder werden über externe Control-/Telemetry-/Voice-Peers angebunden. Die konkrete Peer-Software muss den aktuellen Vertrag unterstützen. Die verlinkten MMI-/BlueStation-Projekte sind Herkunfts- und Integrationsreferenzen, keine hier überprüfte Installation.

Vorhandener Code und Softwaretests belegen keine aktuelle Funkabnahme mit deinem SDR, deiner PA oder einem bestimmten Endgerät. Der MS-Workspace ist Forschungs-/Experimentalsoftware. Authentisierung, OTAR, Air-Interface-Verschlüsselung, DMO und SNDCP-Paketdaten sind im beschriebenen MS-Modus nicht implementiert.

## Herkunft

Der Workspace baut auf TETRA BlueStation auf. Die ursprünglichen Danksagungen gelten Harald Welte und osmocom, Tatu Peltola für SoapySDR-Zeitstempel und Viterbi-Arbeit sowie Stichting NLnet/RETETRA3. Weitere historische Hintergrundinformationen stehen in der [BlueStation-Dokumentation](https://github.com/MidnightBlueLabs/tetra-bluestation-docs/wiki).
