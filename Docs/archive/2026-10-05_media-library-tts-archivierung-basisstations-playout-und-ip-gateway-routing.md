# Brainstorming: Media Library, zentrale TTS, Archivierung und Basisstations-Playout

**Archivhinweis, 09.10.2026:** Diese Datei dokumentiert das im Titel beziehungsweise Quellenrahmen genannte Gespräch und dessen damalige Entscheidungen. Historische Kommandos, Planungen, Versionsangaben und Fehlerbilder bleiben erhalten; der heutige technische Stand steht in der [Gesamtroadmap](../roadmaps/gesamtroadmap.md) und in den [aktuellen Dienstanleitungen](../services/README.md). Die Archivaufnahme bestätigt keine spätere Implementierung oder Live-Abnahme.

## 1. Rahmen

| Feld | Wert |
|---|---|
| Projekt | NetCore-Tetra |
| Thema | Media Library, Recorder-Import, zentrale Piper/TTS-Erzeugung, NFS-/SMB-Archivierung, TTS-Archivkategorien, Aussendung über Media Switch bzw. Basisstation sowie Diagnose des IP-Gateways |
| Notizstand | 2026-10-05 |
| Zielbranch für die Archivierung | `Archiving` |
| Geprüfter Repository-Ausgangsstand | `Archiving` @ `8539085862651887ee1b560a9893c943501e0f22` |
| Historischer Entwicklungszweig | überwiegend `swmi`; einzelne ZIP-/Patchstände wurden separat bereitgestellt, sind aber als Arbeitsartefakte nicht automatisch mit einem Git-Commit gleichzusetzen |
| Repository | `JanHG98/netcore-tetra` |

### Arbeitsgrundlage und zeitlicher Bezug

Grundlage sind Laufzeit-Ausgaben, Screenshots, der Textdump mit Status, Jobs, Codec-Konfiguration, Assets und Media-Switch-Sessions sowie der am **2026-10-05** geprüfte Repository-Stand. Ältere ZIP-/Patchstände wurden nicht erneut byteweise geprüft. Ausgelieferte Arbeitsstände, eingecheckter Code und reale Betriebsnachweise bleiben deshalb getrennt.

Die Laufzeitlogs tragen überwiegend Zeitstempel vom **2026-07-30**. Sie belegen den damaligen Systemzustand; daraus folgt keine erneute Ausführung am Dokumentationsdatum **2026-10-05**.

### Statusbegriffe in diesem Dokument

- **Idee** – diskutierter oder vorgeschlagener Ansatz ohne Festlegung.
- **Beschlossen/geplant** – als Zielbild oder nächster Schritt festgelegt, aber nicht zwingend umgesetzt.
- **Implementiert** – im aktuell geprüften Repository-Code bzw. in zugehörigen Repository-Dateien vorhanden.
- **Getestet** – durch konkrete historische Tests/Kommandos oder durch erkennbare Repository-Tests belegt; Repository-Tests wurden bei der Bestandsaufnahme nicht automatisch ausgeführt.
- **Im Betrieb bestätigt** – durch Laufzeit-Ausgaben auf den realen NetCore-Systemen belegt.

---

## 2. Ziel, Ausgangslage und behandelte Themen

Die Media Library soll drei zusammenhängende Aufgaben zentral übernehmen:

1. **Aufzeichnungen aus der Basisstation zentralisieren**, ohne die lokale Ausfallsicherheit zu verlieren.
2. **TTS/Piper aus der Basisstation herauslösen** und vollständig in die Media Library verlagern.
3. **Freigegebene Medien und TTS tatsächlich über TETRA aussenden**, ohne Live-Streaming über NFS/HTTP und ohne einen zweiten, unnötigen zentralen TETRA-Codec zu erzwingen.

Parallel wurde ein Betriebsproblem am **IP-Gateway** diagnostiziert. Der Dienst selbst lief, wurde vom Node-Gateway jedoch als ausgefallen/Fallback angezeigt. Die Ursache war letztlich keine HTTP- oder Packet-Core-Anwendungsstörung, sondern eine persistente, über die IP-Gateway-WebUI angelegte Route, die das komplette Managementnetz `10.0.1.0/24` fälschlich über `ntc-tun0` schickte. Dadurch versuchte das IP-Gateway den Packet Core `10.0.1.166:8160` mit Quelladresse `10.0.0.1` über den TETRA-TUN zu erreichen.

Gemeinsames Ziel ist die Nutzung zentraler Dienste mit **lokalen Fallbacks und klaren Netz-/Datenpfaden**. Für die Media Library ist diese Trennung im geprüften Stand weitgehend umgesetzt. Beim IP-Gateway fehlt weiterhin ein Schutz gegen Routen, die das Managementnetz in den TETRA-TUN umbiegen.

---

## 3. Endgültige Anforderungen und Entscheidungen

### 3.1 Gemeinsame Archivstruktur

**Beschlossen/geplant und am Prüfstand 05.10.2026 im Repository implementiert:** Die Media Library verwendet drei physisch getrennte Archivwurzeln:

```text
/mnt/nfs-share/Media-Library
/mnt/nfs-share/Recordings
/mnt/nfs-share/TTS-Dateien
```

Die Zuordnung ist:

```text
kind = recording  -> /mnt/nfs-share/Recordings
kind = tts        -> /mnt/nfs-share/TTS-Dateien
sonstige Medien   -> /mnt/nfs-share/Media-Library
```

Darunter wird nach `YYYY/MM/DD` strukturiert. Die Dateinamen sollen aus Metadaten lesbar abgeleitet werden, statt UUID-Verzeichnisse als Bedienoberfläche zu verwenden. Für Recordings werden u. a. Rufart, GSSI/ISSI, Quelle, Zeit und Dauer in den Dateinamen eingearbeitet; TTS erhält einen `TTS`-Typmarker und den Titel.

**Spätere Korrektur mit Vorrang:** TTS-Dateien dürfen **nicht** unter `Recordings` einsortiert werden. Im frühen Arbeitsstand lagen TTS-Dateien fälschlich im Recording-Baum. Dieser Stand ist überholt. `Docs/MEDIA_LIBRARY_TTS_ARCHIVE_ROUTING_FIX.md` und `migrate-archive-layout.py` im aktuellen Repository erzwingen die getrennten Wurzeln.

