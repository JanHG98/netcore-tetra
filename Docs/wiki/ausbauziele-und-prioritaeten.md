# Ausbauziele und offene Entscheidungen

**Aktuelle Gesamtfolge / nächster Schritt:** [Zentrale Roadmap NETCORE-MASTER-01](../roadmaps/gesamtroadmap.md). Sie führt die Prioritäten und Abnahmebedingungen; diese Seite ergänzt die Vorhaben und offenen Entscheidungen. Für neue Status- / Fortsetzungsfragen zuerst die zentrale Roadmap mit dem aktuellen Repositorystand abgleichen.

**Z01-Quellenstand 09.10.2026:** Z01.1–Z01.3 sind auf `main` integriert; Z01.4 bündelt die verbleibenden Anlagenprüfungen. Imagebuilder, Discovery und VPN-Heimnetzregel sind im Code vorhanden. Die [datierte Integration](../integration/Z01-2026-10-07/README.md) enthält Historie und Nachweise; aktuelle Prioritäten und Betreiberbefunde werden ausschließlich in der zentralen Roadmap fortgeführt.

Hier stehen Vorhaben, die aus Projektplanung und Gesprächen hervorgehen. **Ein Ziel in dieser Liste ist keine zugesicherte Funktion des aktuellen Builds.** Für vorhandene Dienste und Tests siehe [Projektstand und Nachweisgrenzen](projektstand-und-nachweise.md) und [Dienstkatalog](dienstkatalog.md).

| Vorhaben / heutiger Stand | Nutzen | Offene Abnahme oder Ausbaufrage |
|---|---|---|
| Imagebuilder auf Ubuntu-VM: Code vorhanden | personalisiertes Pi-Image mit ARM64-NetCore, SDR-Treiber und optionalem VPN | tatsächlichen Download mit Manifest/SHA-256 sowie physischen Boot, SXceiver und VPN-Wechsel abnehmen; Release-Signatur bleibt ein eigenes Ausbauziel |
| Discovery mit Suchknopf: Code vorhanden | Multicast im gemeinsamen Layer-2-Netz, Unicast-Seeds über geroutete Netze | Peer-Zuordnung, Konflikte, Lease und Wiederkehr am Standort prüfen; Open-Lab-Discovery ist nicht authentisiert |
| TBS-Profil und Imageformular: Code vorhanden | Standort-TOML, Netz-/Zellkennungen, Benutzer/SSH, WLAN und optionales OpenVPN im Image | echte Profilwerte, Versionsnachweis und Hardwarestart prüfen; allgemeines Web-IAM und normative TETRA-Schlüsselvergabe sind eigene Vorhaben |
| Durchgängiger Mehrzellenbetrieb | Serving-TBS, Kontextwechsel und laufende Rufe | MM/CMCE-Restore, Timing, Media Switch, Endgeräte-Interoperabilität |
| GPIO-/Rack-Platine | Sensorik, Lüfter, Statusanzeigen, Watchdog, Stromversorgung | echte HAT-Pinbelegung, EMV, 230-V-Sicherheit, Footprints und PCB-Abnahme |
| Control-Room-Arbeitsplatz | Audio, skalierbare Oberfläche, Rollen, NFC/AD | Sicherheitsmodell, Operator-Identität, Ruf-Autorität und E2E-Tests |
| HA-/Homematic-Aktionspfad | gezielte, quittierte Aktionen aus Funkereignissen | Topic-Vertrag, Default-Deny-Policy, Automation, Rückmeldung und Fehlerschutz |
| [Zentrale Anmeldung / RBAC (NETCORE-IAM-01)](../roadmaps/zentrale-anmeldung-und-berechtigungen.md) | Gemeinsamer Login, Gruppen und Dienstrollen für das NetCore-Ökosystem einschließlich Drive; Ressourcenrechte im jeweiligen Backend | Identity-Produkt / Pilot, optionale AD-Anbindung, Gast- und Dienstidentitäten, Sperrfristen und Ausfallverhalten; weiterhin geplant |
| [NetCore Drive (NETCORE-DRIVE-01)](../roadmaps/dateicloud-und-ordnerfreigaben.md) | Eigene Dateicloud: Core Console, Workspace, Archive Studio; Nextcloud-ähnlicher Datei-Funktionsumfang und OneDrive-einfache Bedienung; externe Ordnerfreigaben; lokaler Start mit Konten / Gruppen und vollständigen Ressourcenrechten ohne zentrales RBAC | Designrichtung bestätigt; Backend offen, Nextcloud Files + PostgreSQL empfohlen; NAS / Dateispeicher ohne MariaDB-Pflicht; D0–D5 unabhängig von IAM, zentrale Migration in D6 mit Freigabeerhalt und geprüftem Ausfallverhalten; Gastverifikation und echte Sync-Tests |
| [Drive-Plugins und Browser-Dateinutzung](../roadmaps/dateicloud-und-ordnerfreigaben.md#5-plugins-und-dateinutzung-im-browser) | PDF, Word / Excel / Präsentationen, 3D, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio im Browser; erweiterbare Viewer, Player und geeignete Editoren | Geplant in D7–D9 ohne zentrale IAM-Abhängigkeit; Plugin-Vertrag / Verwaltung, unterstützte Formate, Darstellung / Rückexport, Isolation und identische Dateirechte abnehmen |
| [Einseitiges TETRA-Empfangsgateway (Z11.1)](../roadmaps/gesamtroadmap.md#z111-einseitiges-tetra-empfangsgateway) | Empfangene Gruppengespräche mit zugeordneter Sprecherkennung in NetCore-Receive-only-Gruppen einspeisen; langfristiger P2-Backlog ohne Sprachrückweg | Decoder / NETSYMS-Anbindung, Audio- und Rufdatenkorrelation, MCC-/MNC-/ISSI-/GSSI-Mapping, Brew-Medienformat, netzseitige Sendesperre sowie Motorola-/Sepura-Endgeräteabnahme; offen / geplant, ohne Terminbindung |


## Reihenfolge für eine belastbare Erweiterung

1. Problem und Systemgrenze auf [Architektur und Datenwege](architektur-und-datenwege.md) und [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) einzeichnen.
2. Vertrag, Konfigurationsschema, Backward Compatibility und Failure Mode festlegen.
3. Statische Tests, mutierende Labortests und bei Funkfunktionen On-Air-Test ergänzen.
4. Betreiberverfahren für Installation, Sicherung, Rückweg, Monitoring und Fehlersuche dokumentieren.
5. Status mit seiner Nachweisstufe dokumentieren: geplant, Code vorhanden, statisch geprüft, am Standort geprüft oder On Air abgenommen. Fortschritt mit Aufgaben-ID in der [zentralen Roadmap](../roadmaps/gesamtroadmap.md) pflegen.

## Quellen zur Pflege dieser Seite

[Zentrale Aufgaben und Nachweise](../roadmaps/gesamtroadmap.md) · [Deployment-Core-Umfang](../../system-backend/deployment-core/main.py).
