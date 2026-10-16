# Basisstationen automatisch am SIP-Switch einrichten

Stand: 17. Oktober 2026 · Z01.4. Für den normalen SIP-Betrieb genügt der Name
der Basisstation. Der SIP-Switch erstellt ihren Benutzer und ihr Passwort und
ermittelt seine erreichbare SIP-Adresse. Ein direkter **FreePBX-Fallback** ist
optional und hat eigene Zugangsdaten.

## Bestehenden SIP-Switch aktualisieren

Im **zentralen SIP-Switch-LXC als root** ausführen. Das aktualisiert auch die
systemd-Schreibfreigabe für `/etc/netcore`; ein reines Runtime-Dateiupdate reicht
für diese erstmalige Übernahme nicht. Vorhandene SIP-/PBX-Konfigurationen bleiben
beim Update erhalten. Der geprüfte Runtime-Updater startet den Verwaltungsdienst
neu und schreibt den Versionsbeleg für Discovery/Deploy. Den Updateblock außerhalb eines laufenden Telefonieversuchs ausführen.

```bash
(
  set -Eeuo pipefail
  NC_SRC=$(mktemp -d /tmp/netcore-sip-ui.XXXXXX)
  git -C "$NC_SRC" init -q
  git -C "$NC_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_SRC" config http.version HTTP/1.1
  git -C "$NC_SRC" config remote.origin.promisor true
  git -C "$NC_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_REF=$(git -C "$NC_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_SRC" sparse-checkout init --cone
  git -C "$NC_SRC" sparse-checkout set tools/operator system-backend/sip-switch system-backend/deployment-core
  git -C "$NC_SRC" checkout --detach "$NC_REF"
  [[ "$(git -C "$NC_SRC" rev-parse HEAD)" == "$NC_REF" ]]
  install -m 0644 "$NC_SRC/system-backend/sip-switch/systemd/netcore-sip-switch.service" \
    /etc/systemd/system/netcore-sip-switch.service
  systemctl daemon-reload
  python3 -B "$NC_SRC/tools/operator/update-netcore-service.py" \
    --service sip-switch --commit "$NC_REF" --apply
  systemctl is-active netcore-sip-switch.service asterisk.service
  printf 'SIP-Switch aktualisiert: %s\n' "$NC_REF"
)
```

Danach die SIP-Switch-Seite mit **Strg+F5** neu laden. Die folgende Deploy-VM-Aktualisierung übernimmt das erweiterte Importformat.

## Deploy-VM für den optionalen FreePBX-Fallback aktualisieren

Diese Erweiterung benötigt auch den neuen Importer auf der Deploy-VM. Der
bisherige Importer verlangt noch einen PBX-Ersatzweg. In der SSH-Sitzung
**jhoffmeister auf VM-H-DEPLOY-01** ausführen, bevor ein neues Image gebaut wird:

```bash
(
  set -Eeuo pipefail
  NC_VM_SRC=$(mktemp -d /tmp/netcore-sip-auto-vm.XXXXXX)
  git -C "$NC_VM_SRC" init -q
  git -C "$NC_VM_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_VM_SRC" config http.version HTTP/1.1
  git -C "$NC_VM_SRC" config remote.origin.promisor true
  git -C "$NC_VM_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_VM_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_VM_COMMIT=$(git -C "$NC_VM_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_VM_SRC" sparse-checkout init --cone
  git -C "$NC_VM_SRC" sparse-checkout set tools/operator system-backend/deployment-core
  git -C "$NC_VM_SRC" checkout --detach "$NC_VM_COMMIT"
  sudo python3 -B "$NC_VM_SRC/tools/operator/update-vm119-image-features.py" \
    --commit "$NC_VM_COMMIT" --apply
)
```

Erwartet ist `passed` oder `already_current`. Controller, Builder,
Image-Rezept und Formular werden gemeinsam aktualisiert; vorhandene
Konfigurationen, Profile und Dateien bleiben erhalten. Aktive oder ungeklärte
Aufträge verhindern das Update. Danach die Deploy-Seite mit **Strg+F5** laden.

