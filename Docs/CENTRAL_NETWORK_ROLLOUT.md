# Zentraler Netzbetrieb auf dem MAIN-COMPAT-Funkstack

Dieser Stand ergänzt den bestehenden Funkstack um zentrale Rufkommandos und eine
begrenzte ACELP-Medienbrücke. Die vorhandene PBX bleibt die Telefonanlage.
SIP-Telefonie nutzt weiterhin den geprüften Codec-Pfad der TBS:

`Funkgerät → native TBS-Bridge → lokaler Asterisk → zentraler SIP-Switch → vorhandene PBX`

Der getrennte Core-Medienpfad transportiert bereits codierte TETRA-Sprachframes:

`TBS ↔ Node Gateway ↔ Media Switch`, gesteuert durch Call Control.

## Änderungen und Grenzen

- SIP-Routen warten nicht mehr auf MQTT oder Audit-Dateien. Nebenwirkungen laufen
  über eine begrenzte Warteschlange; Überläufe und Fehler stehen unter
  `/api/v1/status` in `side_effects`. Rufzustand wird unabhängig davon koalesziert
  gespeichert. Bei einem harten Prozessabbruch kann der letzte noch nicht
  geschriebene Zustand fehlen; dies ist keine transaktionale Abrechnung.
- Maximal acht parallele Routenauflösungen (konfigurierbar), begrenzte HTTP-Worker,
  kurze Abhängigkeits-Timeouts und begrenzte abgeschlossene Rufhistorie.
  Laufende Rufe werden nicht für neue Historieneinträge verdrängt.
- Mobility muss einen registrierten Teilnehmer auf einem verbundenen, nicht
  veralteten Knoten melden. SIP-Kontakte müssen `Avail` sein; `Unregistered`,
  `Unavail` und lediglich vorhandene AoRs zählen nicht als erreichbar.
- Verspätete Zustandsmeldungen können beendete Rufe nicht wieder aktivieren.
- Der lokale Fallback prüft zusätzlich die Routing-Bereitschaft des zentralen
  Switches. Drei Fehlprüfungen lösen den Wechsel aus, 30 stabile Sekunden die
  Rückkehr. Fehlgeschlagene Umschaltungen werden zurückgerollt und nach einem
  Neustart mit Dateizustand und AstDB abgeglichen.
- Im bestätigten Direktbetrieb gehen neue Rufe unmittelbar zur PBX. Beide
  eingehenden Wege werden entsprechend dem aktiven Modus gesperrt/freigegeben.
  Bestehende SIP-Dialoge werden nicht zwischen Wegen verschoben. Eine entfernte
  Registrierung kann bei Netztrennung bis zum Ablauf ihrer TTL sichtbar bleiben.
- Zentrale Rufkommandos verwenden die bestehende lokale CMCE-Zustandsmaschine.
  Ein zentraler Einzelruf benötigt keinen erfundenen lokalen Asterisk-Dialog mehr.
  Annahme und Medienfreigabe erfolgen erst nach U-CONNECT.
- TBS-Medien werden ausschließlich über begrenzte, nicht blockierende Kanäle
  ausgetauscht; pro Tick werden höchstens 16 Downlink-Frames und je acht
  Steuerkommandos pro Control-Endpunkt bearbeitet. Netzwerk-I/O und Transcoding
  liegen außerhalb des Funkthreads.
- Downlink-Frames tragen die Operation-ID des **Zielrufzweigs**. Die TBS nimmt sie
  nur auf einem aktiv zugeordneten Traffic-Slot an. Beim Schließen wird die
  Zuordnung sofort entfernt. Das schützt gegen verspätete Frames nach Slot-Reuse.
- Vollständiges Handover laufender Rufe zwischen Zellen ist **nicht enthalten**.
  Die MAIN-COMPAT-Laufzeit besitzt noch keinen integrierten Import/Export der
  MM/CMCE-Restore-Kontexte. `call_restore_context`, `subscriber_policy` und
  `group_policy` werden deshalb nicht fälschlich als verfügbar angekündigt.
