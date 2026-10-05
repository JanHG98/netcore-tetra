# NetCore-Tetra – zentrale Gesamtroadmap

| Feld | Wert |
| --- | --- |
| Roadmap-ID | NETCORE-MASTER-01 |
| Erstellt / aktualisiert | 2026-10-05, Europe/Berlin |
| Geltungsbereich | Gesamtes NetCore-Tetra-Repository: TBS / Funkstack, Core, Deployment, Betrieb, IAM, Drive, Plugins und weitere Integrationen |
| Geprüfter Ausgangsstand | `main@e5d825b33db2e1fce14bcb2cc23e73c241873a18`; `Archiving@9af2de92b120dc612d83a17f78eeee9a97f2f775` |
| Ergänzende Gruppenprüfung | `main@07609fb56f412ebe6e36655323e8fc6359cf90ec`, 2026-10-05; statischer Befund für Z02.5 |
| Planungsstand | Reihenfolge vom Nutzer zur zentralen Ablage freigegeben; technische Aufgaben bleiben offen, soweit kein eigener Nachweis vorliegt |
| Aktueller erster Schritt | **Z01.1: fehlende Deployment- / Syslog-Arbeit gegen aktuelles main abgleichen und den Integrationsplan vorbereiten** |
| Zeitplanung | Arbeitsblöcke und Abnahmebedingungen; keine zugesagten Kalendertermine |

**Dies ist der zentrale Einstieg für „Was machen wir als Nächstes?“.** Die Gesamtpriorität steht hier. Fachroadmaps beschreiben den jeweiligen Umfang und die technische Abnahme. Ältere Phasenlisten oder archivierte Gesprächsstände ersetzen diese Reihenfolge nicht. Vor einer Empfehlung den aktuellen Repositorystand und die zuletzt dokumentierten Ergebnisse prüfen; spätere ausdrückliche Nutzerentscheidungen haben Vorrang.

## 1. Aktueller nächster Schritt

**Z01.1 – kontrollierte Wiederaufnahme der fehlenden Deployment- und Syslog-Entwicklung.**