## Basisstation nur mit Namen anlegen

1. Die SIP-Switch-WebUI auf Port8300 öffnen und **Basisstation anlegen** wählen.
2. Unter **Name der Basisstation** genau den Namen des Deploy-TBS-Profils
   eintragen, zum Beispiel `SRV-M-TBS-01`.
3. **Anmeldung am SIP-Switch · automatisch** zeigt die ermittelte Adresse und
   den automatisch abgeleiteten Benutzer. Das private Passwort wird beim
   Speichern erzeugt und bei späterem Bearbeiten erhalten.
4. **FreePBX-Fallback bei Ausfall des SIP-Switch aktivieren** ausgeschaltet
   lassen, wenn nur der Zentralweg benötigt wird. Dafür sind keine
   FreePBX-Benutzer- oder Passwortangaben nötig.
5. **Speichern und TOML herunterladen** übernimmt den SIP-Switch-Zugang
   dauerhaft, lädt Asterisk neu und lädt die private TOML direkt herunter.
   **Speichern und anwenden** speichert ohne Download. Ein erneuter Download ist
   in der Stationsliste bei **TOML bereit** über **SIP-TOML ↓** möglich.

Die Adresse wird aus einer expliziten Asterisk-Advertise-Adresse, einem
konkreten SIP-Bind oder der lokalen Route zur konfigurierten Telefonanlage
ermittelt. Die Routenwahl sendet kein UDP-Paket. Bei mehreren Netzen kann ein
Administrator `asterisk.advertise_host` zentral vorgeben. Die Adresse aus dem
HTTP-Host-Header wird nicht als SIP-Ziel übernommen.

Aliase, Kontaktlimit, Aktivierung und eine abweichende erreichbare
SIP-Switch-Adresse liegen unter **Erweiterte Stationseinstellungen**. Bereits
bestehende SIP-Benutzer und Passwörter werden beim Bearbeiten erhalten.
Die normale Anlage verlangt keine dieser erweiterten Eingaben.

## Optional: direkter FreePBX-Fallback

Nur für die direkte Anmeldung dieser TBS an FreePBX bei bestätigtem Ausfall
des SIP-Switch:

1. In FreePBX einen eigenen TBS-Zugang vorbereiten.
2. Bei der Station **FreePBX-Fallback bei Ausfall des SIP-Switch aktivieren**
   einschalten.
3. **FreePBX-Benutzer** und **FreePBX-Passwort** dieses Zugangs eingeben.
   Die FreePBX-Adresse und der Port werden aus der zentralen
   Telefonanlagenkonfiguration des SIP-Switch übernommen. Auth-/From-/Contact-
   Benutzer folgen dem eingegebenen FreePBX-Benutzer; abweichende Werte stehen
   unter **Erweiterte FreePBX-Einstellungen**.
4. **Speichern und TOML herunterladen** wählen. Die Datei enthält die
   automatisch erzeugten SIP-Switch-Daten und den eingeschalteten
   FreePBX-Fallback samt dessen eigenen Zugangsdaten. Sie lässt sich wie die
   Datei ohne Fallback beim gleichnamigen Deploy-Profil importieren.

Der gemeinsame FreePBX-Trunk des SIP-Switch ist ein anderer Zugang. Gleicher
PBX-Host, Port und REGISTER-Benutzer wie beim registrierenden Switch-Trunk
oder dem aktivierten FreePBX-Fallback einer anderen Station sperren den
Fallback-Export mit einer konkreten Meldung und HTTP409. Auch der direkte
Speichern-und-Download-Knopf umgeht diese Prüfung nicht: Eine erfolgreiche
Speicherung und ein anschließend gesperrter Download werden getrennt gemeldet.
Die bereits gespeicherte Station über **Bearbeiten** korrigieren und erneut
**Speichern und TOML herunterladen** wählen. Dabei bleibt ein vorhandenes
FreePBX-Passwort bei leerem Passwortfeld erhalten. Hostnamen werden
unabhängig von Groß-/Kleinschreibung und einem abschließenden Punkt verglichen;
DNS-Aliase werden nicht aufgelöst. Ein im Switch deaktivierter zentraler
Stationsendpoint reserviert seinen weiterhin konfigurierten aktiven Fallback,
weil das Deaktivieren des zentralen Endpoints einen vorhandenen Pi nicht stoppt.

