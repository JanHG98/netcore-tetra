# Deploy-Abnahme fortsetzen: Images, Backend, VPN, SIP und Anwendungsreset

**Gespräch:** „Deploy Abnahme fortsetzen“, 12.–16.10.2026.  
**Übergabe:** 16.10.2026, Europe/Berlin, wegen erreichter maximaler Chatlänge.  
**Aufgabe:** Z01.4 – reale Anlagenabnahme und gezielte Nachbesserungen.  
**Geprüfter Repository-Stand beim Chatwechsel:** `main@d54cb89f27e7c33bba73b0de430c7eac3dee768e`.  
**Offener Stand beim Chatwechsel:** [PR #100 – Deployment-Anwendungsreset](https://github.com/JanHG98/netcore-tetra/pull/100), am Übergabepunkt offen und noch mit einer fehlgeschlagenen nativen Prüfung.

**Fortsetzung abgeschlossen:** PR #100 mit bestandenem Loop-Fix nach `main@e589287457be441f65f480df2386d00f808ccbfc` übernommen; tatsächliche VM-Ausführung offen. Siehe Abschnitt10.

[Gesamtroadmap](../roadmaps/gesamtroadmap.md) · [Archivübersicht](README.md) · [Z01-Nachweise](../integration/Z01-2026-10-07/README.md)

## Quellenumfang und Verlässlichkeit

Diese Notiz führt den wiedergewonnenen Gesprächskontext mit den datierten Repository-Nachweisen, vorhandenen Arbeitsdateien, aktuellen Merge-Ständen und dem CI-Ergebnis des letzten offenen Auftrags zusammen. Sie ist eine technische Übergabe, kein wortgetreuer Export des gesamten Chats. Der direkte Gesprächszugriff war bei der Übergabe technisch gestört; eine vollständige Nachrichten-, Anhangs- oder Bildinventur war deshalb nicht möglich.

Die wichtigsten Ergebnisse sind bereits in den unten verlinkten Fachanleitungen und Z01-Belegen dokumentiert. Diese bleiben die maßgeblichen ausführbaren Anleitungen. Historische Befehle werden hier nicht zu einer zweiten Betriebsanleitung kopiert. Vorbereitete Dateien, bestandene Tests, übernommener Code und Betreiberbestätigungen werden getrennt geführt. Aus einem Merge oder einem grünen CI-Lauf folgt kein Update der Anlage.

Die [maschinenlesbare Quellenliste](assets/2026-10-16_deploy-abnahme/uebergabe-nachweise.json) hält Ausgangsstand, offene PR und Grenzen fest. Der [Fehlerauszug des Resettests](assets/2026-10-16_deploy-abnahme/reset-ci-ubuntu-24.04.txt) sichert den konkreten letzten Prüfstand. Vorhandene datierte JSON-Belege unter Z01 bleiben an ihrem ursprünglichen Ablageort. Nicht eindeutig zuordenbare Bilder aus anderen Chats werden nicht als Bilder dieses Gesprächs übernommen. Fehlende Originalanhänge bleiben eine benannte Quellenlücke.

## 1. Ziel und verbindliche Betreiberentscheidungen

Jan will ein fertiges, auf eine SD-Karte schreibbares Stationsimage. Die notwendige Einrichtung soll durch Deploy übernommen werden; eine lange nachträgliche Reparaturfolge auf jedem Pi erfüllt dieses Ziel nicht.

Im Verlauf konkretisiert wurden:

- KDE Plasma als gewünschter Desktop; weitere verfügbare Desktopvarianten bleiben auswählbar.
- Audio-/Aufzeichnungsverzeichnisse und Dienstrechte bereits im Image korrekt anlegen.
- Träger-, Telegram-, Nachbarzellen-, WLAN- und Dienstbedienung über die vorgesehenen Oberflächen ermöglichen.
- Standort-/SIP-Daten importierbar machen, statt vertrauliche Zugangsdaten manuell in mehreren Dateien zu verteilen.
- OpenVPN samt Benutzer/Passwort und grafisch sichtbarem Profil vollständig vorbereiten; Heimnetzregel berücksichtigen.
- Bestehende zentrale Dienste mit konkreten, kopierbaren Updatewegen aktualisieren.
- Hängende oder nur scheinbar laufende Controlleraufträge anhand tatsächlicher Agentenquittungen klären.
- Zum Schluss die Deploy-Anwendung mit leeren Builds, Caches, Profilen, Einstellungen und Verlauf neu beginnen lassen.

Die ausdrücklichen Zwischenaufträge haben Vorrang vor der allgemeinen Roadmapfolge. Andere Dienste oder den Funkprozess nicht aus einer alten Anleitung heraus ungefragt neu starten.

## 2. Zusammenfassung des Übergabestands

| Bereich | Nachgewiesen | Weiter offen |
| --- | --- | --- |
| KDE / Pi-Funktionen | Betreiber bestätigt am 13.10. KDE-Boot, Rufe, SDS, lokale Aufzeichnung/RF-Wiedergabe und WLAN-Bedienung; am 16.10. erneut „KDE geht“ für das Image an `5ea8d3be` | Neuer vollständiger Build mit den späteren VPN-/Zeitkorrekturen; Dauerlauf und weitergehende physische Abnahmen |
| Image-/UI-Nachbesserungen | PR #70/#71 und Folgekorrekturen übernommen; datierte native, Browser- und ARM64-Personalisierungsprüfungen vorhanden | Übernahme der jeweiligen neueren Rezept-/Laufzeitstände auf den Zielhosts gesondert belegen |
| Discovery-Agenten | Proxmox-Bericht: 28 geplant, vorbereitet und aktualisiert, Ergebnis `completed` | Aktualisierung und Fachprüfung der eigentlichen Backend-Dienste ist ein eigener Schritt |
| Alte Remote-Aufträge | Betreiber bestätigt Endbelegabgleich; globale Prüfung ohne ungeklärte Remote-Aufträge | Ein neuer vollständiger Backend-Sammellauf ist noch nicht belegt |
| Logarchiv / Mediencache | PR #95/#96/#97 übernommen; native Tests und konkrete CT138-Anleitungen vorhanden | Erfolgreiche NAS-Übertragung, tatsächliche Speicherentlastung und Rückholung/Aussendung auf CT138 belegen |
| Image-VPN / Zeitbroadcast | PR #98 in `main@03422f36`; echter isolierter VPN-Tunnel, ARM64-Personalisierung und Zeitencoderfälle geprüft | Echter VPN-Server, Standortwechsel und Funkgeräte-Ortszeit auf der Anlage abnehmen |
| SIP-Stationsverwaltung | PR #99 in `main@d54cb89f`; CRUD, privater TOML-Export und echter Deploy-Import geprüft | SIP-Switch-LXC übernehmen und reale TBS-/PBX-/Funkrufe abnehmen |
| Vollständiger Deploy-Reset | PR #100 mit Operator-Helfer, Anleitung und 19 Prüffällen vorbereitet | Ubuntu-24.04-Looptest fehlerhaft; PR noch offen, kein Reset auf VM119 belegt |

## 3. Image, Desktop und Stationsbedienung

### 3.1 Ausgangsprobleme und gebündelte Nachbesserung

Die frühe Fortsetzung behandelt fehlerhafte beziehungsweise unvollständige Medienverzeichnisse und Bedienabläufe. PR #70 korrigiert Recorder-/Audioverzeichnisse und Dienstrechte. PR #71 bündelt KDE-Auswahl, Carrier-/Telegram-TOML, WLAN, SIP-Import, kompakte Dienstanzeige, zentrale Responsiveness und Nachbarzellenverwaltung.

Die datierte Abschlussprüfung an `d3ea8283` umfasst 120 VM-Guardfälle, 14 KDE-, zehn SIP-Setup-, neun SIP-Rechte-, 72 Dashboard-Rust-, 64 zentrale Backendfälle und 149 Dashboard-Browserprüfungen. Beide echten ARM64-Personalisierungen und systemd-Verzeichnisprüfungen sind dokumentiert; alle sechs damaligen Workflows bestanden. Diese Zahlen beschreiben den damaligen Prüfumfang, keine heutige Gesamtanlagenabnahme.

Der konsolidierte [Image-/UI-Bericht](../integration/Z01-2026-10-07/image-ui-desktop-fixes-2026-10-12.md) enthält die einzelnen Befunde, Rückwege und den VM119-Updater. Die Nachweise [Image-/KDE-CI](../integration/Z01-2026-10-07/evidence/image-ui-kde-ci-2026-10-12.json), [erster VM-Kandidat](../integration/Z01-2026-10-07/evidence/vm119-imagebuilder-candidate-2026-10-12.json) und [Firstboot](../integration/Z01-2026-10-07/evidence/vm119-imagebuilder-firstboot-2026-10-12.json) bleiben dort erhalten.

### 3.2 Betreiberabnahme auf dem Pi und Nachbarzellenkorrektur

Am 13.10. bestätigt Jan KDE-Boot, Rufe, SDS, lokale Aufzeichnung/RF-Wiedergabe und WLAN-Bedienung. Der [Pi-Abnahmebeleg](../integration/Z01-2026-10-07/evidence/pi-kde-acceptance-2026-10-13.json) hält diesen Umfang fest.

Die Nachbarzelleneingabe und der softwareseitige Broadcast sind belegt. Die dabei gefundene falsche Broadcast-Ankündigung wird korrigiert; für das damalige Image ist `neighbor_cell_broadcast=2` als begrenzter Übernahmeweg dokumentiert. Ein tatsächlicher Zellwechsel eines Mobilgeräts ist damit nicht nachgewiesen. SIP, VPN und Dauerlauf bleiben eigene Prüfungen.

## 4. Updatewege, Discovery und Backendaufträge

### 4.1 Dienstweise Updates und gemeinsame Sammelupdates

Am 13.10. werden auf Betreiberwunsch 33 konkrete Updateblöcke für 26 reguläre Laufzeitrollen und sieben weitere Dienste/Komponenten dokumentiert. Die tatsächlichen Quell-/Bestandspfade, Benutzer, Ausführungshosts und Abschlussprüfungen sind in [NetCore-Dienste aktualisieren](../deployment/netcore-dienste-aktualisieren.md) und dem [Befehlsnachweis](../integration/Z01-2026-10-07/evidence/service-update-commands-2026-10-13.json) festgehalten. Die Prüfung der Befehle installiert noch keinen Dienst.

PR #84 ergänzt die WebUI-Sammelplanung mit gemeinsam aufgelöstem `main`-Commit, per-Dienst-Aufträgen, kompaktem Livezustand und Schutz gegen Doppelstarts. Discovery erhält Löschen/Wiederherstellen und Filter nach Typ, Rolle, Onlinezustand und Text. Der datierte Nachweis umfasst 13 Sammelupdate-, 14 Discovery-Lifecycle-, fünf Queue-/Datenschutzfälle, 143 Backend-/143 VM-Guards sowie den echten Desktop-/Mobil-Browserablauf an `84356fa9`.

Registry, Inventory und Agentenkatalog sind unterschiedliche Sichten: 26 reguläre Runtime-Dienste; der optionale Provisioning Core und die TBS werden nicht durch eine ungenaue Gesamtzahl zu regulären installierten Backends erklärt.

### 4.2 Git-Transport und 28 übernommene Discovery-Agenten

Der Betreiberoutput des IP-Gateways belegt `curl 56`, Verbindungsreset und `early EOF` beim Git-Abruf. PR #90 verwendet HTTP/1.1, flachen Partial Clone und gezielt die Laufzeitquellen statt des großen Dokumentationsbestands. Reine Git-Vorbereitung darf bei Transportabbrüchen begrenzt wiederholt werden; der Installer startet erst nach erfolgreicher Commit-/Checkoutprüfung.

Am produktiven Stand `6d9db1cd` bestehen auf Ubuntu24.04/26.04 je19 Git-/Cache-/Lockfälle, zwei Pinning-/Streamfälle und39 Proxmox-Fälle. Der Runtime-Baum umfasst30 Cargo-Pakete und27 Dienstvorlagen einschließlich TBS; der Agententransfer bleibt ein kleiner gezielter Quelltransfer.

Jans Proxmox-Bericht vom 15.10. auf `SRV-H-PVE-01` bestätigt:

- `planned=28`, `prepared=28`, `updated=28`, Gesamtergebnis `completed`.
- Keine fehlgeschlagenen, eingeschränkten oder ausgelassenen Aktualisierungen.
- Ziele CT102, CT108, CT123–CT147 und CT149.
- Bericht `/var/tmp/netcore-lxc-update-20261015-rpwlliez/report.json`.

Der Quellcommit fehlt in dieser Abschlussübersicht; er wird nicht nachträglich geraten. Die bestätigte Agentenübernahme ist kein bestätigter Backend-Gesamtrollout.

### 4.3 Hängender Controllerstatus und tatsächliche Agentenabschlüsse

Die letzte sichtbare Compilerzeile `Compiling clap v4.6.0` beweist keinen weiterhin laufenden Build. Der Betreiberbericht vom 15.10.,21:44 enthält29 jüngste Dienstziele: acht lokal erfolgreich und21 lokal fehlgeschlagen.

Zwei Agenten haben trotz verlorener Controllerverbindung abgeschlossen: Application-Gateway mit13m19s Buildzeit und `live=true,ready=true`; Control-Room mit16m59s und `live=true,ready=false`. Beide bestätigen den älteren Zielstand `20c3dfa7`. Weitere Endbelege enthalten Timeouts und Git128-Fehler. Ein RAM-/OOM-Fehler wird durch die leeren Compilerproben nicht belegt.

Die Abgleichhelfer aus PR #87 und #93 prüfen exakte Identität, Dienst, Commit, Zeitfenster und einen zweiten Endbeleg. Ungeklärte oder noch laufende Remote-Installationen bleiben geschützt. Ein negativer fachlicher Endzustand wird durch das Auflösen einer Transportsperre nicht erfolgreich.

Jans weitere Ausgaben bestätigen die atomaren Übernahmen für Application-Gateway und Control-Room sowie die vier zuvor fehlenden Quittungen auf warn-control, Security-Core und Media-Library. Die globale Schlussprüfung meldet keine ungeklärten Remote-Aufträge. Die fehlgeschlagenen Installationen bleiben fehlgeschlagen; eine frische WebUI-Sammelvorschau mit anschließendem Backend-Rollout ist weiter offen.

## 5. CT138: tatsächlicher Speicherverbrauch, NAS-Logs und Mediencache

### 5.1 Plattenbefund und Logrotation

Auf Media-Library CT138 belegen Medienassets nur etwa20MB, `/var/log` dagegen22GB physisch auf ZFS. Die sehr große logische Länge von `syslog.1` (`316805205418` Bytes) darf nicht mit tatsächlich belegtem Speicher gleichgesetzt werden. Das NFS4-Ziel unter `/mnt/nfs-share` ist eingehängt.

PR #95 ergänzt die Minutenprüfung ab64MB und Archivpfade nach Host/Jahr/Monat/Tag. Vor lokaler Freigabe wird der übertragene Inhalt über SHA-256 geprüft. Die erste Installation scheitert vor dem Archivstart an fehlenden globalen Ubuntu-`su`-/`create`-Vorgaben im eigenständigen logrotate-Kontext; der Installer stellt die alte Regel wieder her.

PR #96 übernimmt diese Vorgaben explizit. Die native Prüfung an `a3323cd4` auf Ubuntu24.04/26.04 reproduziert den Fehler und bestätigt Rotation sowie Wiederanlage mit ursprünglicher Gruppe/Modus. [Logarchiv auf NAS](../deployment/logarchiv-auf-nas.md) enthält den korrigierten festen Installationsblock. Eine erfolgreiche erneute CT138-Installation und tatsächliche Speicherentlastung sind zum Übergabepunkt nicht belegt.

### 5.2 Medien nach Archivierung zurückholen statt dauerhaft lokal behalten

PR #97 ergänzt den zuvor nur kopierenden Medienarchivpfad um geprüfte private Inhaltsbelege, echte NFS-/SMB-Mountprüfung und begrenzte lokale Vorhaltung. Standardwerte sind2GiB Cacheziel,24Stunden Vorhaltung und2GiB Dateisystemreserve.

Vorschau, Wellenform, Aussendung und erneute Verarbeitung holen ein ausgelagertes Asset zurück. Aktive oder wartende Jobs halten ihren Cache fest; beschädigte Einzelassets verhindern nicht jede weitere Archivierung. Die Abschlussprüfung an `93f8e8bb` besteht auf Ubuntu24.04/26.04 je41 Rustfälle und zwei Integrationsfälle einschließlich echtem HTTP-/Stations-Fixture-Transfer, NAS-Ausfall, Retryfairness und Wiederverwendung einer vollständigen lokalen Crashkopie.

Die [Media-Library-Anleitung](../services/media-library/installation-im-lxc.md) hält den CT138-Updateweg fest. Reale Rückholung, Aussendung und tatsächliche Entlastung bleiben Betreiberabnahmen.

## 6. VPN im fertigen Image und korrigierter Zeitbroadcast

### 6.1 Import des echten Clientexports

Der portable OpenVPN-Export wird zunächst wegen `lport 0` abgewiesen. PR #81 unterstützt gültige numerische lokale Ports, erhält Zertifikats-/Inline-TLS-Material und trennt Zugangsdaten. Weitere Importfehler nennen die ursprüngliche Zeile ohne ihren vertraulichen Inhalt. Der datierte Linux-/Browsernachweis an `4b2e5c7` besteht mit24 VPN-,18 Desktop-,116 Backend- und143 VM-Fällen.

Ein akzeptierter Import ist noch kein aufgebauter Tunnel zum tatsächlichen Server.

### 6.2 Fehlendes Profil und zwei Stunden falsche Funkgerätezeit

Am 16.10. testet Jan das Image an `5ea8d3be`: KDE funktioniert, unter Netzwerkverbindungen fehlt jedoch das VPN. Außerdem zeigt das Funkgerät bei16:40 nur14:40; eine Änderung im Radio Manager löst das Problem nicht.

PR #98 (`main@03422f36920e0a94e0238fc1ceaa13a901177c77`) provisioniert das grafisch sichtbare NetworkManager-Profil **NetCore VPN**, private Zugangsdaten und die Heimnetzregel bereits während der Image-Personalisierung. Die Stationszeitzone wird übernommen; der 48-Bit-Zeitencoder korrigiert UTC/2, Vorzeichen, Viertelstundenoffset, Jahr und reservierte Bits.

Datierte Nachweise:

- Ubuntu24.04/26.04 und Debian Bookworm: jeweils38 Pythonfälle plus echter isolierter zertifikat-/passwortgeschützter VPN-Tunnel, Datenverkehr, Heimnetzabschaltung und Wiederverbindung nach NetworkManager-Neustart.
- Neun Rustfälle für Zeit, Sommerzeitwechsel und PDU-Codierung.
- Echte ARM64-Personalisierung und VM-Übernahmeprüfung mit NM-Profil, geschützten Zugangsdaten, UUID und Stationszeitzone.
- Quellgate, Browser und TBS-Link mit Defaultfeatures bestanden.

[Ausführlicher Bericht und vollständiger VM-Updateblock](../integration/Z01-2026-10-07/image-vpn-und-zeitbroadcast-2026-10-16.md). Die gesonderte Ubuntu24.04-Desktopmatrix ist durch fehlendes `qemu-aarch64`-binfmt blockiert. Der neue vollständige Imagebuild, der echte Standort-/VPN-Wechsel und die Ortszeit am realen Funkgerät bleiben offen.

## 7. SIP-Switch: Stationen pflegen und direkt nach Deploy übernehmen

Jans Zwischenauftrag: Basisstationen im SIP-Switch-WebUI anlegen, bearbeiten und löschen; die zugehörige SIP-TOML herunterladen und in Deploy importieren.

PR #99 ist in `main@d54cb89f27e7c33bba73b0de430c7eac3dee768e` übernommen. Pro Station werden Zentralzugang und eigener PBX-Ersatzweg gepflegt. Leere Passwortfelder behalten bestehende Passwörter. Revisionsprüfung, Schutz aktiver Rufe, atomare Speicherung, Endpointprüfung und Rücknahme gehören zur Umsetzung. Der Download enthält private Phase-11c-Daten und gehört zum passenden gleichnamigen Deploy-Profil.

Die gezielte Abschlussprüfung am produktiven Code `e85cf239` besteht auf Ubuntu24.04/26.04 mit je55 Pythonfällen. Der echte Chromium-Download wird durch Deploy-WebUI, Controller und Worker importiert. Echte Asterisk22.11.0-Änderungen prüfen unter anderem Umbenennen, Passwort/Kontaktlimit, Entfernen und maximale Abschnittslänge.

[Basisstationen im SIP-WebUI](../services/sip-switch/basisstationen-im-webui.md) enthält den vollständigen LXC-Updateblock, einschließlich systemd-Schreibfreigabe für `/etc/netcore` und Runtime-/Discovery-Versionsbeleg. Die Betreiberübernahme und reale TBS-/PBX-/Funkabnahme sind noch offen.

## 8. Letzter Auftrag: Deploy auf leeren Anwendungszustand zurücksetzen

### 8.1 Gewünschter Umfang und vorbereitete Umsetzung

Der letzte Betreiberauftrag lautet sinngemäß: alle Builds entfernen, Auftragsverlauf und Einstellungen leeren und Deploy wieder wie frisch eingerichtet beginnen lassen.

[PR #100](https://github.com/JanHG98/netcore-tetra/pull/100), Branch `fix/deployment-application-reset-20261016`, Head `191f3384131b8e8b49e13c3ad63759d28bf66bc1`, enthält:

- `tools/operator/reset-deployment-vm.py`: Vorschau beziehungsweise ausdrückliches `--apply`.
- `tools/test_deployment_reset.py`:19 Fälle mit realen temporären Dateien/SQLite, Guards und Rücknahme.
- `Docs/deployment/deployment-vm-zuruecksetzen.md`: konkreter VM-Befehlsblock.
- `.github/workflows/deployment-reset.yml`: native Ubuntu24.04/26.04-Prüfung mit eigenen Loopgeräten und privatem Mountnamespace.

Der Helfer leert lokale Images, Buildreste, Image-/Quellcaches, Auftrags-/Sammelverlauf, Profile, Standort-Template, Inventar samt Entfernungsliste und UI-Einstellungen. Konfiguration und private Endbelege werden geschützt unter `/var/lib/netcore-deployment-resets/` erhalten. VM-Netz, SSH, installierter Code und bestehende LXC-Dienste sind nicht Gegenstand des Anwendungsresets.

Lokale Builds dürfen beendet werden; laufende oder ungeklärte Remote-Aufträge blockieren den Reset. Controller und Builder werden mit vollständigen Prozessgruppen gestoppt. Pfade, Cgroups, Mounts auch in fremden Prozessnamespaces und exakt zugeordnete Build-Loops werden geprüft. Andere Loopgeräte dürfen nicht getrennt werden.

### 8.2 Tatsächlicher letzter Prüfstand

Die [native Resetprüfung](https://github.com/JanHG98/netcore-tetra/actions/runs/38962651476) ist am Übergabepunkt **nicht vollständig bestanden**:

| Prüfung | Ergebnis |
| --- | --- |
| Reset Ubuntu26.04 | Alle19 Fälle bestanden |
| Reset Ubuntu24.04 |18 Fälle bestanden; `test_actual_loop_detach_preserves_unrelated_loop` mit Fehler |
| Runtime-Updater Ubuntu24.04/26.04 | Beide erfolgreich |
| Quellkonsistenz / Syslog-Runtime | Erfolgreich |

Der Ubuntu24.04-Fehler lautet `Build-Loopgeraet ist noch belegt; keine Dateien loeschen`. Er entsteht nach `losetup --detach` beim unmittelbar folgenden Sichtbarkeitstest. Der Helfer bricht dabei vor Umbenennen/Löschen der Anwendungsdaten ab. Dieser Prüfstand ist weder eine Freigabe des Resets noch ein Nachweis seiner Ausführung auf VM119.

### 8.3 Konkreter Einstieg im neuen Chat

**Aufgabe Z01.4:** zuerst den noch offenen Loop-Abschluss im Resetpfad beheben und die native Matrix wieder vollständig prüfen. Abnahmekriterium: eigene Build-Loops sind tatsächlich getrennt; verzögertes Trennen wird begrenzt berücksichtigt; bei weiter belegtem Gerät erfolgt kein Dateiumzug/-löschen; fremde Geräte bleiben erhalten.

Danach den vollständigen, auf den geprüften Stand gepinnten Resetblock für `VM-H-DEPLOY-01` bereitstellen beziehungsweise nach vorhandenem Anlagenzugriff ausführen. Erfolg erst bei `phase=completed` und echter HTTP-Bestätigung für Readiness, verfügbaren Builder sowie leere Image-, Build-, Profil- und Sammellisten. Es gibt zum Übergabepunkt keinen bestätigten direkten Anlagenzugriff und keinen erfolgreichen VM119-Reset.

Anschließend Profile/Standortdaten und private SIP-TOML neu einrichten, ein Image mit aktuellem VPN-/Zeitfix bauen und am Pi abnehmen. Neue Backendrollouts, CT138-NAS-Abnahme, physische SIP-/VPN-/Zeitprüfung und Dauerlauf behalten ihren eigenen offenen Status.

## 9. Abschlussgrenzen und Fortsetzungsregeln

- Diese Übergabe bewahrt vorhandene Belege und bezeichnet ihre Grenzen; sie ersetzt fehlende Chat-Originale nicht durch erfundene Vollständigkeit.
- Den offenen Reset-PR fortführen, keinen zweiten konkurrierenden Reset anlegen.
- Eine behobene Controllerquittung, ein fertiger Build, ein gestarteter Prozess und eine fachlich bereite Anlage bleiben unterschiedliche Ergebnisse.
- Für VM- und LXC-Änderungen den jeweils dokumentierten festen Quellstand und passenden Ausführungshost verwenden.
- Vorbereitete Updateblöcke nicht als bereits auf der Anlage ausgeführt kennzeichnen.
- Frühere RF-/Pi-Teilabnahmen bewahren; neuere Korrekturen benötigen die dazu passende erneute Abnahme.

## 10. Fortsetzung im neuen Chat am 16.10.2026

Der Loop-Abschluss wurde in `2166fa3a` korrigiert; `1dab9ec2416d744f065e03a667bdb56ae07c0664` enthält zusätzlich die aufgelöste Dokumentationsübergabe. Der Helfer wartet nach dem Trennen höchstens zehn Sekunden auf das tatsächliche Verschwinden aller ausgewählten Geräte. Bei weiter belegtem Gerät erfolgt weiterhin kein Umbenennen oder Löschen der Anwendungsdateien.

Die neue Regression reproduziert den Fehler mit der vorherigen Quelle und besteht mit der Korrektur. Die [native Matrix](https://github.com/JanHG98/netcore-tetra/actions/runs/38965045061) besteht auf Ubuntu24.04 und26.04 jeweils alle20 Fälle ohne Skip, einschließlich echter eigener/fremder Loopgeräte, privatem Mountnamespace, Symlinkschutz, SQLite und Rücknahme. Der [maschinelle Folgebefund](assets/2026-10-16_deploy-abnahme/reset-fortsetzung-nachweise.json) hält Quelle und Grenzen fest. systemd-Steuerung und die abschließenden HTTP-Antworten sind in den Tests simuliert.

Der [vollständige Resetblock](../deployment/deployment-vm-zuruecksetzen.md#auf-vm-h-deploy-01-ausfuehren) ist auch auf Bash-Syntax geprüft. Seine Ausführung auf VM-H-DEPLOY-01 sowie der leere reale HTTP-Anwendungszustand bleiben vom Betreiber zu bestätigen. Die Angaben in Abschnitt8 bewahren den ursprünglichen, noch fehlgeschlagenen Übergabestand.
