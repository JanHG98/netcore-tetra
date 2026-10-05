# NetCore-Tetra – Installationsanleitung ab 0: TBS-02, Build, SXceiver und systemd

## 1. Metadaten und Ergebnis

| Feld | Verifizierter Wert / Abgrenzung |
|---|---|
| Ursprünglicher Chattitel | **Installationsanleitung ab 0** |
| Chat-ID | `6aa54239-3fa0-83ed-b3e2-0744529f9e27` |
| Chatlink | [ChatGPT-Verlauf](https://chatgpt.com/c/6aa54239-3fa0-83ed-b3e2-0744529f9e27), [Referenz in der App](chatgpt-conversation://6aa54239-3fa0-83ed-b3e2-0744529f9e27) |
| Erstellungsdatum dieser Dokumentation | **2026-10-05**, Datumsbezug Europe/Berlin |
| Chat laut Metadaten angelegt | 2026-09-12, 14:14:50 Uhr Europe/Berlin |
| Zugänglicher technischer Dialog | 2026-09-12, 15:47–16:38 Uhr Europe/Berlin; vier technische Frage-/Antwortabschnitte |
| Archivauftrag im Quellchat | 2026-10-05, 23:08 Uhr Europe/Berlin; fünfter zugänglicher Gesprächsabschnitt |
| Repository | `JanHG98/netcore-tetra` |
| Zielbranch | **`Archiving`**, bereits vorhanden |
| Vor dem Schreiben geprüfter Archivbranch | **`45c13959b0e030e4162344fbf2b5d29fbcd820d3`** |
| Zusätzlich geprüfter aktueller Codebranch | **`main` @ `9116c15d645458f99e236712b67a1ad970432791`** |
| Historische Binary-Kennung | `v1.3.0-b63b251b`, zweimal im Originalanhang; kein vollständig verifizierter historischer Source-Commit |
| Historischer Zielhost | `SRV-M-TBS-02`; interaktiver Benutzer `jan`, Laufzeitbenutzer `netcore` |
| Archivdatei | `Docs/archive/2026-10-05_installationsanleitung-ab-null-tbs02-build-sdr-systemd.md` |
| Zugehörige Belege | [Bereinigter Laufzeitlog](assets/2026-10-05_installation-ab-null-tbs02/laufzeit-20260912-bereinigt.txt), [Herkunft, Prüfsummen und Ereigniszählung](assets/2026-10-05_installation-ab-null-tbs02/provenienz.json) |

**Erreichter historischer Stand:** Der Benutzer bestätigt einen erfolgreichen Build. Der anschließend als `netcore` gestartete Prozess erkennt den SXceiver und verarbeitet eine reale Anmeldung, Gruppenaffiliation, einen SDS-Eingang und einen Gruppenruf. Der Log enthält außerdem einen SIP-Rufaufbau bis `media ready`. Recorder und AudioPlayer sind wegen Dateisystemrechten deaktiviert, der SNDCP-Paketgateway scheitert an `CAP_NET_ADMIN`, und der Node Gateway ist beim zweiten Start nicht erreichbar. Eine abschließend installierte und nach Reboot geprüfte `tetra.service` ist nicht belegt.

Die geprüften Commitangaben bezeichnen den **Repository-Stand vor dieser Archivänderung**. Der veröffentlichende Archiv-Commit ergibt sich aus der Git-Historie dieser Datei und wird in der Abschlussmeldung genannt; er wird nicht als selbstreferenzieller Hash in den Dateiinhalt erfunden.

## 2. Quellenumfang, Zugänglichkeit und Beweismaßstab

### 2.1 Tatsächlich verfügbare Quellen

Der Chatabruf am 2026-10-05 lieferte fünf vollständige Gesprächsabschnitte, `hasMore=false` und keinen `nextCursor`. Der jüngste Abschnitt enthält den ausführlichen Archivauftrag sowie die Übergabe an den Work-Chat. Die vier technischen Abschnitte behandeln, chronologisch:

1. Kernel-OOM beim Rust-Build und Vorschläge zu Swap und Build-Parallelität.
2. Die Benutzerbestätigung, dass der Build durchgelaufen ist, sowie vorgeschlagene Installation, manueller Start und eine vorläufige systemd-Unit.
3. Den gescheiterten Start als `jan` wegen unlesbarer Haupt- und Fallback-Konfiguration sowie die Korrektur auf den Dienstbenutzer.
4. Die Benutzerbestätigung „läuft an sich“, die Erkennung des AliExpress-SDRs, den noch zu bearbeitenden lokalen Fallback und den hochgeladenen Laufzeitlog mit anschließender Diagnose.

Der bereitgestellte Vorschauausschnitt war kürzer als die nun gelesenen Antworten. Insbesondere Gerätezugriff, Packet-Gateway-Helfer, Node-Gateway-Ausfall und Einordnung der Start-Overruns konnten aus der vollständigen abrufbaren letzten Antwort ergänzt werden.

Der Originalanhang **`Eingefügter Text(20260912-143852).txt`** war lokal lesbar. Er enthält **244.548 Bytes und 1.146 physische Zeilen**, einschließlich zweier manueller Starts, einer Konfigurationsbearbeitung zwischen den Starts und zweier geordneter Abbrüche. Das Log enthält Uhrzeiten, aber keine Datumsangabe pro Zeile. Der Tagesbezug 2026-09-12 stammt aus Dateiname und zugehörigem Chatturn; der Suffix `143852` des Dateinamens wird nicht mit den Laufzeituhrzeiten verwechselt.

### 2.2 Explizite Lücken

- Der früher als 15:47 Uhr liegende Installationsdialog wird nicht geliefert, obwohl der Chat laut Metadaten bereits um 14:14 Uhr angelegt wurde. Die ursprünglichen Schritte „ab 0“ – OS-Installation, Paketinstallation, Anlage des Dienstbenutzers, Codec-/Soapy-Build und Auswahl des Quellstands – sind daher **nicht vollständig rekonstruierbar**.
- Kein vollständiger Chat-Export und keine frühere Konfigurationsdatei stehen für diese Auswertung zur Verfügung. Es gibt keinen auswertbaren Diff der im Log sichtbaren `nano`-Bearbeitung.
- Es wurde **ein Textanhang und kein eigenständiger Bildanhang** geliefert. Für diesen Chat werden deshalb keine Bilder hochgeladen. Das ist eine Aussage über die zugänglichen Quellen, keine Behauptung über den gesamten historischen Chat.
- Die allgemeinen ETSI-PDFs unter `sources/` sind Projektmaterial; sie sind kein Ersatz für fehlende Chatabschnitte und wurden nicht verändert oder als Chatbilder kopiert.
- Es fehlen die ursprüngliche Buildausgabe, `free -h`/Swap-Ausgaben nach der Reparatur, vollständige `ldd`-Ergebnisse, finale Unit und Geräteberechtigungen sowie ein Boot-/Dauerlaufprotokoll.
- Kein vollständiger historischer Source-Snapshot oder Binary-Hash liegt vor. `b63b251b` ließ sich in den für `Archiving` und `main` geladenen Objekten nicht als Commit auflösen. Daraus folgt keine Aussage, dass dieser Commit weltweit oder in alten Archiven nicht mehr existiert.

### 2.3 Statuslegende

| Status | Verwendung in dieser Dokumentation |
|---|---|
| **Idee** | Option, für die keine endgültige Auswahl belegt ist. |
| **Beschlossen/geplant** | Als nächster Ablauf vorgeschlagen oder festgelegt; Durchführung nicht belegt. Ein Assistentenvorschlag wird dabei ausdrücklich als solcher benannt. |
| **Implementiert** | Im angegebenen, heute gelesenen Source-Stand vorhanden; keine Aussage über Installation auf TBS-02. |
| **Getestet** | Konkrete Testausgabe oder ausdrücklich bezeichnete Benutzerbestätigung vorhanden. |
| **Im Betrieb bestätigt** | Konkretes Verhalten auf dem realen Host im historischen Log bzw. durch den Benutzer bestätigt. Der Umfang ist auf das beobachtete Verhalten und den kurzen Mitschnitt begrenzt. |

Repository-Präsenz, vorhandener Testcode, ein erfolgreicher Prozessstart und eine Ende-zu-Ende-Abnahme sind unterschiedliche Belege. In diesem Archivierungslauf wurden keine TBS-Dienste gestartet, keine Funkhardware bedient und keine Cargo-Tests ausgeführt.

## 3. Ziel, Ausgangslage und endgültige Richtung

Ziel war, die frisch aufgebaute Basisstation auf `SRV-M-TBS-02` von einem erfolgreichen Rust-Build zu einem reproduzierbaren Betrieb mit SXceiver zu führen. Die historische Binary bezeichnet ihren Radio-Runtime-Pfad als `MAIN-COMPAT (local MM/MLE/CMCE state machines)`.

Die am Ende des zugänglichen Dialogs maßgebliche Reihenfolge lautet:

1. Bestehende Buildartefakte behalten und nach OOM nicht erneut `cargo clean` ausführen.
2. Binary und native Laufzeitabhängigkeiten prüfen; die Konfiguration mit eingeschränkten Rechten installieren.
3. Als tatsächlicher Dienstbenutzer `netcore` manuell starten und die direkten Fehler auswerten.
4. Schreibrechte für die konfigurierten Recording-/Audio-Verzeichnisse berichtigen.
5. Die endgültige `tetra.service` mit SNDCP-Capabilities und Zugriff auf TUN, SPI, GPIO und ALSA erstellen bzw. vervollständigen.
6. Automatischen Start nach Reboot und die noch offenen Funktionen am Zielgerät prüfen.

Der frühe Entwurf der Unit ist durch die spätere Erkenntnis zum fehlenden `CAP_NET_ADMIN` **unvollständig geworden**. Die letzte Antwort kündigt den vollständigen systemd-Schritt an; eine nachfolgende Umsetzung ist nicht zugänglich. Der Benutzer benennt außerdem den lokalen Fallback als noch zu bearbeitende Baustelle. Seine genaue gewünschte Änderung ist nicht spezifiziert und darf nicht durch eine erfundene Fallback-Anforderung ersetzt werden.

## 4. Historische Architektur und technische Parameter

Die im Chat behandelte Kette ist:

```text
/etc/netcore/config.toml  -> bluestation-bs als netcore
                              |
                              +-- lokale MM/MLE/CMCE, LLC, UMAC/LMAC, PHY
                              |                         |
                              |                    SoapySDR -> SoapySX -> SXceiver
                              |
                              +-- Brew/TetraPack über WebSocket
                              +-- Asterisk/SIP und Medienpfad
                              +-- Dashboard HTTP
                              +-- lokaler Recorder / AudioPlayer
                              +-- SNDCP -> Linux-TUN / IP-Gateway
                              +-- Control-Room-Worker -> Node Gateway
                                      mit lokaler Edge-Autorität bei Ausfall
```

Diese Zeichnung beschreibt Komponenten und Beobachtungen des zugänglichen Ausschnitts. Sie ist keine Bestätigung, dass alle Zweige funktionsfähig waren.

### 4.1 Funk- und Hardwareparameter aus beiden Starts

| Parameter | Historischer Logwert / Bedeutung |
|---|---|
| Binary | `/usr/local/bin/bluestation-bs`, Version `v1.3.0-b63b251b` |
| SDR-Argumente | `driver=sx, label=sx`; erkannte `driver_key` und `hardware_key` jeweils `sx` |
| SoapySX | `9705147`, vollständige ausgegebene Kennung `9705147dd8c189625071f3f163ea56119bda4a05` |
| Hardwareversion | `1.2` laut Treiber; keine separat geprüfte Hersteller-/Modellzuordnung des AliExpress-Geräts |
| Takterkennung | `Clock detection failed, assuming 38.4 MHz`; Annahme des Treibers, kein gemessener/erfolgreich erkannter Takt |
| Abtastrate / Hardwarezeit | `fs=600000.0`, `use_get_hardware_time=true` |
| Kanäle / Antennen | RX-Kanal 0 / TX-Kanal 0; Antennen `RX` und `TX` |
| RX-Gain | `LNA=42.0`, `PGA=16.0` |
| TX-Gain | `DAC=9.0`, `MIXER=30.0` |
| Stream-Argumente | RX und TX jeweils `period=900` |
| Hauptcarrier | 680; DL 417.000000 MHz, UL 407.000000 MHz |
| Zweiter Carrier | 681; DL 417.025000 MHz, UL 407.025000 MHz |
| Center-Frequenzen | RX 407.012500 MHz, TX 417.012500 MHz, jeweils `explicit center override` |
| Frequenzkorrektur | `PPM=0.00`, ausgegebener Fehler und Korrektur 0 Hz |
| Netzkennung | MCC `901`, MNC `1510`, Colour Code `1` |
| Historischer UMAC-Banner | `dual-carrier logical-timeslot mapper v2.9`, `C2 idle-silent traffic carrier`, Traffic `TS5–TS7` |
| SYSINFO | `sndcp_service=true`, `advanced_link=true`, `voice_service=true`, `circuit_mode_data=true`, `radio_dl_timeout=0`, `neighbours=0` |
| Healthmonitor | Intervall 300 s, Watchdog-Restart aus; der Mitschnitt ist kürzer als ein vollständiges Intervall |
| Weitere geladene Umgebung | UHD-Ausgabe: Linux, GNU C++ 14.2.0, Boost 1.83.0, UHD 4.8.0.0+ds1-2; das identifiziert weder das OS-Image noch die Pi-RAM-Größe |

Die Carrierparameter sind beobachtete historische Einstellungen. Eine Abnahme des zweiten Carriers unter Last oder mit parallelen Rufen folgt daraus nicht: Die dokumentierten Rufe belegen Traffic auf **Carrier 680 / TS2**.

### 4.2 Teilnehmer, Backhaul und Dienste

| Element | Historischer Befund |
|---|---|
| Funkteilnehmer | ISSI `5102`; Gruppen `15201` und `15501` |
| Teilnehmerfähigkeiten | Unter anderem `voice=true`, `authentication=true`, `tetra_packet_data=false`, `air_interface_version=2`, `common_scch=true`; `StayAlive`, gewöhnlicher MCCH statt zugewiesenem Frame-18-common-SCCH |
| RSSI | Beispiel `-17.1 dBFS`; kein kalibrierter dBm-/Empfindlichkeitsnachweis |
| SDS | Von ISSI `5102` an ISSI `4010001`, 115 Bit; Debugdecoder `Type4`, Informationsausgabe `type=3` als ausgegebener Code. Diese Darstellungen werden nicht zu einem vermeintlichen Typwiderspruch umgedeutet. |
| Brew/TetraPack | `ws://10.0.1.22:8081`; verbunden. Fehlender `X-Brew-Version`-Handshakeheader führt laut Log zu späterer Versionserkennung. |
| Dashboard | Bind auf `http://0.0.0.0:8080`, HTTP Basic Auth eingeschaltet; erfolgreiche Logins um 16:35:18 und 16:36:06. `0.0.0.0` ist eine Bindadresse. |
| Dienststeuerung | Konfigurierter `service_name=tetra`; das beweist keine bereits existierende systemd-Unit. |
| Asterisk | Integration eingeschaltet; gewählte Nummer `91103` wird zu SIP-Nebenstelle `103`; SIP-/RTP-Adressen und Ports sind im verfügbaren Beleg nicht vollständig ersichtlich. |
| Control Room / Node Gateway | Beim zweiten Start aktiviert; keine Credentials für die Node-Verbindung konfiguriert, laut Log erwartbar im `open_lab`-Modus. Zieladresse/Port nicht aus dem Log rekonstruierbar. |
| Recorder / AudioPlayer | Konfiguriert, beim Start aber wegen Berechtigungen deaktiviert. Der Fehler des Recorders nennt keinen konkreten Verzeichnispfad. |
| Lokale TTS | Bewusst deaktiviert/deprecated; Verweis auf zentrale Media Library / TTS / Piper. Kein Nachweis der zentralen Erreichbarkeit oder Sprachgenerierung. |
| Weitere Integrationen | Snom-Notify-Worker und Telegram-Alerter gestartet; GeoAlarm, MeshCom-UDP, DAPNET und EchoLink deaktiviert. Kein Zustell-/Funktionsnachweis allein durch Workerstart. |

## 5. Chronologie und erreichte Zustände

### 5.1 Build: OOM bestätigt, später Erfolg vom Benutzer bestätigt

Im Chat steht eine tatsächlich ausgeführte Kerneljournal-Abfrage. Sie zeigt am 12.09. um **15:16:20** und **15:45:54** OOM-Kills von `rustc`, einmal nach `rustc invoked oom-killer` und einmal nach `cargo invoked oom-killer`. Betroffen sind PIDs 5430 und 5553, UID 1001; die angegebenen Resident-Anteile liegen bei rund 471 bzw. 477 MB, der virtuelle Speicher jeweils bei `2094396kB`.

Damit ist Speicherdruck als Ursache dieser Abbrüche **getestet/belegt**. Die Zeilen geben nicht die gesamte RAM-Größe an und belegen nicht, wie viele Cargo-Jobs beim zweiten Versuch tatsächlich aktiv waren. Die damalige Aussage, weniger Parallelität habe bereits nicht ausgereicht, geht über die sichtbare Ausgabe hinaus.

Vorgeschlagen wurden `free -h`, `swapon --show`, temporärer Swap mit bis zu 8 GB und ein fortgesetzter Build mit `CARGO_BUILD_JOBS=1`. Danach schreibt der Benutzer: „so, build leif durch, nun?“ Dies ist eine **Benutzerbestätigung eines erfolgreichen Builds**, jedoch kein Nachweis, welcher der vorgeschlagenen Swap-/Codegen-Schritte den Erfolg verursacht hat.

### 5.2 Konfigurationsrechte: fehlgeschlagener und erfolgreicher Start

Der erste sichtbare Fehlstart als `jan` erreicht Banner und Konfigurationsparser. Sowohl `/etc/netcore/config.toml` als auch `/etc/netcore/config.toml.fallback` scheitern mit `Permission denied (os error 13)`, anschließend bricht die Anwendung ab.

Die Chatdiagnose verweist auf die zuvor vorgeschlagenen Rechte `0640 netcore:netcore`. Ein tatsächliches `ls -l`-Ergebnis ist nicht vorhanden; Eigentümer und Modus sind daher **plausible Diagnose bzw. Installationsvorgabe**, keine gemessenen Dateimetadaten. Die spätere erfolgreiche Ausführung als `netcore` ist dagegen direkt im Anhang belegt.

Das ist außerdem von einem **Edge-/Core-Fallback** zu trennen: Der Dateifallback ist eine alternative TOML-Datei; die spätere lokale Edge-Autorität ist ein Netz-/Dienstzustand. Die beiden Fehlerklassen werden nicht vermischt.

### 5.3 Erster manueller Lauf, etwa 16:35:05–16:35:49

Zeile 1 des Anhangs zeigt den tatsächlich ausgeführten Start:

```bash
sudo -u netcore -H env RUST_LOG=info \
  /usr/local/bin/bluestation-bs /etc/netcore/config.toml
```

Der Shell-Arbeitsordner ist `~/sxxcvr/tetra-codec`; ein Wechsel in `/opt/netcore-tetra/netcore` ist an dieser Stelle nicht sichtbar. Das ist bei relativen Datenpfaden für eine Fortsetzung zu beachten.

- **16:35:06:** SXceiver geöffnet, Hardware 1.2 erkannt, Frequenzen und zwei Carrier initialisiert. Recorder-/Audio-Rechtefehler und SNDCP-Capabilityfehler treten auf.
- **16:35:06:** Brew-WebSocket verbunden; lokales MM behält RF-Registrierungen beim Backhaul-Reconnect.
- **16:35:34.609:** `ULocationUpdateDemand` / `ItsiAttach` für ISSI 5102, danach `DLocationUpdateAccept` und CMCE-Registrierung.
- **16:35:34.893:** Gruppen 15201 und 15501 affiliiert, einschließlich Brew-AFFILIATE.
- **16:35:42.033:** Ein `U-SDS-DATA` mit 115 Bit von 5102 an 4010001 empfangen. Die Zustellung am Empfänger oder eine Anwendungsantwort ist nicht belegt.
- Danach `Ctrl+C`, geordnetes Stoppen/Resetten der Streams, SoapySX-Uninitialisierung und Brew-Deaffiliation/Deregistrierung; Transport normal geschlossen.

### 5.4 Konfigurationsbearbeitung und zweiter Lauf, etwa 16:36:02–16:37:50

Zeile 221 zeigt `sudo nano /etc/netcore/config.toml`, Zeile 222 erneut den Start als `netcore`. Nur im zweiten Start erscheint `NetCore Control-Room node enabled`. Eine Aktivierung dieses Zweigs während der Bearbeitung ist **naheliegend**, der genaue TOML-Diff bleibt unbekannt.

| Uhrzeit im Log | Ereignis und belastbare Aussage |
|---|---|
| 16:36:03 | SDR/Carrier wieder initialisiert; dieselben Verzeichnis- und SNDCP-Fehler. Brew verbunden. |
| 16:36:13.602 | Control-Room-TCP-Verbindung läuft in Timeout; Retry in 10 s. Edge-Zustand `Isolated -> Degraded`, lokale Autorität aktiv. |
| 16:36:23.609 | `Degraded -> Isolated`; spätere Retrymeldungen bleiben bei `Isolated`. Das Log zeigt keine Erholung nach `Online`. |
| 16:36:40.824 | Wiederkehrende ISSI 5102 mit `RoamingLocationUpdating`; Antwort als `PeriodicLocationUpdating` und Aufforderung zum Gruppenreport. |
| 16:36:41.163 | Erster Gruppenrufversuch zu 15201 abgewiesen: `no listeners`. Dieser fehlgeschlagene Versuch gehört zum Ergebnis. |
| 16:36:41.618 | Gruppenreport/Affiliation 15201 und 15501 nachgeführt. Die zeitliche Reihenfolge legt eine noch fehlende Gruppenzuordnung beim ersten Ruf nahe; sie ist kein Nachweis der vollständigen Ursache. |
| 16:37:05.871 | Neuer Gruppenruf von 5102 an 15201, `call_id=4`, `usage=4`, `ts=2`; `DConnect` mit `Granted` und Kanalzuweisung. |
| 16:37:05.872 | Weiterleitung des lokalen Gruppenrufs an TetraPack; Recorder-Nachrichten können nicht zugestellt werden, weil die Entity beim Start ausgefallen ist. |
| 16:37:10.588–.589 | `U-TX CEASED`, `DTxCeased`, Hangtime für TS2 eingeschaltet; Brew meldet `GROUP_IDLE`, `frames=77`. Dies sind gezählte Frames, keine gehörte Sprachqualitätsabnahme. |
| 16:37:12.004–.118 | `U-DISCONNECT` durch den Rufinhaber, `UserRequestedDisconnection`, FACCH/STCH-Abbau und Schließen der DL-/UL-Circuits für Carrier 680 / TS2. |
| 16:37:22.417–.422 | Wahl 91103; Ziel nicht lokal registriert. Weiterleitung an Asterisk als Nummer 103, `call_id=5`, `ts=2`, `duplex=1`; Asterisk akzeptiert Setup. |
| 16:37:22.743 | `DAlert` für Ruf 5. |
| 16:37:25.165–.178 | Asterisk-Connect, `DConnect` mit `Infinite`, `Granted`, Duplex und `media ready`; kein Ende-zu-Ende-Audionachweis. |
| 16:37:25.321 | Einzelne Warnung `rx_mac_data: empty PDU not passed to LLC`. Im Ausschnitt ist keine kausale Zuordnung zu einem hörbaren Fehler möglich. |
| 16:37:43.653 | Asterisk-Release `cause=16 (UnknownTetraIdentity)`, anschließend `DRelease` und Circuit-Abbau. Nicht als bewiesenes Fehlen der angerufenen Identität behandeln; heutige Cause-Zuordnung siehe Abschnitt 8. |
| 16:37:50.837 | Nach erneutem `Ctrl+C` geordnetes Beenden und normaler Brew-Verbindungsabbau. |

### 5.5 Statusmatrix zum historischen Chatende

| Bereich | Höchster belegter Status | Grenze |
|---|---|---|
| Rust-Build | **Getestet – Benutzerbestätigung** | Keine vollständige Buildausgabe, keine bewiesene Swap-Konfiguration. |
| Start als `netcore`, Konfiguration lesbar | **Im Betrieb bestätigt** | Dateiinhalt und effektive Rechte nicht vollständig geliefert. |
| SXceiver-/SoapySX-Erkennung | **Im Betrieb bestätigt** | Takt nur angenommen; exakte Hardware-/OS-Baseline fehlt. |
| Registrierung, Affiliation, SDS-Eingang | **Im Betrieb bestätigt** | Ein Teilnehmer; SDS-Endzustellung und Packet Data nicht nachgewiesen. |
| Gruppenruf auf Carrier 680 / TS2 | **Im Betrieb bestätigt** | Signalisierung/Frames/Abbau belegt, keine zweite Empfangsseite oder Audioaufnahme. |
| SIP-Ruf | **Getestet, Teilpfad im Betrieb belegt** | Bis `media ready`; Release-Ursache und beidseitige Sprache offen. |
| Dual-Carrier | **Initialisierung im Betrieb bestätigt** | Keine belegte C2-Belegung, Parallelruf- oder Kapazitätsabnahme. |
| Lokaler Recorder / AudioPlayer | **Fehler im Betrieb bestätigt** | Nicht erfolgreich initialisiert. |
| SNDCP-Gateway | **Fehler im Betrieb bestätigt** | TUN-Erzeugung scheitert; kein erfolgreicher PDP-/IP-Test. |
| Lokale Edge-Autorität | **Zustandswechsel im Betrieb bestätigt** | Node Gateway bleibt unerreichbar; keine Recovery-/zentralen Diensttests. |
| Finale systemd-/Bootintegration | **Beschlossen/geplant** | Nur Vorentwurf und nächste Schritte vorhanden. |
| Dauerhafter Swap / alternative Codegen-Einstellung | **Idee** | Auswahl und Ausführung nicht belegt. |

## 6. Fehler, Diagnose und Grenzen der damaligen Lösungen

### 6.1 Recorder und AudioPlayer

Beide Starts melden `Recorder disabled: cannot initialize recording directory: Permission denied` und `Audio player disabled: cannot create /var/lib/netcore/audio: Permission denied`. Die nachfolgenden `entity Recorder not found`-Warnungen passen zur fehlgeschlagenen Registrierung der Recorder-Entity; diese Kette ist heute auch im Source nachvollziehbar.

Vorgeschlagen war, `/var/lib/netcore/audio` und `/var/lib/netcore/recordings` anzulegen und `/var/lib/netcore` rekursiv auf `netcore:netcore` und `0750` zu setzen. **Keine Ausführung oder erfolgreiche Aufnahme danach ist belegt.** Der konkrete Recording-Pfad muss aus der tatsächlich aktiven Konfiguration gelesen werden; `/var/lib/netcore/recordings` ist der vorgeschlagene Pfad, nicht ein explizit ausgegebener Logwert.

Für eine Fortsetzung sind gezielte Verzeichnisrechte vorzuziehen. Ein pauschales `chmod -R 0750` setzt auch für reguläre Dateien Ausführungsbits und kann bestehende Daten-/Rechtekonzepte verändern. Der Archiveintrag dokumentiert den alten Vorschlag, führt ihn aber nicht aus.

### 6.2 SNDCP, TUN und Gerätezugriff

Die Fehlermeldung nennt konkret `TUNSETIFF requires CAP_NET_ADMIN in the basis-station systemd unit`. Sie wiederholt sich ungefähr im 30-Sekunden-Abstand. Ein normaler manueller Start mit `sudo -u netcore` überträgt die benötigte Capability nicht automatisch.

Der letzte Chatstand fordert systemd-Capabilities und Zugriff auf `/dev/net/tun`, `/dev/spidev0.0`, `/dev/gpiochip0` und ALSA. Der heutige Repository-Drop-in enthält zusätzlich `CAP_NET_RAW`, die Gerätefreigaben und Cleanup. Das korrigiert den unvollständigen ersten Unitentwurf auf Dokumentationsebene, belegt aber keine Installation auf TBS-02.

Zwei weitere Grenzen: Die ausgesendete SYSINFO-Ankündigung `sndcp_service=true` ist kein Nachweis eines funktionierenden IP-Pfads. Außerdem meldet das beobachtete MS 5102 **`tetra_packet_data=false`**. Für eine Paketdatenabnahme ist daher ein geeignetes und passend konfiguriertes Endgerät nötig; der erfolgreiche Sprachteilnehmer allein genügt nicht.

### 6.3 Node Gateway und lokale Autorität

Brew ist verbunden, während die Control-Room-/Node-Gateway-Verbindung in Timeouts läuft. Diese Transportpfade sind getrennt zu diagnostizieren. Der Wechsel nach `Degraded`/`Isolated` und die Meldung `local edge authority active` belegen lokale Reaktion auf den Ausfall. Sie beweisen weder die vollständige Funktionsfähigkeit sämtlicher lokaler Ersatzdienste noch eine spätere Synchronisierung mit dem Core.

Die historische Bewertung „kein Basisstationsproblem“ ist deshalb enger zu lesen: Der RF-Pfad arbeitet in diesem Ausschnitt weiter; die zentrale Integration bleibt ein offener Betriebsfehler. Der genaue Grund des Timeouts – Adresse, Routing, Firewall, nicht gestarteter Dienst oder anderes – ist nicht belegt.

### 6.4 Start-Overruns, Takt und ALSA

Die Zählung über den vollständigen Anhang ergibt:

| Meldung | Anzahl | Einordnung |
|---|---:|---|
| `RX buffer overrun` | 2 | Einmal pro Start; 361.800 bzw. 359.100 übersprungene Samples. |
| `Too late to produce TX block` | 65 | 32 im ersten und 33 im zweiten Start; ausschließlich etwa 16:35:06.734–.783 bzw. 16:36:03.598–.647. |
| `SNDCP: packet gateway startup failed` | 6 | Wiederholter Fehler, keine erfolgreiche Erholung. Die jeweils zusätzliche Worker-Fehlermeldung wird nicht als weiterer Startversuch doppelt gezählt. |
| `ControlRoom transport connection failed` | 10 | Zweiter Lauf; Timeouts mit Wiederholungen. |
| `edge fallback transition` | 10 | Historisch auch wiederholte `Isolated -> Isolated`-Meldungen. |
| `entity Recorder not found` | 389 | 5 Control-Meldungen und 384 TmdSap-Meldungen nach ausgefallenem Recorderstart. |

Die Entscheidung, Start-Overruns zunächst zu beobachten, ist für diesen Mitschnitt nachvollziehbar: Nach der jeweiligen Startphase erscheinen hier keine weiteren gleichartigen TX-Deadlinewarnungen. Es gibt aber keinen mehrstündigen Lasttest. Die negativen `Lost -1200 samples`-Ausgaben werden als Diagnosewortlaut erhalten, nicht physikalisch als negative Verluste interpretiert.

ALSA meldet `Invalid CTL pulse`, Fehler 524 beim HDMI-Playback-Probe und nicht gefundene Device-IDs. Danach öffnet SoapySX dennoch das Gerät. Ein erfolgreicher SDR-Start ist damit belegt, ein generell fehlerfreies ALSA-Setup nicht. Ebenso bleibt die angenommene 38,4-MHz-Clock eine zu prüfende Hardwareeigenschaft.

## 7. Historische Befehle und Fortsetzungsabläufe

Alle folgenden Blöcke sind dokumentierte Abläufe. Die Statusangaben unterscheiden tatsächliche Ausführung und bloßen Vorschlag. In diesem Archivauftrag wurden sie nicht auf der Basisstation ausgeführt.

### 7.1 OOM-Diagnose und Buildfortsetzung

**Im Chat ausgeführt und mit Ausgabe belegt:**

```bash
sudo journalctl -k -b --no-pager | \
  grep -Ei 'oom|out of memory|killed process|memory cgroup'
```

**Vorgeschlagen; konkrete Ausführung dieser Variante nicht belegt:**

```bash
free -h
swapon --show
cd /opt/netcore-tetra/netcore
CARGO_BUILD_JOBS=1 cargo build --release -p bluestation-bs
```

Der historische Swapvorschlag lautete: `fallocate -l 8G /swapfile`, Modus `0600`, `mkswap`, `swapon`; nach dem Build optional `swapoff` und Entfernen der temporären Datei, alternativ ein Eintrag `/swapfile none swap sw 0 0` in `/etc/fstab`. Auch 4 GB wurden als mögliche Größe bei mehr RAM genannt. **Keine dieser Größen oder Persistenzentscheidungen ist als umgesetzt belegt.** Vor einer Wiederholung müssen vorhandene Swapdatei, freier Speicher und aktive Swapnutzung geprüft werden; der alte Vorschlag ist kein Auftrag, eine bestehende `/swapfile` zu überschreiben oder zu entfernen.

Als weitere Option wurde `CARGO_PROFILE_RELEASE_CODEGEN_UNITS=1` vorgeschlagen. Eine garantierte Verringerung des Spitzen-RAM lässt sich aus dem Chat nicht ableiten; insbesondere sind weniger Codegen-Units nicht pauschal eine Speicherreparatur. Der belegte Befund bleibt der OOM-Kill, der bestätigte Erfolg der spätere Buildabschluss. Bereits gebaute Crates sollten erhalten bleiben.

### 7.2 Binary und Konfiguration installieren

**Historisch vorgeschlagen:**

```bash
cd /opt/netcore-tetra/netcore
ls -lh target/release/bluestation-bs
file target/release/bluestation-bs
ldd target/release/bluestation-bs

sudo install -o root -g root -m 0755 \
  target/release/bluestation-bs /usr/local/bin/bluestation-bs
sudo ldconfig
ldd /usr/local/bin/bluestation-bs

sudo mkdir -p /etc/netcore
sudo install -o netcore -g netcore -m 0640 \
  config.toml /etc/netcore/config.toml
sudo install -o netcore -g netcore -m 0640 \
  config.toml.fallback /etc/netcore/config.toml.fallback
ls -lah /etc/netcore/
```

Der spätere Logstart belegt eine ausführbare Binary am Zielpfad und eine lesbare Hauptkonfiguration für `netcore`. Er bestätigt nicht jede einzelne Installationszeile, die Herkunft der installierten Binary oder eine vollständige Bibliotheksprüfung. Beim erneuten Einsatz dürfen vorhandene Konfigurationen nicht ohne Sicherung durch Repository-Beispiele ersetzt werden.

Die damaligen Kurzprüfungen `ldd ... | grep 'not found' || echo '... gefunden'` sind als alleiniger Erfolgsnachweis ungeeignet: Auch ein fehlgeschlagenes `ldd` kann dazu führen, dass `grep` keinen Treffer findet. Für eine Fortsetzung sind vollständige Ausgabe **und Exitstatus von `ldd`** zu prüfen. Native Versionen von `libtetra-codec`, `libgsm` und SoapySDR sind in diesem Chat nicht vollständig dokumentiert.

### 7.3 Richtiger Benutzer und optionale Gruppenmitgliedschaft

**Tatsächlich erfolgreich gestartet:**

```bash
sudo -u netcore -H env RUST_LOG=info \
  /usr/local/bin/bluestation-bs /etc/netcore/config.toml
```

**Nur optional vorgeschlagen:**

```bash
sudo usermod -aG netcore jan
newgrp netcore
cat /etc/netcore/config.toml >/dev/null && echo "Config lesbar"
```

Die damalige Formulierung „lesen/bearbeiten“ wird präzisiert: Gruppenmitgliedschaft ermöglicht bei `0640` Gruppenlesezugriff, **kein Gruppenschreibrecht**. Die Datei pauschal auf `0644` zu öffnen, wurde ausdrücklich verworfen. Verzeichnis-Durchsuchungsrechte und der tatsächlich verwendete Dienstbenutzer sind zusätzlich zu prüfen.

### 7.4 Datenverzeichnisse

Der historische Vorschlag umfasste `mkdir -p` für Audio und Recordings sowie rekursives `chown`/`chmod` unter `/var/lib/netcore`. Die Durchführung ist offen. Eine **heute abgeleitete, ebenfalls nicht ausgeführte** gezielte Variante für genau diese konfigurierten Pfade wäre:

```bash
sudo install -d -o netcore -g netcore -m 0750 \
  /var/lib/netcore/audio /var/lib/netcore/recordings
sudo -u netcore test -w /var/lib/netcore/audio
sudo -u netcore test -w /var/lib/netcore/recordings
```

Vorher den tatsächlichen Recording-/Cache-/Archivpfad und bestehende Eigentümer ermitteln. Diese Verzeichnisbefehle reparieren nicht automatisch bereits vorhandene Dateien, andere konfigurierte Ziele oder systemd-Schreibschutz. Eine anschließende reale Aufnahme und Wiedergabe ist erforderlich.

### 7.5 Vorläufige `tetra.service` und noch fehlende Ergänzung

**Historischer Vorentwurf, nicht als installiert nachgewiesen:**

```ini
[Unit]
Description=NetCore TETRA Basisstation
Wants=network-online.target
After=network-online.target

[Service]
Type=simple
User=netcore
Group=netcore
SupplementaryGroups=audio
WorkingDirectory=/opt/netcore-tetra/netcore
ExecStart=/usr/local/bin/bluestation-bs /etc/netcore/config.toml
Environment=RUST_LOG=info
Restart=on-failure
RestartSec=5
TimeoutStopSec=20
LimitNOFILE=65536

[Install]
WantedBy=multi-user.target
```

Geplanter Ort: `/etc/systemd/system/tetra.service`. Der Entwurf enthält weder SNDCP-Capabilities noch die später geforderte vollständige Gerätefreigabe. `SupplementaryGroups=audio` allein belegt außerdem keine Unix-Zugriffsrechte auf SPI/GPIO. Er darf nicht als endgültige abgenommene Unit übernommen werden.

Die im Chat vorgeschlagenen nächsten Verwaltungsbefehle waren:

```bash
sudo systemctl daemon-reload
sudo systemctl enable tetra.service
sudo systemctl start tetra.service
sudo systemctl status tetra.service --no-pager --full
sudo journalctl -u tetra.service -b -n 200 --no-pager
sudo journalctl -u tetra.service -f
```

Der damalige Assistent begrenzte den unmittelbaren Schritt ausdrücklich auf den manuellen Start; deshalb gelten diese systemd-Befehle **nicht** als bereits ausgeführt.

### 7.6 Heute vorhandenen Paketgateway-Helfer verwenden

Im aktuellen Repository ist der vom historischen Log genannte Helfer implementiert. Für eine Fortsetzung mit bereits existierender Unit und vollständigem, ausgewähltem Quellstand ist die konkrete Syntax:

```bash
cd /opt/netcore-tetra/netcore
sudo sh contrib/packet-data/netcore-tetra-packet-gateway-install tetra.service
```

**Implementierter Helfer, auf TBS-02 hier nicht ausgeführt.** Er verlangt root, eine vorhandene Unit, `ip` und `nft` oder `iptables`, `/dev/net/tun` sowie die im Quellbaum enthaltenen Drop-in-/Cleanup-Dateien. Er kopiert den Drop-in nach `/etc/systemd/system/tetra.service.d/20-packet-data-gateway.conf` und den Cleanup-Helfer nach `/usr/local/libexec/netcore-tetra-packet-gateway-cleanup`. Eine bereits aktive Unit wird vom Helfer neu gestartet; eine inaktive Unit wird nicht automatisch gestartet. Ein kurzer `is-active`-Check im Helfer ersetzt keine Funk-/IP-Abnahme.

**Geplante Nachkontrolle, nicht historisch ausgeführt:**

```bash
sudo systemctl cat tetra.service
sudo systemctl show tetra.service \
  -p User -p Group -p SupplementaryGroups \
  -p AmbientCapabilities -p CapabilityBoundingSet \
  -p PrivateDevices -p DevicePolicy -p DeviceAllow \
  -p ProtectKernelTunables
sudo journalctl -u tetra.service -b --no-pager
ip -brief link
```

Den tatsächlichen TUN-Namen aus der aktiven Konfiguration bzw. dem erfolgreichen Startlog entnehmen. Der vorliegende fehlerhafte Start belegt noch keine erzeugte TUN-Schnittstelle und keinen bestimmten Schnittstellennamen.

## 8. Heute verifizierter Repository-Stand – getrennt vom Chat

Die folgenden Befunde stammen aus direkter Source-Lektüre am 2026-10-05. Die relevanten Bereiche `Cargo.toml`, `bins/bluestation-bs`, `crates/tetra-entities`, `crates/tetra-pdus`, `contrib/packet-data`, `contrib/systemd`, `install` und `sxxcvr-main/SoapySX` wurden zwischen dem genannten `Archiving`- und `main`-Commit verglichen; dafür ergab sich **kein Inhaltsunterschied**. Die Quelllinks sind auf den geprüften Archivbasis-Commit festgelegt.

| Behauptung / Thema | Heutiger Befund | Status und Konsequenz |
|---|---|---|
| Hauptkonfiguration mit `.fallback` | [R1]: Loader versucht exakt `<config>.fallback`, protokolliert beide Fehler und beendet ohne gültige Datei. | **Implementiert**; passt zum historischen Berechtigungsfehler. Kein Beleg für heutige Rechte auf TBS-02. |
| `MAIN-COMPAT` | [R1]: Banner und lokale Runtime vorhanden. | **Implementiert**; Banner allein identifiziert keinen unveränderten historischen Source-Stand. |
| Recorder-/Audio-Ausfall | [R1], [R2], [R3]: Entities werden nach erfolgreicher Initialisierung registriert; Verzeichnisfehler verhindern dies. | **Implementiert**; erklärt die Folge `Recorder not found`. Keine heute bestätigte Rechtekorrektur. |
| Native Audioabhängigkeit | [R4], [R5]: Defaultfeatures `asterisk`, `recording`, `audio-player`; Codec-FFI mit `#[link(name = "tetra-codec")]`, Suchpfad über `pkg-config` im Buildskript. | **Implementiert**; die tatsächliche native Bibliotheksinstallation auf TBS-02 bleibt unvermessen. Alte Packaging-Kommentare ersetzen die Featuredefinition nicht. |
| Soapy-/Center-Unterstützung | [R6]: unterstütztes Device öffnen, Center-Overrides verarbeiten. | **Implementiert**; die konkreten 600 kS/s/Gains/Perioden stammen aus dem historischen Log. |
| Dual-Carrier-Modell | [R7]: aktueller Banner **v2.8**, `C2 TS1 control/guard`, Traffic `TS5–TS7`, `SecondaryBcchNoMcch`. | **Abweichung zum historischen v2.9 / idle-silent**. Beide Stände getrennt halten; weder stillschweigend gleichsetzen noch einen heutigen Rollback aus dem Log ableiten. |
| TUN-Berechtigungsfehler | [R8]: `TUNSETIFF` und gezielte Diagnose bei `PermissionDenied`. | **Implementiert**; historisch fehlgeschlagen, heutiger Gerätebetrieb nicht geprüft. |
| systemd-Paketgateway | [R9], [R10]: `CAP_NET_ADMIN CAP_NET_RAW`, `PrivateDevices=no`, TUN/SPI/GPIO/`char-alsa`, `ProtectKernelTunables=no`, Runtime-Verzeichnis mit `0750`, Cleanup beim Stop. | **Implementiert**; umfangreicher als die zwei Capability-Zeilen des historischen Assistentenvorschlags. |
| Edge-Fallback | [R11]: Gateway-Verbindung, Frische der Healthmatrix und benötigte Dienste bestimmen `Degraded`/`Isolated`; zeitabhängige Erholung über `Recovering` nach `Online`. | **Implementiert**; heutiger Setter loggt nur bei geänderter Mode/Begründung. Historische wiederholte identische `Isolated`-Meldungen nicht als heutiges Verhalten ausgeben. |
| Config-Reparatur | [R12]: bestehende Unit/Config nötig; Dienstbenutzer/-gruppe ermitteln, Config-Verzeichnis `root:<Dienstgruppe>`/`0750`, Hauptdatei `root:<Dienstgruppe>`/`0660`, Lesetest und Dienststart. | **Implementiert, anderes Rechtekonzept** als historisch `netcore:netcore`/`0640`. Schreibt Gruppenrechte und behandelt nicht automatisch die `.fallback`-Datei. Kein universeller Ersatz für die historische Anleitung. |
| Update mit Rechteerhalt | [R13]: Parser-Regressionstests, Build, optionale TTS-Konfigurationsmigration mit Wiederherstellung von UID/GID/Modus, Binary-Backup und Fehlerbehandlung. | **Implementiert**, in diesem Auftrag nicht ausgeführt. Der Helfer erzwingt in seinem Buildaufruf nicht selbst `CARGO_BUILD_JOBS=1`; Low-RAM-Buildbedingungen gesondert festlegen. |
| Zentralisierung TTS | [R1]: lokale Basisstations-TTS wird als deprecated deaktiviert und auf Media Library verwiesen. | **Implementiert**; zentrale Verfügbarkeit nicht aus dem historischen Log ableitbar. |
| SIP-Abbruchgründe | [R14], [R15]: explizite SIP-/Q.850-zu-TETRA-Abbildung; Q.850 16/26 wird `UserRequestedDisconnection`. BYE/CANCEL und fehlerhafte INVITEs haben Regressionstestcode. | **Korrektur im heutigen Source implementiert**; Tests hier nicht ausgeführt und Einbau in historische Binary nicht belegt. |

### 8.1 Besondere Korrektur: `cause=16` ist ohne Namensraum mehrdeutig

Im historischen Log interpretiert CMCE Asterisk-Release 16 als `UnknownTetraIdentity`. Der aktuelle TETRA-Enum definiert diesen Wert tatsächlich als 16; der heutige SIP-Grenzadapter übersetzt Q.850-Causes dagegen vor der Übergabe an CMCE. Seine Tests erwarten für normales BYE/CANCEL TETRA-Cause 1.

**Schlussfolgerung, keine bewiesene historische Ursache:** Der alte Befund ist mit einer früheren Vermischung der Cause-Namensräume vereinbar. Ohne SIP-Mitschnitt und exakten historischen Source kann nicht entschieden werden, ob die Gegenstelle wirklich eine unbekannte Identität meldete oder ein normales Auflegen falsch benannt wurde. Der heutige Code adressiert gerade diese Fehlerklasse. Offen bleibt ein Test mit der tatsächlich eingesetzten Binary und den SIP-Reason-Informationen; ein neuer Identitätsfehler darf nicht allein aus dem alten Wortlaut behauptet werden.

## 9. Ersetzte, präzisierte und nicht bestätigte Aussagen

| Historische Aussage / Ansatz | Für die Fortsetzung maßgebliche Einordnung |
|---|---|
| Manueller Start im aktuellen Benutzerkontext | Nach `Permission denied` ausdrücklich auf `sudo -u netcore -H ...` korrigiert; Erfolg im Anhang sichtbar. |
| „Komplette HF-Kette real arbeitet“, praktisch vollständiger Integrationstest | Anmeldung, Signalisierung, Gruppenruf und Frames sind belegt. Sprachqualität, RF-Konformität, C2-Last, Packet Data, Recorder und Dauerbetrieb bleiben offen. |
| Node Gateway sei „kein Basisstationsproblem“ | Lokaler RF-Betrieb geht weiter; die zentrale Anbindung ist weiterhin ausgefallen. |
| Einfache Unit mit nur `SupplementaryGroups=audio` | Spätere Anforderung ergänzt SNDCP-Capabilities und Gerätezugriff; finale Unit fehlt. |
| `jan` durch Gruppenzugehörigkeit lesen und bearbeiten lassen | `0640` erlaubt Gruppenlesen; Bearbeiten braucht ein gesondertes Rechte-/Adminverfahren. |
| Rekursiv `/var/lib/netcore` auf `0750` setzen | Alter, unbestätigter Vorschlag; heute gezielte Verzeichnis-/Dateirechte anhand aktiver Pfade und bestehender Daten vorsehen. |
| `codegen-units=1` sei sicher deutlich speichersparender | Nicht durch einen Vergleich belegt; als optionale historische Idee erhalten. |
| `ldd`-Pipeline ohne Treffer bedeute alle Bibliotheken vorhanden | Vollständige Ausgabe und Prozessstatus erforderlich. |
| Aktueller Repo-Stand entspreche der laufenden v1.3.0-Binary | Nicht belegt; historische Kurzkennung nicht aufgelöst, UMAC-Banner und Cause-Behandlung unterscheiden sich. |

Die letzte ausdrückliche Benutzeranforderung für diese Arbeit beschränkt sämtliche Repository-Änderungen auf `Docs/archive/`, erlaubt Commit und Push auf `Archiving` und schließt Force-Push und Merge aus. Die hier aufgeführten Verbesserungen bleiben Dokumentation und Roadmap; an Runtime, Installationsskripten oder Konfigurationen wird dafür nichts geändert.

## 10. Offene Aufgaben und nächste Schritte

Die Reihenfolge **Rechte → endgültige systemd-Integration → autonomer Rebootstart** folgt dem letzten technischen Chatstand. Die zusätzlich aus Log und Repository abgeleiteten Prüfungen sind als solche gekennzeichnet; sie waren keine bereits abgeschlossene Vereinbarung.

1. **Aktiven Zielstand sichern und zuordnen.** Installierte Binary samt Prüfsumme, vollständigen Source-Commit, SoapySX-Version, OS/Architektur, RAM/Swap, aktive Unit und bestehende Konfigurationen lokal sichern. Die Kennung `b63b251b` und die tatsächliche Carrier-Semantik nachvollziehen. Zugangsdaten nicht in neue Logs/Archive übernehmen.
2. **Rechte und Datenpfade korrigieren – historisch nächste Aufgabe.** Haupt- und Fallback-Konfiguration als Dienstbenutzer prüfen; tatsächlich konfigurierte Recording-, Audio-, Cache- und Archivpfade feststellen. Gezielt Berechtigungen reparieren, danach Recorder-/AudioPlayer-Initialisierung und reale Aufnahme/Wiedergabe belegen.
3. **`tetra.service` vervollständigen – historisch geplant.** Arbeitsverzeichnis und Binarypfad bestätigen, Unix-Gruppen/ACLs und systemd-Gerätefreigaben zusammen prüfen, Paketgateway-Drop-in integrieren. Manuellen und systemd-Start nicht gleichzeitig auf dieselbe SDR-Hardware richten. Anschließend Start/Stop, Neustart und Reboot testen.
4. **SNDCP Ende-zu-Ende abnehmen – aus dem Fehler abgeleitet.** Erfolgreiche TUN-Erzeugung, Schnittstellen-/Routing-/Firewallzustand und PDP/IP-Verkehr mit einem packet-data-fähigen MS dokumentieren. Ein `sndcp_service=true`-Broadcast und das hier beobachtete MS-Profil 5102 reichen nicht.
5. **Node Gateway und lokalen Fallback fertigstellen – Benutzerwunsch, Details noch offen.** Reale Zieladresse und Betriebsmodus prüfen, TCP-/Anwendungskonnektivität und Healthmatrix belegen; anschließend `Recovering -> Online` sowie erneuten kontrollierten Ausfall/Recovery mit lokalen Funktionen prüfen. Brew-Verbindung und Core-Verbindung getrennt protokollieren.
6. **SIP-Cause-Regression und Sprache prüfen – zusätzliche Logerkenntnis.** Aktuelle Mappingimplementierung bzw. deren Installation feststellen; normalen Ruf 91103 → 103, beidseitige Audioübertragung und Auflegen testen. SIP-Reason und TETRA-Cause korrelieren. Vorhandene Regressionstests auf der passenden Linux-Buildumgebung ausführen.
7. **RF-/Dauerlasttest nachholen – zusätzliche Absicherung.** Reale Clock-/Hardwaredaten erfassen, ALSA-Probewarnungen einordnen, Overrun-/Deadline-Verhalten nach Warmstart und unter Last messen; zweiten Carrier und parallele Rufe gezielt testen. Kein pauschales Tuning allein aus den Startmeldungen ableiten.
8. **Fehlende Dokumentationsbasis ergänzen.** Frühen Installationsdialog, verwendete OS-/Paket-/Codec-/Soapy-Kommandos, finale Unit und spätere Erfolgsausgaben nachtragen, sobald verfügbar. Es liegen keine belegten weiteren Nebenideen aus dem abgeschnittenen frühen Verlauf vor.

Offene Ideen bleiben: Swap nur temporär oder dauerhaft, optionale Leseberechtigung für `jan` und die alternative Codegen-Einstellung. Sie wurden im sichtbaren Verlauf nicht endgültig ausgewählt.

## 11. Anhangsarchiv, Bereinigung und Prüfsummen

Der lesbare Originalanhang wurde vollständig für Ereignisse, Fehlerklassen und Zeitabläufe ausgewertet. Veröffentlicht wird eine **bereinigte Ableitung mit identischer Zeilenzahl und Reihenfolge**, keine behauptete byteidentische Originalkopie.

- **Original:** 244.548 Bytes, 1.146 Zeilen, SHA-256 `d45e214803fc796801b700df174ea953c230b6377bfedbd20112c470c570644b`.
- **Archivierte Ableitung:** 235.981 Bytes, 1.146 Zeilen, UTF-8/LF, SHA-256 `3b909deb3e0c41e05ba626b536b8a1db5c526668a0b25438f8fcc4c8d42514c5`.
- Entfernt wurden **187 opake Bit-Dumps** sowie die **dezimalen und hexadezimalen Darstellungen des einen SDS-Payloads**. Die Stellen tragen `[REDACTED-BITS]` bzw. `[REDACTED-SDS-PAYLOAD]`. Ungeprüfte Binärnutzdaten werden so nicht als möglicherweise vertraulicher Inhalt veröffentlicht.
- Zeitstempel, Fehler, Protokollfelder, Call-IDs, ISSI/GSSI, interne Betriebsadressen und Hardwareparameter bleiben für die technische Fortsetzung erhalten. Passwörter, Tokens oder Schlüssel wurden nicht übernommen. Konfigurationsdateien mit möglichen Secrets wurden nicht ins Archiv kopiert.
- Details und reproduzierbare Ereigniszählungen stehen in [provenienz.json](assets/2026-10-05_installation-ab-null-tbs02/provenienz.json).

Orientierung in der archivierten, zeilengleich gebliebenen Ableitung:

| Zeilen | Inhalt |
|---|---|
| 1–68 | Erster Start, Version, Frequenzen, SDR-Settings, Verzeichnis-/SNDCP-Fehler, Brew-Verbindung |
| 119–208 | Erstregistrierung, Gruppenaffiliation und SDS-Eingang |
| 214–222 | Geordnetes Beenden, `nano`-Aufruf und zweiter Start |
| 256–349 | Zweite SDR-Initialisierung, Control-Room-Aktivierung, Timeouts und lokaler Fallback |
| 350–472 | Rückkehr des MS, zunächst abgewiesener Ruf, nachgeführte Affiliation |
| 532–726 | Erfolgreich signalisierter Gruppenruf, TX-Ende, Hangtime und geordneter Abbau |
| 736–782 | Asterisk-Ruf 91103 → 103 bis `media ready` |
| 1106–1146 | SIP-/TETRA-Release, Circuit-Abbau und manuelles Beenden |

## 12. Repository-Quellen und Verweise

Die folgenden Links bezeichnen den geprüften Code am Archivbasis-Commit, nicht einen nachträglich veränderlichen Branchinhalt:

- [R1] – Start, Konfigurationsfallback, Entityregistrierung, Laufzeitbanner und TTS-Hinweis: `bins/bluestation-bs/src/main.rs`.
- [R2] – Recorder-Initialisierung: `crates/tetra-entities/src/net_recorder/entity.rs`.
- [R3] – AudioPlayer-Verzeichnisinitialisierung: `crates/tetra-entities/src/net_audio_player/service.rs`.
- [R4] – Binary-Features: `bins/bluestation-bs/Cargo.toml`.
- [R5] – Native Codec-Anbindung: `crates/tetra-entities/src/net_audio/codec.rs`; ergänzend `crates/tetra-entities/build.rs`.
- [R6] – Soapy-Geräte-/Centerpfad: `crates/tetra-entities/src/phy/components/soapyio.rs`.
- [R7] – Heutiges Dual-Carrier-/Timeslotmodell: `crates/tetra-entities/src/umac/umac_bs.rs`.
- [R8] – TUN-Erzeugung und Rechtefehler: `crates/tetra-entities/src/sndcp/packet_gateway.rs`.
- [R9] – systemd-Drop-in: `contrib/systemd/tetra.service.d/20-packet-data-gateway.conf`.
- [R10] – Installationshelfer: `contrib/packet-data/netcore-tetra-packet-gateway-install`.
- [R11] – Gateway-/Dienstzustände und lokale Autorität: `crates/tetra-entities/src/net_control_room/worker.rs`.
- [R12] – Konfigurationsrechtereparatur: `install/repair-basisstation-config-permissions.sh`.
- [R13] – Updateablauf: `install/update-basisstation.sh`.
- [R14] – SIP-/Q.850-Cause-Abbildung: `crates/tetra-entities/src/net_asterisk/causes.rs`.
- [R15] – Verwendung und vorhandene Regressionstests: `crates/tetra-entities/src/net_asterisk/entity.rs`.

[R1]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/bins/bluestation-bs/src/main.rs
[R2]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_recorder/entity.rs
[R3]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_audio_player/service.rs
[R4]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/bins/bluestation-bs/Cargo.toml
[R5]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_audio/codec.rs
[R6]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/phy/components/soapyio.rs
[R7]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/umac/umac_bs.rs
[R8]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/sndcp/packet_gateway.rs
[R9]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/contrib/systemd/tetra.service.d/20-packet-data-gateway.conf
[R10]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/contrib/packet-data/netcore-tetra-packet-gateway-install
[R11]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_control_room/worker.rs
[R12]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/install/repair-basisstation-config-permissions.sh
[R13]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/install/update-basisstation.sh
[R14]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_asterisk/causes.rs
[R15]: https://github.com/JanHG98/netcore-tetra/blob/45c13959b0e030e4162344fbf2b5d29fbcd820d3/crates/tetra-entities/src/net_asterisk/entity.rs

Verwandte, bereits vorhandene Archive wurden vor dem Schreiben geprüft und nicht überschrieben:

- [Basisstation Clean Install, SXceiver/SoapySX, SNDCP/TUN und Healthchecks](2026-10-05_basisstation-clean-install-sxceiver-soapysx-sndcp-healthchecks.md): anderer historischer Zeitraum und Host TBS-01; kein Ersatz für diesen TBS-02-Chat.
- [Main-kompatibler RF-Pfad, Pi-Neuinstallation und SWMI-Fallback](2026-10-05_main-kompatibler-rf-pfad-pi-neuinstallation-sxceiver-und-swmi-fallback.md): anderer historischer Chat/Source-Stand.
- [Archivindex](README.md): vorhandene Einträge bleiben erhalten; dieser eindeutig durch Chat-ID und Titel zugeordnete Eintrag wird ergänzt.

## 13. Prüfung und Publikationsumfang dieses Archivauftrags

Für die Archivierung wurden der Remote-Branch und ein sauberer separater Checkout geprüft, vorhandene Archive nach Chat-ID, Titel, Host und Binarykennung durchsucht und der Index gelesen. Es gab keinen eindeutig diesem Chat zugeordneten bestehenden Eintrag. Die neue Datei überschreibt deshalb keine fremde Zusammenfassung.

Der Publikationsumfang umfasst ausschließlich diese Dokumentation, die neue Indexzeile sowie den bereinigten Log und seine Provenienzdatei unter `Docs/archive/`. Die Prüfung umfasst Pfadbeschränkung, unveränderte bisherige Indexinhalte, lokale Links und festgelegte Source-Referenzen, Logprüfsummen, unveränderte Zeilenzahl, Ereigniszählungen und die Bereinigung opaker Nutzdaten. Vor und nach der Veröffentlichung werden Commitumfang und Remote-Stand kontrolliert.

Diese Dokumentationsprüfung ist **kein** Rust-Build, kein Deployment, kein heutiger RF-Test und keine Abnahme der offenen Betriebsaufgaben. Eine PR- oder Releasezuordnung zum historischen Build ist nicht belegt und wird nicht erfunden.