### 3.2 Rechte auf dem NFS-/SMB-Archiv

Im Open-Lab wurde bewusst auf einfache parallele NFS-/SMB-Nutzung gesetzt. Die finale Repository-Umsetzung ist differenzierter als ein früher Zwischenstand:

- lokaler Media-Library-Zustand bleibt restriktiv mit `UMask=0077`;
- gemeinsame Archivverzeichnisse werden explizit auf `0777` gesetzt;
- gemeinsame Archivdateien werden explizit auf `0666` gesetzt;
- systemd erlaubt Schreibzugriff gezielt auf `/var/lib/netcore-media-library` sowie die drei Archivwurzeln.

**Überholt:** Ein früher Änderungsstand sprach von `UMask=0000` für die Media-Library-Unit. Der aktuell geprüfte Stand nutzt wieder `UMask=0077` und öffnet ausschließlich die freigegebenen Archivbäume per `chmod`. Dieser geprüfte Stand hat Vorrang.

### 3.3 Recorder-Import: Basisstation bleibt zunächst Quelle

**Beschlossen und implementiert:** Eine fertige Basisstationsaufnahme wird lokal zuverlässig abgeschlossen. Danach meldet die Basisstation die Aufnahme der Media Library. Die Media Library zieht die WAV über HTTP von einer schmalen Export-Route der TBS, verarbeitet sie und archiviert sie zentral. `source + source_reference` dienen als Idempotenzschlüssel; ein lokaler Marker verhindert Doppelimporte.

Zielbild:

```text
TBS Recorder
  -> lokale WAV + JSON
  -> POST import-url an Media Library
  -> Media Library GET auf TBS-Recording-Export
  -> Verarbeitung / Preview
  -> Archiv /mnt/nfs-share/Recordings/YYYY/MM/DD
```

Wichtig ist die Richtung: Die Basisstation schreibt nicht blind direkt auf das NFS-Archiv. Fällt die Media Library aus, bleibt die lokale Aufnahme erhalten und kann später erneut übertragen werden.

### 3.4 Zentrale TTS/Piper-Zuständigkeit

**Beschlossen und im Repository implementiert:** Piper und TTS-Verwaltung sollen ausschließlich im Media-Library-LXC laufen. Die Basisstation soll keine TTS-Dateien mehr lokal synthetisieren.

Finaler Soll-Datenfluss:

```text
Operator
  -> Media-Library-WebUI
  -> Piper HTTP auf 127.0.0.1:5005
  -> WAV-Asset kind=tts
  -> Preview / Freigabe
  -> /mnt/nfs-share/TTS-Dateien/YYYY/MM/DD
  -> TBS lädt vollständige Preview in lokalen Cache
  -> lokale Funkaufbereitung
```

Der aktuelle Installer verwaltet standardmäßig diese Piper-Stimmen:

```text
de_DE-thorsten-medium
de_DE-thorsten-high
de_DE-karlsson-low
de_DE-pavoque-low
de_DE-thorsten_emotional-medium
```

**Spätere Korrektur mit Vorrang:** Ein Installationslauf wurde versehentlich auf der TBS statt im Media-Library-LXC ausgeführt. Das danach beobachtete `cargo: command not found` war deshalb keine belastbare Media-Library-LXC-Diagnose. Nach Erkennen des Bedienfehlers sollte der lokale Piper-Dienst auf der TBS wieder deaktiviert/entfernt werden. Ob diese Bereinigung anschließend tatsächlich durchgeführt wurde, wurde nicht bestätigt.

### 3.5 Kein Live-Streaming während der Funkaussendung

**Beschlossen:** Vor dem Rufaufbau muss die komplette Audiodatei lokal auf der Basisstation verfügbar und vorbereitet sein. Während der Aussendung wird weder von NFS noch aus der Media Library oder direkt von Piper gestreamt.

Begründung:

- der Funkruf soll nicht von schwankender HTTP-/NFS-Latenz abhängen;
- lokale Basisstationsfunktionen bleiben die letzte Instanz für Codec und RF-Zeitverhalten;
- zentrale Dienste dürfen ausfallen, ohne einen bereits lokal gestarteten Audiopfad mitten im Ruf zu zerlegen.

### 3.6 Aussendearchitektur: Basisstation statt zentralem WAV->TACELP-Zwang

Das frühe Media-Library-Aussendeformular war praktisch nur für einen bestehenden Media-Switch-Ruf geeignet. Im `shadow`-Modus wurde ein Job lediglich protokolliert. Im `authoritative`-Modus hätte der alte Pfad einen vorhandenen `audio.tacelp`-Cache und eine existierende Media-Switch-Session benötigt.

**Endgültige Entscheidung und geprüfter Repository-Stand:** Für normale WAV-/TTS-Aussendungen wird der neue Modus `basisstation` verwendet. Die Media Library delegiert an den lokalen TBS-AudioPlayer. Die Basisstation übernimmt Download, Cache, nativen TETRA-Codec, Rufaufbau, Aussendung und Rufende.

Der alte direkte Media-Switch-Pfad bleibt als Legacy-/Spezialpfad `media_switch` erhalten.

Damit gilt:

```text
Basisstationsmodus:
ready + approved + preview.wav
  -> ausgewählte TBS
  -> /api/audio/play
  -> lokaler Codec und Rufaufbau

Direkter Media-Switch-Modus:
broadcast_ready + audio.tacelp + vorhandene session_id
  -> /api/v1/sessions/<id>/inject
```

### 3.7 IP-Gateway: Managementnetz darf niemals in den TETRA-TUN umgebogen werden

**Im Betrieb diagnostiziert, aber als Code-Schutz noch offen:** Das IP-Gateway muss verhindern, dass das eigene Managementnetz, die eigene Managementadresse oder die Packet-Core-Adresse über `ntc-tun0` geroutet werden. Der konkrete Fehler war:

```text
10.0.1.0/24 dev ntc-tun0
```

Dadurch ergab:

```text
ip route get 10.0.1.166
10.0.1.166 dev ntc-tun0 src 10.0.0.1
```

statt der korrekten Managementroute über `eth0` mit Quelle `10.0.1.142`.

Diese Schutzregel ist **noch nicht** im aktuell geprüften Repository implementiert und ist daher ein P0-Roadmap-Kandidat.

