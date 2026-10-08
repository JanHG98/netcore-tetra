# NetCore-Tetra – zentrale Gesamtroadmap

| Feld | Wert |
| --- | --- |
| Roadmap-ID | NETCORE-MASTER-01 |
| Erstellt / aktualisiert | 2026-10-05 / 2026-10-08, Europe/Berlin |
| Geltungsbereich | Gesamtes NetCore-Tetra-Repository: TBS / Funkstack, Core, Deployment, Betrieb, IAM, Drive, Plugins und weitere Integrationen |
| Geprüfter Ausgangsstand | Z01: `main@dae9363062a1442664a083e1fc32e6944b3be9a6`, historische Quelle `bbf039729b9b05f8d623b11195ca24a124f68d16`; Integration `67b2f0d6e2fbcbf7013ee9b0dae9b2a25c2d9315` auf `feature/z01-deployment-consolidation` |
| Ergänzende Gruppenprüfung | `main@07609fb56f412ebe6e36655323e8fc6359cf90ec`, 2026-10-05; statischer Befund für Z02.5 |
| Planungsstand | Reihenfolge vom Nutzer zur zentralen Ablage freigegeben; technische Aufgaben bleiben offen, soweit kein eigener Nachweis vorliegt |
| Aktueller erster Schritt | **Z01.4: Logmarker über TCP-Syslog, lokale Vorschau und gzip-NAS-Archiv auf CT136 abnehmen; anschließend ARM64-/Pi-/VPN-Abnahme** |
| Zeitplanung | Arbeitsblöcke und Abnahmebedingungen; keine zugesagten Kalendertermine |

**Dies ist der zentrale Einstieg für „Was machen wir als Nächstes?“.** Die Gesamtpriorität steht hier. Fachroadmaps beschreiben den jeweiligen Umfang und die technische Abnahme. Ältere Phasenlisten oder archivierte Gesprächsstände ersetzen diese Reihenfolge nicht. Vor einer Empfehlung den aktuellen Repositorystand und die zuletzt dokumentierten Ergebnisse prüfen; spätere ausdrückliche Nutzerentscheidungen haben Vorrang.

## 1. Aktueller nächster Schritt

**Z01.4 – Installation, Upgrade und Recovery auf dem gemeinsamen Stand abnehmen.**

Z01.1 ist abgeschlossen: die vollständigen Bäume `main@dae9363` und `bbf0397` wurden direkt verglichen (2.655 Pfade, 75 historisch fehlende Dateien, 72 unterschiedliche gemeinsame Dateien). Übernahmepakete, Inhaltskonflikte, konkrete Konfigurationsentscheidungen, Tests und Rückwege stehen im [Z01-Integrationsbericht](Docs/integration/Z01-2026-10-07/README.md), einschließlich vollständiger Datei-/SHA-Tabellen.

Z01.2 ist auf `feature/z01-deployment-consolidation` implementiert und lokal geprüft: Deployment/Discovery/Imagebuilder/Pi-VPN sowie Syslog aufgenommen, aktuelle UI/Fachänderungen erhalten. Z01.3 vereinheitlicht 26 Inventardienste, Katalog/Registry/Health/Fallback, erzwingt semantische Readiness vor Folgepaketen und bewahrt vorhandene Konfigurationen. Gemeinsames Quellgate und isolierte Unit-/HTTP-/Wire-/Native-/Browserprüfungen bestehen. Die [Nachweise](Docs/integration/Z01-2026-10-07/validation.json) nennen Umfang, übersprungenen Unix-Workertransport und offene Grenzen.

