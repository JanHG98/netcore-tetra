# TBS-Standort-Templates in Deploy erstellen, anzeigen und bearbeiten

Stand: 17. Oktober 2026 · Z01.4

Eine neue Deploy-VM benötigt keine zuvor exportierte TBS-Konfiguration mehr.
Unter **Neue Basisstation** stehen **TBS-Template erstellen**, **TBS-Template
importieren** und für gespeicherte Vorlagen **Template anzeigen / bearbeiten**
zur Verfügung.

## Erstes Template anlegen

1. **TBS-Template erstellen** öffnen.
2. Hauptträger und bei Bedarf zweiten Träger einstellen. Für einen einzelnen
   Träger **Zweiten Träger verwenden** ausschalten.
3. SDR-Samplerate und optional die SoapySDR-Geräteparameter eingeben. Ein leeres
   Gerätefeld überlässt die Auswahl dem SoapySDR-Treiber.
4. Unter **Erweiterte Frequenzeinstellungen** bei Bedarf Band, Offset,
   Duplexabstand-Code, eigenen Duplexabstand oder umgekehrte Frequenzlage ändern.
5. **Vorschau anzeigen** prüft die Werte und zeigt TBS-TX/Downlink, TBS-RX/Uplink,
   beide SDR-Mitten und die erzeugte TOML. Die Vorschau speichert noch nichts.
6. **Template erstellen** speichert die Vorlage dauerhaft auf der Deploy-VM.
   Danach das TBS-Profil mit Name, MCC/MNC, ISSI, Location Area und Colour Code
   anlegen und unter **Pi-Images** verwenden.

Die vorbelegten Träger 720/721 mit Band 4, Offset 0 und Duplexabstand-Code 0 ergeben
418,000/418,025 MHz Downlink und 408,000/408,025 MHz Uplink. Die SDR-Mitten liegen
bei 418,0125/408,0125 MHz; die Samplerate beträgt 600000 Hz. TX-/RX-Frequenzen und
Mitten werden aus den Eingaben berechnet und können nicht unabhängig davon
auseinanderlaufen. Die zwei vollständigen 25-kHz-Kanäle müssen in die Samplerate
passen. Unbekannte Felder, gleiche Träger, ungültige Zahlen und für das Band
nicht definierte Duplexabstände werden vor dem Speichern abgelehnt.

Die Vorlage enthält die benötigten TBS-Grundabschnitte, eine WebUI auf Port 8080
und den Node-Gateway-Anschluss. Ein bereits durch Discovery aufgelöster
Node-Gateway wird übernommen; sonst bleibt seine Dienstadresse stehen und der
vorhandene Discovery-Bindungsweg löst sie auf dem Pi auf. Ein Template enthält
keine kopierten Brew-, SIP- oder Dashboard-Zugangsdaten. SIP wird über die
[gesonderte SIP-Switch-TOML](../services/sip-switch/basisstationen-im-webui.md)
beim Imageauftrag eingerichtet.

Es gibt weiterhin ein gemeinsames Standort-Template. Die Erstellen-Funktion
überschreibt eine bereits vorhandene Vorlage nicht; auch gleichzeitiges
Erstellen wird mit HTTP 409 abgefangen. Eine andere geprüfte TOML kann über den
bestehenden Importweg übernommen oder die gespeicherte Vorlage direkt über
**Template anzeigen / bearbeiten** geändert werden. Bereits erzeugte Images
ändern sich dadurch nicht. Vorschau und **Abbrechen** verändern keine Vorlage.

## Gespeichertes Template anzeigen und bearbeiten

1. Unter **Neue Basisstation** auf **Template anzeigen / bearbeiten** klicken.
2. Der Editor lädt die vollständige, aktuell auf der Deploy-VM gespeicherte TOML.
   Bei einem Import ist das dessen Inhalt einschließlich Kommentaren und
   zusätzlicher Abschnitte; auch ältere importierte Vorlagen sind sichtbar.
3. Gewünschte Werte direkt in der TOML ändern und **Änderungen speichern** wählen.
   Geprüft werden UTF-8, TOML-Syntax und die Grundabschnitte **net_info**,
   **cell_info** und **phy_io**. Zusätzliche Einstellungen werden erhalten.
4. **Abbrechen** verwirft die Eingabe. Das automatische Aktualisieren der
   Deploy-Seite verändert einen geöffneten Editor nicht.

Vor jedem tatsächlichen Austausch wird die vorherige Fassung als
**tbs-site-template.toml.bak** neben dem Standort-Template gesichert. Vorlage
und Sicherung sind nur für den Dienstbenutzer beziehungsweise root lesbar.
Der Template-Inhalt wird erst beim ausdrücklichen Öffnen geladen, mit
**Cache-Control: no-store** ausgeliefert und nicht in Statuslisten veröffentlicht.

Ein zweiter Browser oder ein neuer Import kann die gespeicherte Vorlage
zwischenzeitlich ändern. Dann lehnt **Änderungen speichern** den alten Stand
mit HTTP 409 ab und lässt die eigene Eingabe im Editor stehen.
**Gespeicherte Version neu laden** lädt den aktuellen Stand; bei einer eigenen
ungespeicherten Änderung wird das Verwerfen vorher bestätigt.

Der TOML-Editor bearbeitet die Werte direkt. Bei manuellen RF-Änderungen müssen
Träger, TX-/RX-Frequenzen, SDR-Mitten und Samplerate zusammenpassen. Die
automatische Frequenzberechnung des Erstellformulars wird dabei nicht über die
importierte Konfiguration gelegt. Änderungen gelten für folgende Imageaufträge
und Neuinstallationen; bereits erzeugte Images ändern sich dadurch nicht.