Die Vorprüfung vom 2026-10-05 belegt eine Übernahmelücke: Discovery, Imagebuilder, Pi-VPN-Policy und die neuere Syslog-Pipeline liegen im historischen Feature-Stand `bbf039729b9b05f8d623b11195ca24a124f68d16`, fehlen aber im geprüften `main`. [PR #57](https://github.com/JanHG98/netcore-tetra/pull/57) wurde in `feature/openlab-discovery-deployment` integriert. Eine Integration dieser Entwicklung in den heutigen Hauptzweig ist damit nicht belegt.

Die nächste Aufgabe ist ein vollständiger, inhaltlicher Abgleich mit einem konkreten Integrationsplan:

1. Aktuelles `main` und den historischen Feature-Commit festhalten. **Die beiden vollständigen Tip-Bäume und erforderlichen Dateiinhalte direkt vergleichen.** Der [GitHub-Vergleich](https://github.com/JanHG98/netcore-tetra/compare/e5d825b33db2e1fce14bcb2cc23e73c241873a18...bbf039729b9b05f8d623b11195ca24a124f68d16) zeigt Änderungen seit der gemeinsamen Basis und ist allein kein vollständiger Vergleich mit dem heutigen main.
2. Fehlende Dateien, überlappende Änderungen und Integrationskonflikte für Deployment / Discovery, Imagebuilder, VPN-Policy, Observability und deren Tests erfassen. Neuere UI- und sonstige main-Änderungen erhalten; keinen ganzen historischen Commitstapel ungeprüft übernehmen.
3. Repository-Inventar, generierten Katalog und zugängliche Installationsnachweise mit Hostrolle, Quell-SHA, Binaryversion und Konfiguration abgleichen. Nicht zugängliche Live-Daten als unbekannt markieren; keine aktuelle Flotte aus alten Betreiberangaben ableiten.
4. Einen prüfbaren Integrationsplan mit betroffenen Dateien, Reihenfolge, Konfigurationsübernahme, benötigten Tests und Rückweg erstellen. Vorhandene gültige Befunde wiederverwenden; noch offene Inhalte gezielt prüfen.

**Abnahme Z01.1:** Eine Vergleichstabelle und ein Integrationsplan mit Quellen-SHAs, Konflikten, bekannten Abnahmen und klar benannten Lücken liegen vor. Fehlender Live-Zugriff blockiert den Quellvergleich nicht. Danach folgt **Z01.2: die geplante Integration umsetzen und prüfen**. Die abgeschlossene Analyse allein bedeutet nicht, dass Discovery oder Syslog schon in main integriert sind.

Eine Frage nach dem nächsten Schritt verlangt zunächst diese Empfehlung. Die tatsächliche Umsetzung richtet sich nach dem aktuellen Auftrag und bereits erteilten Freigaben; diese Roadmap ist keine pauschale Erlaubnis für Installationen oder Änderungen an laufenden Anlagen.

## 2. Belegter Ausgangsstand und seine Grenzen

Die folgenden Befunde beziehen sich auf die oben genannten Quellstände; die Gruppensteuerung wurde am ergänzend genannten main-Stand geprüft. Sie sind vor späteren Statusmeldungen gegen neue Änderungen zu prüfen.

| Bereich | Belegter Stand am 2026-10-05 | Konsequenz |
| --- | --- | --- |
| Branches / Archiv | Aktuelle Branchliste: `main` und `Archiving`; Archivzweig ergänzt Gesprächsdokumentation / Assets, keinen fehlenden Deployment-Dienst | Archiv als Wissensquelle nutzen; alte Vorschläge von späteren Festlegungen unterscheiden |
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
| Z01 | P0 · sofort, erster Block | Repository und Installation konsolidieren: Deployment / Discovery / Imagebuilder / VPN / Syslog übernehmen; Bestandsdrift, Ready-Schranke und CI schließen | Keine; Vorprüfung vorhanden | Quellen / Konfigurationen zugeordnet; Integration geprüft; Installation und Upgrade reproduzierbar; Schritte Z01.1–Z01.4 dokumentiert |
| Z02 | P0 · sofort, gezielt parallel zu Z01 | Beide Restore-Fixes, Schutz von Management- / Packet-Core-Netzen und tatsächliche Ausführung zentraler Gruppenzuweisungen auf der TBS umsetzen; aktive SAP- / Downlinkwege prüfen | Aktueller Funk- / Gateway-Code; technische Änderungen getrennt integrierbar | Gezielte positive / negative Regressionen; keine doppelte Sprechfreigabe, Uplink-Watchdog aktiv, keine schädliche TUN-Route; Gruppenaufträge mit fachlichem Ergebnis statt stillem Ignorieren |
| Z03 | P0 · erstes Systemgate nach Z01 / Z02 | Einzelzelle und Core-Anbindung abnehmen; lokale Funkfunktionen, Dual Carrier, Matrix und Edge-Fallback | Z01 / Z02 für den gemeinsamen geprüften Stand | Reproduzierbare Labor- und On-Air-Nachweise mit Motorola / Sepura; Build, Konfiguration, Gerätefirmware und Logs festgehalten |
| Z04 | P1 · Architektur ab jetzt, anschließend Pilot | Zentralen IAM- / RBAC-Plan umsetzen: Inventur / Rollenvertrag, Identity-Pilot, eine TBS und Control Room, danach Dienste | M0 / M1 können parallel beginnen; Pilot auf isolierter, prüfbarer Baseline; breite Migration nach Pilotabnahme | Anmeldung, Ressourcenrechte, Sperren, Maschinenidentitäten, Ausfall und Rückweg funktionieren; Details IAM M0–M8 |
| Z05 | P1 · Alltagspfade nach Grundabnahme | SDS / Status / GPS, HA / MQTT, Warnzentrale, SIP / RTP, Recording, TTS / Media Library und Paketdaten durchgehend schließen | Z03 für belastbare Systemabnahme; einzelne Diagnosen können vorher erfolgen | Tatsächliches Ziel erreicht; Rückmeldung nachvollziehbar; Neustart, Deduplizierung und Abhängigkeitenausfall geprüft |
| Z06 | P1 · eigener Produktstrang nach den ersten P0-Fixes | Drive-Grundversion mit lokalem Login, Konten / Gruppen, Dateien, Freigaben, Versionen, Papierkorb, Sicherung und Sync | Kein zentraler IAM-Dienst nötig; eigene Backend- / Betriebsentscheidung D0 | Eigenständiger Betrieb einschließlich externem Ordnerzugang und Restore; Details D0–D5; Plugin-Vertrag von Beginn an |
| Z07 | P1 · nach stabiler Einzelzelle / Core-E2E | Zwei TBS und Mehrzellenbetrieb stufenweise integrieren: neue Registrierungen / Rufe, Kontexttransfer / Restore, laufende Medienpfade | Z02 / Z03 und relevante Z05-Pfade; zweite TBS separat abgenommen | Erst danach Seamless-Handover-Abnahme mit gemessener Audio-Unterbrechung, Floor und Fehlerfällen |
| Z08 | P1 · weiterer praktischer Ausbau | Leitstellen-Audio / Headset / Mikro / PTT; reale HA-Aktoren, Rack-Sensoren / RF-Messhardware; Asset- / Task-Rückweg | Funktionsfähiger Pilot für den jeweiligen Z03 / Z05-Pfad; betroffene Schreibrechte geregelt | Bedienung und Hardware zeigen tatsächlichen Zustand; Auftrag / Wartung abgeschlossen; Timeout und Wiederkehr geprüft |
| Z09 | P1 · nach Drive-Dateikern, parallel zum IAM-Ausbau | Drive-Plugin-Plattform: PDF / Bilder / Audio / Video, danach Office und technische Module für 3D / Schaltpläne / Vektoren | D7 nach D0 / D2–D4; D8 / D9 nach D7; keine D6- / IAM-Startabhängigkeit | Pro Format Anzeige, Bearbeitung / Export und identische Rechte geprüft; Plugin-Update / Deaktivierung / Rückweg funktionieren |
| Z10 | P1 / P2 · auf abgenommenem Grundbetrieb | Gesicherten Betrieb vervollständigen, Last / Dauerlauf, Upgrade / Restore; danach Backend-HA, produktive Funk-Security / OTAR, Mehrregionen und ETSI ISI | IAM-Pilot / relevante Dienstmigration; stabile Fachpfade; eigene Architektur je Ausbau | Gemessene Betriebs- / Wiederherstellungsgrenzen und eigener Sicherheits- / Interoperabilitätsnachweis je Ausbaustufe |
| Z11 | P2 · danach nach praktischem Nutzen | Android / Hybrid, Zebra / QR-Inventur, WERMA, USB-Audio / PTT, FRN / Zello / weitere Brücken, weitere Fachanwendungen | Bestehende autoritative Dienste und passender Pilot; kein konkurrierendes Registry- / Rechte-System | Klare Zuständigkeit, benötigte Hardware und eigener Abnahmepfad; archivierte Visionen erst nach aktueller Prüfung übernehmen |

Arbeitsorganisation: Z01 / Z02 zuerst fokussiert abschließen; IAM-Inventur und Drive-Architektur dürfen parallel entstehen. Danach Betriebs-E2E, IAM-Pilot und lokaler Drive-Betrieb als getrennte Stränge mit jeweils prüfbaren Zwischenergebnissen führen. Die Roadmap ist keine Aufforderung, alle großen Blöcke gleichzeitig zu starten.

## 4. Die nächsten konkreten Arbeitspakete

### Z01 – konsistenten Gesamtstand herstellen

| Aufgabe | Status am 2026-10-05 | Ergebnis / Abnahme |
| --- | --- | --- |
| Z01.1 · vollständiger Quellvergleich und Integrationsplan | Offen; Übernahmelücke und Inventardrift in der Vorprüfung belegt | Direkter Tip-Vergleich und Integrationsplan nach Abschnitt 1; vorhandene gültige Quellenprüfung nicht wiederholen |
| Z01.2 · fehlende Entwicklung kontrolliert übernehmen | Offen | Deployment / Discovery, Imagebuilder / VPN und Syslog integriert; aktuelle UI, Fachänderungen und lokale Konfigurationen erhalten; passende Tests bestanden |
| Z01.3 · Inventory, Ready-Schranke und CI vereinheitlichen | Offen; 25 / 24-Drift und verworfene Ready-Rückgabe im [Deploy-Runner](deploy/open-lab/netcore-deploy.py) belegt | Generatoren / Katalog / Health- und Fallbackmatrix konsistent; negatives Ready-Ergebnis beendet fehlgeschlagenes Deployment; gemeinsame Prüfungen erreichbar |
| Z01.4 · Installation, Upgrade und Recovery abnehmen | Offen | Neu- / Wiederholungsinstallation, Controller- und NAS-Ausfall, begrenzte Logpuffer / Archivprüfung; vollständiger ARM64-Build und echter Pi-/SXceiver-Boot mit VPN-Wechsel; dokumentierter Rückweg |

Historische Tests des Imagebuilders belegen Teile der Image-Personalisierung und VM-Installation. Sie ersetzen keinen vollständigen NetCore-Image-Build und keinen physischen Pi-/SXceiver-Test. Quellarchive, Binärartefakte und OS-Images erhalten jeweils passende Versions- / Prüfsummennachweise; nach gemeinsamer Abnahme einen konsolidierten Release festlegen.

### Z02 – gezielte Stabilität vor weiterem Funk-Ausbau

- **Z02.1 Einzelruf-Restore:** Sprechrecht nur vergeben, wenn frei oder bereits beim anfragenden Teilnehmer; passenden Floor-Owner bei Simplex setzen. Gegensprechenden Teilnehmer, gleiche / andere Identität und erneuten Restore prüfen.
- **Z02.2 Gruppenruf-Restore:** Nach Floor-Grant die untere Funksteuerung benachrichtigen, damit ausbleibende Uplink-Sprachframes überwacht werden. Stummen Restoreteilnehmer, Timer, Release und sofortige Wiederverwendung prüfen.
- **Z02.3 IP-Routenschutz:** Managementadresse / -netz und Packet-Core-Abhängigkeiten vor unzulässigen TUN-Routen schützen: beim Schreiben, vor Kernel-Reconcile und beim Wiederherstellen gespeicherter Regeln. Historisch gespeicherte Konflikte mit behandeln; daraus keine aktuelle Live-Störung ableiten.
- **Z02.4 aktive Runtimepfade:** Tatsächlich verwendete SAP- / SNDCP- / MLE-Downlinkpfade und Capability-Anzeigen mit dem Codebestand abgleichen. Isolierte Restore- / Two-Cell-Bausteine erst nach Laufzeitintegration als Funktionsfortschritt melden.
- **Z02.5 zentrale Gruppenzuweisungen:** Gruppenprofile, Mitgliedschaften und zentrale DGNA-Aufträge auf der Basisstation vollständig verarbeiten und die nötigen Attach- / Detach-Aktionen ausführen; Details und Abnahme im folgenden Arbeitspaket. Offen / geplant, P0; unabhängig vom Z01-Quellvergleich bearbeitbar.

### Z02.5 – zentrale Gruppenzuweisungen auf der TBS umsetzen

**Ziel:** Alle unterstützten Gruppenaufträge vom Group Core werden auf der zuständigen Basisstation fachlich umgesetzt und mit einem korrelierten Ergebnis beantwortet. Sie dürfen nicht im allgemeinen Zweig für unbekannte / nicht unterstützte Befehle verschwinden. Dieser Auftrag ergänzt die Planung; die technische Behebung ist noch offen. **Z01.1 bleibt der erste Gesamtschritt**, Z02.5 kann als gezielte P0-Arbeit parallel erfolgen.

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