[PR #62](https://github.com/JanHG98/netcore-tetra/pull/62) ist übernommen: `main@c45a2ec1f5b7cdcfb766a2d81e8901b65f3dbacf`. Die drei main-Workflows Deployment-Inventar/Quellgate, Deployment/Discovery und Warning Service sind am tatsächlichen Mergecommit erfolgreich abgeschlossen. Details und datierte Anlagenbefunde stehen im [Z01.4-Laufzeitnachtrag](Docs/integration/Z01-2026-10-07/runtime-z014.md); der [frühere CI-Nachtrag](Docs/integration/Z01-2026-10-07/ci.md) bleibt als Historie erhalten.

**Erreichte Anlagenabnahme:** Der Betreiber hat VM119 (`VM-H-DEPLOY-01`, `10.0.1.131`) auf diesen Commit aktualisiert und den Git-Prüfauftrag erfolgreich abgeschlossen. Auf dem isolierten Ubuntu-24.04-Testagent CT150 (`z014-test-hw`, DHCP derzeit `10.0.1.191`) bestehen Hardware-Gateway-Neuinstallation und Wiederholungsupdate auf denselben Commit. Der benutzerdefinierte Konfigurationswert `heartbeat_timeout_secs=37` und die vollständige Konfigurations-Prüfsumme bleiben erhalten; physische Ausgänge sind deaktiviert. Dies bestätigt noch keinen Hardware-Versionswechsel, MQTT-Fachbetrieb, Flottenrollout oder vollständigen ARM64-/Pi-/NAS-/On-Air-Nachweis.

**Nächster ausführbarer Betriebsauftrag:** CT150 besteht die tatsächliche negative Readiness und Recovery mit dem korrigierten Agent (`31829fc`) und Helper `b2c0208`: erwartetes failed / unbestätigter Marker, danach succeeded / installed c45a2ec; Konfiguration und Wert37 erhalten, keine Jobstatus-GET-Fehler / HTTP500. [Ergebnisbericht](Docs/integration/Z01-2026-10-07/evidence/ct150-readiness-2026-10-08.json). CT136 (`Observability`, `10.0.1.143`) ist inzwischen vom Betreiber erfolgreich mit dem gepinnten Stand `3d96a9a` aktualisiert: Management- und localhost-API bereit, Originalkonfiguration bytegleich erhalten, Preview-Puffer0 / kein Fehler. Echte NFS-Schreib-/Lese-/Entfernungsprüfung als Dienstbenutzer besteht. [Befund und Prüfgrenzen](Docs/integration/Z01-2026-10-07/observability-ct136.md). Jetzt einen eindeutigen Logmarker über TCP514 empfangen, in beiden NMS-Ansichten wiederfinden und nach dem echten Archivdienstlauf im gzip auf dem NAS prüfen. Ein leerer Preview-Puffer ersetzt diesen Zustellnachweis nicht. Vollständiger ARM64-Build, physischer Pi/SXceiver und VPN-Wechsel sowie weitere Controller-/NAS-Ausfall- und Versionswechselnachweise bleiben offen.

Gezielte Z02-P0-Fixes bleiben parallel möglich. Die Implementierung von Z01.1–Z01.3 behebt keine Restore-/Gruppenhandler-/IP-Routenschutz-Lücken automatisch.

## 2. Belegter Ausgangsstand und seine Grenzen

Historischer Ausgangsbefund vom 05.10.2026: `main@e5d825b33db2e1fce14bcb2cc23e73c241873a18`, Archiv `9af2de92b120dc612d83a17f78eeee9a97f2f775`; Gruppenprüfung zusätzlich an `07609fb56f412ebe6e36655323e8fc6359cf90ec`. Die Tabelle bewahrt diesen Befund. Der aktuelle Z01-Abschluss und seine Grenzen stehen in Abschnitt 1 und im Integrationsbericht; die alte Übernahmelücke/25-24-Drift ist auf dem Z01-Arbeitsbranch behoben.

| Bereich | Belegter Stand am 2026-10-05 | Konsequenz |
| --- | --- | --- |
| Branches / Archiv | Branchliste am 05.10.: `main` und `Archiving`; Archivzweig ergänzt Gesprächsdokumentation / Assets, keinen fehlenden Deployment-Dienst | Archiv als Wissensquelle nutzen; alte Vorschläge von späteren Festlegungen unterscheiden |
| Deployment / Discovery / Syslog | Historischer Feature-Stand enthält `system-backend/deployment-core/`, Image- / VPN-Bausteine, neuere Observability-Discovery und rsyslog- / Archivpakete; diese fehlen im geprüften main | Z01 zuerst; es handelt sich um vorhandene Entwicklungsarbeit mit fehlender Übernahme |
| Bestandsdrift | [Inventory](deploy/open-lab/inventory.example.toml): 25 Dienste; [generierter Katalog](deploy/open-lab/generated/service-catalog.json) und [statischer Audit](Docs/generated/full-system-integration-audit.md): 24; älterer Text nennt 17 / 18 | Inventory, Generatoren, Endpunkt- / Fallbackmatrix und Texte konsistent machen; keine Live-Dienstzahl daraus behaupten |
| TBS / Core | Funkstack und zahlreiche zentrale Dienste vorhanden; Mock- / statische Prüfungen und echte Endgeräteabnahme sind getrennte Nachweisstufen | Aktive Runtimepfade, Service-Matrix, Fachfunktionen und Ausfälle abnehmen |
| Zentrale Gruppenzuweisungen | Group Core sendet `GroupAccessPolicyApply` / `GroupDgnaApply`; der TBS-Worker routet beide an MM. Der aktive [MM-Dispatcher](crates/tetra-entities/src/mm/mm_bs.rs) verarbeitet für die Gruppensteuerung jedoch nur das lokale `Dgna` und ignoriert die zentralen Typen als nicht unterstützt. `group_policy=false` wird weiterhin angekündigt | Z02.5 P0: vorhandene zentrale Befehle tatsächlich umsetzen, Fähigkeiten korrekt melden und Ergebnisse bis zum Endgerät nachvollziehen; Codebefund, keine neue Live-Abnahme |
| Restore-Stabilität | [Aktiver Restorehandler](crates/tetra-entities/src/cmce/subentities/cc_bs/procedures/restoration.rs) benötigt weiterhin beide Korrekturen aus Abschnitt 4 | Z02.1 / Z02.2 vor weiterem Restore- / Handover-Ausbau |
| IP Gateway | [Routenvalidierung](system-backend/ip-gateway/src/state.rs) und [Kernel-Anwendung](system-backend/ip-gateway/src/kernel.rs) schützen Management- / Packet-Core-Ziele nicht vor überlappenden TUN-Routen | Z02.3; aktueller Codebefund, keine Behauptung einer aktuellen Anlagenstörung |
| Dienst-WebUIs / Dark Mode | [PR #59](https://github.com/JanHG98/netcore-tetra/pull/59) ist integriert | Ausrollen und Funktionen prüfen; keine erneute komplette Designrunde als Startaufgabe |
| Mehrzellenbetrieb | Fortgeschrittene Bausteine vorhanden; [MAIN-COMPAT](Docs/CENTRAL_NETWORK_ROLLOUT.md) hat noch keinen vollständig integrierten MM- / CMCE-Restore-Kontexttransfer für laufende Rufe | Z07 stufenweise; vorhandene Tests allein sind kein laufendes Seamless Handover |
| IAM / Drive | [IAM](Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md) und [Drive](Docs/NETCORE_DRIVE_ROADMAP.md) sind geplant; Control Room besitzt bereits eigene lokale Konten / Rollen | IAM-Pilot planen; Drive lokal unabhängig davon starten |
| Sicherheit / Regionen | Viele Dienste unterstützen aktuell ausschließlich Open-Lab-Management; [KMF](system-backend/kmf/README.md) hat Lab-Provider / spätere Air-OTAR-Grenzen, [Transit](system-backend/transit/README.md) ein NetCore-eigenes Protokoll | Gesicherter Betrieb, normative Funk-Security und ETSI ISI sind eigene Entwicklungs- und Abnahmeaufgaben |

„Im Code“, „statisch geprüft“, „im Labornetz geprüft“ und „On Air geprüft“ bleiben getrennt. Ein eingechecktes PASS oder ein grüner historischer CI-Lauf beweist keine aktuelle Gesamtanlage. Die [Nachweisgrenzen](wiki/Projektstand.md) und der [E2E-Runner](tests/e2e/README.md) erläutern die Stufen; dort genannte ältere Dienstzahlen müssen im Zuge von Z01 abgeglichen werden.

## 3. Gesamtfolge und parallele Arbeitsstränge

Alle folgenden technischen Arbeitsblöcke sind **offen / geplant**, soweit die Statuspflege keinen eigenen Abschlussnachweis einträgt. Vorhandene Implementierungen werden innerhalb des Blocks vervollständigt und abgenommen, statt nochmals als Neubau behandelt zu werden. Die IDs sind dauerhaft; parallele Blöcke müssen nicht numerisch nacheinander fertig werden.

| ID | Priorität / Zeitpunkt | Arbeitsblock | Abhängigkeit | Abschluss / nächster Übergang |
| --- | --- | --- | --- | --- |
| Z01 | P0 · aktueller erster Block | Z01.1–01.3 auf main übernommen / CI bestanden; Z01.4 in Arbeit, Controllerupdate und isolierter Hardware-Install-/Wiederholungspilot bestanden | CT150-Readiness/Recovery bestanden; CT136-Binaryupdate / beide APIs und NAS-Dateizugriff bestanden; Logmarker bis ins gzip-Archiv prüfen | [Z01-Bericht](Docs/integration/Z01-2026-10-07/README.md), [Laufzeitnachtrag](Docs/integration/Z01-2026-10-07/runtime-z014.md); Teilabnahme, keine vollständige Flotten-/On-Air-Abnahme |
| Z02 | P0 · sofort, gezielt parallel zu Z01 | Beide Restore-Fixes, Schutz von Management- / Packet-Core-Netzen und tatsächliche Ausführung zentraler Gruppenzuweisungen auf der TBS umsetzen; aktive SAP- / Downlinkwege prüfen | Aktueller Funk- / Gateway-Code; technische Änderungen getrennt integrierbar | Gezielte positive / negative Regressionen; keine doppelte Sprechfreigabe, Uplink-Watchdog aktiv, keine schädliche TUN-Route; Gruppenaufträge mit fachlichem Ergebnis statt stillem Ignorieren |
| Z03 | P0 · erstes Systemgate nach Z01 / Z02 | Einzelzelle und Core-Anbindung abnehmen; lokale Funkfunktionen, Dual Carrier, Matrix und Edge-Fallback | Z01 / Z02 für den gemeinsamen geprüften Stand | Reproduzierbare Labor- und On-Air-Nachweise mit Motorola / Sepura; Build, Konfiguration, Gerätefirmware und Logs festgehalten |
| Z04 | P1 · Architektur ab jetzt, anschließend Pilot | Zentralen IAM- / RBAC-Plan umsetzen: Inventur / Rollenvertrag, Identity-Pilot, eine TBS und Control Room, danach Dienste | M0 / M1 können parallel beginnen; Pilot auf isolierter, prüfbarer Baseline; breite Migration nach Pilotabnahme | Anmeldung, Ressourcenrechte, Sperren, Maschinenidentitäten, Ausfall und Rückweg funktionieren; Details IAM M0–M8 |
| Z05 | P1 · Alltagspfade nach Grundabnahme | SDS / Status / GPS, HA / MQTT, Warnzentrale, SIP / RTP, Recording, TTS / Media Library und Paketdaten durchgehend schließen | Z03 für belastbare Systemabnahme; einzelne Diagnosen können vorher erfolgen | Tatsächliches Ziel erreicht; Rückmeldung nachvollziehbar; Neustart, Deduplizierung und Abhängigkeitenausfall geprüft |
| Z06 | P1 · eigener Produktstrang nach den ersten P0-Fixes | Drive-Grundversion mit lokalem Login, Konten / Gruppen, Dateien, Freigaben, Versionen, Papierkorb, Sicherung und Sync | Kein zentraler IAM-Dienst nötig; eigene Backend- / Betriebsentscheidung D0 | Eigenständiger Betrieb einschließlich externem Ordnerzugang und Restore; Details D0–D5; Plugin-Vertrag von Beginn an |
| Z07 | P1 · nach stabiler Einzelzelle / Core-E2E | Zwei TBS und Mehrzellenbetrieb stufenweise integrieren: neue Registrierungen / Rufe, Kontexttransfer / Restore, laufende Medienpfade | Z02 / Z03 und relevante Z05-Pfade; zweite TBS separat abgenommen | Erst danach Seamless-Handover-Abnahme mit gemessener Audio-Unterbrechung, Floor und Fehlerfällen |
| Z08 | P1 · weiterer praktischer Ausbau | Leitstellen-Audio / Headset / Mikro / PTT; reale HA-Aktoren, Rack-Sensoren / RF-Messhardware; Asset- / Task-Rückweg | Funktionsfähiger Pilot für den jeweiligen Z03 / Z05-Pfad; betroffene Schreibrechte geregelt | Bedienung und Hardware zeigen tatsächlichen Zustand; Auftrag / Wartung abgeschlossen; Timeout und Wiederkehr geprüft |
| Z09 | P1 · nach Drive-Dateikern, parallel zum IAM-Ausbau | Drive-Plugin-Plattform: PDF / Bilder / Audio / Video, danach Office und technische Module für 3D / Schaltpläne / Vektoren | D7 nach D0 / D2–D4; D8 / D9 nach D7; keine D6- / IAM-Startabhängigkeit | Pro Format Anzeige, Bearbeitung / Export und identische Rechte geprüft; Plugin-Update / Deaktivierung / Rückweg funktionieren |
| Z10 | P1 / P2 · auf abgenommenem Grundbetrieb | Gesicherten Betrieb vervollständigen, Last / Dauerlauf, Upgrade / Restore; danach Backend-HA, produktive Funk-Security / OTAR, Mehrregionen und ETSI ISI | IAM-Pilot / relevante Dienstmigration; stabile Fachpfade; eigene Architektur je Ausbau | Gemessene Betriebs- / Wiederherstellungsgrenzen und eigener Sicherheits- / Interoperabilitätsnachweis je Ausbaustufe |
| Z11 | P2 · danach nach praktischem Nutzen | Android / Hybrid, Zebra / QR-Inventur, WERMA, USB-Audio / PTT, FRN / Zello / TETRA-Empfangsgateway (Z11.1) / weitere Brücken, weitere Fachanwendungen | Bestehende autoritative Dienste und passender Pilot; kein konkurrierendes Registry- / Rechte-System | Klare Zuständigkeit, benötigte Hardware und eigener Abnahmepfad; archivierte Visionen erst nach aktueller Prüfung übernehmen |

Arbeitsorganisation: Z01 / Z02 zuerst fokussiert abschließen; IAM-Inventur und Drive-Architektur dürfen parallel entstehen. Danach Betriebs-E2E, IAM-Pilot und lokaler Drive-Betrieb als getrennte Stränge mit jeweils prüfbaren Zwischenergebnissen führen. Die Roadmap ist keine Aufforderung, alle großen Blöcke gleichzeitig zu starten.

## 4. Die nächsten konkreten Arbeitspakete

### Z01 – konsistenten Gesamtstand herstellen

| Aufgabe | Aktueller Status / Nachweis | Ergebnis / Abnahme |
| --- | --- | --- |
| Z01.1 · vollständiger Quellvergleich und Integrationsplan | Erledigt gemäß Quellabnahme 2026-10-07 | [Vollständiger Vergleich, Entscheidungen und Plan](Docs/integration/Z01-2026-10-07/README.md) mit beiden SHAs und Grenzen; gültigen Vergleich nicht erneut durchführen |
| Z01.2 · fehlende Entwicklung kontrolliert übernehmen | Erledigt gemäß Quell-/CI-Abnahme; PR #62 auf `main@c45a2ec` übernommen, 2026-10-08 geprüft | Deployment / Discovery, Imagebuilder / VPN und Syslog aufgenommen; aktuelle UI und Standortwerte erhalten; Anlagenabnahme separat in Z01.4 |
| Z01.3 · Inventory, Ready-Schranke und CI vereinheitlichen | Erledigt gemäß Quell-/CI-Abnahme; main-Workflow erfolgreich, 2026-10-08 geprüft | 26er Registry / Inventory / Katalog / Matrix konsistent; semantisch negatives Ready beendet Deployment; Config-Erhalt und Drift-/Negativfälle geprüft |
| Z01.4 · Installation, Upgrade und Recovery abnehmen | In Arbeit, 2026-10-08: Betreiberbefunde an `c45a2ec`, Controllerupdate / CT150 Hardware-Install und Wiederholungsupdate PASS; Agentkorrektur 31829fc sowie tatsächliche negative Readiness / Recovery mit Helper b2c0208 PASS; [Nachweis](Docs/integration/Z01-2026-10-07/runtime-z014.md) | CT136-Previewzugang installiert / geprüft; TCP-Logmarker und gzip-NAS-Archiv abnehmen; echter Hardware-Versionswechsel, Controller-/NAS-Ausfall, begrenzte Logpuffer / Archivprüfung; vollständiger ARM64-Build und physischer Pi-/SXceiver-Boot mit VPN-Wechsel weiter offen |

Historische Tests des Imagebuilders belegen Teile der Image-Personalisierung und VM-Installation. Sie ersetzen keinen vollständigen NetCore-Image-Build und keinen physischen Pi-/SXceiver-Test. Quellarchive, Binärartefakte und OS-Images erhalten jeweils passende Versions- / Prüfsummennachweise; nach gemeinsamer Abnahme einen konsolidierten Release festlegen.

### Z02 – gezielte Stabilität vor weiterem Funk-Ausbau

- **Z02.1 Einzelruf-Restore:** Sprechrecht nur vergeben, wenn frei oder bereits beim anfragenden Teilnehmer; passenden Floor-Owner bei Simplex setzen. Gegensprechenden Teilnehmer, gleiche / andere Identität und erneuten Restore prüfen.
- **Z02.2 Gruppenruf-Restore:** Nach Floor-Grant die untere Funksteuerung benachrichtigen, damit ausbleibende Uplink-Sprachframes überwacht werden. Stummen Restoreteilnehmer, Timer, Release und sofortige Wiederverwendung prüfen.
- **Z02.3 IP-Routenschutz:** Managementadresse / -netz und Packet-Core-Abhängigkeiten vor unzulässigen TUN-Routen schützen: beim Schreiben, vor Kernel-Reconcile und beim Wiederherstellen gespeicherter Regeln. Historisch gespeicherte Konflikte mit behandeln; daraus keine aktuelle Live-Störung ableiten.
- **Z02.4 aktive Runtimepfade:** Tatsächlich verwendete SAP- / SNDCP- / MLE-Downlinkpfade und Capability-Anzeigen mit dem Codebestand abgleichen. Isolierte Restore- / Two-Cell-Bausteine erst nach Laufzeitintegration als Funktionsfortschritt melden.
- **Z02.5 zentrale Gruppenzuweisungen:** Gruppenprofile, Mitgliedschaften und zentrale DGNA-Aufträge auf der Basisstation vollständig verarbeiten und die nötigen Attach- / Detach-Aktionen ausführen; Details und Abnahme im folgenden Arbeitspaket. Offen / geplant, P0; unabhängig vom Z01-Quellvergleich bearbeitbar.

### Z02.5 – zentrale Gruppenzuweisungen auf der TBS umsetzen

**Ziel:** Alle unterstützten Gruppenaufträge vom Group Core werden auf der zuständigen Basisstation fachlich umgesetzt und mit einem korrelierten Ergebnis beantwortet. Sie dürfen nicht im allgemeinen Zweig für unbekannte / nicht unterstützte Befehle verschwinden. Dieser Auftrag ergänzt die Planung; die technische Behebung ist noch offen. **Z01.4 ist nach der Quellenkonsolidierung der nächste Z01-Schritt**, Z02.5 kann als gezielte P0-Arbeit parallel erfolgen.

**Belegter Anschlussbedarf an `main@07609fb56f412ebe6e36655323e8fc6359cf90ec`:** [Group Core](system-backend/group-core/src/state.rs), [Befehlsvertrag](crates/tetra-entities/src/net_control/commands.rs) und [TBS-Worker](crates/tetra-entities/src/net_control_room/worker.rs) kennen `GroupAccessPolicyApply` und `GroupDgnaApply` bereits. Im [MM-Dispatcher](crates/tetra-entities/src/mm/mm_bs.rs) fehlen beide Handler. Der vorhandene lokale `do_dgna`-Pfad aktualisiert Gruppenstände und reiht eine Funknachricht ein; Terminalantworten werden bisher nur protokolliert. Die [Capability-Ankündigung](crates/tetra-entities/src/net_control_room/protocol.rs) meldet `dgna=true`, aber `group_policy=false`. Das ist eine statische Quellprüfung, kein Nachweis einer aktuell laufenden Installation.

- **Dispatcher und Rückweg schließen:** Beide zentralen Typen validieren, ausführen und mit `GroupAccessPolicyApplied` / `GroupDgnaApplied` beantworten. Vorhandenen lokalen DGNA-Pfad weiterverwenden und erhalten. Capability-Ankündigung, Schema / Version und Backend-Auswahl müssen zur tatsächlich implementierten Unterstützung passen. Ein an MM verteilter Auftrag allein ist kein fachlicher Erfolg.
- **Gruppenregeln und Mitgliedschaften übernehmen:** Revisionierte Gruppenprofile und Teilnehmerzuweisungen einschließlich `class_of_usage`, `auto_attach`, `locked`, Mitgliedschaftsprüfung und `reconcile_registered` nach dem vorhandenen Vertrag anwenden. Entzogene Mitgliedschaften wirksam entfernen; andere bestehende Gruppen erhalten. Gruppenruf- und SDS-Zustand in MM, Subscriberzustand und CMCE konsistent halten.
- **Am richtigen Funkgerät ausführen:** Zentrale DGNA-Zuweisung und Entziehung über den vorhandenen Attach- / Detach-Funkpfad an den registrierten Teilnehmer der zuständigen TBS bringen. Gültige ISSI / GSSI, Gerätefähigkeit und statische versus dynamische Gruppen unterscheiden. `force` bleibt ein bewusster Operator-Override gemäß Vertrag und ersetzt weder Registrierung noch gültige Gruppenadressen. Ein Wechsel der Serving-TBS darf keinen falschen Abschluss erzeugen.
- **Fehler und Wiederkehr behandeln:** Unbekannte / nicht unterstützte Befehlsarten, ungültige Gruppen, unzulässige Mitgliedschaften, Offline-Teilnehmer, Serialisierungs- / Sendefehler, Terminalablehnung und Timeout liefern ausdrückliche korrelierte Fehler oder einen klaren ausstehenden Zustand. Keine stillschweigende Ignorierung und kein positiver Abschluss ohne passende Wirkung. Doppelte Aufträge, veraltete Policyrevisionen, konkurrierende Zuweisung / Entziehung, Reconnect und erneute Registrierung dürfen entzogene Rechte nicht wiederherstellen; Abgleich und begrenzte Wiederholungen auf den aktuellen Sollzustand beziehen.

Die Ergebnisführung unterscheidet diese Nachweisstufen:

| Stufe | Erforderlicher Nachweis |
| --- | --- |
| Im Core gespeichert / von TBS angenommen | Auftrag und Ziel sind bekannt; Annahme oder Worker-ACK belegt noch keine Umsetzung |
| Lokal auf TBS angewendet | Gruppenregeln und lokale Mitgliedschaften sind tatsächlich übernommen; fachliche Antwort nennt Ergebnis und gegebenenfalls Teilfehler |
| Funkaktion eingereiht / ausgesendet | Erfolgreiches Einreihen und tatsächliches Aussenden getrennt belegen; Queueing bestätigt keine Endgerätewirkung |
| Am Endgerät bestätigt | Terminalantwort dem Auftrag zuordnen, soweit Protokoll / Gerätefähigkeit dies ermöglichen; andernfalls Wirkung ausdrücklich als unbestätigt führen und bei der On-Air-Abnahme prüfen |

Jeder Auftrag bleibt über Core, Node Gateway, TBS und Rückmeldung mit Korrelations-ID, ISSI / GSSI, zuständiger TBS und relevanter Policyrevision nachvollziehbar. Mehrere gleichzeitige Gruppenänderungen und Teilfehler brauchen eindeutige Reihenfolge und Abschlusszustände.

**Abnahme Z02.5:** Reproduzierbarer zentraler Auftrag → Node Gateway → MM → lokale Gruppenstände → Funkgerät → korrelierte Rückmeldung. Zuweisung und Entziehung am realen Endgerät prüfen; lokale Gruppenrufe / Gruppen-SDS müssen den wirksamen Stand verwenden. Bestehendes lokales DGNA bleibt funktionsfähig. Negative Fälle und Wiederkehr aus den obigen Punkten prüfen; Core-Sollzustand, TBS-Zustand und tatsächliche Endgerätewirkung getrennt dokumentieren. Gezielte Handler- / Vertragsprüfungen ersetzen die On-Air-Abnahme nicht.

### Z03 / Z05 – vorhandene Funktionen als Abläufe abnehmen

- Eine TBS meldet sich am richtigen Gateway; Serving-TBS, Gruppen und aktuelle Service-Matrix stimmen. Ausfall / Isolation und kontrollierte Wiederkehr erhalten die dokumentierten lokalen Funktionen.
- Zentrale Gruppenprofile / Mitgliedschaften und DGNA aus Z02.5 durchgehend abnehmen: Zuweisung und Entziehung am richtigen Endgerät, korrelierte Ergebnisse, Gruppenruf / SDS danach, andere Gruppen unverändert. Wiederholung, veraltete Policy, Offline / erneute Registrierung, Zuständigkeitswechsel, Terminalablehnung und unbekannter Befehl dürfen keinen falschen Erfolg erzeugen.
- Registrierung, Einzel- / Gruppenruf, Simplex-Floor, Hangtime / Release und freie Timeslots funktionieren. Dual Carrier mit tatsächlicher Belegung prüfen; Carrierzahl ist kein Nachweis zusätzlicher eigenständiger Kontrollkanäle.
- SDS / Status vom Endgerät über TBS / Router bis Control Room und den vorgesehenen HA-Pfad verfolgen; einen nachvollziehbaren Rückweg prüfen. Verbundene MQTT-Clients allein beweisen diesen Ablauf nicht.
- Warnungen, Alarm- und Taskaufträge auf Neustart / Timeout / Wiederholung prüfen. TBS-Annahme, Endgerätzustellung und Lesebestätigung unterscheiden.
- TTS-Erzeugung, Medienfreigabe, Archiv, vollständige hörbare Aussendung und Rufabbau als einen Ablauf prüfen; Recorder- / NAS-Ausfall darf den RF-Pfad nicht blockieren.
- SIP-Signalisierung und bidirektionales RTP, Fallback für neue Rufe sowie Packet-Data-Uplink / Downlink / WAP mit der passenden Teilnehmer- und Kontextzuordnung abnehmen.
- Ergebnisse mit [E2E-Artefakten](tests/e2e/README.md) und getrennter [On-Air-Abnahme](wiki/Abnahme.md) dokumentieren; Ausfälle von LXC, Gateway, VPN, Speicher und Strom samt Rückkehr berücksichtigen.

## 5. Grenzen und Reihenfolge der späteren Stränge

**IAM und gesicherter Betrieb:** M0 / M1 jetzt vorbereiten, dann isolierten Pilot und eine TBS / Control Room. Breite Migration nach erfolgreicher Abnahme; lokale Control-Room-Rollen berücksichtigen. Zentrales Web-IAM, TETRA-Authentisierung und KMF sind getrennte Aufgaben. Dienste mit ausschließlich `open_lab` benötigen echte Implementierungsarbeit für geschützte APIs / Transporte. Backup / Restore bereits in frühen Blöcken prüfen; Backend-Hochverfügbarkeit ist ein eigener späterer Entwurf und nicht durch Edge-Fallback belegt.

**Drive und Browser-Plugins:** Zunächst lokale Konten / Gruppen, stabiler Dateikern und vollständige Freigaberechte. PDF-, Bild-, Audio- und Videobasis auf den versionierten Plugin-Vertrag aufbauen; anschließend Word / Excel / Präsentationen sowie 3D / CAD, Schaltpläne und Vektoren. Zentrale Drive-Anmeldung D6 folgt, wenn IAM bereit ist, und erhält Eigentum / Freigaben. Weder D0–D5 noch D7–D9 warten auf D6. Externe Freigaben benötigen auch lokal ihre abgesicherte Anmeldung bzw. begrenzten Linkzugang, HTTPS und serverseitige Rechte; dafür keine offenen Open-Lab-Oberflächen verwenden.

**Mehrzellenbetrieb:** Zweite TBS unabhängig abnehmen; Zellregistrierung und neue Rufe zuerst. Danach MM- / CMCE-Kontexttransfer und Restore, zentralen Medienpfad und Floor integrieren. Erst anschließend laufende Gruppen- / Einzelrufe und gegebenenfalls SIP-Rufmigration als Seamless-Ziel testen. Unterbrechung messbar erfassen; zwei erreichbare Zellen oder bestandene isolierte Tests sind kein solcher Nachweis.

**Leitstellen, Hardware und Integrationen:** Bestehende Control-Room-, IoT-, RF-, Asset-, Task-, Application-Gateway- und Media-Library-Dienste verwenden. Reale Aktor-Rückmeldung und geschlossenen Wartungsprozess herstellen. Sensorik zunächst mit einem funktionierenden Pilot, kalibrierte RF-Messung und physische Ausgänge gesondert prüfen. Headset / PTT / Audio-Konsole und mobile Clients erhalten eigene Ressourcen- / Ausfallregeln. Bereits integriertes Design ausrollen und prüfen; kein erneuter kompletter Designneubau als Voraussetzung.

**Regionen / Funk-Security / weitere Produkte:** Vorhandenen NetCore-Transit durch reale Mehrregionen- / Failovertests führen; standardisiertes ETSI ISI gesondert entwickeln. Lab-Provider / Lab-Vault von normativer Funk-Security, HSM und tatsächlichem Air-OTAR trennen. Archivideen wie FRN, WERMA, Zebra, USB / Hybrid oder Gesprächssimulator nach praktischem Nutzen und aktuellen Zuständigkeiten aufnehmen, ohne historischen Konzeptstatus als heutigen Betrieb auszugeben.

### Z11.1: Einseitiges TETRA-Empfangsgateway

**Priorität / Status:** P2, langfristiger Backlog, offen / geplant; auf Nutzerwunsch vom 08.10.2026 aufgenommen. Keine Terminbindung und kein akuter Implementierungsauftrag. Die laufenden Z01-/Z02-Aufgaben und die aktuelle Gesamtfolge behalten Vorrang.

**Ziel:** Empfangene TETRA-Gruppengespräche aus einem oder mehreren Quellnetzen in zugeordnete NetCore-Empfangsgruppen einspeisen, einschließlich zuverlässig dekodierter Sprecherkennung. NetCore-Funkgeräte hören die Gespräche über auswählbare Receive-only-Gruppen; ein Sprachrückweg ins Quellnetz ist nicht vorgesehen.

- **Empfang und Decoder:** SDR++ `tetra_demodulator` im NETSYMS-Modus liefert einen UDP-Bitstream an einen externen TETRA-Decoder, beispielsweise auf Osmocom-Basis. OSMO-TETRA ist die alternative lokale Decoder-Betriebsart desselben Plugins; beide Modi sind keine eigenen Quellnetze. Das Gateway muss Sprache und Signalisierung pro Ruf zusammenführen und Verkehrskanalwechsel berücksichtigen. Streaming allein stellt weder vollständigen Netzempfang noch zugängliche Sprache sicher.
- **Identitäten und Routing:** Quellnetz und MCC/MNC-Kontext zusammen mit ISSI/GSSI erfassen; Gruppen und Sprecher auf eindeutige NetCore-GSSI/ISSI abbilden. Gleiche Kurzkennungen aus verschiedenen Netzen dürfen nicht kollidieren. Originalkennungen nur bei passender, konfliktfreier Zuordnung übernehmen; Herkunft zusätzlich in Anzeige / Logs erhalten. Fehlende oder unsichere Sprecherkennung als unbekannt führen oder eine ausdrücklich konfigurierte Gateway-Identität verwenden.
- **NetCore-Anschluss und Medien:** Die vorhandene [Brew-Gruppenrufsignalisierung](crates/tetra-entities/src/net_brew/protocol.rs) mit Quell-ISSI, Ziel-GSSI und Rufzuständen als möglichen Einstieg prüfen; Audio beziehungsweise Sprachframes in das benötigte Medienformat bringen. Rufbeginn, Sprecherwechsel, Übertragungsende, Hangtime und Rufabbau korrelieren; Streamverlust oder Decoder-Neustart dürfen keinen hängenden Ruf hinterlassen.
- **Empfangsgruppen und Sendesperre:** Motorola-/Sepura-Geräte passend zum Modell und Codeplug als Receive-only konfigurieren und am realen Gerät prüfen. NetCore zusätzlich so erweitern, dass lokale Teilnehmer auf diesen Gruppen weder neue Sprachrufe beginnen noch Sprechfreigaben in laufenden Rufen erhalten; Einspeisung nur über den vorgesehenen Gateway-Pfad zulassen. Normale Registrierung und Gruppenanmeldung im NetCore-Netz bleiben möglich.

**Eintrittskriterien:** Stabile Einzelzelle und abgenommener Brew-/Gruppenrufpfad aus Z03 / den relevanten Z05-Pfaden; nutzbarer Decoder mit belegbarer Audio-/Metadatenzuordnung. Zunächst ein Quellnetz und eine isolierte Testgruppe, danach mehrere Quellen. Bidirektionale Netzkopplung und ETSI ISI bleiben eigene Vorhaben.

**Abnahme:** Hörbare Einspeisung mit korrekter Zielgruppe und belegbarer Sprecherzuordnung; wechselnde Sprecher und kollidierende Kurzkennungen aus zwei Quellnetzen korrekt behandeln. Fehlende Metadaten, Kanalwechsel, Streamabbruch, Wiederkehr und Rufabbau prüfen. PTT sowie neue Rufe und Sprechfreigaben lokaler Teilnehmer auf der Empfangsgruppe wirksam sperren; normale NetCore-Gruppen weiter betreiben. Wirkung mit den vorgesehenen Motorola-/Sepura-Endgeräten nachweisen.

**Nachweisgrenze:** Dokumentierte Ausbauidee mit vorhandenem Brew-Anknüpfungspunkt; Decoder-Gateway, netzseitige RX-Gruppenregel und Endgerätewirkung sind noch nicht implementiert beziehungsweise abgenommen.

## 6. Fortsetzung in neuen Chats und Statuspflege

Für Fragen nach der nächsten Aufgabe gilt:

1. **Aktuelle `ROADMAP.md` auf main lesen.** Kopf, nächsten Schritt und Statusdaten zusammen betrachten; den inzwischen aktuellen main-Stand ermitteln. Relevante Fachroadmap und vorhandene aktuelle Arbeitsnachweise ergänzend lesen.
2. Geänderte relevante Dateien / PRs / Ergebnisse prüfen. Gültige Belege bei unverändertem Inhalt weiterverwenden; keine Vollinventur in jedem Chat. Ein Dokumentationscommit allein beweist keine erledigte technische Aufgabe.
3. Die höchste offene Priorität mit erfüllten Abhängigkeiten innerhalb des Nutzerauftrags auswählen. Z02 darf unabhängig von einer blockierten Z01-Unteraufgabe fortgeführt werden. Parallelstränge nur mit ihrem eigenen Eintrittskriterium beginnen.
4. Einen konkreten nächsten Schritt mit **Aufgaben-ID, Grund, Arbeitsumfang und Abnahmekriterium** nennen. Eine blockierte P0-Aufgabe samt fehlendem Beleg / Zugriff benennen und die mögliche unabhängige Arbeit angeben. Keine erzwungene erneute Zustimmung für bereits autorisierte Arbeit einführen.
5. Empfehlungen von Umsetzung unterscheiden. Nur eine Frage nach der Reihenfolge startet keine automatische Änderung; ausdrückliche Aufträge und fortgeltende Freigaben bestimmen den Handlungsspielraum.

Bei tatsächlichem Fortschritt Aufgaben-ID erhalten, Status und Nachweise aktualisieren, Abhängigkeiten prüfen und den Kopf / Abschnitt 1 auf den neuen ersten Schritt umstellen. Eine abgeschlossene Analyse wie Z01.1 darf erledigt sein, während die Integration Z01.2 noch offen ist. Neue Befunde können Prioritäten ändern; Grund und Datum in der Historie festhalten.

Jeder aktualisierte Aufgabenstatus enthält: Datum, geprüften Quell-SHA / Branch, Ergebnis, Nachweisort, erreichte Nachweisstufe und verbleibende Grenzen. Als Statuswerte eignen sich **offen**, **in Arbeit**, **blockiert** und **erledigt gemäß eigener Abnahme**. Entwicklungsstand **Code vorhanden**, **statisch geprüft**, **Lab geprüft** und **On Air geprüft** zusätzlich getrennt angeben. Keine erfundenen Termine, Prozentwerte oder stillschweigend verschwundenen Aufgaben.

Regelmäßige NetCore-Projektstatusläufe verwenden diese Gesamtroadmap als Einstieg und gleichen die betroffenen Fachroadmaps ab. Diese Ablage erstellt oder verändert keinen Zeitplan / keine Automation.

## 7. Fachroadmaps und Quellen

- [SwMI- / Backend-Roadmap](system-backend/roadmap.md): Protokollphasen und technische Historie; globale aktuelle Reihenfolge steht hier.
- [NETCORE-IAM-01](Docs/CENTRAL_IDENTITY_RBAC_ROADMAP.md): Anmeldung, Rollen, lokale Ausnahmezugänge, Migration und Abnahme.
- [NETCORE-DRIVE-01](Docs/NETCORE_DRIVE_ROADMAP.md): Dateicloud, externe Ordnerfreigaben, lokaler Betrieb, spätere SSO-Migration und Browser-Plugins.
- [Wiki-Roadmap](wiki/Roadmap.md), [Projektstand](wiki/Projektstand.md), [Dienstkatalog](wiki/Dienstkatalog.md) und [Systemabnahme](wiki/Abnahme.md).
- [Archivindex auf Archiving](https://github.com/JanHG98/netcore-tetra/blob/Archiving/Docs/archive/README.md): historische Entscheidungen / offene Ideen, keine automatische Aktualisierung des Produktstands.
- [Discovery- / Syslog-PR #57](https://github.com/JanHG98/netcore-tetra/pull/57) und [historischer Feature-Stand](https://github.com/JanHG98/netcore-tetra/tree/bbf039729b9b05f8d623b11195ca24a124f68d16).
- [MAIN-COMPAT-Rollout](Docs/CENTRAL_NETWORK_ROLLOUT.md), [Edge-Fallback](Docs/EDGE_FALLBACK.md), [Open-Lab-Deployment](Docs/OPEN_LAB_LXC_DEPLOYMENT.md), [E2E-Runner](tests/e2e/README.md) und [statischer Integrationsaudit](Docs/generated/full-system-integration-audit.md).

## 8. Änderungsverlauf

| Datum | Änderung | Nachweisgrenze |
| --- | --- | --- |
| 2026-10-05 | Nutzerfreigegebene Gesamtfolge zentral abgelegt; geprüfte Übernahmelücke / Stabilitätsaufgaben, parallele IAM- / Drive-Stränge, Abnahmegates und Fortsetzungsregeln dokumentiert | Dokumentation; keine technische Umsetzung oder neue Live-Abnahme durch diesen Auftrag |
| 2026-10-05 | Auf Nutzerwunsch Z02.5 als P0 ergänzt: zentrale Gruppenzuweisungen / DGNA auf der TBS umsetzen; fehlende MM-Handler an `main@07609fb` belegt; Rückweg, Fähigkeiten, Fehler / Wiederkehr und Endgeräteabnahme festgelegt | Roadmap-Ergänzung und statische Quellprüfung; Handler noch nicht implementiert, keine neue Lab- / On-Air-Abnahme; Z01.1 bleibt erster Gesamtschritt |
| 2026-10-07 | Z01.1 vollständiger Tipvergleich abgeschlossen; Z01.2 Deployment/Discovery/Imagebuilder/VPN/Syslog kontrolliert aufgenommen; Z01.3 26er Inventory, semantische Ready-Gates, Config-Erhalt, Audit und CI integriert; nächste Aufgabe Z01.4 | Baseline main dae9363 / historisch bbf0397, Implementierung auf feature/z01-deployment-consolidation; Quellgate und isolierte lokale Tests PASS; PR-/native CI-/Anlagen-/On-Air-Abnahme getrennt und offen |
| 2026-10-08 | PR #62 / main-CI bestätigt; Betreiber aktualisiert VM119 und prüft Hardware-Gateway-Neuinstallation / Wiederholungsupdate auf CT150; SQLite-Sperre anhand Agentjournal zugeordnet und gezielt korrigiert | Anlagen-Teilnachweis an c45a2ec, Konfiguration unverändert mit Wert37; PR #63 / CI bestanden, Agentkorrektur auf CT150 installiert; negative Readiness / Recovery unter korrigiertem Agent bestanden, Jobstatus ohne GET-Fehler / HTTP500; vollständiges Z01.4 / MQTT / NAS / ARM64 / Pi / On Air offen |

| 2026-10-08 | CT136-Bestand vom Betreiber erfasst: IP-only-HTTP aktiv, localhost-Preview nicht erreichbar; zusätzliche lokale HTTP-Annahme mit Listener- und realer SQLite-/HTTP-Regression ergänzt | Befund und Quellkorrektur; CI-Ergebnis getrennt im CT136-Nachtrag, Update / tatsächliche Markerzustellung / NAS-Archiv noch offen |

| 2026-10-08 | PR #64 auf main96db88d übernommen; Betreiber bestätigt echten NAS-Dateizugriff als UID999/GID989, Cargo/Rust1.97.1 und7.9GiB frei auf CT136; gezielter Binary-Updateblock bereitgestellt | Dateizugriff im Lab bestanden; Buildaustausch, Vorschau-Marker und tatsächliches gzip-Archiv noch offen |
| 2026-10-08 | Z11.1 auf Nutzerwunsch als langfristigen P2-Backlog ergänzt: einseitige TETRA-Einspeisung über SDR++ / Decoder / Gateway, Netzkontext und ISSI-/GSSI-Mapping, Brew-Anschluss und Receive-only-Gruppen mit netzseitiger Sendesperre | Planung ohne Terminbindung; keine Implementierung oder neue Lab-/On-Air-Abnahme, aktuelle Z01-/Z02-Prioritäten bleiben erhalten |

| 2026-10-08 | Betreiber aktualisiert CT136 erfolgreich aus3d96a9a: Releasebuild3m26s, Management-/localhost-API bereit, Original-TOML und Syslog-JSON unverändert; Preview-Puffer0 ohne Fehler | Reale Binary-/API-Teilabnahme; TCP-Markerzustellung und tatsächliches gzip-NAS-Archiv bleiben offen |