## Bestehende Deploy-VM aktualisieren

In der SSH-Sitzung **jhoffmeister auf VM-H-DEPLOY-01** ausführen. Der vorhandene
Operatorhelfer übernimmt Controller, Builder, Rezept und WebUI gemeinsam und
erhält vorhandene Konfigurationen, Profile, Templates und Images:

~~~bash
(
  set -Eeuo pipefail
  NC_TEMPLATE_SRC=$(mktemp -d /tmp/netcore-template-vm.XXXXXX)
  git -C "$NC_TEMPLATE_SRC" init -q
  git -C "$NC_TEMPLATE_SRC" remote add origin https://github.com/JanHG98/netcore-tetra.git
  git -C "$NC_TEMPLATE_SRC" config http.version HTTP/1.1
  git -C "$NC_TEMPLATE_SRC" config remote.origin.promisor true
  git -C "$NC_TEMPLATE_SRC" config remote.origin.partialclonefilter blob:none
  git -C "$NC_TEMPLATE_SRC" fetch --depth 1 --filter=blob:none origin refs/heads/main
  NC_TEMPLATE_COMMIT=$(git -C "$NC_TEMPLATE_SRC" rev-parse FETCH_HEAD)
  git -C "$NC_TEMPLATE_SRC" sparse-checkout init --cone
  git -C "$NC_TEMPLATE_SRC" sparse-checkout set tools/operator system-backend/deployment-core
  git -C "$NC_TEMPLATE_SRC" checkout --detach "$NC_TEMPLATE_COMMIT"
  sudo python3 -B "$NC_TEMPLATE_SRC/tools/operator/update-vm119-image-features.py" \
    --commit "$NC_TEMPLATE_COMMIT" --apply
)
~~~

Erwartet ist **passed** oder **already_current**. Aktive oder ungeklärte Aufträge
verhindern das Update. Anschließend die Deploy-WebUI mit **Strg+F5** laden.

## Nachweise und Grenzen

Die gezielten Prüfungen decken Vorlagenvorschau, gültige und ungültige RF-Werte,
dauerhafte Erstellung, konkurrierendes Erstellen, bestehenden Import,
Profilübernahme und einen gepinnten Imageauftrag ohne vorherigen Import ab.
Ein nativer Rust-Test übergibt tatsächlich vom Python-Generator erzeugte
Einzel-/Doppelträgerkonfigurationen an den TBS-Parser samt RF-Validierung.
Der Browser prüft den vollständigen Weg ab leerem Zustand, Vorschau/Abbrechen,
Profil, Imageauftrag sowie Desktop- und Mobilansicht. Der eigentliche Imagebau
und die Dienstinstaller sind dabei simuliert; ein neues vollständiges Image
und die physische Pi-/Funkabnahme sind gesonderte Anlagenbelege.

[PR #104](https://github.com/JanHG98/netcore-tetra/pull/104) ist übernommen.
[Deployment-Abschlussprüfung](https://github.com/JanHG98/netcore-tetra/actions/runs/38979603088)
an `c629e0a6`: auf Ubuntu24.04/26.04 jeweils8 neue Python-Fälle und der native
Rust-Test mit drei tatsächlich erzeugten Konfigurationen bestanden. Der volle
Deployment-Discovery-/Browserjob bestätigt den Weg ohne Vorlagenimport bis zum
Imageauftrag und anschließend den weiterhin verwendbaren Dateiimport.
[Datierter Nachweis](../integration/Z01-2026-10-07/evidence/tbs-template-erstellen-2026-10-17.json).

Die aktuelle lesende Prüfung auf VM-H-DEPLOY-01 bestätigt eine vorhandene
Vorlage und bereits die Erstellen-Funktion. Der neue Knopf **Template anzeigen /
bearbeiten** fehlt dort noch. Für diese Ergänzung den Updateblock oben erneut
ausführen. Die installierte Vorlage wird dabei erhalten.

## Ergänzende Abnahme der Template-Bearbeitung

[PR #105](https://github.com/JanHG98/netcore-tetra/pull/105) ist übernommen.
[Abschlusslauf](https://github.com/JanHG98/netcore-tetra/actions/runs/38981621563)
an `edd99f1e`: auf Ubuntu 24.04 und 26.04 jeweils 14 Template-API-/Persistenzfälle
und ein nativer Rust-Test mit drei erzeugten und bearbeiteten TBS-Konfigurationen
bestanden. Geprüft sind das importierte Original, Kommentare und Zusatzabschnitte,
Syntaxfehler, Versionskonflikte, geschützte Sicherungen bei Bearbeitung und erneutem
Import sowie die Übernahme einer bearbeiteten Vorlage in einen neuen Imageauftrag.

Der vollständige Deployment-Discovery-/Browserjob besteht mit 168 Python-Fällen
(zwei bestehende Skips), 143 Operator-Updateprüfungen und dem Browserweg über
Abbrechen, Speichern, Konflikt und bestätigtes Neuladen bis zum Imageauftrag.
Desktop- und Mobilansicht sowie das Quellgate sind geprüft.
[Datierter Nachweis](../integration/Z01-2026-10-07/evidence/tbs-template-bearbeiten-2026-10-17.json).
Der eigentliche Imagebau und die Dienstinstaller sind in diesen Browser-/API-Fällen
simuliert. Neuer vollständiger Imagebau und physische Pi-/Funkabnahme bleiben eigene
Anlagenbelege; die Bearbeiten-Funktion muss noch auf die Deploy-VM übernommen werden.
