# Security Core – Authentisierung und Sicherheitsrichtlinien

Der Security Core verwaltet Sicherheitsprofile je ISSI, Security Classes 1/2/3, Challenge/Response, kurzlebige DCK-Kontexte sowie Teilnehmer- und Gerätesperren. Die eigene Weboberfläche und REST-API verwenden standardmäßig TCP **8180**.

**Stand: 9. Oktober 2026.** Abgeglichen mit [Konfiguration](../../../system-backend/security-core/config/security-core.example.toml), [API](../../../system-backend/security-core/src/http.rs) und [Zustandsverwaltung](../../../system-backend/security-core/src/state.rs). Dies beschreibt den vorhandenen Lab-Code, keine produktive oder On-Air-Abnahme.

## Vorhandener Umfang

- persistente globale und teilnehmerspezifische Sicherheitsrichtlinien;
- Class-Aushandlung; Class 3 verlangt Authentisierung und anschließenden DCK-Workflow;
- Authentisierung mit TTL, begrenzten Antwortversuchen und Lockout;
- nodebezogene Challenge-, DCK-, Sperr- und Widerrufsaktionen;
- Alarme, begrenzte Audit-Historie, Export und Metadatenbackup;
- Beobachtung des Node Gateways und davon abhängige Readiness;
- Neustartbehandlung, die offene Authentisierungen beendet und vorhandene DCK-Kontexte widerruft.

## Lab-Grenze

`lab_hmac_sha256` ist der einzige implementierte Authentisierungsprovider. Er erzeugt reproduzierbare Lab-Prüfwerte und implementiert keine TETRA-TA-Algorithmen. Auch die vorhandene [KMF](../kmf/README.md) bleibt ein Lab-Lifecycle-Dienst für CCK/GCK/SCK; sie ersetzt den Security-Core-Provider derzeit nicht.

Managementantworten enthalten Metadaten und Fingerprints. Der getrennte Edge-Claim kann kurzlebige Challenges und DCK-Rohmaterial liefern. Beide API-Bereiche teilen denselben ungeschützten HTTP-Listener: **keine Anmeldung, Tokens oder TLS**. Die Beispielkonfiguration bindet an `0.0.0.0:8180`; das Netz muss deshalb isoliert sein.

## Start und Betrieb

Aus dem Repository-Hauptverzeichnis, auf einem vorbereiteten Lab-Host mit Rust/Cargo:

```bash
sudo system-backend/security-core/install/install.sh
systemctl status netcore-security-core
source /etc/netcore/lxc-network.env
curl --fail "${NETCORE_WEBUI_URL}health/live"
curl --fail "${NETCORE_WEBUI_URL}health/ready"
```

Der Installer legt Dienstkonto, Datenverzeichnis und Konfiguration an und baut die Release-Binärdatei. [LXC-Installation](lxc-installation.md) beschreibt Pfade und Rechte.

`shadow` berechnet und protokolliert die Abläufe, gibt aber keine Edge-Aktionen per Claim frei. `authoritative` erlaubt Claims. Bei aktiviertem `node_gateway.observe_nodes` meldet `/health/ready` im authoritative-Modus **503**, solange das Gateway nicht verbunden ist; die Readiness ist keine Funkabnahme.

## Weiterführende Anleitungen

- [Architektur und Zuständigkeiten](architektur-und-zustaendigkeiten.md)
- [Authentisierungsablauf](authentisierungsablauf.md)
- [Edge-API und Quittierungen](edge-api-und-quittierungen.md)
- [Lab-Provider und Geheimnisse](lab-provider-und-geheimnisse.md)
- [Offene Testumgebung](offene-testumgebung.md)
- [LXC-Installation](lxc-installation.md)
- [API-Beispiele](tests/api-beispiele-im-labor.md)
