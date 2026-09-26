# Ausbauziele und offene Entscheidungen

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

## Reihenfolge für eine belastbare Erweiterung

1. Problem und Systemgrenze auf [[Architecture]] und [[Netzwerk-und-Ports]] einzeichnen.
2. Vertrag, Konfigurationsschema, Backward Compatibility und Failure Mode festlegen.
3. Statische Tests, mutierende Labortests und bei Funkfunktionen On-Air-Test ergänzen.
4. Betreiberverfahren für Installation, Sicherung, Rückweg, Monitoring und Fehlersuche dokumentieren.
5. Erst mit Messdaten die Funktion von „geplant“ nach [[Projektstand]] übernehmen.