---

## 4. Architektur, Komponenten, Schnittstellen und Abhängigkeiten

### 4.1 Media Library

**Dienst:** `netcore-media-library.service`\
**Historische Live-Konfiguration:** `/etc/netcore/media-library.toml`\
**Repository-Beispiel:** `system-backend/media-library/config/media-library.example.toml`\
**Zustand:** `/var/lib/netcore-media-library/state.json`\
**Asset-Root:** `/var/lib/netcore-media-library/assets`\
**Temp:** `/var/lib/netcore-media-library/tmp`\
**Backups:** `/var/lib/netcore-media-library/backups`\
**Historischer Web/API-Port:** `8230`\
**Live-Bind im gezeigten System:** `10.0.1.154:8230`

Wichtige API-Bereiche:

```text
GET  /api/v1/status
GET  /api/v1/assets
GET  /api/v1/assets/<asset-id>
GET  /api/v1/assets/<asset-id>/preview
POST /api/v1/assets/import-url
POST /api/v1/dispatch
GET  /api/v1/jobs
POST /api/v1/jobs/<job-id>/cancel
POST /api/v1/jobs/<job-id>/retry

GET  /api/v1/tts/status
GET  /api/v1/tts/voices
GET  /api/v1/tts/templates
POST /api/v1/tts/templates/save
POST /api/v1/tts/templates/delete
POST /api/v1/tts/generate
```

### 4.2 Piper/TTS

**Soll-Host:** ausschließlich Media-Library-LXC\
**Dienst:** `netcore-piper.service`\
**Venv:** `/opt/netcore-piper`\
**Voice-Root:** `/var/lib/netcore-media-library/piper`\
**TTS-Cache:** `/var/lib/netcore-media-library/tts/cache`\
**Vorlagen:** `/var/lib/netcore-media-library/tts/templates`\
**Endpoint:** `http://127.0.0.1:5005`

Der Installer prüft `piper`, `piper.http_server` und `piper.download_voices`, lädt fehlende Stimmen nach, schreibt die Unit für den Media-Library-Serviceaccount und wartet derzeit bis zu 30 Sekunden auf `/voices`.

### 4.3 Basisstation / AudioPlayer

Für den finalen Playout-Modus benötigt die Media Library eine oder mehrere TBS-Zieldefinitionen:

```toml
[playout]
mode = "basisstation"
default_station = "srv-m-tbs-01"
request_timeout_secs = 15
completion_timeout_secs = 900
poll_interval_ms = 500

[[playout.stations]]
id = "srv-m-tbs-01"
name = "SRV-M-TBS-01"
base_url = "http://<TBS>:8080"
enabled = true
```

Optional kann die Station Benutzername/Passwort für den vorhandenen Cookie-Login der TBS erhalten. Zugangsdaten gehören **nicht** in diese Archivdokumentation.

Der Worker verwendet aktuell:

```text
GET  <TBS>/api/audio/status
POST <TBS>/api/audio/play
POST <TBS>/api/audio/stop
POST <TBS>/api/login       (nur falls Station-Login konfiguriert)
```

Das `POST /api/audio/play` erhält u. a.:

```json
{
  "source_type": "media",
  "source_id": "media-library",
  "path": "<asset-id>",
  "target_type": "group|individual",
  "target_id": 15201,
  "priority": 4
}
```

Die TBS liefert eine `job_id`; die Media Library pollt `/api/audio/status` und spiegelt `sent_blocks`, `total_blocks`, Fehler, Abschluss und Abbruch in den eigenen Dispatch-Job.

### 4.4 Media Switch

**Live-Abhängigkeit im gezeigten Media-Library-System:** `http://10.0.1.159:8130`

Der Legacy-Modus `media_switch` verwendet:

```text
POST /api/v1/sessions/<session-id>/inject
```

und benötigt gepackte TETRA-Sprachframes mit:

```text
frame_bytes = 35
frame_interval_ms = 60
```

Die Sessionabfrage `GET /api/v1/sessions` ergab zum Diagnosezeitpunkt:

```json
[]
```

Damit gab es zu diesem Zeitpunkt keine existierende Session, in die der alte Job hätte einspeisen können.

### 4.5 Recorder und Application Gateway

Live im gezeigten Media-Library-System:

```text
Recorder:            http://10.0.1.170:8140
Application Gateway: http://10.0.1.144:8220
```

Der Media-Library-Status meldete beide als verbunden.

### 4.6 IP-Gateway / Packet Core

Im gezeigten Betrieb:

```text
IP-Gateway Management-IP: 10.0.1.142/24
IP-Gateway API/WebUI:      10.0.1.142:8170
TUN:                       ntc-tun0
TUN-Adresse:               10.0.0.1/24
Packet Core:               10.0.1.166:8160
DNS auf Packet-Data-Seite: 10.0.0.1:53
HTTP-Testserver:            0.0.0.0:8088
UDP-Echo:                   0.0.0.0:7007
```

Die gezeigte IP-Gateway-Konfiguration war `authoritative`. Das TUN wurde geöffnet. DNS meldete kurz nach dem Start zunächst `Cannot assign requested address`, lief wenige Sekunden später aber korrekt auf `10.0.0.1:53`; diese Startwarnung war daher nicht die Ursache des Ausfalls.

---

## 5. Erreichter Entwicklungs- und Betriebsstand

### 5.1 Im Repository implementiert

Der am 2026-10-05 geprüfte Branch `Archiving` enthält bereits die finalen Codepfade dieser Planung:

- zentrale TTS/Piper-Integration in der Media Library;
- getrennte Archivwurzeln für `Media-Library`, `Recordings` und `TTS-Dateien`;
- Archivmigration einschließlich physischer Umsortierung alter TTS-Assets;
- Media-Library-Import von Basisstationsrecordings;
- `[media_library]`-Unterstützung und Update-/Parser-Checks auf der TBS;
- `playout.mode = "basisstation"` und `playout.mode = "media_switch"`;
- mehrere konfigurierbare Basisstationsziele;
- TBS-Cookie-Login im Media-Library-Worker;
- Remote-Job-Tracking und Abbruch;
- Diagnoseskript `diagnose-basisstation-playout.py`;
- Cargo-Pfadbehandlung im TBS-Update für Rustup-Installationen außerhalb von `sudo secure_path`.

