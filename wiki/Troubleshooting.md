# Fehlersuche

## Vorgehensweise

1. Fehlerzeitpunkt und betroffene ISSI/GSSI notieren.
2. Dienstzustand und ersten Fehler im Journal prüfen.
3. Konfiguration gegen Fallback und letzte Sicherung vergleichen.
4. RF-, Protokoll- und Integrationsfehler getrennt betrachten.
5. Nur eine Variable gleichzeitig ändern.
6. Nach einem Fix den vollständigen Ablauf einschließlich Release erneut testen.

## Dienst startet nicht

```bash
sudo systemctl status tetra.service --no-pager
sudo journalctl -u tetra.service -b -n 300 --no-pager
/usr/local/bin/bluestation-bs /etc/netcore/config.toml
```

Typische Ursachen:

- TOML-Fehler oder falsche `config_version`
- SDR nicht gefunden
- Sample-Rate nicht unterstützt
- Dual-Carrier-Passband ungültig
- Port bereits belegt
- fehlende native Bibliothek
- Verzeichnis nicht beschreibbar

## Dashboard nicht erreichbar

```bash
ss -ltnp | grep -E ':8080|bluestation'
curl -I http://127.0.0.1:8080/
```

Bind-Adresse, Port, Firewall und Auth-Konfiguration prüfen. Nach UI-Updates Browser hart neu laden.

## Funkgerät registriert nicht

- MCC/MNC, Frequenz, Colour Code und Location Area prüfen.
- Downlink-Signal und Uplink-Empfang getrennt betrachten.
- Gerätelogs auf Location Update prüfen.
- Allowlist/Recovery-Regeln kontrollieren.
- Zeitbasis und SDR-Buffer beobachten.

## Gruppenruf ohne Sprache

- Affiliation vorhanden?
- Traffic-Carrier/Timeslot zugewiesen?
- Floor Grant erhalten?
- Uplink-Sprachrahmen sichtbar?
- Audio-Codec oder SDR-Pfad gestört?
- Hangtime-Retake eines älteren Geräts?

## Call bleibt hängen

- U-DISCONNECT/D-RELEASE im Log suchen.
- `ul_inactivity_secs` prüfen.
- AudioPlayer-Release-Guard abwarten.
- Carrier-/Timeslot-Anzeige mit dem tatsächlichen Scheduler vergleichen.
- anschließend gezielt Gerät re-registrieren, nicht sofort alle Teilnehmer kicken.

## Directory-Namen fehlen

```bash
curl -fsS http://<DIRECTORY-IP>:8095/api/health
sudo journalctl -u tetra.service -n 300 --no-pager | grep -i directory
```

Base-URL, Timeout und Firewall prüfen. Numerische IDs sollten trotzdem sichtbar bleiben.

## Audio/TTS gestört

Siehe [[Audio-Zentrale]]. Besonders `ffmpeg`, Piper-Endpunkt, Cache-Rechte, NFS-Mount und Ruf-Release prüfen.

## Nach Änderung schlimmer als vorher

- Basisstation stoppen.
- Primärkonfiguration sichern.
- bekannten Stand oder Fallback manuell testen.
- Clean-Build ausführen.
- erst danach wieder automatisch starten.

## Backend ist erreichbar, aber fachlich nicht bereit

Für den betroffenen Host zuerst **Liveness**, dann **Readiness**, dann den konkreten API-Vertrag und schließlich den Nutzungsfall prüfen. Eine `503` auf `/health/ready` kann eine ausgefallene Abhängigkeit anzeigen, obwohl der Prozess läuft. Die echte URL steht in `inventory.toml` und der gerenderten Dienst-TOML; [[Dienstkatalog]] und [[Netzwerk-und-Ports]].

```bash
systemctl status netcore-node-gateway.service --no-pager
journalctl -u netcore-node-gateway.service -b -n 200 --no-pager
curl -sS -w '\nHTTP %{http_code}\n' http://<GATEWAY-IP>:8080/health/ready
```

Widersprechen sich TBS-`[control_room].host` und Inventory-Gateway, zuerst die **tatsächlich installierten** Dateien angleichen. Control Room `9010` ist nicht das TBS-Ziel `/ws/node`.

## SDS kommt nicht in MQTT oder HA an

Mit **einer Testnachricht und Zeitstempel** TBS-RX → lokale SDS-Verarbeitung → Node Gateway/SDS Router → IoT Gateway → Broker-Topic → HA-Trigger prüfen. System-ISSI `4010001` kann eine lokale Steueradresse sein. Nur „MQTT verbunden“ oder ein vorhandenes Discovery-Entity reicht als Nachweis nicht. [[MQTT-und-Home-Assistant]]

## SIP klingelt, Audio fehlt oder Ruf hängt

PJSIP-Registrierung, erreichbaren Kontakt, Ruf-Setup/U-CONNECT, RTP beider Richtungen und TETRA-Codec-Pfad getrennt prüfen. Lokalen TBS-Asterisk und zentralen SIP Switch nicht auf denselben Host installieren; Fallback-Meldung und aktive Route lesen. Bei Rufende Floor, Hangtime und Release auf TBS und Asterisk vergleichen. [[SIP-und-Brew]] · [[Calls]]

## Packet Data/WAP scheitert

PDP/NSAPI im Packet Core, TUN und Lease im IP Gateway, Route/NAT/Firewall/DNS und Endgeräte-URL getrennt prüfen. TBS-WAP-Port `9200`, Testserver `8088` und Management `8170` sind verschiedene Rollen. [[Paketdaten-und-WAP]]

## Belegpaket für eine reproduzierbare Meldung

Commit/Tag und lokale Änderungen; Unit und geladene TOML ohne Geheimnisse; Fehlerminute mit Zeitzone; betroffene ISSI/GSSI, Carrier/Slot; erstes auffälliges Log jeder beteiligten Komponente; erwartetes und beobachtetes Ergebnis. [[Abnahme]]
