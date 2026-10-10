# Glossar

| Begriff | Bedeutung im Projekt |
|---|---|
| TBS | TETRA-Basisstation am Funkstandort, insbesondere `bluestation-bs` |
| SwMI | Switching and Management Infrastructure; Netzlogik jenseits der einzelnen Zelle |
| AI | Air Interface, die TETRA-Luftschnittstelle |
| MCC/MNC | Netzkennung; zusammen mit Standort-/Zellparametern für Registrierung relevant |
| LA/CC | Location Area und Colour Code; keine Teilnehmerkennung |
| ISSI/GSSI | individuelle Teilnehmer- und Gruppenkennung; [ISSI und GSSI](teilnehmer-und-gruppenkennungen.md) |
| SDS/U-STATUS | Kurzdaten- bzw. numerische Statusmeldung; [SDS und U-STATUS](kurznachrichten-und-status.md) |
| LIP | Location Information Protocol für Positionsmeldungen; [Positionen über LIP und GPS anzeigen](positionen-lip-und-gps.md) |
| CMCE | Call Management/Control Entity, etwa Setup/Floor/Release |
| SNDCP/PDP/NSAPI | Funktionen und Identitäten des Paketdatenpfads; [Paketdaten, IP Gateway und WAP](paketdaten-und-wap.md) |
| Node Gateway | TBS-/Backend-Verbindung, Service-Matrix und netzweite Ereignisse |
| Directory | Namen und Metadaten, keine Funkzulassung; [Namen und Metadaten im NetCore Directory](namen-und-metadaten-im-directory.md) |
| Control Room | Leitstellenansicht und Operatorsteuerung, nicht der Node Gateway |
| Edge-Fallback | lokaler Betrieb der TBS bei teilweise oder ganz fehlendem Backend |
| Open Lab | ausdrücklich isolierter Testmodus; WebUIs teils ohne Auth/TLS |
| Liveness/Readiness | Prozess lebt / fachliche Voraussetzungen sind vorhanden |
| SIP/RTP | Rufsignalisierung / Audiotransport der Telefoniekopplung |
| Brew | optionales TETRA-Gatewayprotokoll zu einem Brew-Peer; [SIP, lokaler Asterisk und Brew](telefonie-sip-und-brew.md) |
| Deployment Core | Controller für Discovery, Agentenaufträge und Imageverwaltung auf der Deployment-VM |
| Discovery-Agent | meldet installierte Rollen, löst Dienstendpunkte auf und führt freigegebene Controlleraufträge aus |
| Imagebuilder | eigener lokaler Dienst für ARM64-Pi-Images; Build, Download und Hardwareboot sind getrennte Nachweise |
| Lease | begrenzte Gültigkeit einer Service-Matrix oder eines Discovery-Peers |

Für technische Werte sind [Konfiguration der TBS](basisstation-konfigurieren.md), [Netzwerk, Ports und Protokolle](netzwerk-und-ports.md) und die jeweilige installierte TOML maßgeblich.

## Quellen zur Pflege dieser Seite

[Dienstrollen](../../system-backend/services.toml) · [Lokal genutzte Funkentitäten](../../bins/bluestation-bs/src/main.rs).