### 5.2 Im Betrieb bestätigt

Aus den historischen Live-Ausgaben sind folgende Punkte tatsächlich belegt:

- `netcore-ip-gateway.service` lief `active (running)` und lauschte auf `10.0.1.142:8170`.
- Der IP-Gateway-DNS-Dienst lauschte nach kurzer Anlaufphase auf `10.0.0.1:53`.
- Packet Core antwortete **lokal auf dem Packet-Core-LXC** auf `http://10.0.1.166:8160/health/live` mit HTTP 200.
- Vom IP-Gateway-LXC aus timeouteten Verbindungen nach `10.0.1.166:8160`.
- `ip route get 10.0.1.166` zeigte die falsche Route über `ntc-tun0`.
- Die IP-Gateway-API enthielt persistente Routen für `192.168.50.0/24` und `10.0.1.0/24`, beide auf `ntc-tun0`.
- Die Media Library lief zunächst im `shadow`-Modus mit allen abhängigen Diensten als verbunden.
- Die Media Library enthielt 38 Assets; alle 38 waren `ready`, alle 38 `approved`, alle 38 `preview_ready`, aber **0** waren `broadcast_ready`.
- Der alte Aussendeversuch für GSSI `15201` wurde als Job `shadowed` beendet, `frame_index=0`, `frame_count=0`, `queued_targets=0`.
- Nach Änderung von `operating_mode = "authoritative"` meldete die Media Library den autoritativen Modus erfolgreich.
- Die Media-Switch-Sessionliste war zu diesem Zeitpunkt leer.

### 5.3 Nicht im Betrieb bestätigt

Folgendes ist **nicht** durch einen anschließenden Live-Nachweis bestätigt:

- erfolgreiche Entfernung der fehlerhaften IP-Gateway-Route aus dem persistenten State und anschließende grüne Readiness;
- erfolgreiche Verbindung IP-Gateway -> Packet Core nach der Routenbereinigung;
- Rückkehr der Node-Gateway-Karte von `AUSGEFALLEN/FALLBACK` zu `VERFÜGBAR`;
- End-to-End-Aussendung eines Media-Library-TTS über den finalen `basisstation`-Playoutpfad;
- Installation/Start von Piper auf dem **richtigen** Media-Library-LXC nach dem versehentlichen TBS-Lauf;
- Entfernung des versehentlich installierten Piper-Dienstes auf der TBS;
- physische Umsortierung aller bestehenden TTS-Archive im realen NFS-Baum nach `TTS-Dateien`.

---

## 6. Relevante Dateien und geprüfter Repository-Abgleich

### 6.1 Media Library / TTS / Playout

Aktuell relevante Repository-Dateien:

```text
CHANGES-CENTRAL-MEDIA-LIBRARY-TTS.md
CHANGES-MEDIA-LIBRARY-INTEGRATION.md
CHANGES-MEDIA-LIBRARY-BASISSTATION-PLAYOUT.md
Docs/MEDIA_LIBRARY_BASISSTATION_INTEGRATION.md
Docs/MEDIA_LIBRARY_BASISSTATION_PLAYOUT.md
Docs/MEDIA_LIBRARY_CENTRAL_TTS.md
Docs/MEDIA_LIBRARY_TTS_ARCHIVE_ROUTING_FIX.md
system-backend/media-library/config/media-library.example.toml
system-backend/media-library/src/config.rs
system-backend/media-library/src/http.rs
system-backend/media-library/src/state.rs
system-backend/media-library/src/worker.rs
system-backend/media-library/src/tts.rs
system-backend/media-library/install/update.sh
system-backend/media-library/install/ensure-piper.sh
system-backend/media-library/install/shared-storage.sh
system-backend/media-library/install/migrate-archive-layout.py
system-backend/media-library/install/migrate-playout-config.py
system-backend/media-library/install/diagnose-basisstation-playout.py
system-backend/media-library/systemd/netcore-media-library.service
install/update-basisstation.sh
install/remove-local-tts-config.py
```

Wichtige am Prüfstand 05.10.2026 verifizierte Codeaussagen:

- `config.rs` definiert `BASISSTATION_PLAYOUT_MODE = "basisstation"` und `MEDIA_SWITCH_PLAYOUT_MODE = "media_switch"`.
- `basisstation` ist der Default-Playoutmodus; eine Station muss aber konfiguriert sein, bevor sie als Default verwendet werden kann.
- `state.rs` verlangt im autoritativen Basisstationsmodus ein `ready` + `approved` Asset mit existierender WAV-Preview; `broadcast_ready` und `tetra_path` sind dabei **nicht** erforderlich.
- Nur der direkte `media_switch`-Modus verlangt `broadcast_ready = true` und einen realen TACELP-Pfad.
- `worker.rs` delegiert im Basisstationsmodus an `/api/audio/status`, `/api/audio/play` und `/api/audio/stop` der ausgewählten TBS.
- Der alte Media-Switch-Pfad bleibt mit `/api/v1/sessions/<session>/inject` erhalten.
- `migrate-archive-layout.py` mappt `recording -> recording_archive_root`, `tts -> tts_archive_root`, sonstige Medien -> `archive_root` und verschiebt bereits archivierte Dateien auf die richtige Wurzel.
- `netcore-media-library.service` nutzt `User/Group=netcore-media-library`, `UMask=0077`, `ProtectSystem=strict` und explizite `ReadWritePaths` für die drei Archivwurzeln.

### 6.2 IP-Gateway

Relevante aktuelle Dateien:

```text
system-backend/ip-gateway/config/ip-gateway.example.toml
system-backend/ip-gateway/src/http.rs
system-backend/ip-gateway/src/state.rs
system-backend/ip-gateway/src/kernel.rs
system-backend/ip-gateway/src/packet_core.rs
system-backend/ip-gateway/src/runtime.rs
system-backend/ip-gateway/web-ui/index.html
```

**Wichtiger geprüfter Befund:** Der Repository-Code reproduziert die strukturelle Ursache des Vorfalls weiterhin:

- `state.rs::validate_route()` validiert CIDR, Gateway-IP und Interface-Syntax, prüft aber **keine Überlappung mit dem Managementnetz** und schützt weder die eigene Bind-Adresse noch die Packet-Core-Adresse.
- `kernel.rs` führt für jede aktivierte persistente Route `ip route replace <destination> ...` aus.
- Die WebUI schlägt beim Route-Dialog als Beispiel `192.168.50.0/24` und als optionales Interface standardmäßig `ntc-tun0` vor.
- Damit kann eine über die WebUI angelegte Route wie `10.0.1.0/24 dev ntc-tun0` den direkt verbundenen Managementpfad verdrängen und wird durch den Reconcile-Prozess wiederhergestellt.

Dieser Fehler ist **am Prüfstand 05.10.2026 noch offen** und sollte nicht als bereits behoben archiviert werden.

---

## 7. Wichtige Befehle und Abläufe mit Ausführungsstatus

### 7.1 Media-Library-Status prüfen – ausgeführt und erfolgreich

```bash
ML="http://$(hostname -I | awk '{print $1}'):8230"

curl -fsS "$ML/api/v1/status" | python3 -m json.tool
curl -fsS "$ML/api/v1/jobs?limit=10" | python3 -m json.tool
```

Ergebnis vor Moduswechsel:

```text
operating_mode: shadow
ready: true
media_switch_connected: true
recorder_connected: true
application_gateway_connected: true
assets_total: 38
assets_ready: 38
assets_approved: 38
preview_ready: 38
broadcast_ready: 0
```

### 7.2 Codec-Konfiguration prüfen – ausgeführt

```bash
grep -n -A25 '^\[codec\]' /etc/netcore/media-library.toml
```

Relevantes Ergebnis:

```toml
frame_bytes = 35
encoder_command = []
decoder_command = []
```

Damit war der direkte Media-Switch-Weg für normale WAV-/TTS-Dateien nicht sendefähig.

### 7.3 Autoritativen Modus aktivieren – ausgeführt und bestätigt

Die Live-Konfiguration wurde auf:

```toml
[runtime]
operating_mode = "authoritative"
```

geändert und der Dienst neu gestartet:

```bash
sudo systemctl restart netcore-media-library.service
```

Der anschließende Status meldete `operating_mode: authoritative` und weiterhin `ready: true`.

### 7.4 Media-Switch-Sessions prüfen – ausgeführt

```bash
MS="$(
  sed -n \
    's/^[[:space:]]*media_switch_base_url[[:space:]]*=[[:space:]]*"\([^"]*\)".*/\1/p' \
    /etc/netcore/media-library.toml
)"

curl -fsS "$MS/api/v1/sessions" | python3 -m json.tool
```

Ergebnis:

```json
[]
```

Damit existierte keine Session, in die der alte Job mit der manuell eingetragenen `session_id = "01"` hätte einspeisen können.

### 7.5 Finaler Basisstations-Playout – implementiert, aber nicht live bestätigt

Vorgesehene Prüfung nach Deployment:

```bash
/opt/netcore-media-library/bin/diagnose-basisstation-playout.py
```

Erwartet werden u. a. erreichbarer TBS-AudioPlayer, Media-Library-Quelle und ggf. erfolgreicher Cookie-Login. Anschließend ist eine echte Testaussendung auf eine GSSI mit parallelem Logstream zu prüfen:

```bash
journalctl -u netcore-media-library.service -f
journalctl -u tetra.service -f
```

### 7.6 IP-Gateway -> Packet Core – ausgeführt, Fehler reproduziert

Vom IP-Gateway:

```bash
curl -sv --connect-timeout 3 --max-time 10 \
  http://10.0.1.166:8160/health/live
```

Ergebnis: TCP-Timeout.

Die Route zeigte:

```bash
ip route get 10.0.1.166
```

```text
10.0.1.166 dev ntc-tun0 src 10.0.0.1
```

Gesamtrouting:

```text
default via 10.0.1.1 dev eth0 proto static
10.0.0.0/24 dev ntc-tun0 proto kernel scope link src 10.0.0.1
10.0.1.0/24 dev ntc-tun0 scope link
192.168.50.0/24 dev ntc-tun0 scope link
```

### 7.7 Persistente IP-Gateway-Routen lesen – ausgeführt

```bash
curl -fsS http://10.0.1.142:8170/api/v1/routes | python3 -m json.tool
```

Gefunden wurden:

```text
84f4a04b-4e12-4c85-bf98-ffbfbcf533c5
  192.168.50.0/24 -> ntc-tun0

dc3e00fb-bf75-4226-826c-3eb04350967e
  10.0.1.0/24 -> ntc-tun0
```

### 7.8 Persistente Fehlroute löschen – vorgeschlagen, nicht bestätigt

Der korrekte permanente Reparaturweg ist die DELETE-API, nicht nur ein manuelles `ip route del`, weil der Reconcile-Prozess persistente Routen sonst erneut setzt:

```bash
curl -i -X DELETE \
  http://10.0.1.142:8170/api/v1/routes/dc3e00fb-bf75-4226-826c-3eb04350967e
```

Für die zweite Test-/Fehlroute analog:

```bash
curl -i -X DELETE \
  http://10.0.1.142:8170/api/v1/routes/84f4a04b-4e12-4c85-bf98-ffbfbcf533c5
```

Danach müssen Route und Readiness geprüft werden. Ein erfolgreicher Abschluss wurde nicht mehr dokumentiert.

---

## 8. Fehler, Diagnose, Ursachen und Lösungen

### 8.1 TTS im falschen Archivbaum

**Symptom:** TTS-WAVs erschienen unter `Recordings`.

**Ursache:** Archiv-/Browserkategorisierung war in einem Zwischenstand nicht streng genug nach Assettyp getrennt.

**Lösung im aktuellen Repository:** eigene `tts_archive_root`, physische Migration alter Archive, Update von `state.json` und Metadaten, virtuelle Baumwurzel `TTS-Dateien`.

**Status:** implementiert; reale Vollmigration im gezeigten NFS-Bestand nicht abschließend per `find`/State-Ausgabe bestätigt.

### 8.2 Media-Library-Update versehentlich auf der TBS ausgeführt

**Symptom:** Piper-Pakete/Stimmen wurden installiert, danach `system-backend/media-library/install/update.sh: line 27: cargo: command not found`.

**Korrektur:** Betriebsrückmeldung: Das Skript lief versehentlich auf der TBS. Damit war die vorherige Cargo-Diagnose für den Media-Library-LXC gegenstandslos.