Bei älteren Stationen bleibt ein bereits eingetragener FreePBX-Ersatzweg
aktiviert. Für den Zentralweg allein den Fallback im neuen Editor ausdrücklich
ausschalten und speichern. Seine inaktiven Zugangsdaten werden nicht exportiert.
Das ist insbesondere bei der früheren Doppelbelegung von `101` an
`10.0.1.21:5060` möglich: Ein eigenes FreePBX-Konto ist nur Voraussetzung für
den eingeschalteten direkten Ersatzweg, nicht für die Anmeldung am SIP-Switch.

## TOML in Deploy importieren und Pi starten

In Deploy unter **Images** das gleichnamige TBS-Profil wählen oder nach einem
Anwendungsreset neu anlegen. Die heruntergeladene Datei unter
**SIP-Switch-Konfiguration** auswählen und ein neues Image bauen. Der Import
prüft die Stationskennung und übernimmt die automatisch erzeugten
SIP-Switch-Zugangsdaten. Bei ausgeschaltetem FreePBX-Fallback enthält das Image
keine direkte PBX-Registrierung und keinen PBX-Ersatzrufweg. Auch nach längerem
Zentralausfall bleibt dieser Ersatzweg aus; eine manuell erzwungene Umschaltung
auf `pbx_direct` wird abgelehnt.

Der lokale Asterisk im Pi verbindet die native TBS auf Port5062 mit dem zentralen
SIP-Switch. Deshalb zeigt die native TBS-WebUI `127.0.0.1:5060` als lokale
SIP-Gegenstelle. Die zentrale Adresse steht in `[central]` der privaten TOML.
Bei aktivem FreePBX-Fallback gilt weiterhin die exklusive, bestätigte Umschaltung
mit Hysterese; beide Registrierungen laufen nicht gleichzeitig.

Der vorhandene Image-SIP-Pfad unterstützt UDP. Ein zentral auf TCP betriebener
Switch wird beim Export abgewiesen. Ein neues Profil oder eine neue TOML ändert
kein bereits erzeugtes Image. Die echte Anmeldung am gestarteten Pi und
Telefonie-/Funkrufe bleiben eigene Anlagenprüfungen.

## Speicherung und gleichzeitige Änderungen

Die vorherige private Konfiguration wird als `.toml.bak` gesichert. Scheitern
Schreiben, Asterisk-Neuladen oder Endpointprüfung, wird die Änderung
zurückgenommen. Aktive Rufe der betroffenen Station sperren Bearbeiten und
Löschen. Gleichzeitige oder externe Änderungen führen zu einem Konflikt.
Öffentliche Stations- und Konfigurationsantworten enthalten keine Passwörter.
Für die WebUI gilt weiterhin der vorhandene OPEN-LAB-Betrieb.

## Nachweise der automatischen Einrichtung

Gezielte Prüfungen decken die Anlage allein mit Namen, automatisch erzeugte
SIP-Switch-Zugangsdaten, zentralen Export ohne FreePBX, den tatsächlichen
Offline-Renderer und den ausgeschalteten Fallback bei Zentralausfall ab. Der
Browser prüft beide Downloads und den Import durch den tatsächlichen
Deploy-Controller/Worker. Die echte Asterisk-Prüfung verlangt einen SIP401/200-
REGISTER mit den automatisch exportierten Daten.

