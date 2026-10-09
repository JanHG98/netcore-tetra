# NetCore-Tetra: gesicherter Zwischenstand vom 08.10.2026

Stand: **08.10.2026, 17:03 Europe/Berlin**. Nutzerauftrag: aktuellen Zwischenstand sichern und Roadmap / Projektunterlagen abgleichen. Geprüftes main vor dieser Dokumentationssicherung: **`001fb84ac566bb0f95e18d22439ee664fe0093e4`**. Dieser Checkpoint ergänzt die [zentrale Roadmap](../../roadmaps/ROADMAP.md); er dokumentiert den vorhandenen Stand und löst keinen zusätzlichen Anlagenauftrag aus.

## Projektstand

| Aufgabe | Stand | Beleg |
| --- | --- | --- |
| Z01.1 | Erledigt gemäß Quellabnahme: 2.655 Pfade vollständig verglichen; Integrationspakete / Konflikte / Rückwege festgelegt | [Integrationsbericht](README.md), Vergleichstabellen |
| Z01.2 | Auf main übernommen: Deployment / Discovery / Imagebuilder / Pi-VPN / Syslog; aktuelle UI und Fachänderungen erhalten | PR #62 / mainc45a2ec, [Bericht](README.md) |
| Z01.3 | Quell-/CI-Abnahme bestanden: 26er Inventory und gemeinsame Verträge, semantische Readiness, Konfigurationserhalt | [Bericht](README.md), [CI](ci.md) |
| Z01.4 | In Arbeit: CT150-/CT136-Teilabnahmen bestanden, Imagebuilder-Vorprüfung und DNS-Fortschritt bestätigt; vollständiger ARM64-Build / Artefakte / Pi / VPN offen | [Laufzeitnachtrag](runtime-z014.md), [Imagebuilder](imagebuilder-vm119.md) |

Z02 bleibt der parallele P0-Strang für Restore, Management-/Packet-Core-Routenschutz und zentrale Gruppenaufträge. Die übrige Reihenfolge einschließlich IAM / Drive und langfristigem Z11.1 wird erhalten. Diese Sicherung nimmt keine weitere Implementierung vor.

## Bestätigte Anlagenbefunde

| Anlage | Bestätigt | Fortsetzungsrelevante Grenze |
| --- | --- | --- |
| VM119 `VM-H-DEPLOY-01`, `10.0.1.131:8320` | Controllerinstallation und Git-Pinning an c45a2ec; Imagebuilder-Werkzeuge / ARM64-Emulation / Template / drei Profile vorhanden, bei Vorprüfung rund78,78GiB frei | Aktuelles main ist nicht pauschal die installierte VM-Version; jobs.py- und Gast-APT-Hotfixübernahme noch nicht durch neue Betreiber-Ausgabe bestätigt |
| CT150 `z014-test-hw`, DHCP `10.0.1.191:8321` | Hardware-Neuinstallation und Wiederholungsupdate an c45a2ec; Agentfix31829fc installiert; erwartete negative Readiness und Recovery bestanden; Konfigurationswert37 und vollständige Datei erhalten, Ausgänge deaktiviert | Kein echter Hardware-Versionswechsel, MQTT-/Aktor-/RF-Nachweis; DHCP-Adresse ist der damalige Befund |
| CT136 `Observability`, `10.0.1.143:8210` | Binaryupdate aus3d96a9a, beide APIs und unveränderte Standortkonfiguration; realer NFS-Dateizugriff als999:989; ein TCP-Marker in beiden Vorschauen und vollständigem NAS-gzip; isolierter Mountfehler erhält Raw / Outbox | Keine reale NFS-Störung / Stall oder Flottensenderabnahme; beobachtete HTTP-Latenz bis4,952s bleibt ungeklärt |

Bestehende Ergebnisdateien bleiben die datierten Anlagenbelege:

- [CT150 Readiness / Recovery](evidence/ct150-readiness-2026-10-08.json), Original auf CT150: `/var/tmp/netcore-z014-readiness-c45a2ec1/result.json`.
- [CT136 TCP / Vorschau / NAS](evidence/ct136-syslog-2026-10-08.json), Original: `/var/tmp/netcore-ct136-syslog-oggg829q/result.json`.
- [CT136 isolierter Mountfehler](evidence/ct136-archive-negative-2026-10-08.json), Original: `/var/tmp/netcore-ct136-archive-negative-3dcor89e/result.json`.
- [VM119 Vorprüfung](evidence/vm119-imagebuilder-preflight-2026-10-08.json), [erster DNS-Abbruch](evidence/vm119-imagebuilder-dns-2026-10-08.json), [zweiter dpkg-Abbruch](evidence/vm119-imagebuilder-conffile-2026-10-08.json).