**Folge:** Lokalen Piper auf der TBS deaktivieren/entfernen und Media-Library-Installer nur im vorgesehenen LXC ausführen.

**Status:** Fehlbedienung erkannt; Cleanup nicht bestätigt.

### 8.3 `cargo` unter `sudo` nicht gefunden

Unabhängig vom obigen Fehlbedienungsfall war im Projekt bereits ein reales Muster bekannt: Rustup-Cargo liegt häufig unter `/home/<user>/.cargo/bin/cargo` und verschwindet aus `sudo secure_path`.

**Geprüfter Repository-Stand:** `install/update-basisstation.sh` ermittelt `SUDO_USER`, dessen Home, `.cargo/bin/cargo`, `CARGO_HOME` und `RUSTUP_HOME` und führt den Build kontrolliert als Build-Benutzer aus. Dieser Fix ist im aktuellen Repository vorhanden.

### 8.4 Vorschau auswählbar, Aussende-Assetliste leer

**Symptom:** Im Media-Library-Screenshot konnte unter „Vorschau“ ein Asset gewählt werden; im Bereich „Asset aussenden“ war das Assetfeld leer.

**Zwischenursache:** Der Aussendedialog filterte strenger als die Vorschau und war für den damaligen Shadow-/TACELP-Pfad ausgelegt.

**Zwischenfix:** Im Shadow-Modus sollten `ready + approved` Assets sichtbar sein, ohne `broadcast_ready` zu verlangen.

**Geprüfte Lösung:** Der aktuelle `basisstation`-Playoutpfad benötigt in `authoritative` nur eine gültige Preview-WAV und delegiert Codec/Rufaufbau an die TBS. `broadcast_ready` bleibt nur für den direkten Media-Switch-Pfad relevant.

### 8.5 Klick auf „Aussendung“ erzeugt keinen Funkruf

**Tatsächlich beobachteter Job:**

```text
job_id: 86bbed2e-6acf-4dfe-a41e-6564560e9726
asset_id: 632aa34b-6007-4cc6-933f-313928833742
session_id: 01
destination_kind: group
destination_id: 15201
priority: 4
state: shadowed
frame_index: 0
frame_count: 0
queued_targets: 0
```

Mehrere Ursachen trafen gleichzeitig zu:

1. Media Library lief noch in `shadow`.
2. `broadcast_ready = 0` für alle 38 Assets.
3. `encoder_command = []`.
4. Kein Asset besaß `tetra_path`.
5. Die Media-Switch-Sessionliste war leer.
6. `session_id = "01"` war lediglich ein manueller Eingabewert, keine belegte reale Session.

**Historischer Schluss:** Der damalige Button war inhaltlich eher „Audio in existierende Session injizieren“ als „Gruppenruf aufbauen und Datei aussenden“.

**Finaler Architekturfix:** Delegation an die Basisstation mit GSSI/ISSI und Priorität. Die TBS baut den echten Ruf auf und verwendet ihren bestehenden nativen Audiopfad.

### 8.6 IP-Gateway meldet Packet Core getrennt, obwohl Packet Core lokal lebt

**Symptom:** Node-Gateway-Karte zeigte `AUSGEFALLEN` + `FALLBACK`; IP-Gateway-Dienst lief. Der lokale Packet-Core-Healthcheck war 200, vom IP-Gateway jedoch Timeout.

**Entscheidende Diagnose:**

```text
ip route get 10.0.1.166
-> dev ntc-tun0 src 10.0.0.1
```

**Ursache:** Persistente WebUI-Route `10.0.1.0/24 -> ntc-tun0` überschreibt die normale direkt verbundene Managementroute. Der IP-Gateway-Reconcile setzt diese Route erneut, weshalb ein rein manueller `ip route replace ... dev eth0` nicht dauerhaft greift.

**Korrekte Reparatur:** Persistente Route per API löschen, Kernelroute bereinigen, Reconcile/Service neu prüfen.

**Codeproblem bleibt:** Die API validiert keine Managementnetz-Überlappung.

---

## 9. Durchgeführte Tests und ihre Grenzen

### Historische Betriebsprüfungen

| Test | Ergebnis | Bewertung |
|---|---|---|
| Packet Core lokal `/health/live` auf `10.0.1.166:8160` | HTTP 200 | Packet-Core-Dienst selbst lebte |
| IP-Gateway -> Packet Core `/health/live`, `/api/v1/status`, `/api/v1/contexts` | Timeout | Netz-/Routingproblem vor HTTP |
| `ip route get 10.0.1.166` | über `ntc-tun0`, Quelle `10.0.0.1` | Ursache eindeutig eingegrenzt |
| IP-Gateway `/api/v1/routes` | zwei persistente TUN-Routen | Reconcile-Ursache bestätigt |
| Media Library `/api/v1/status` | ready, zuerst shadow, später authoritative | Dienst und Moduswechsel bestätigt |
| Media Library Assets | 38 ready/approved/preview, 0 broadcast-ready | Preview-Pfad funktioniert, direkter TACELP-Pfad nicht |
| Media Library Jobs | Testjob `shadowed` | Shadow-Verhalten bestätigt |
| Media Switch `/api/v1/sessions` | `[]` | keine nutzbare direkte Session vorhanden |

### Im Repository vorhanden, aber bei Archivierung nicht ausgeführt

Der geprüfte Code enthält Unit-/Regressionstests u. a. für Dispatch-Modi und die `[media_library]`-Parserregistrierung. Bei dieser Archivierung wurde **kein Cargo-Build, kein CI-Lauf und kein Hardware-/RF-Test** ausgeführt. Das Vorhandensein eines Tests im Repository ist daher nicht mit einem frischen Testerfolg gleichzusetzen.

### Wichtige noch fehlende Tests

1. End-to-End: Media Library -> TBS -> echter Gruppenruf -> Funkgerät hört vollständige TTS.
2. Abbruch während laufendem Basisstations-Playout.
3. sehr kurze TTS, die zwischen POST und erstem Poll bereits endet.
4. TBS beschäftigt / anderer Remote-Job / TBS-Neustart während Playout.
5. Media Library verliert Netzwerk nach lokalem TBS-Cache-Download.
6. Mehrere TBS-Ziele und Default-Station.
7. IP-Gateway-Routen-Schutztests gegen Managementnetz, eigene Bind-IP und Packet-Core-IP.
8. Reconcile nach Route-DELETE sowie Neustartpersistenz.
9. physische Archivmigration inkl. bestehender TTS-Dateien und parallelem SMB-Zugriff.