- Der SIP-Switch betreibt weiterhin `edge_media`. Zentrales SIP-Transcoding und
  automatische Migration laufender SIP-Dialoge sind nicht enthalten. Die neue
  Medienbrücke gilt für zentral zugeordnete Rufzweige; lediglich beobachtete lokale
  Rufe erhalten dadurch keine automatische Downlink-Übernahme.

## Reihenfolge und Voraussetzungen

Die folgenden Befehle werden **auf dem jeweils genannten Host** als root ausgeführt.
Das Repository wird hier unter `/opt/netcore-tetra` angenommen. Bestehende TOML-
Dateien werden von den Updates erhalten. Nur Installationsskripte übernehmen bei
fehlenden Dateien die Beispiele. Tatsächliche LXC-Adressen aus der eigenen
Installation verwenden; `10.0.1.XX` in Beispielen ist kein lauffähiger Endpunkt.

Nach dem Merge auf jedem betroffenen Host:

```bash
cd /opt/netcore-tetra
git switch mqtt
git pull --ff-only origin mqtt
```

Für einen gezielten Test vor dem Merge stattdessen den PR-Branch auschecken:

```bash
cd /opt/netcore-tetra
git fetch origin mqtt-feature/central-sip-network
git switch --track origin/mqtt-feature/central-sip-network
```

Vor Backend-/TBS-Neustarts laufende Testgespräche beenden. Zuerst Call Control und
Media Switch aktualisieren, anschließend die TBS. Der neue Zielrufbezug erfordert
den passenden Media-Switch- und TBS-Stand. Node Gateway und Mobility Core brauchen
für dieses Paket keinen eigenen Code-Update, müssen aber verbunden und erreichbar sein.

## LXC: Call Control (8120)

Neuinstallation:

```bash
cd /opt/netcore-tetra
bash system-backend/call-control/install/install.sh
```

Update:

```bash
cd /opt/netcore-tetra
bash system-backend/call-control/install/update.sh
systemctl status netcore-call-control --no-pager
```

In `/etc/netcore/call-control.toml` müssen `node_gateway.url` und
`mobility_core.base_url` auf die vorhandenen Dienste zeigen. Das Update baut vor
jedem Neustart, erhält die Konfiguration und bewahrt das vorherige Binary als
`/usr/local/bin/netcore-call-control.previous` auf.

## LXC: Media Switch (8130)

Neuinstallation:

```bash
cd /opt/netcore-tetra
bash system-backend/media-switch/install/install.sh
```

Update:

```bash
cd /opt/netcore-tetra
bash system-backend/media-switch/install/update.sh
systemctl status netcore-media-switch --no-pager
```

In `/etc/netcore/media-switch.toml` müssen `node_gateway.url` sowie
`call_control.url`, `events_url` und `route_ready_url` auf die tatsächlichen Dienste
zeigen. Das Update baut vor dem Neustart und bewahrt das vorherige Binary als
`/usr/local/bin/netcore-media-switch.previous` auf.

## LXC: zentraler SIP-Switch (8300 / SIP 5060)

Neuinstallation:

```bash
cd /opt/netcore-tetra
bash system-backend/sip-switch/install/install.sh
```

Danach `/etc/netcore/sip-switch.toml` konfigurieren: Mobility-Core-Adresse,
PBX-Trunk zur vorhandenen PBX `10.0.1.21`, dessen Zugangsdaten und pro TBS einen
aktivierten `[[tbs]]`-Eintrag. `node_id` muss exakt der Gateway-/Mobility-Kennung
entsprechen; `username` und Passwort müssen mit dem lokalen TBS-Asterisk stimmen.
Kein vorhandenes PBX- oder TBS-Passwort wird automatisch übernommen.

Update bzw. Übernahme der bearbeiteten Konfiguration:

```bash
cd /opt/netcore-tetra
bash system-backend/sip-switch/install/update.sh
curl -f http://127.0.0.1:8300/health/ready
asterisk -rx 'pjsip show registrations'
asterisk -rx 'pjsip show contacts'
```

