# Basisstationen im SIP-Switch verwalten und für Deploy exportieren

Stand: 16. Oktober 2026. Der SIP-Switch verwaltet seine Basisstationen unter
**Basisstationen** in der WebUI auf Port **8300**.

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

Danach die SIP-Switch-Seite vollständig neu laden. Ein vorhandener Deploy-Stand
mit dem Feld **SIP-Fallback-Konfiguration** benötigt für diesen Export kein neues
Importformat und kein zusätzliches VM-Update.

## Station anlegen, bearbeiten und löschen

1. **Basisstation anlegen** öffnen. Die Stationskennung muss genau dem
   TBS-Profil in Deploy und dem Serving-Node im Mobility Core entsprechen.
2. Endpoint-ID und SIP-Benutzer bei Bedarf eintragen; leere Felder werden bei
   neuen Stationen aus der Stationskennung abgeleitet. Ein leeres Zentralpasswort
   erzeugt für neue Stationen ein zufälliges Passwort. Beim Bearbeiten bedeutet
   ein leeres Passwortfeld immer **vorhandenes Passwort behalten**.
3. Unter **Zentralweg für das Image** die vom Pi erreichbare SIP-Switch-Adresse
   und den SIP-Port angeben. Die WebUI-Adresse wird als Hilfe vorbelegt; beim
   Zugriff über einen Proxy oder eine andere Managementadresse anpassen.
4. Unter **Eigener PBX-Ersatzweg dieser TBS** den vorbereiteten TBS-Zugang am
   vorhandenen PBX eintragen: Adresse, Port, Benutzer und gegebenenfalls
   Auth-Benutzer/Passwort. Der zentrale gemeinsame PBX-Trunk wird nicht kopiert.
   Ohne Auth-Benutzer darf das Passwort bei einem entsprechend eingerichteten
   PBX leer bleiben. From-/Contact-Werte sind optional.
5. **Speichern und anwenden** schreibt `/etc/netcore/sip-switch.toml`, sichert
   die vorherige Fassung als private `.toml.bak`, rendert die Asterisk-Includes,
   lädt Asterisk neu und prüft den betroffenen Endpoint. Scheitert das Schreiben,
   Neuladen oder die Endpointprüfung, wird die Änderung zurückgenommen.

**Bearbeiten** und **Löschen** stehen an jeder Tabellenzeile. Löschen entfernt
den zentralen SIP-Zugang dieser Station. Aktive Rufe der betroffenen Station
sperren Änderungen bis zum Rufende. Gleichzeitige oder externe
Konfigurationsänderungen führen zu einem Konflikt, nicht zum Überschreiben.
Bei extern bearbeiteter TOML den SIP-Switch-Dienst neu starten, bevor die WebUI
wieder schreibt.

Die Stationsliste und die Konfigurations-API liefern keine Passwörter. Für den
SIP-Switch gilt weiterhin der vorhandene OPEN-LAB-Betrieb ohne WebUI-Anmeldung.

## TOML in Deploy importieren

Bei **TOML bereit** auf **SIP-TOML ↓** klicken. Die Datei heißt zum Beispiel
`SRV-M-TBS-01-sip-fallback.toml`. Sie enthält die privaten Zugangsdaten für
`[central]` und `[fallback_pbx]` sowie die Stationskennung unter `[service]`.
Ist die Station deaktiviert oder fehlen notwendige Angaben, zeigt die Tabelle
die Ergänzung an und bietet keinen unvollständigen Download an.

In Deploy unter **Images** dasselbe TBS-Profil auswählen, **SIP für diese Station
einrichten** öffnen und die Datei unter **SIP-Fallback-Konfiguration** auswählen.
Beim Imagebau wird der vorhandene Phase-11c-Importer verwendet. Er richtet den
lokalen Asterisk, die private native TBS-Anmeldung und den exklusiven Zentralweg
mit PBX-Ersatzweg ein. Ein anderes TBS-Profil wird abgelehnt. Für die Verwendung
im Pi ist ein neues Image mit dieser Datei erforderlich; ein bereits laufender
Build wird dadurch nicht geändert.

Der vorhandene lokale Image-Fallback unterstützt UDP. Bei einem zentral auf TCP
konfigurierten Switch weist der Export darauf hin; es wird kein irreführendes
UDP-Profil ausgegeben. Die PBX-Zugänge und die Reihenfolge ihrer eingehenden
Routen müssen im vorhandenen PBX eingerichtet sein.

## Nachweise und Grenzen

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