---

## 10. Verworfene oder ersetzte Ansätze

### 10.1 TTS unter `Recordings`

**Verworfen.** TTS ist eine eigene Archivkategorie und liegt unter `/mnt/nfs-share/TTS-Dateien`.

### 10.2 Lokaler Piper auf jeder Basisstation

**Ersetzt.** Piper ist zentrale Media-Library-Funktion; Basisstationen konsumieren fertige Media-Library-Assets.

### 10.3 Zentrale WAV->TACELP-Konvertierung als Voraussetzung für normale TTS-Aussendung

**Als primärer Weg ersetzt.** Der zentrale Media-Switch-Pfad bleibt optional bestehen, aber normale TTS/WAVs werden an die TBS delegiert. Dadurch wird der bereits vorhandene lokale TETRA-Audiopfad wiederverwendet.

### 10.4 Manuell eingetragene `Media Session ID` als normale Bedienlogik

**Für den Basisstationsmodus ersetzt.** Eine Session-ID ist nur noch für den direkten `media_switch`-Spezialpfad relevant. Im Basisstationsmodus werden Zielart und Ziel-ID an die TBS gegeben.

### 10.5 `shadow` als echte Aussendung behandeln

**Klarstellung:** Shadow ist weiterhin absichtlich nicht sendend. Ein Shadow-Job kann zur Logik-/UI-Prüfung dienen, aber ist kein Funknachweis.

### 10.6 `UMask=0000` für die gesamte Media Library

**Ersetzt durch restriktiveren aktuellen Stand:** `UMask=0077` lokal, explizite offene Rechte nur auf den drei freigegebenen Archivbäumen.

### 10.7 Nur Kernelroute manuell reparieren

**Unzureichend.** Solange die Route im IP-Gateway-State gespeichert ist, setzt der Reconcile-Prozess sie wieder. Permanente Reparatur muss den State/API-Eintrag entfernen.

---

## 11. Offene Aufgaben, Ideen und Roadmap-Kandidaten

### P0 – vor weiterer produktiver Nutzung

1. **IP-Gateway-Routenvalidator absichern.**
   - Managementnetz des Hosts/LXC erkennen bzw. konfigurieren.
   - Route ablehnen, wenn Zielnetz Management-IP, Managementnetz, Packet-Core-IP oder zentrale Abhängigkeitsadressen über den TUN verdrängen würde.
   - sinnvolle Fehlermeldung in WebUI/API.
   - Tests für Overlap, Hostroute, Default-Route und zulässige Packet-Data-Routen.
   - WebUI-Default `ntc-tun0` für freie Routen überdenken; kein gefährlicher Standard für Managementziele.

2. **Live-IP-Gateway bereinigen und Readiness beweisen.**
   - die beiden festgestellten persistenten Routen prüfen/löschen;
   - `ip route get 10.0.1.166` muss über `eth0 src 10.0.1.142` laufen;
   - Packet-Core-API vom IP-Gateway erreichbar;
   - `/health/ready` HTTP 200;
   - Node-Gateway-Karte zurück auf verfügbar, kein Fallback.

3. **Finalen Basisstations-Playoutpfad deployen und on-air abnehmen.**
   - Media Library auf `authoritative` + `playout.mode="basisstation"`;
   - reale TBS-URL und ggf. Login konfigurieren;
   - `diagnose-basisstation-playout.py` erfolgreich;
   - Testasset `TEST_Gruppenruf` auf GSSI `15201` aussenden;
   - Jobfortschritt, Rufaufbau, Audio und geordnetes Rufende dokumentieren.

4. **Piper-Ort bereinigen.**
   - auf der TBS sicherstellen, dass kein lokaler `netcore-piper.service` mehr aktiv ist;
   - im Media-Library-LXC `/voices` und TTS-Status prüfen;
   - mindestens eine neue TTS erzeugen, freigeben, archivieren und über TBS aussenden.

### P1 – Stabilisierung

5. Bestehende TTS-Archive per Migration physisch nach `TTS-Dateien/YYYY/MM/DD` prüfen und `state.json`/Manifest gegen Dateisystem validieren.
6. `CHANGES-MEDIA-LIBRARY-INTEGRATION.md` hinsichtlich des alten `UMask=0000`-Textes an den aktuellen `UMask=0077`-Stand angleichen.
7. Credentials für TBS-Playout langfristig aus Klartext-TOML herausführen; Open-Lab ist temporär, kein Produktions-Sicherheitsmodell.
8. Mehr-TBS-Fehlerfälle testen: Default-Station fehlt, Ziel deaktiviert, TBS busy, Login fehlgeschlagen, Jobwechsel, Timeout.
9. UI im Basisstationsmodus so gestalten, dass keine irrelevante Session-ID mehr prominent verlangt wird.
10. Retry-/Recovery-Verhalten für Remote-Playout nach TBS- oder Media-Library-Neustart definieren.
11. Archiv-/State-Migration vor Änderungen automatisch sichern und Recovery dokumentieren.
12. ARM64-/Zielplattform-Build und systemd-Deployment für Media Library und TBS in CI/Releaseprozess abbilden.

### P2 – optionale Weiterentwicklung

13. Direkten Media-Switch-Pfad nur für bewusst vorcodierte TACELP-Assets weiterpflegen und klar als Spezialmodus kennzeichnen.
14. Später zentralen Codec nur dann ergänzen, wenn ein echter Use Case unabhängig von einer TBS entsteht; nicht als Voraussetzung für normale Durchsagen.
15. Mehrere Basisstationen mit Standort-/Zellenbezug und automatischer Zielauswahl für Multisite-Playout anbinden.
16. Job-/Audit-Ansicht um Station, GSSI/ISSI, Remote-Job-ID, Cachephase, Rufaufbauphase und RF-Sendephase erweitern.
17. Das entwickelte TTS-Vorlagen-Namensschema (`TEST_`, `TECH_`, `OPS_`, `EVENT_`, `ALARM_`, `INFO_`) samt Platzhaltern wie `{ORT}`, `{UHRZEIT}` und `{GRUPPE}` als optionalen Konventionsstandard dokumentieren; kurze Sätze, ausgeschriebene kritische Zahlen und möglichst wenige Abkürzungen bleiben als Sprachqualitätsregel sinnvoll.