Bei Bindung an eine konkrete LXC-IP diese statt `127.0.0.1` verwenden.
Das Update lädt Asterisk neu, statt ihn hart neu zu starten. Änderungen an
SIP-Transport-Bindings können dennoch einen geplanten Asterisk-Neustart erfordern.
Neue optionale Werte unter `[management]`: `route_workers = 8`,
`side_effect_queue_size = 512`, `call_history_limit = 2000`.
Bestehende Konfigurationen verwenden diese Defaults ohne Migration.

## TBS / lokaler Asterisk: Fallback und Funkbinary

Neuinstallation des lokalen Asterisk/Fallbacks:

```bash
cd /opt/netcore-tetra
bash system-backend/sip-switch/install/install-tbs-local-fallback.sh
```

In `/etc/netcore/tbs-sip-fallback.toml` den zentralen SIP-Endpunkt und den
PBX-Ersatztrunk zur PBX `10.0.1.21` konfigurieren. Unter `[central]` bei abweichender
Management-Adresse setzen:

```toml
health_url = "http://SIP_SWITCH_IP:8300/health/ready"
health_timeout_secs = 1
```

Ohne `health_url` wird `http://<central.host>:8300/health/ready` verwendet.
Ein ausdrücklich leerer Wert deaktiviert nur die HTTP-Prüfung, beispielsweise
bei einem fremden SIP-Peer ohne NetCore-Managementdienst.

Update:

```bash
cd /opt/netcore-tetra
bash system-backend/sip-switch/install/update-tbs-local-fallback.sh
bash system-backend/sip-switch/install/tbs-fallback-status.sh
```

Der Controller wird beim Update neu gestartet; Asterisk erhält einen Reload.
Die generierte native `[asterisk]`-Sektion bei Erstinstallation gezielt über das
vorhandene Apply-Skript in die tatsächlich verwendete TBS-Konfiguration übernehmen:

```bash
bash system-backend/sip-switch/install/apply-tbs-local-asterisk-config.sh --config /opt/netcore-tetra/config.toml
```

Das Skript erhält andere TBS-Einstellungen und legt eine Sicherung an. Bei einem
anderen produktiven Konfigurationspfad diesen verwenden.

Für die zentrale Ruf-/ACELP-Anbindung muss `[control_room]` in der TBS-Konfiguration
aktiviert und mit dem Node Gateway verbunden sein. Anschließend das Funkbinary bauen:

```bash
cd /opt/netcore-tetra
cargo build --release -p bluestation-bs
```

Die manuell laufende Basisstation erst nach erfolgreichem Build beenden und mit
ihrem bisherigen Konfigurationspfad starten, beispielsweise:

```bash
./target/release/bluestation-bs ./config.toml
```

## Prüfung nach dem Update

1. Node Gateway zeigt die TBS verbunden; Mobility zeigt ISSI 5102 auf dem richtigen
   Knoten als `confirmed`, `registered=true`, `node_connected=true`, `node_stale=false`.
2. Zentraler SIP-Switch `/health/ready` liefert HTTP 200, der lokale Fallback zeigt
   `CENTRAL_ACTIVE`, und nur die zentrale TBS-Registrierung ist geladen.
3. Einen SIP-Ruf in jede Richtung prüfen und danach parallelen Gruppenverkehr
   erzeugen. Die bestehende lokale Sprachstrecke bleibt der Vergleichspfad.
4. Einen zentralen Ruf über Call Control aufbauen, annehmen und beenden; bei einem
   Einzelruf muss der Zustand erst nach der Radio-Annahme aktiv werden.
5. In einem geplanten Test die zentrale Routing-Erreichbarkeit unterbrechen:
   Fallback muss nach drei Fehlprüfungen auf direkten PBX-Betrieb wechseln und erst
   nach 30 stabilen Sekunden zurückkehren. Ein Ausfall von MQTT allein darf diesen
   Wechsel nicht auslösen.

Automatische Tests prüfen blockiertes MQTT, begrenzte Routenbearbeitung,
veraltete Mobility-Ziele, terminale Rufzustände, Fallback-Rollback/Hysterese sowie
Rufannahme ohne lokalen SIP-Dialog und die Medienbindung bei Slot-Wiederverwendung.
Funkverhalten und Audioqualität dieser neuen zentralen Anbindung müssen auf der
realen TBS geprüft werden; CI ersetzt diesen Nachweis nicht.
