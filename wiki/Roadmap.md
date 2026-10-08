# Ausbauziele und offene Entscheidungen

**Aktuelle Gesamtfolge / nächster Schritt:** [Zentrale Roadmap NETCORE-MASTER-01](https://github.com/JanHG98/netcore-tetra/blob/main/ROADMAP.md). Sie führt die Prioritäten und Abnahmebedingungen; diese Seite ergänzt die Vorhaben und offenen Entscheidungen. Für neue Status- / Fortsetzungsfragen zuerst die zentrale Roadmap mit dem aktuellen Repositorystand abgleichen.

**Z01-Fortschritt 07.10.2026:** Z01.1 abgeschlossen; Z01.2/01.3 auf `feature/z01-deployment-consolidation` implementiert und lokal geprüft. [Quellvergleich, Pakete, Tests und Rückwege](../Docs/integration/Z01-2026-10-07/README.md). Nach PR-/CI-Prüfung folgt Z01.4; keine Betreiberinstallation durch diesen Auftrag.

Hier stehen Vorhaben, die aus Projektplanung und Gesprächen hervorgehen. **Ein Ziel in dieser Liste ist keine zugesicherte Funktion des aktuellen Builds.** Für vorhandene Dienste und Tests siehe [[Projektstand]] und [[Dienstkatalog]].

| Vorhaben | Geplanter Nutzen | Vor einer Zusage zu klären |
|---|---|---|
| Imagebuilder auf zentraler VM | pro TBS provisioniertes Pi-Image zum erneuten Flashen | reproduzierbare Releases, Secret-Injektion, Rollback, Signatur, Versionierung |
| Netzweite Auto-Discovery plus manueller Suchknopf | gegenseitiges Finden aller Dienste im selben VLAN | Identität, Authentisierung, Konflikte, TTL, bewusstes Netzgebiet |
| Wizard „Neue TBS“ | Name, MCC/MNC, ISSI, LA, CC → Konfig, VPN/Keys, Image | sichere Schlüsselvergabe, eindeutige IDs, Betrieb ohne zentrale Verbindung |
| Durchgängiger Mehrzellenbetrieb | Serving-TBS, Kontextwechsel und laufende Rufe | MM/CMCE-Restore, Timing, Media Switch, Endgeräte-Interoperabilität |
| GPIO-/Rack-Platine | Sensorik, Lüfter, Statusanzeigen, Watchdog, Stromversorgung | echte HAT-Pinbelegung, EMV, 230-V-Sicherheit, Footprints und PCB-Abnahme |
| Control-Room-Arbeitsplatz | Audio, skalierbare Oberfläche, Rollen, NFC/AD | Sicherheitsmodell, Operator-Identität, Ruf-Autorität und E2E-Tests |
| HA-/Homematic-Aktionspfad | gezielte, quittierte Aktionen aus Funkereignissen | Topic-Vertrag, Default-Deny-Policy, Automation, Rückmeldung und Fehlerschutz |
| [Zentrale Anmeldung / RBAC (NETCORE-IAM-01)](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md) | Gemeinsamer Login, Gruppen und Dienstrollen für das NetCore-Ökosystem einschließlich Drive; Ressourcenrechte im jeweiligen Backend | Identity-Produkt / Pilot, optionale AD-Anbindung, Gast- und Dienstidentitäten, Sperrfristen und Ausfallverhalten; weiterhin geplant |
| [NetCore Drive (NETCORE-DRIVE-01)](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/NETCORE_DRIVE_ROADMAP.md) | Eigene Dateicloud: Core Console, Workspace, Archive Studio; Nextcloud-ähnlicher Datei-Funktionsumfang und OneDrive-einfache Bedienung; externe Ordnerfreigaben; lokaler Start mit Konten / Gruppen und vollständigen Ressourcenrechten ohne zentrales RBAC | Designrichtung bestätigt; Backend offen, Nextcloud Files + PostgreSQL empfohlen; NAS / Dateispeicher ohne MariaDB-Pflicht; D0–D5 unabhängig von IAM, zentrale Migration in D6 mit Freigabeerhalt und geprüftem Ausfallverhalten; Gastverifikation und echte Sync-Tests |
| [Drive-Plugins und Browser-Dateinutzung](https://github.com/JanHG98/netcore-tetra/blob/main/Docs/NETCORE_DRIVE_ROADMAP.md#5-plugins-und-dateinutzung-im-browser) | PDF, Word / Excel / Präsentationen, 3D, Schaltpläne, Vektorgrafiken, Bilder, Video und Audio im Browser; erweiterbare Viewer, Player und geeignete Editoren | Geplant in D7–D9 ohne zentrale IAM-Abhängigkeit; Plugin-Vertrag / Verwaltung, unterstützte Formate, Darstellung / Rückexport, Isolation und identische Dateirechte abnehmen |
| [Einseitiges TETRA-Empfangsgateway (Z11.1)](../ROADMAP.md#z111-einseitiges-tetra-empfangsgateway) | Empfangene Gruppengespräche mit zugeordneter Sprecherkennung in NetCore-Receive-only-Gruppen einspeisen; langfristiger P2-Backlog ohne Sprachrückweg | Decoder / NETSYMS-Anbindung, Audio- und Rufdatenkorrelation, MCC-/MNC-/ISSI-/GSSI-Mapping, Brew-Medienformat, netzseitige Sendesperre sowie Motorola-/Sepura-Endgeräteabnahme; offen / geplant, ohne Terminbindung |


## Reihenfolge für eine belastbare Erweiterung

1. Problem und Systemgrenze auf [[Architecture]] und [[Netzwerk-und-Ports]] einzeichnen.
2. Vertrag, Konfigurationsschema, Backward Compatibility und Failure Mode festlegen.
3. Statische Tests, mutierende Labortests und bei Funkfunktionen On-Air-Test ergänzen.
4. Betreiberverfahren für Installation, Sicherung, Rückweg, Monitoring und Fehlersuche dokumentieren.
5. Erst mit Messdaten die Funktion von „geplant“ nach [[Projektstand]] übernehmen.