---

## 12. Screenshots aus den Entwicklungsnotizen

Die gesicherten Screenshots wurden farbreduziert; die Media-Library-Vollbildaufnahme wurde zusätzlich auf 960 px Breite skaliert. Die UI-Zustände bleiben erkennbar. Die gesicherten Kopien ersetzen keine unveränderten Originaldateien.

### Media-Library-Aussendemaske im Shadow-Zwischenstand

Das Bild dokumentiert den Zustand, in dem Vorschau-Assets auswählbar waren, während das Aussende-Assetfeld leer blieb. Rechts oben ist außerdem der damalige `shadow`-Modus sichtbar.

![Media Library Aussendung im Shadow-Modus](assets/2026-10-05_media-library-aussendung-shadow-ui.png)

### Node-Gateway-Karte zum IP-Gateway-Ausfall

Das Bild dokumentiert die Betreiberansicht `AUSGEFALLEN`/`FALLBACK` mit `connect failed: connection timed out`. Später wurde die Ursache auf die falsche Managementroute über `ntc-tun0` eingegrenzt.

![IP-Gateway Fallback Timeout](assets/2026-10-05_ip-gateway-fallback-timeout.png)

---

## 13. Relevante Arbeitsartefakte und Anhänge

Im Verlauf wurden mehrere Arbeits-/Übergabepakete erzeugt. Sie sind als historische Arbeitsartefakte zu verstehen; maßgeblich für den geprüften Codeabgleich ist der aktuelle Repository-Stand.

Bekannte Artefakte aus den Entwicklungsnotizen:

```text
netcore-tetra-swmi-tts-archive-routing-fix.zip
netcore-tetra-swmi-media-library-cargo-toolchain-fix.zip
netcore-tetra-swmi-media-library-shadow-asset-fix.zip
netcore-tetra-swmi-media-library-basisstation-playout.zip
INSTALLATION-MEDIA-LIBRARY-BASISSTATION-PLAYOUT.md
netcore-tetra-swmi-media-library-basisstation-playout.zip.sha256
Eingefügter Text(17).txt
```

Bewertung:

- Der `cargo-toolchain-fix` entstand unmittelbar vor der Klarstellung, dass das Media-Library-Update versehentlich auf der TBS gestartet worden war. Er ist daher **nicht** als Beweis für einen realen Media-Library-LXC-Fehler zu lesen.
- Der `shadow-asset-fix` war ein Zwischenstand zur Bedienbarkeit im Shadow-Modus; die spätere Basisstations-Playoutarchitektur ist der maßgebliche Weg für echte Aussendung.
- Das finale Basisstations-Playoutpaket entspricht konzeptionell dem am Prüfstand 05.10.2026 im Repository vorhandenen `basisstation`-Modus; ein späterer Live-On-Air-Test nach Installation wurde nicht mehr dokumentiert.

---

## 14. Relevante Repository-Quellen

Die folgenden Dateien wurden bei der Quellprüfung auf `Archiving` geprüft bzw. als maßgebliche geprüfte Referenz herangezogen:

```text
CHANGES-CENTRAL-MEDIA-LIBRARY-TTS.md
CHANGES-MEDIA-LIBRARY-INTEGRATION.md
CHANGES-MEDIA-LIBRARY-BASISSTATION-PLAYOUT.md
Docs/MEDIA_LIBRARY_BASISSTATION_INTEGRATION.md
Docs/MEDIA_LIBRARY_BASISSTATION_PLAYOUT.md
Docs/MEDIA_LIBRARY_CENTRAL_TTS.md
Docs/MEDIA_LIBRARY_TTS_ARCHIVE_ROUTING_FIX.md
system-backend/media-library/config/media-library.example.toml
system-backend/media-library/src/config.rs
system-backend/media-library/src/http.rs
system-backend/media-library/src/state.rs
system-backend/media-library/src/worker.rs
system-backend/media-library/install/update.sh
system-backend/media-library/install/ensure-piper.sh
system-backend/media-library/install/shared-storage.sh
system-backend/media-library/install/migrate-archive-layout.py
system-backend/media-library/systemd/netcore-media-library.service
install/update-basisstation.sh
system-backend/ip-gateway/config/ip-gateway.example.toml
system-backend/ip-gateway/src/http.rs
system-backend/ip-gateway/src/state.rs
system-backend/ip-gateway/src/kernel.rs
system-backend/ip-gateway/web-ui/index.html
```

Ein separater Implementierungscommit oder PR lässt sich dem Arbeitsstand nicht eindeutig zuordnen. Maßgeblich für den Codevergleich ist der oben genannte `Archiving`-Commit.

---

## 15. Konkrete nächste Schritte

Die Fortsetzung dieses Themenstrangs sollte in dieser Reihenfolge erfolgen:

1. **IP-Gateway live reparieren:** persistente Managementroute löschen, Route/Packet-Core/Readiness/Node-Gateway-Anzeige verifizieren.
2. **IP-Gateway-Code absichern:** Managementnetz-/Dependency-Overlap-Guard plus Tests und sichere WebUI-Defaults implementieren.
3. **Media-Library-Basisstationsmodus auf dem realen LXC deployen:** reale Station konfigurieren, Diagnose ausführen.
4. **On-Air-Test GSSI 15201:** `TEST_Gruppenruf` bzw. eine kurze neue TTS, parallel Media-Library- und TBS-Logs sichern.
5. **Piper-Ort verifizieren:** nur Media-Library-LXC aktiv; TBS lokal sauber entfernt.
6. **Archivmigration prüfen:** TTS ausschließlich unter `TTS-Dateien`, Recordings ausschließlich unter `Recordings`, allgemeine Medien unter `Media-Library`.
7. **Dokumentations-/Konfigurationshygiene:** veraltete `UMask=0000`-Aussage korrigieren, Zugangsdaten aus späterem Produktionsmodell herauslösen und Multi-TBS-Betriebsfälle ergänzen.

Für die Fortsetzung sind vor allem die Routenabsicherung und die fehlenden Ende-zu-Ende-Nachweise maßgeblich.