## Imagebuilder: zuletzt nachgewiesener Stand

1. Erster Build `98a096a050284d6e952bdff78eff6caa` endet beim Gast-Paketdownload mit DNSfehlern. Fix8359f75 setzt den temporären Gastresolver auf0644 und prüft DNS als `_apt`; Host-DNS und Worker-UMask bleiben erhalten.
2. Zweiter Build `2dce020e97d745408fe5221453ad63ac` erreicht ARM64-Paketentpackung / -konfiguration und endet bei `initramfs-tools-core`: `end of file on stdin at conffile prompt` für `initramfs.conf`. Die Logauszüge dokumentieren Unmounts und Loopdetach. Diese Kennungen sind Build-IDs, keine bekannten Imagejob-IDs.
3. Aktueller Fix **001fb84** ergänzt im Gastrezept `--force-confdef` / `--force-confold` und strikten APT-Indexfehler-Abbruch. Lokale Deployment-Suite:66 Tests /65PASS /1 erwarteter Unix-Socket-Skip; VM119-Operator-Suite:18/18PASS; echte isolierte native dpkg-Regression reproduziert den alten Prompt und bewahrt mit der Korrektur die angepasste Konfiguration.
4. Beide main-Workflows dieses exakten Commits sind inzwischen **abgeschlossen / erfolgreich**. [CI-Beleg](evidence/checkpoint-ci-2026-10-08.json). Ein voller ARM64-NetCore-Build ist damit noch nicht nachgewiesen. Seit dem zweiten Abbruch liegt kein neuer terminaler Betreiberstatus oder Artefaktbeleg vor.

Bereitgestellter gezielter Updateweg auf VM119: [vm119-image-apt-update.py](vm119-image-apt-update.py), aus dem vollständigen Commit `001fb84ac566bb0f95e18d22439ee664fe0093e4`. Er ersetzt ausschließlich `/usr/local/lib/netcore-deployment/image/build-guest.sh` nach Host-/Idle-/Konfigurations-/Rückwegprüfung. Erwartete neue SHA-256: `84b8d4070730b2fe94c4ddf4514411fadd08f19f72bfb3ce0adc86f14d1d8c3c`. Der Operatorblock und seine Grenzen stehen im [Imagebuilder-Nachtrag](imagebuilder-vm119.md).

Die DNS-Laufzeitkorrektur ist durch den zweiten Buildfortschritt belegt. Für den gesonderten VM119-jobs.py-Fix liegt bislang nur der alte c45a2ec-Fingerprint aus der Vorprüfung vor; ein späterer Austausch bleibt unbestätigt. Der Operatorpfad bleibt `/var/tmp/netcore-z014.e9BGTg`, sofern dieser Checkout noch existiert. CT136-Quellen liegen nach Betreiberangabe unter `/var/tmp/netcore-obs-source.VCkhk2`; der dortige Checkout allein benennt nicht die installierte Binaryversion.

## Konkrete Fortsetzung

1. **Z01.4 / VM119:** Gastrezept-Fixübernahme bestätigen und einen bereits gestarteten Build über den bestehenden Auftrag verfolgen. Den tatsächlichen Imagejob mit Status / `request.build_id` / Quellcommit aus `/api/v1/images` dokumentieren. Ein laufender Build benötigt aufgrund dieser Sicherung keinen weiteren POST oder Dienstneustart.
2. Bei Erfolg Image, SHA-256 und Manifest herunterladen; Prüfsumme und Commit **001fb84** abgleichen. Imagejob-ID und Build-ID unterscheiden; Controller-`/api/v1/jobs/<id>` ist kein Imagejob-Statuspfad. `boot_tested=false` bleibt bis zur tatsächlichen physischen Abnahme korrekt.
3. Danach bewusst gewählte Stationsidentität für Pi-/SXceiver-Boot verwenden und anschließend VPN-Wechsel prüfen. Ein eigener OS-Hostname ändert die Profil-/Funkidentität nicht.
4. Z01.4 insgesamt erst nach den zugehörigen Ausfall-/Versionswechselnachweisen schließen: Controller-Ausfall / Wiederkehr / Fehlerweitergabe, realer Hardware-Versionswechsel, NFS-Störung, Flottensender und begrenzte Logpuffer bleiben offen.

Ein CI-PASS, ein vorbereiteter Betreiberbefehl und ein laufender Build sind getrennte Nachweisstufen. Zugangsdaten bleiben lokal; für diese Sicherung wurde kein direkter Hostzugriff vorgenommen.
