# Glossar

| Begriff | Bedeutung im Projekt |
|---|---|
| TBS | TETRA-Basisstation am Funkstandort, insbesondere `bluestation-bs` |
| SwMI | Switching and Management Infrastructure; Netzlogik jenseits der einzelnen Zelle |
| AI | Air Interface, die TETRA-Luftschnittstelle |
| MCC/MNC | Netzkennung; zusammen mit Standort-/Zellparametern für Registrierung relevant |
| LA/CC | Location Area und Colour Code; keine Teilnehmerkennung |
| ISSI/GSSI | individuelle Teilnehmer- und Gruppenkennung; [[ISSI-and-GSSI]] |
| SDS/U-STATUS | Kurzdaten- bzw. numerische Statusmeldung; [[SDS-and-U-STATUS]] |
| LIP | Location Information Protocol für Positionsmeldungen; [[LIP-and-GPS]] |
| CMCE | Call Management/Control Entity, etwa Setup/Floor/Release |
| SNDCP/PDP/NSAPI | Funktionen und Identitäten des Paketdatenpfads; [[Paketdaten-und-WAP]] |
| Node Gateway | TBS-/Backend-Verbindung, Service-Matrix und netzweite Ereignisse |
| Directory | Namen und Metadaten, keine Funkzulassung; [[NetCore-Directory]] |
| Control Room | Leitstellenansicht und Operatorsteuerung, nicht der Node Gateway |
| Edge-Fallback | lokaler Betrieb der TBS bei teilweise oder ganz fehlendem Backend |
| Open Lab | ausdrücklich isolierter Testmodus; WebUIs teils ohne Auth/TLS |
| Liveness/Readiness | Prozess lebt / fachliche Voraussetzungen sind vorhanden |
| SIP/RTP | Rufsignalisierung / Audiotransport der Telefoniekopplung |
| Brew | optionales TETRA-Gatewayprotokoll zu einem Brew-Peer; [[SIP-und-Brew]] |

Für technische Werte sind [[Configuration]], [[Netzwerk-und-Ports]] und die jeweilige installierte TOML maßgeblich.