[PR #102](https://github.com/JanHG98/netcore-tetra/pull/102) ist übernommen;
[SIP-Abschlussprüfung](https://github.com/JanHG98/netcore-tetra/actions/runs/38973913722)
am Stand `df9ccc09`: auf Ubuntu24.04/26.04 je65 Fälle ohne Skip, beide tatsächlichen
privaten Downloads/Deploy-Importe und echter SIP401/200-REGISTER mit automatisch
erzeugten Daten an Asterisk22.11.0 bestanden. Der laufende SIP-Switch ist über
seinen verifizierten Agenten auf `57510d25` aktualisiert: Auftrag
`f205f11b8c8443558435c1802e033425`, `succeeded`, `live=true`, `ready=true`.
Manifest, automatische Adresse `10.0.1.125:5060` und neue Oberfläche sind live
bestätigt; Desktop und dunkle Mobilansicht wurden ohne Speichern einer Station
geprüft. [Maschinenlesbarer Nachweis](../../integration/Z01-2026-10-07/evidence/sip-namensanlage-freepbx-optional-2026-10-17.json).

Der noch erforderliche Deploy-VM-Updateblock steht oben. Die separate
Ubuntu24.04-Desktop-/Image-CI bleibt bereits an der fehlenden
`qemu-aarch64`-Registrierung des Runners blockiert. Ein vollständiger neuer
Imagebau und die tatsächliche Pi-/PBX-/Funkabnahme sind damit nicht belegt.

## Nachweise der Registrierungskorrektur

[PR #101](https://github.com/JanHG98/netcore-tetra/pull/101) ist in
`main@0abea260` übernommen. Am produktiven Code `6e958c29` bestehen auf
Ubuntu24.04/26.04 jeweils59 Fälle ohne Skip, einschließlich der neuen
PBX-Kontenkonflikte, einer echten HTTP409-Antwort ohne private TOML und des
unveränderten Offline-SIP-Renderers. Chromium bestätigt den tatsächlichen
Download und Deploy-Import. Die Dashboard-Prüfung besteht150 Browserfälle,
77 Rust-Fälle und den Defaultfeature-Check der Basisstation; insbesondere
bleiben `registering`/`not registered` ohne Grünstatus und Statusfehler entfernen
eine vorherige Grünanzeige. Eine echte Asterisk22.11.0-Instanz bestätigt weiterhin
Endpoint/Auth/AOR, Passwort-/Kontaktlimitänderung und Entfernen ohne
Daemon-Neustart. [Prüfnachweis und Anlagenabgrenzung](../../integration/Z01-2026-10-07/evidence/sip-registrierung-und-export-2026-10-16.json).

Die separate Ubuntu24.04-Image-/Desktop-CI scheitert weiterhin bereits an der
fehlenden `qemu-aarch64`-binfmt-Registrierung des Runners. Damit ist kein neuer
vollständiger Desktopimagebau oder echter Pi-Registrierungsablauf belegt.

## Frühere Nachweise und Grenzen

[Abschlussprüfung](https://github.com/JanHG98/netcore-tetra/actions/runs/38954506305) am produktiven
Code `e85cf239`: auf Ubuntu 24.04 und 26.04 jeweils 55 Python-Fälle
bestanden; Chromium bestätigt den vollständigen Download-/Deploy-Importablauf.
Eine echte Asterisk-22.11.0-Instanz übernimmt Endpoint, Passwort und Kontaktlimit,
benennt den Endpoint um und entfernt Endpoint/Auth wieder ohne Daemon-Neustart.
Dabei werden auch 74 Zeichen Endpoint-ID und 79 Zeichen Auth-Abschnitt geprüft;
längere Kennungen lehnt die API vor dem Schreiben ab.

Gezielte Prüfungen: `tools/test_sip_station_management.py` für Persistenz,
Konflikte, aktive Rufe, Rücknahme, private HTTP-Downloads und tatsächliche
Offline-Image-Einrichtung; `tools/test_sip_station_ui.cjs` für die echte
SIP-WebUI und den Import ihrer heruntergeladenen Datei durch die bestehende
Deploy-WebUI samt Controller-/Worker-Validierung; `tools/test_sip_station_asterisk.py`
für geladene Endpoints, Auth-Daten, Kontaktlimit und Löschen in einer echten
isolierten Asterisk-Instanz. Die Browserprüfung verwendet einen simulierten
Imagebau; die Offline-Prüfung führt den tatsächlichen SIP-Renderer aus.
Betreiberübernahme, Registrierung der echten TBS und Funk-/Telefonieruf mit dem
exportierten Profil sind eigenständige Anlagenprüfungen.
