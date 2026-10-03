# Technische Chat-Abschlussdokumentation: Startbanner, Runtime-Diagnose, Versionierung, ARM64-Releases und rustfmt

> **Archivdokument, kein Implementierungs- oder Betriebsnachweis.** Dieses Dokument trennt die im Chat erarbeiteten Vorschläge von einem am 03.10.2026 zusätzlich geprüften Repository-Stand. Insbesondere sind eine Beispielausgabe, ein vorgeschlagener Workflow und ein im Chat eingefügter Rust-Ausschnitt kein Beleg für einen erfolgreichen Build oder eine produktive Installation.

## 1. Metadaten und Auswertungsumfang

| Merkmal | Wert |
|---|---|
| Projekt / Zielrepository | `JanHG98/netcore-tetra` |
| Thema | NetCore-Startbanner in `bluestation-bs`, Konfigurationsübersicht beim Start, `STACK_VERSION`, Cargo-/Git-Versionierung, ARM64-Release-Automatisierung und Rustfmt-Edition |
| Ursprünglicher Chattitel | Nicht verfügbar; der Titel dieses Dokuments ist eine beschreibende Archivbezeichnung. |
| Ursprünglicher Chatlink | Nicht verfügbar; es wird kein Chatlink konstruiert. |
| Erstellungsdatum | **2026-10-03**, Zeitzone `Europe/Berlin` |
| Historischer Gesprächszeitraum | Im unmittelbar sichtbaren Einzelchat nicht zuverlässig datiert; nicht mit dem Erstellungsdatum dieses Archivs gleichsetzen. |
| Geprüfter Zielbranch | `Archiving` |
| Für die technische Prüfung fixierter Commit | `64b381186d02445a3839d737c928ac15d42b4648` |
| Zusätzlich aufgelöster Referenzbranch | `main` bei `6aa9be8f74ab731f72dc133a5f8e90c5018c626d` |
| Vergleich dieser beiden Prüfsnapshots | `Archiving` war sieben Commits voraus und null zurück. Der Vergleich enthielt auch WebUI-/Quellcodeänderungen, nicht ausschließlich Archivdateien. |
| Historisch zuerst betrachtetes Fremdrepository | `razvanzeces/flowstation`, insbesondere `bins/bluestation-bs/src/main.rs` |
| Ablagepfad | `Docs/archive/2026-10-03_startbanner-runtime-versionierung-arm64-release-rustfmt.md` |
| Archivindex | `Docs/archive/README.md` |
| Änderungsumfang dieses Auftrags | Diese Archivdatei und der Archivindex; keine Produktcode-, Workflow-, Versions-, Tag- oder Releaseänderung. |

**Prüfcommit und Ablagecommit sind unterschiedliche Begriffe.** Die oben genannten Hashes fixieren den untersuchten Code. Der Commit, der dieses Dokument speichert, wird erst beim Schreiben erzeugt und ist über die Git-Historie dieser Datei und die Abschlussmeldung nachvollziehbar. Bei der Ablage ist der dann neueste Stand von `Archiving` als Elternstand zu verwenden; zwischenzeitliche Archivbeiträge dürfen nicht ersetzt werden.

Ausgewertet wurde der verfügbare Verlauf vom ersten FlowStation-Link über die Banner- und Diagnoseblöcke, die Pseudoausgabe, die Versions- und Workflowfragen bis zur abschließenden Frage nach `edition = "2026"`. Eine ergänzende Verlaufssuche lieferte keinen belastbaren Originaltitel und keinen eindeutigen Chatlink. Allgemeine Projektinformationen und benachbarte Chats wurden nicht als Entscheidungen dieses Einzelchats übernommen.

Die 25 zugänglichen PDF-Anhänge sind in Abschnitt 13 inventarisiert. Ihre Dateien und Seitenzahlen wurden geprüft; ihre im Kontext sichtbaren Titel dienen der Einordnung. Eine vollständige normative Auswertung sämtlicher ETSI-Dokumente wurde **nicht** durchgeführt und ist für die hier besprochenen Banner-, Cargo- und Workflowänderungen nicht erforderlich. Es gibt im sichtbaren Verlauf keine aus diesen Anhängen abgeleitete Implementierungsentscheidung zu diesem Thema.

### 1.1 Statusbegriffe

| Status | Bedeutung in diesem Dokument |
|---|---|
| **Idee** | Diskutierte Möglichkeit ohne verbindliche Umsetzung oder Abnahme. |
| **Beschlossen/geplant** | Ausdrücklicher Nutzerwunsch oder konkret gewählte Arbeitsrichtung; Umsetzung noch separat nachzuweisen. |
| **Implementiert** | Im bezeichneten Repository-Snapshot tatsächlich als Code oder Konfiguration gefunden. |
| **Getestet** | Ein konkreter ausgeführter Test mit Ergebnis ist benannt. Quellcodelektüre allein reicht dafür nicht. |
| **Im Betrieb bestätigt** | Eine zuordenbare Laufzeitbeobachtung auf dem Zielsystem liegt vor. |

## 2. Zusammenfassung für die Fortsetzung

Der Nutzer wollte aus dem bisherigen BlueStation-Starttext einen deutlich erkennbaren NetCore-Tetra-Startbildschirm machen: zunächst Branding, anschließend bewusst humorvolle Bootmeldungen und schließlich eine technisch brauchbare Übersicht über die geladene Funkzellen- und Dienstkonfiguration. Wichtig war eine **direkt in `main()` einfügbare Lösung ohne zusätzliche Hilfsfunktionen oder zusätzliche Imports**.

Im weiteren Verlauf wurde geklärt, dass die sichtbare Stackversion aus der Cargo-Paketversion und einer beim Kompilieren ermittelten Git-Kennung zusammengesetzt wird. Der Nutzer zeigte seine damalige Workspace-Version `0.0.8` und einen bestehenden ARM64-Release-Workflow. Die Vorschläge wechselten deshalb von einem zunächst unpassenden nativen x86_64-Build zu einer Erweiterung des vorhandenen `cross`-Builds für `aarch64-unknown-linux-gnu`.

**Das heutige Prüfergebnis ist nicht identisch mit dem Chatentwurf:**

- Das Repository enthält ein kompaktes NetCore-Banner und den Hinweis `Radio runtime: MAIN-COMPAT (local MM/MLE/CMCE state machines)`, aber nicht den großen humorvollen Banner-/Runtime-Summary-Block aus dem Chat.
- Die Workspace-Version ist im Prüfsnapshot `1.3.0`, nicht `0.0.8` oder eines der späteren Beispiel-Bumps. `STACK_NAME`, `STACK_CODENAME` und `STACK_DISPLAY` sind inzwischen vorhanden.
- Die aktuelle Git-Kennung besitzt **kein vorangestelltes `g`**. Die Option für `-modified` wurde inzwischen bewusst entfernt; der Codekommentar nennt lokale Operator-Patches und den OTA-Commitvergleich als Hintergrund.
- Im geprüften `.github/workflows/` befindet sich **kein Release-Workflow**. Der damals gezeigte Workflow und die vorgeschlagenen Erweiterungen sind dort somit nicht als aktueller Releasepfad bestätigt.
- `rustfmt.toml` verwendet weiterhin `edition = "2024"`; ein explizites `style_edition = "2024"` wurde im Prüfsnapshot nicht gefunden.

Quellen für diese heutigen Feststellungen: [R1](#r1-einstiegspunkt-und-startreihenfolge), [R2](#r2-versionierung), [R3](#r3-manifeste), [R4](#r4-formatierung-und-workflows). Diese Befunde sind eine Quellcodeprüfung, kein erfolgreicher Funk- oder Releasebetrieb.

## 3. Ziel, Ausgangslage und Gesprächsverlauf

### 3.1 Ursprünglicher Einstieg

Der Nutzer verwies auf `https://github.com/razvanzeces/flowstation` und dort auf `bins/bluestation-bs/src/main.rs`. Der zitierte Ausgangsblock druckte ein dreizeiliges TETRA-BlueStation-Banner, `Wouter Bokslag / Midnight Blue`, den Verweis auf `MidnightBlueLabs/tetra-bluestation` und `tetra_core::STACK_VERSION`.

Der Nutzer hatte nach eigener Aussage bereits ein eigenes ASCII-/Unicode-Banner und folgende Texte vorbereitet:

```text
Netcore-Tetra Systems
Sichere Kommunikation auf die man sich verlassen kann.
Systems up. Nothing broken (yet).
https://netcore-tetra.de
Version: <tetra_core::STACK_VERSION>
```

Im dazu geposteten Rust-Code standen außerdem zweimal `eprintln!("/n");`. Das war ein echter Fehler im **gezeigten Ausschnitt**: `/n` ist normaler Text und kein Zeilenumbruch. Ein Build- oder Betriebsfehler wurde dazu nicht berichtet.

### 3.2 Entwicklung der Lösung

| Schritt | Inhalt | Einordnung |
|---|---|---|
| Branding erweitern | Banner, Website, technische Version und ein NetCore-Slogan | Nutzerwunsch; eigener Ausgangsentwurf lag im Chat vor. |
| Erste strukturierte Lösung | `print_startup_banner()` und `print_runtime_summary(...)` | Assistentenvorschlag, später als Einbauform verworfen. |
| Einfügen ohne Hilfsfunktionen | Direkt hintereinander stehende `eprintln!`-Anweisungen in `main()` | Ausdrückliche Anforderung des Nutzers. |
| Humor verstärken | Weitere Bootmeldungen und Schlusszeilen | Auf ausdrücklichen Wunsch erweitert. |
| Systemparameter ergänzen | Aus `cfg.config()` abgeleitete Übersicht plus `cfg.state_read()` | Assistent lieferte einen Inline-Entwurf. |
| Nutzer setzt Blöcke zusammen | Geposteter Beginn von `fn main()` mit Banner und Runtime Summary | Belegt einen zusammengestellten Codeausschnitt, nicht dessen Commit oder Ausführung. |
| Pseudoausgabe | Muster mit erfundenen Funk-, Versions- und Dienstwerten | Ausschließlich Layoutdemonstration; enthält technische Unstimmigkeiten. |
| Version festlegen | Workspace-`Cargo.toml`, `CARGO_PKG_VERSION`, Git-Kennung | Erklärter Mechanismus; spätere Prüfung unten. |
| GitHub-Automatisierung | Erst nativer Linux-Build, nach gezeigter Bestands-YAML ARM64-Cross-Build | ARM64 ist die konkretisierte Richtung. |
| Cargo-Version ändern | Nutzer zeigt `version = "0.0.8"`; manuelle Änderung wird erläutert | Keine endgültige neue Releaseversion vom Nutzer festgelegt. |
| Rustfmt 2026? | `edition = "2024"` beibehalten; `style_edition` als Zusatz vorschlagen | Keine bestätigte Änderung im Chat; heutiger Stand geprüft. |

Die vom Assistenten anfänglich genannten Upstream-Funktionslisten und eine angeblich aktuelle Upstream-Version `0.0.9` waren Kontextbehauptungen. Der später vom Nutzer gezeigte lokale Ausschnitt mit `0.0.8` ist für seinen **damaligen gezeigten Stand** vorrangig. Ein damaliger Upstream-Commit wurde nicht festgehalten und lässt sich aus den Chatbehauptungen nicht nachträglich erfinden.

## 4. Anforderungen, Entscheidungen und nicht getroffene Entscheidungen

### 4.1 Gesicherte Anforderungen aus dem Chat

1. **NetCore-Tetra-Branding statt des bisherigen sichtbaren BlueStation-Banners.** Das Banner soll Wiedererkennungswert haben; technische Paketnamen müssen deshalb nicht automatisch umbenannt werden.
2. **Copy-and-paste direkt in `main()`.** Die zuletzt gewünschte Einbauform benötigt keine separat zu definierende Bannerfunktion und keine neuen Imports allein für die Ausgabe.
3. **Humor ist ausdrücklich gewünscht.** Die Gestaltung darf deutlich über eine nüchterne Versionszeile hinausgehen.
4. **Zusätzlich echte System-/Konfigurationsparameter ausgeben.** Der Nutzer fragte ausdrücklich nach weiteren druckbaren Parametern und setzte den vorgeschlagenen Datenblock in seinen Ausschnitt ein.
5. **Die Versionsquelle verstehen und automatisierte Releases ermöglichen.** Der Nutzer zeigte schließlich den vorhandenen ARM64-Workflow als Grundlage; diese Information ersetzt die vorherige x86_64-Annahme.

### 4.2 Nicht als endgültig beschlossen zu behandeln

Die Beispiele `0.0.9`, `0.1.0` und zugehörige Tags waren Vorschläge, keine vom Nutzer verbindlich freigegebene neue Produktversion. Ebenso wenig wurden im sichtbaren Chat ein tatsächlicher Tag-Push, ein GitHub-Release, ein erfolgreicher ARM64-Build oder die Installation des Diagnoseblocks bestätigt.

Die Änderung von `authors`, die Umstellung von User-Agents, zufällige Bootzitate, `Monster armed`, das Präfix `NetCore-Tetra` direkt in `STACK_VERSION` und die Ergänzung von `style_edition` waren Vorschläge. Der spätere Repository-Zustand kann einzelne ähnliche Konzepte enthalten, beweist aber nicht, dass sie aufgrund dieses Chats umgesetzt wurden.

### 4.3 Fachliche Grenze der humorvollen Ausgabe

Die Texte `Systems up`, `Carrier armed`, `ready` und `standby` waren im Entwurf statische Strings. Sie führen keine Prüfung aus. Der Bannerblock steht sogar vor dem Parsen der Kommandozeile und vor dem Laden der Konfiguration. Er darf daher nicht als Zustandsnachweis für SDR, Funkträger, SDS, Audio, Authentisierung oder Netzverbindung interpretiert werden.

**Heutige Review-Empfehlung, keine nachträglich unterstellte Chatentscheidung:** Humor optisch von einer datengetriebenen Statusübersicht trennen. Vor erfolgter Initialisierung `starting`, `configured` oder `not checked` statt einer tatsächlichen Betriebsbereitschaft ausgeben.

## 5. Historischer Bannerentwurf

Der letzte vom Nutzer zusammengestellte große Block entspricht in seiner statischen Ausgabe diesem Entwurf; die Versionszeile bleibt dynamisch:

```text
░█▄░█░█▀▀░▀█▀░█▀▀░█▀█░█▀▄░█▀▀░░░░░▀█▀░█▀▀░▀█▀░█▀▄░█▀█
░█░▀█░█▀▀░░█░░█░░░█░█░█▀▄░█▀▀░▄▄▄░░█░░█▀▀░░█░░█▀▄░█▀█
░▀░░▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀░▀░▀▀▀░░░░░░▀░░▀▀▀░░▀░░▀░▀░▀░▀

════════════════════════════════════════════════════════════
  NetCore-Tetra Systems
  digital. dezentral. skalierbar.
  Sichere Kommunikation, auf die man sich verlassen kann.
════════════════════════════════════════════════════════════

  Website:        https://netcore-tetra.de
  Stack:          FlowStation / BlueStation Core
  Runtime:        TETRA Base Station
  Version:        <tetra_core::STACK_VERSION>

────────────────────────────────────────────────────────────
  [BOOT]          waking up radio gremlins
  [CONFIG]        loading operational chaos containment
  [RF]            preparing carrier, timing and mild anxiety
  [MAC]           sharpening access control crayons
  [MM]            waiting for terminals to introduce themselves
  [SDS]           ready for tiny packets with big opinions
  [VOICE]         group call machinery on standby
  [SECURITY]      checking who is allowed to annoy the ether
  [LOGGING]       preparing receipts for future debugging pain
════════════════════════════════════════════════════════════
  Systems up. Nothing broken yet.
  Carrier armed. Logs armed. Coffee substitute armed.
  If this works: engineering.
  If this fails: undocumented field test.
════════════════════════════════════════════════════════════
```

Leerzeilen sollten im Rust-Entwurf mit `eprintln!();` erzeugt werden. `eprintln!("\n");` gibt zusätzlich zum Zeilenumbruch im String den vom Makro angehängten Zeilenumbruch aus; es ist deshalb nicht die gleiche Layoutentscheidung wie genau eine leere Ausgabezeile. Das Makro schreibt nach Standardfehlerausgabe, nicht nach Standardausgabe. [E1](#e1-rust-standardbibliothek-und-cargo)

### 5.1 Weitere im Chat genannte Gestaltungsideen

Als Zwischenvarianten wurden `Operator mood: cautiously optimistic`, `RF forecast: 80% chance of packets, 20% chance of swearing`, `Systems up. Nothing broken yet. Probably.` und `If this smokes, it was probably not the ASCII art.` vorgeschlagen. Diese Zeilen wurden nicht alle in den letzten Nutzerblock übernommen.

Eine Liste zufällig auswählbarer Bootzitate wurde als spätere Option genannt:

```rust
const BOOT_QUOTES: &[&str] = &[
    "Systems up. Nothing broken yet.",
    "RF gremlins currently sleeping.",
    "Carrier stable. Ego also.",
    "Booting responsibly. Mostly.",
    "If this explodes, blame the config.",
];
```

Die Auswahl-/Zufallslogik wurde nicht geliefert oder nachgewiesen. Ebenfalls nur vorgeschlagen wurde der Austausch von `Coffee substitute armed` gegen `Monster armed`. Keine dieser Spielereien ist eine notwendige Abhängigkeit der technischen Diagnoseausgabe.

## 6. Historische Runtime Summary: Architektur und vollständiger Parameterumfang

### 6.1 Ursprünglicher Einfügepunkt

Der vorgeschlagene Inline-Block sollte nach diesen damaligen Zeilen beginnen:

```rust
let stack_cfg = load_config_from_toml(&args.config);
let mut cfg = SharedConfig::from_parts(stack_cfg, None);
let runtime_cfg = cfg.config();
```

Dann folgen `eprintln!`-Ausgaben und für optionale Bereiche `if let Some(...)`-/`match`-Blöcke. Für den veränderlichen Zustand war ein kurzer separater Bereich vorgesehen:

```rust
{
    let state = cfg.state_read();
    eprintln!("  ├─ Network connected: {}", state.network_connected);
    eprintln!(
        "  └─ Registered ISSIs:  {}",
        state.subscribers.all_registered_issis().count()
    );
}
```

**Nicht als heutige Einbauanweisung verwenden:** Der aktuelle Einstieg lädt Konfiguration mit Fallback und stellt vor dem Erzeugen von `SharedConfig` einen Edge-Policy-Cache wieder her. Die alte Initialisierung mit `None` darf diesen Ablauf nicht ersetzen. Der vom Nutzer gepostete Ausschnitt war außerdem keine vollständige `main.rs`: Der restliche Stackaufbau und der abschließende Funktionskörper waren darin nicht enthalten. [R1](#r1-einstiegspunkt-und-startreihenfolge)

### 6.2 Datenquellen und Ausgabefelder

Die folgende Tabelle bewahrt den vollständigen fachlichen Umfang des vorgeschlagenen Diagnoseblocks. Die Pfade beziehen sich auf den **Chatentwurf**; nicht jedes Feld wurde in dieser Archivierung separat kompiliert oder zur Laufzeit abgefragt.

| Bereich | Felder / Abfragen im Chatentwurf | Ausgabe und Besonderheiten |
|---|---|---|
| Allgemein | `args.config`, `runtime_cfg.stack_mode`, `tetra_core::STACK_VERSION`, `runtime_cfg.debug_log.is_some()` | Konfigurationspfad, Debug-Darstellung des Stackmodus, Stackversion, `configured` bzw. `default` für vorhandene Logkonfiguration |
| Network | `net.mcc`, `net.mnc` | Netzkennungen; keine im Chat verbindlich festgelegten Betriebswerte |
| Cell: Frequenz | `cell.main_carrier`, `freq_band`, `freq_offset_hz`, `duplex_spacing_id`, `custom_duplex_spacing`, `reverse_operation` | Frequenzoffset und Custom-Duplex in Hz; fehlender Custom-Wert als `none` |
| Cell: Identität | `cell.location_area`, `colour_code`, `system_code`, `subscriber_class` | Location Area, Colour Code, Systemcode, Teilnehmerklassenfeld |
| Cell: Funkbetrieb | `cell.sharing_mode`, `ts_reserved_frames`, `ms_txpwr_max_cell`, `late_entry_supported`, `u_plane_dtx`, `frame_18_ext` | Rohwerte/Flags; beim TX-Leistungsfeld wurde im Code keine physikalische Einheit ausgegeben |
| Services | `cell.registration`, `deregistration`, `voice_service`, `sndcp_service`, `circuit_mode_data_service`, `aie_service`, `advanced_link`, `priority_cell`, `migration`, `system_wide_services` | Konfigurierte Flags; keine vollständigen End-to-end-Funktionstests |
| Timers | `cell.hangtime_secs`, `call_timeout_secs`, `ul_inactivity_secs`, `periodic_registration_secs` | Jeweils Sekunden |
| Time / Broadcast | `cell.timezone.as_deref()`, `cell.neighbor_cell_broadcast` | Zeitzone oder `not configured`; Broadcast-Rohwert |
| PHY | `phy_io.backend`, `phy_io.soapysdr` | Backend per `Debug`; optionale SoapySDR-Unterkonfiguration |
| SDR-Auswahl | `soapy.device`, `rx_ch`, `tx_ch`, `rx_ant`, `tx_ant` | `auto` bzw. `default`, wenn nicht ausdrücklich konfiguriert |
| Sampling | `soapy.fs` | Ganzzahlig formatiert als `S/s`, andernfalls `device default` |
| Frequenzen | `soapy.ul_freq`, `dl_freq`, `ppm_err` | Nominalwerte in Hz sowie PPM-Wert |
| Frequenzkorrektur | `soapy.ul_freq_corrected()`, `dl_freq_corrected()` | Korrigierter Wert in Hz plus vorzeichenbehafteter Fehlerbetrag in Hz |
| Verstärkung | `soapy.rx_gains`, `tx_gains` | Debug-Darstellung der Maps oder `device default` |
| Lokale Zugriffsliste | `security.issi_whitelist.is_empty()`, `.len()` | `disabled`/`enabled`; Zahl der Einträge |
| Teilnehmerliste | `security.issi_whitelist` bei höchstens 20 Einträgen | Bis 20 Einträge vollständig; darüber `hidden, too many entries` |
| Nachbarzellen | `cell.neighbor_cells_ca.is_empty()`, `.len()`, `.iter().enumerate()` | Anzahl und je Nachbar eine Zeile |
| Nachbarzellenfelder | `cell_identifier_ca`, `main_carrier_number`, `cell_load_ca`, `neighbor_cell_synchronized`, optionale `mcc`, `mnc`, `location_area` | Nummerierung ab 1; fehlende optionale Kennungen als `-` |
| Brew / TetraPack | `brew.host`, `port`, `tls`, `username.is_some()`, `reconnect_delay.as_secs()`, `jitter_initial_latency_frames`, `feature_sds_enabled`, `whitelisted_ssis` | Nur Vorhandensein des Benutzernamens; Reconnect in Sekunden; Jitter in Frames; Remote-Liste nur als Anzahl |
| Dashboard | `dashboard.bind`, `port` | Vorhandene Konfiguration als `Enabled: true`; kein erfolgreicher HTTP-Bind nachgewiesen |
| Telemetry | `telemetry.host`, `port`, `use_tls`, `ca_cert.is_some()`, `credentials.is_some()` | TLS-Flag und Vorhandensein von CA-/Auth-Konfiguration; keine Zugangsdaten ausgeben |
| Control / Command | `control.host`, `port`, `use_tls`, `ca_cert.is_some()`, `credentials.is_some()` | Gleiche Darstellung wie Telemetry |
| Runtime State | `cfg.state_read()`, `state.network_connected`, `state.subscribers.all_registered_issis().count()` | Zeitpunktbezogener Zustand; im vorgeschlagenen Ablauf vor dem vollständigen Stackstart |

Der Abschluss der Runtime Summary war:

```text
────────────────────────────────────────────────────────────
  Boot verdict:         Config parsed. RF gremlins notified.
  FieldOps warning:     If it fails now, at least the log looks professional.
────────────────────────────────────────────────────────────
```

### 6.3 Heutiger Abgleich der Datenstruktur

`StackConfig` enthält weiterhin die für den historischen Block maßgeblichen Bereiche `phy_io`, `net`, `cell`, `brew`, `dashboard`, `telemetry`, `control` und `security`. Die heutige Struktur umfasst jedoch deutlich mehr Integrationen, darunter `brew2`, `control_room`, `edge_fallback`, `asterisk`, `recording`, `audio_player`, `media_library`, `tts`, `health`, `recovery` und weitere Gateways. Eine Übernahme des alten Blocks würde diese Erweiterungen nicht vollständig abbilden. [R5](#r5-konfigurationsstruktur-und-funkparameter)

Konkret geprüft wurden:

- `CfgCellInfo.neighbor_cell_broadcast` ist `u8`, kein `bool`. `timezone` ist `Option<String>`, `subscriber_class` ist `u16`, `custom_duplex_spacing` ist `Option<u32>` in Hz und `ms_txpwr_max_cell` ist ein `u8`-Rohfeld.
- Die heutigen Timerfelder sind `u32`. Die Kommentare nennen als Defaults Hangtime 5 s, Call-Timeout 120 s, UL-Inaktivität 3 s und periodische Registrierung 3600 s; `0` deaktiviert laut Feldbeschreibung die periodische Registrierung. Diese Defaults sind nicht mit der erfundenen Pseudoausgabe gleichzusetzen.
- `CfgCellInfo` besitzt inzwischen `secondary_carrier: Option<u16>`. Das DTO kann einen konfigurierten Zweitträger über `dual_carrier_enabled` deaktivieren, ohne dessen Wert aus der TOML zu entfernen.
- `CfgSoapySdr` verwendet `f64` für UL, DL und PPM, `Option<f64>` für die Samplingrate, `Option<usize>` für Kanäle und `HashMap<String, f64>` für Gains. Es gibt zusätzliche optionale `rx_center_freq` und `tx_center_freq` samt Korrekturmethoden.
- Die geprüften PPM-Methoden berechnen `err = frequency / 1_000_000 * ppm_err` und liefern `(frequency + err, err)` zurück. Das ist eine Berechnung aus Konfigurationswerten, keine Frequenzmessung.
- Die TOML-/DTO-Namen sind nicht überall mit den Laufzeitfeldnamen identisch: beispielsweise `rx_freq`/`tx_freq` gegenüber `ul_freq`/`dl_freq`, `sample_rate` gegenüber `fs`, `rx_antenna` gegenüber `rx_ant` und `freq_offset` gegenüber `freq_offset_hz`.

Quellen: [R5](#r5-konfigurationsstruktur-und-funkparameter). Ein vollständiger Typcheck des historischen Rust-Blocks fand nicht statt.

### 6.4 Ausgabe von Konfiguration ist nicht gleich Ausgabe des effektiven Zustands

Folgende Trennungen sind für die spätere Umsetzung wesentlich:

**Konfiguriert versus gestartet:** `Some(dashboard)` beweist keine erfolgreiche Portbindung. Vorhandene Telemetry-/Control-/Brew-Konfiguration beweist keine Verbindung, keine erfolgreiche Anmeldung und keinen TLS-Handshake.

**Advertisiert versus implementiert:** Ein gesetztes Dienstflag ist kein vollständiger Nachweis der zugehörigen TETRA-Funktion. Insbesondere ist `aie_service` keine Aussage darüber, dass ein konkreter Ruf tatsächlich verschlüsselt ist. Der Werbetext über sichere Kommunikation ist ebenfalls kein Sicherheitsnachweis.

**Nominal versus tatsächlich im SDR eingestellt:** Der alte Block druckt nominale UL-/DL-Frequenzen und PPM-Rechenwerte. Heute existieren zusätzlich Center-Frequenzen und Zweitträger. Ein zukünftiger Diagnoseblock muss diese Ebenen unterscheiden; aus einer einzelnen nominalen Frequenz darf nicht pauschal auf den aktuellen Hardware-Tuningwert geschlossen werden.

**Lokale Liste versus effektive Admission-Policy:** `CfgSecurity::is_issi_allowed()` akzeptiert bei leerer lokaler Liste alle ISSIs. Im heutigen Startablauf wird aber zusätzlich ein zentraler Admission-/Gruppen-Policy-Cache wiederhergestellt. Deshalb ist `Access mode: open network` allein aufgrund einer leeren lokalen TOML-Liste als globale Aussage zu weitgehend. Zunächst `local ISSI whitelist: not configured` ausgeben; effektive Freigaben gesondert auswerten. [R1](#r1-einstiegspunkt-und-startreihenfolge), [R6](#r6-lokale-security-konfiguration)

**Schnappschuss versus spätere Verbindung:** Ein früher Wert `Network connected: false` oder `Registered ISSIs: 0` darf nicht als Diagnose eines späteren Ausfalls ausgegeben werden. Umgekehrt ist `broadcast table ready` im alten Entwurf nur ein Text nach dem Durchlaufen der Liste, kein Nachweis einer ausgesendeten Nachbarzelleninformation.

**Standardfehlerausgabe versus internes Logging:** Die `eprintln!`-Ausgaben sind keine `tracing`-Events. Der aktuelle Start richtet einen internen Logkanal und anschließend das Logging ein. Es ist nicht nachgewiesen, dass der früh ausgegebene Banner in Dashboard-Logs oder externe Sammeldienste übernommen wird. Die tatsächliche Journal-/Dienstkonfiguration ist auf dem Zielsystem zu prüfen. [E1](#e1-rust-standardbibliothek-und-cargo), [R1](#r1-einstiegspunkt-und-startreihenfolge)

**Datensparsamkeit:** Das Ausgeben der konkreten ISSI-Liste wurde zwar vorgeschlagen, ist aber nicht für jeden Logempfänger sinnvoll. Einträge zählen statt vollständig ausgeben ist ein heutiger Review-Kandidat. Passwörter, Tokens, Schlüssel und Credential-Inhalte dürfen weder durch Komplett-Dumps der Konfiguration noch über URL-/Device-Strings in Logs gelangen.

## 7. Pseudoausgabe: bewahrte Beispielwerte und Korrekturen

Die auf Nutzerwunsch erstellte Ausgabe war ausdrücklich eine **Pseudoausgabe**, nicht die Ausgabe einer angeschlossenen Basisstation. Ihre wichtigsten Beispielwerte waren:

| Parametergruppe | Erfundenes Beispiel aus dem Chat |
|---|---|
| Konfigurationspfad | `/etc/netcore-tetra/config.toml` |
| Modus / Version | `Bs`, `0.1.0-git-a13f42c` |
| Netz / Zelle | MCC `262`, MNC `42`, LA `1`, CC `1`, Main Carrier `1521`, Band `4`, Offset `0 Hz`, Duplex-ID `0`, Reverse `false` |
| Weitere Zellwerte | Systemcode `0`, Subscriber Class `0`, Sharing `0`, Reserved Frames `0`, Max MS TX Power `30`, Late Entry `true`, U-plane DTX `false`, Frame 18 Ext `true` |
| Timer | Hangtime `5 s`, Call-Timeout `120 s`, UL-Inaktivität `3 s`, Periodic Registration `1800 s` |
| Zeitzone / Nachbarn | `Europe/Berlin`, `Neighbor broadcast:false`, keine Nachbarzellen |
| SDR | `SoapySdr`, `driver=lime`, `2000000 S/s`, RX/TX-Kanal jeweils `0`, RX `LNAH`, TX `BAND1` |
| Frequenzen / Korrektur | UL `411025000 Hz`, DL `421025000 Hz`, PPM `0`, jeweils `+0.0 Hz` |
| Gains | RX `LNA=30.0`, `TIA=12.0`, `PGA=19.0`; TX `PAD=40.0` |
| Lokale Whitelist | `[2010001, 2010002, 4010001]` |
| Dienste | Dashboard `0.0.0.0:8080`; Brew, Telemetry und Control deaktiviert |
| Anfangszustand | `Network connected: false`, registrierte ISSIs `0` |

Weitere zuvor verwendete Musterwerte, etwa MCC `204` / MNC `1337`, LA `2`, `./config.toml` und eine andere UL-/DL-Kombination, waren ebenfalls nur Assistentenbeispiele. **Keines dieser Beispiele ersetzt die tatsächlich eingesetzte TOML.**

### 7.1 Erkannte Unstimmigkeiten

**Versionsformat:** `0.1.0-git-a13f42c` entsprach bereits nicht dem später beschriebenen `STACK_VERSION`-Format. Auch das in weiteren Antworten wiederholt gezeigte `v0.1.0-g8f3a91c2` hatte für die angegebenen Git-Argumente ein fälschlich ergänztes `g`.

**Nachbarzellenfeld:** Der Code druckt für `neighbor_cell_broadcast` einen numerischen Wert. Die Pseudoausgabe `false` ist deshalb kein getreues Beispiel dieses Feldes. Der zusätzlich fehlende Abstand lässt sich unabhängig davon so korrigieren:

```rust
eprintln!(
    "  └─ Neighbor broadcast: {}",
    runtime_cfg.cell.neighbor_cell_broadcast
);
```

**Frequenzkonsistenz:** Band `4`, Carrier `1521` und Offset `0` ergeben nach der im Repository verwendeten Kanalberechnung DL `438025000 Hz`. Bei der dortigen Standard-Duplex-ID `0` für Band 4 und `reverse_operation = false` ergibt sich UL `428025000 Hz`, nicht das im Mockup genannte Paar `421025000/411025000 Hz`. Der Mockup mischte also nicht zusammenpassende Kanal- und Frequenzwerte. Diese Feststellung folgt aus dem gelesenen Frequenzcode und seiner Testdefinition, nicht aus einer HF-Messung. [R7](#r7-frequenzberechnung)

**Leistungsfeld:** `Max MS TX power: 30` war ein frei gewählter Mockupwert. Eine Zuordnung zu 30 dBm wurde weder im Ausgabe-Code vorgenommen noch geprüft. Bei einer produktiven Ausgabe Rohcode und gegebenenfalls dekodierten physikalischen Wert ausdrücklich kennzeichnen und gegen die tatsächliche Felddefinition prüfen.

**Subscriber Class:** Der Wert `0` im Mockup war keine gewünschte Zugangsregel. Im heutigen Konfigurationscode ist für das fehlende Feld der Default `65535` mit dem Kommentar zur Zulassung aller Teilnehmerklassen vorgesehen. [R5](#r5-konfigurationsstruktur-und-funkparameter)

**Port und Pfad:** Weder `8080` noch `/etc/netcore-tetra/config.toml` wurden als tatsächlich im Betrieb verwendete Werte belegt. Der heutige Debian-Paketpfad lautet laut Paketmetadaten vielmehr `/etc/flowstation/config.toml`; auch dies beschreibt Paketkonfiguration, nicht den nachgewiesenen lokalen Installationszustand. [R3](#r3-manifeste)

## 8. Versionierung: historische Erklärung und heutiger Code

### 8.1 Historisch gezeigte Workspace-Konfiguration

Der Nutzer zeigte:

```toml
[workspace.package]
version = "0.0.8"
edition = "2024"
authors = ["Razvan Zeces / TetraFlow.RO"]
license = "MIT"
```

Die Antwort erläuterte eine manuelle Änderung von `version`, beispielsweise auf `0.0.9`. In Cargo steht die Versionsnummer ohne das zusätzliche Anzeigepräfix `v`; das `v` wurde für die Git-Tags und die formatierte Anzeige verwendet. Die Workspace-Metadaten wirken auf die Mitglieder, die das betreffende Feld explizit mit `.workspace = true` übernehmen. [E1](#e1-rust-standardbibliothek-und-cargo)

Zwei vorgeschlagene Autorenvarianten waren nur `NetCore-Tetra Systems` oder eine Liste aus bisherigem Autor und NetCore-Tetra Systems. Es gab keine explizite Auswahl des Nutzers. Autorenmetadaten, Bannerbranding und die Pflege von Lizenz-/Urheberhinweisen sind getrennte Aufgaben; dieser Archivauftrag verändert keine davon und ist keine vollständige Lizenzprüfung.

### 8.2 Historisch besprochener Versionscode

Im Chat wurde folgender Mechanismus beschrieben:

```rust
pub const GIT_HASH: &str = git_version::git_version!(
    args = ["--always", "--dirty=-modified", "--match=", "--abbrev=8"],
    fallback = "unknown"
);

pub const STACK_VERSION: &str =
    const_format::formatcp!("v{}-{}", env!("CARGO_PKG_VERSION"), GIT_HASH);
```

Daraus wurde richtig abgeleitet, dass Cargo-Version und Git-Kennung beim Build in die Ausgabe gelangen. Mehrere konkrete Beispielausgaben waren jedoch falsch:

- `--match=` schließt hier Tagtreffer aus; mit `--always` bleibt die abgekürzte Commitkennung.
- In diesem Fallbackformat steht kein automatisches `g` vor dem Hash.
- `--abbrev=8` fordert eine achtstellige Kurzform an; bei nötiger Eindeutigkeit kann Git mehr Stellen verwenden.
- `--dirty=-modified` bezieht sich auf von Git erfasste Änderungen; eine allein vorhandene unversionierte Datei wurde im isolierten Test nicht als `-modified` markiert.
- Ein beliebiger neuer Tag ändert `CARGO_PKG_VERSION` nicht. Ein neuer Tag allein erhöht also die Cargo-Version nicht.

Die `g`-/Dirty-Einordnung wurde bei der Archivierung in einem separaten lokalen Git-Test überprüft, siehe Abschnitt 11. Die allgemeine Bedeutung der Git-Optionen ist in [E2](#e2-git-versionsermittlung) belegt.

### 8.3 Aktuell implementierter Versionscode

Im Prüfsnapshot steht in `crates/tetra-core/src/lib.rs`:

```rust
pub const GIT_HASH: &str = git_version::git_version!(
    args = ["--always", "--match=", "--abbrev=8"],
    fallback = "unknown"
);

pub const STACK_NAME: &str = "NetCore-Tetra";
pub const STACK_CODENAME: &str = "Dual Carrier";
pub const STACK_VERSION: &str =
    const_format::formatcp!("v{}-{}", env!("CARGO_PKG_VERSION"), GIT_HASH);
pub const STACK_DISPLAY: &str =
    const_format::formatcp!("{} {}", STACK_NAME, STACK_VERSION);
```

Die Kommentare erklären ausdrücklich den Verzicht auf `-modified`: Lokale gepatchte Arbeitsbäume sollen nicht bei jeder lokalen Änderung diesen Zusatz im Dashboard anzeigen; OTA vergleicht weiterhin die verkürzte Commitkennung. Das ist eine **heute implementierte Entscheidung laut Codekommentar**, nicht eine im vorliegenden Chat getroffene Entscheidung. [R2](#r2-versionierung)

In der Root-`Cargo.toml` stehen heute `version = "1.3.0"`, `edition = "2024"`, `authors = ["JanHG98 / NetCore-Tetra"]` und `license = "MIT"`. Sowohl `tetra-core` als auch `bluestation-bs` übernehmen die Workspace-Version. Als direkte Versionsabhängigkeiten nennt `tetra-core/Cargo.toml` `git-version = "0.3.9"` und `const_format = "0.2.35"`. [R3](#r3-manifeste)

Ein **frischer Build** des geprüften Archivcommits würde bei funktionierendem Git-Zugriff sinngemäß `v1.3.0-64b38118` enthalten. Dies ist eine aus dem Code abgeleitete Erwartung, **keine aus einem gebauten Binary ausgelesene Ausgabe**. Ohne verwertbaren Git-Kontext ist der konfigurierte Fallback `unknown`. Der bereits laufende Dienst oder ein altes Binary ändert sich durch einen reinen Commit beziehungsweise Manifestedit nicht automatisch.

### 8.4 Branding und technische Kennung trennen

Historisch wurde erwogen, `NetCore-Tetra` direkt vor die Zeichenfolge in `STACK_VERSION` zu setzen. Der Assistent empfahl anschließend, die technische Version dort eher kompakt zu lassen und den Produktnamen separat auszugeben. Der heutige Code bietet dafür bereits `STACK_NAME` und `STACK_DISPLAY`. Der Codename `Dual Carrier` ist vorhanden, wird aber von `STACK_DISPLAY` in der gezeigten Definition nicht automatisch angehängt. [R2](#r2-versionierung)

Die User-Agents sind ebenfalls nicht einheitlich umbenannt: Die heutigen Legacy-Telemetry-/Control-Worker verwenden weiterhin `BlueStation/<STACK_VERSION>`. Der Control-Room-Worker verwendet `NetCore-Tetra/<STACK_VERSION> (<node_id>)`. Der damalige Vorschlag einer pauschalen Umbenennung wurde für die Legacy-Worker damit nicht als umgesetzt gefunden. Vor Änderungen an solchen Identifikatoren die empfangenden Komponenten prüfen; ein tatsächlich vorhandenes serverseitiges Matching wurde in diesem Chat nicht nachgewiesen. [R1](#r1-einstiegspunkt-und-startreihenfolge)

## 9. Release-Automatisierung: Bestands-YAML, Vorschläge und Grenzen

### 9.1 Vom Nutzer gezeigte Bestands-YAML

Der Nutzer bezeichnete diese Datei als seine bisherige YAML. Ihr damaliger genauer Dateiname wurde nicht genannt:

```yaml
name: Release

on:
  push:
    tags:
      - 'v*'

jobs:
  build:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v4

      - name: Install Rust
        uses: dtolnay/rust-toolchain@stable

      - name: Install cross
        run: cargo install cross --git https://github.com/cross-rs/cross

      - name: Build aarch64
        run: cross build --release --target aarch64-unknown-linux-gnu

      - name: Create Release
        uses: softprops/action-gh-release@v2
        with:
          files: target/aarch64-unknown-linux-gnu/release/bluestation-bs
```

**Historischer Status:** Als vorhandene Konfiguration vom Nutzer gezeigt. Es gab keine zugehörige Run-ID, keinen Buildlog und keinen bestätigten erfolgreichen Asset-Upload. Aus der Bezeichnung „bisherige YAML“ darf nicht auf einen erfolgreichen Cross-Build geschlossen werden.

### 9.2 Zuerst vorgeschlagener, später ersetzter Ansatz

Vor Kenntnis dieser Bestandsdatei hatte der Assistent `.github/workflows/release.yml` mit einem nativen Linux-Build vorgeschlagen: `cargo build --release -p bluestation-bs`, Installation von Buildwerkzeugen und Bibliotheken auf einem Ubuntu-Runner, Cargo-Cache, Tarball, Actions-Artefakt und Releaseerzeugung über `gh release create`.

Genannt wurden `pkg-config`, `build-essential`, `cmake`, `clang`, `libclang-dev`, `libssl-dev` und `libsoapysdr-dev`. Die Architektur wurde über `uname -m` in den Dateinamen eingebaut. Das war kein gezielt für den Pi bestimmter ARM64-Build. Die spätere Nutzer-YAML machte deutlich, dass **der vorhandene ARM64-Cross-Build die relevante Grundlage** ist.

Die ursprüngliche Versionsprüfung mit `grep -m1 '^version = '` war außerdem kein robuster TOML-Zugriff auf `[workspace.package]`. Sie wurde im späteren Vorschlag durch einen abschnittsbezogenen `awk`-Ausdruck ersetzt. Beide Varianten wurden im Chat nicht ausgeführt.

### 9.3 Letzter Workflow-Vorschlag im Chat

Der letzte Entwurf ergänzte:

| Element | Vorgeschlagene Ausführung |
|---|---|
| Speicherort | `.github/workflows/`, beispielhaft `release.yml`; nicht `.github/workflow/` |
| Trigger | Tags `v*` plus `workflow_dispatch` |
| Berechtigungen | `permissions: contents: write` |
| Gemeinsame Variablen | `BIN_NAME: bluestation-bs`, `TARGET: aarch64-unknown-linux-gnu` |
| Checkout | `actions/checkout@v4`, `fetch-depth: 0` |
| Toolchain | `dtolnay/rust-toolchain@stable` |
| Cross | Installation aus `cross-rs/cross` per Git, ohne festgelegten Commit |
| Versionsprüfung | Tag ohne führendes `v` mit Workspace-Version vergleichen; nur bei `refs/tags/v...` |
| Build | `cross build --release --target "$TARGET" -p "$BIN_NAME"` |
| Packaging | Binary nach `dist/` kopieren, ausführbar setzen, `.tar.gz` erstellen |
| Assetname | `netcore-tetra-bluestation-bs-<Tag>-aarch64-unknown-linux-gnu.tar.gz` |
| Veröffentlichung | `softprops/action-gh-release@v2`, expliziter Tagname, Titel `NetCore-Tetra <ref_name>`, `generate_release_notes: true` |

Die vorgeschlagene Tagprüfung lautete:

```bash
TAG_VERSION="${GITHUB_REF_NAME#v}"

CARGO_VERSION="$(awk '
  /^\[workspace.package\]/ { in_ws=1; next }
  /^\[/ && in_ws { exit }
  in_ws && /^version = / {
    gsub(/"/, "", $3);
    print $3;
    exit
  }
' Cargo.toml)"

if [ -z "$CARGO_VERSION" ]; then
  echo "::error::Could not read workspace.package version from Cargo.toml"
  exit 1
fi

if [ "$TAG_VERSION" != "$CARGO_VERSION" ]; then
  echo "::error::Git tag v$TAG_VERSION does not match Cargo.toml version $CARGO_VERSION"
  exit 1
fi
```

Dies ist ein **historischer, nicht getesteter Ausschnitt**, kein freigegebener heutiger Releaseworkflow.

### 9.4 Fehler und offene Punkte des letzten Vorschlags

**Manueller Start war nicht gegen Veröffentlichung abgesichert.** Im letzten ARM64-Vorschlag hatte der Schritt `Create Release` kein `if`, obwohl `workflow_dispatch` aktiviert war. `github.ref_name` ist bei einem Branchlauf dessen Branchname. Mit dem ausdrücklich gesetzten `tag_name` konnte der Entwurf daher einen Releaseversuch mit einem Branchnamen auslösen, statt nur einen Testbuild anzulegen. Für einen echten Release muss die Veröffentlichung an einen gültigen Taglauf beziehungsweise einen ausdrücklich validierten Release-Tag gebunden werden. Der alte Ausdruck `VERSION="${GITHUB_REF_NAME:-manual}"` löst das nicht: Ein Branchname ist üblicherweise bereits gesetzt. [E3](#e3-github-actions-und-cross)

**Dateiname bei Branchläufen:** Ein Branchname mit `/`, etwa `feature/example`, gerät im alten Schema direkt in den Assetpfad. Daraus können ungewollte Unterverzeichnisse und Packagingfehler entstehen. Für manuelle Builds einen separaten, bereinigten Artefaktbezeichner verwenden. Dies ist eine statische Fehleranalyse des Entwurfs; der Fehler wurde nicht in Actions reproduziert.

**TOML-Parsing:** Der `awk`-Ausdruck hängt an der konkreten Schreibweise `version = "..."`. Ein TOML-Parser oder gezielte Cargo-Metadaten sind als Verbesserung zu prüfen. Tagmuster `v*` und `v*.*.*` sind GitHub-Globmuster, keine vollständige semantische Versionsvalidierung.

**Keine automatische Versionserhöhung:** Keiner der gezeigten Workflows erhöht selbstständig die Cargo-Version. Der vorgeschlagene Ablauf automatisiert Build, Packaging und Veröffentlichung **nach** der manuellen Versionspflege und dem Tag-Push. Eine Versions-Bump-Automatik wäre eine zusätzliche, bisher nicht spezifizierte Funktion.

**Lockfile und Reproduzierbarkeit:** Der Entwurf verwendet weder beim Projektbuild noch bei der Cross-Installation `--locked`; `cross` wird von einem veränderlichen Git-Stand installiert, ebenso sind `stable` und `ubuntu-latest` bewegliche Bezüge. Das kann spätere Builds verändern. Künftige Pins, Lockfile-Prüfung und Cacheinvalidierung müssen bewusst festgelegt werden; eine vollständige reproduzierbare Buildkette wurde nicht geliefert.

**Cross-Abhängigkeiten:** Native Zielbibliotheken müssen zum ARM64-Buildcontainer passen. Das Installieren von Hostpaketen auf dem x86_64-Runner allein stellt keine ARM64-Bibliotheken bereit. Eine `Cross.toml` beziehungsweise ein passendes eigenes Image wurde im Chat nur als mögliche spätere Lösung genannt. Ob und welche Cross-Konfiguration heute an anderer Stelle existiert, wurde hier nicht umfassend untersucht. [E3](#e3-github-actions-und-cross)

**Featureumfang:** Der heutige `bluestation-bs`-Manifeststand aktiviert standardmäßig `asterisk`, `recording` und `audio-player`. Der neue Releasepfad muss festlegen, ob diese Ausstattung mitgebaut wird und welche nativen Codec-/Audio-Abhängigkeiten benötigt werden. Die Debian-Beschreibung im selben Manifest behauptet teilweise noch einen abweichenden, Asterisk-freien Umfang. Das ist ein dokumentarischer Widerspruch und ein Prüfauftrag, kein bestätigter Fehler des ausgelieferten Pakets. [R3](#r3-manifeste)

**Assetkompatibilität:** Das letzte Packaging benannte nicht nur das Archiv, sondern auch das Binary innerhalb des Archivs um. Installations-/Updatewerkzeuge, die `bluestation-bs` erwarten, könnten dadurch Anpassungen brauchen. Eine robustere zukünftige Variante wäre ein eindeutig benanntes Archiv mit stabilem internen Binarynamen. Ob ein konkreter Updater betroffen ist, wurde nicht geprüft.

**Weitere fehlende Absicherungen:** Erfolgreiche Tests vor Veröffentlichung, Prüfsummen, nachvollziehbarer Feature-/Toolchainnachweis und das gewünschte Verhalten bei bereits existierenden Releases wurden nicht verbindlich festgelegt. Ein separates Actions-Artefakt für manuelle Runs war im ersten nativen Vorschlag enthalten, im letzten ARM64-Vorschlag aber nicht.

**Checkout-Tiefe:** `fetch-depth: 0` wurde als robuste Wahl empfohlen. Die frühere pauschale Begründung, der kurze Hash benötige zwingend die vollständige Historie, war zu stark: Für eine reine HEAD-Kurzkennung ist kein Tagabstand erforderlich. Ein vollständiger Checkout ist eher für Tag-/Historienauswertungen nützlich. Der isolierte Git-Test in diesem Archiv prüft keine Actions-Checkout- oder Cargo-Cacheinvalidierung.

### 9.5 Heutiger Workflow-Stand

Im geprüften Verzeichnis `.github/workflows/` wurden diese sechs Dateien gefunden:

```text
alert-service-tests.yml
asterisk-installer-tests.yml
dashboard-ui-tests.yml
phy-slotter-tests.yml
service-ui-tests.yml
tmp-v170-dual-umac.yml
```

Ein Release-Workflow war in dieser Verzeichnisliste nicht enthalten. Der frühere Nutzerstand und der heutige Zustand weichen also ab. Der Zeitpunkt und Grund einer möglichen Entfernung beziehungsweise Nichtübernahme wurden nicht aus der gesamten Git-Historie rekonstruiert. Auch die Historie aller GitHub-Releases und Actions-Runs wurde nicht vollständig geprüft. [R4](#r4-formatierung-und-workflows)

## 10. Befehle und Abläufe mit Ausführungsstatus

### 10.1 Historischer Releaseablauf — nur vorgeschlagen

Die damalige Anleitung verwendete beispielhaft:

```bash
# Historisches Beispiel; nicht im Archivauftrag ausgeführt.
# Erst die gewünschte Version in Cargo.toml festlegen.
cargo build

git status
git add Cargo.toml Cargo.lock
git commit -m "Bump version to v0.0.9"
git tag -a v0.0.9 -m "NetCore-Tetra v0.0.9"
git push origin main
git push origin v0.0.9
cargo build --release
```

Im Verlauf wurden entsprechende Varianten mit `v0.1.0` gezeigt. Es handelt sich um Musterwerte. Die `main`-Pushes und Tags sind **keine** Anweisung oder Freigabe für diesen Archivauftrag; dessen einziges Schreibziel ist `Archiving`, ausschließlich unter `Docs/archive/`.

Der Befehl `git add Cargo.toml Cargo.lock` ist nur passend, wenn die Änderungen an beiden Dateien überprüft wurden. Ein vorgeschaltetes `cargo build` kann den Lockfile aktualisieren; ein erfolgreicher Build wurde im Chat aber nicht belegt. Für eine spätere Umsetzung sind tatsächliches Releaseziel, Versionsentscheidung, Lockfile, vorhandene Tags und ein funktionierender Releaseworkflow zuerst zu prüfen.

### 10.2 Historische Cross-Befehle — nur gezeigt oder vorgeschlagen

```bash
cargo install cross --git https://github.com/cross-rs/cross
cross build --release --target aarch64-unknown-linux-gnu
# Spätere gezielte Variante:
cross build --release --target aarch64-unknown-linux-gnu -p bluestation-bs
```

Erwarteter Buildpfad im Entwurf: `target/aarch64-unknown-linux-gnu/release/bluestation-bs`. Der Target-Triple bezeichnet ARM64/Linux/GNU und ist nicht gleichbedeutend mit einer geprüften Lauffähigkeit auf jeder Raspberry-Pi-Installation. Es wurde weder ein ARM64-Binary erzeugt noch ein Zielsystemtest durchgeführt.

### 10.3 Für eine spätere Umsetzung vorgeschlagene Prüfschritte

Diese Schritte sind **heutige Fortsetzungsempfehlungen**, keine bereits ausgeführten Repositorytests:

```bash
# Im für die Implementierung freigegebenen Arbeitsbranch:
cargo fmt --check
cargo check --locked -p bluestation-bs

# Nach Einrichtung einer passenden Cross-Buildumgebung:
cross build --locked --release \
  --target aarch64-unknown-linux-gnu \
  -p bluestation-bs
```

Zusätzlich erst einen manuellen Build ohne Veröffentlichung prüfen; dann einen ausdrücklich freigegebenen Test-/Release-Tag mit passender Manifestversion. Ein Negativtest mit Tag-/Manifestabweichung muss vor dem Releaseupload scheitern. Auf ARM64 das heruntergeladene Artefakt, seine dynamischen Bibliotheken und die tatsächlich eingebettete Version prüfen. Der Archivauftrag stößt diese Builds, Tags und Veröffentlichungen nicht an.

## 11. Fehler-, Diagnose- und Testprotokoll

### 11.1 Fehler und Korrekturstatus

| Befund | Ursache / Diagnose | Korrektur oder Handlungsbedarf | Nachweisstatus |
|---|---|---|---|
| `/n` wird sichtbar ausgegeben | Falscher Slash im ursprünglichen Nutzerblock | `eprintln!();` für leere Ausgabezeilen | Im Chat korrekt erklärt und in spätere Chatblöcke übernommen; heutiger kompakter Banner enthält diese falschen Zeilen nicht. |
| `Neighbor broadcast:false` | Fehlender Abstand und falscher Mockupdatentyp | Abstand ergänzen; tatsächlich numerischen Rohwert darstellen | Korrektur des Abstands vorgeschlagen; `u8` heute im Code bestätigt. |
| `g` vor dem Hash | Verwechslung mit dem Tagabstandsformat von `git describe` | Bei den gezeigten `--match=`-Argumenten nackten Kurz-Hash verwenden | Isoliert getestet; heutiger Codekommentar bestätigt dies zusätzlich. |
| `-modified` als heutige Erwartung | Frühere Git-Argumente auf heutigen Stand übertragen | Heutige Entfernung berücksichtigen; nicht stillschweigend wieder einschalten | Im aktuellen Quellcode implementiert. |
| „Systems up“ vor Initialisierung | Statische Dekoration wird wie Laufzeitdiagnose formuliert | Banner und gemessenen/effektiven Status trennen | Statischer Review-Befund, keine Betriebsstörung beobachtet. |
| Ungültige Frequenzkombination im Mockup | Frei zusammengestellte Werte | Aus einer konsistenten Konfiguration oder Kanalberechnung erzeugen | Gegen Frequenzcode rechnerisch abgeglichen; keine HF-Messung. |
| Hilfsfunktionen unerwünscht | Erste Lösung passte nicht zum gewünschten Einbaukomfort | Inline-Code direkt in `main()` | Explizit durch Nutzer korrigiert. |
| x86_64-Vorschlag statt ARM64 | Zielarchitektur zunächst angenommen | Vorhandenen ARM64-Cross-Ablauf als Grundlage verwenden | Durch spätere Nutzer-YAML geklärt. |
| Veröffentlichung bei manuellem Branchlauf | Release-Step ohne passende Bedingung | Tagbindung/Validierung plus separates Buildartefakt | Statisch festgestellt; nicht in Actions ausgeführt. |
| Slash im Assetbezeichner | Unbereinigter Branchname in `GITHUB_REF_NAME` | Separaten sicheren Artefaktnamen verwenden | Statisch festgestellt. |
| Alter Einfügepunkt mit `None` | Startarchitektur hat sich weiterentwickelt | Fallbackkonfiguration und wiederhergestellten Edge-Policy-Zustand erhalten | Heutiger Startcode gelesen; historischer Block nicht portiert. |
| `edition = "2026"` | Edition mit Kalenderjahr verwechselt | `2024` beibehalten | Dokumentation und heutige Konfiguration geprüft; kein lokaler Rustfmt-Lauf. |

### 11.2 Tatsächlich ausgeführte Prüfungen dieser Archivierung

**Repository-Leseprüfung:** Branchrefs, Zielverzeichnis, bestehende Archivstruktur, wichtige Manifest-/Versions-/Konfigurationsdateien, der relevante vollständige `main()`-Ablauf und die Workflow-Verzeichnisliste wurden über den GitHub-Zugang gelesen. Der Vergleich zwischen den fixierten `main`- und `Archiving`-Commits wurde abgerufen. Das sind Struktur- und Quellcodeprüfungen, keine ausführbaren Produktfunktionstests.

**Isolierter Git-Test:** Mit `git version 2.47.3` wurde außerhalb des Projekt-Repositories ein temporäres Testrepository angelegt. Es enthielt eine versionierte Textdatei, einen Commit und einen annotierten Beispieltag `v0.0.9`. Die folgenden Ergebnisse wurden tatsächlich erhalten:

| Testfall | Argumente | Ergebnis |
|---|---|---|
| Sauberer Commit, annotierter Tag auf HEAD | `git describe --always --dirty=-modified --match= --abbrev=8` | `4bd7221c` |
| Sauberer Commit, aktueller Argumentumfang | `git describe --always --match= --abbrev=8` | `4bd7221c` |
| Nur neue unversionierte Datei | Historische Argumente mit `--dirty=-modified` | `4bd7221c` |
| Versionierte Datei geändert | Historische Argumente mit `--dirty=-modified` | `4bd7221c-modified` |
| Versionierte Datei geändert | Aktuelle Argumente ohne `--dirty` | `4bd7221c` |

**`4bd7221c` ist ausschließlich die Kennung dieses temporären Testfixtures und kein NetCore-Tetra-Commit.** Das Testrepository wurde nicht gepusht. Alle fünf erwarteten Ausgaben wurden mit Assertions geprüft. Der Test bestätigt die Git-Optionen, aber weder die Expansion des Rust-Makros noch Cargo-Rebuildverhalten, Cross-Container oder ein Releasebinary.

**Anhangprüfung:** Alle 25 angegebenen PDF-Dateien waren lokal zugänglich; die Seitenzahlen wurden mit einem PDF-Reader ausgelesen. Kein OCR, keine vollständige Normprüfung und kein erneuter Upload der PDFs nach GitHub.

**Toolchaingrenze:** `rustc`, `cargo` und `rustfmt` waren in der verfügbaren lokalen Prüfungsumgebung nicht als ausführbare Programme vorhanden. Deshalb wurden weder `cargo check` noch `cargo build`, `cargo test`, `cargo fmt` oder ein ARM64-Cross-Build ausgeführt. Es wurde keine zusätzliche Toolchain nur für diese Dokumentationsablage installiert.

### 11.3 Nicht belegt

Es liegen in diesem Chat keine echten Startlogs des großen Diagnoseblocks, keine Screenshots seiner tatsächlichen Terminalausgabe, keine RF-Messungen, keine erfolgreiche Teilnehmerregistrierung als Test dieses Blocks, keine bestätigte Live-Verbindung der angezeigten Dienste, keine Release-Run-ID und keine Installation eines hier erzeugten Releaseassets vor.

Der vorhandene Frequenzcode enthält einen Unit-Test zur Kanalberechnung. Dieser wurde gelesen, aber nicht ausgeführt. Auch die vorhandenen CI-YAML-Dateien sind kein Beweis dafür, dass die betreffenden Jobs im geprüften Stand erfolgreich gelaufen sind.

## 12. Rustfmt und Rust-Edition

Der Nutzer zeigte folgende Konfiguration:

```toml
edition = "2024"
max_width = 140
tab_spaces = 4
hard_tabs = false
reorder_modules = true
reorder_imports = true
fn_params_layout = "Tall"
use_field_init_shorthand = true
use_try_shorthand = true
```

Die Frage war, ob die Edition einfach auf `2026` geändert werden könne. Die Antwort lautete: nicht auf das Kalenderjahr ändern. Editionen sind definierte Sprach-/Kompatibilitätsstände, nicht eine frei zu vergebende Jahresangabe. Für diesen Projektstand bleibt `2024` die passende Konfiguration. Ein Compilerupgrade und eine Änderung der Projektversionsnummer sind davon getrennt. [E4](#e4-rust-edition-und-rustfmt)

Der Assistent schlug zusätzlich vor:

```toml
edition = "2024"
style_edition = "2024"
```

Dabei ist eine Präzisierung wichtig: `style_edition` steuert die Formatierungsregeln; es ersetzt nicht die Bedeutung der Parser-/Sprach-Edition. Die frühere Formulierung „zusätzlich oder statt edition“ war insofern missverständlich. Die offizielle Rustfmt-Migrationsdokumentation empfiehlt, den gewünschten Formatierungsstil für Editoren und direkte Rustfmt-Aufrufe ausdrücklich konsistent festzulegen. [E4](#e4-rust-edition-und-rustfmt)

**Aktueller Status:** Alle oben gezeigten ursprünglichen Einstellungen stehen im geprüften `rustfmt.toml`; `style_edition` fehlt. Das ist nicht automatisch ein Formatierungsfehler: Laut Dokumentation folgt die Style-Edition standardmäßig der verwendeten Edition. Eine explizite Ergänzung ist eine mögliche Präzisierung und wurde hier nicht vorgenommen. Es gab keinen lokalen Rustfmt-Test und keine Umstellung auf `2026`. [R4](#r4-formatierung-und-workflows)

## 13. Anhänge und Quellenabdeckung

### 13.1 ETSI-Anhanginventar

Die folgenden Dateien waren zugänglich. Titel-/Versionsangaben wurden aus dem bereitgestellten Dateikontext übernommen; Seitenzahlen wurden zusätzlich direkt an den Dateien geprüft. Die Einträge sind ein Inventar, keine Behauptung, alle Dokumente vollständig fachlich ausgewertet zu haben.

| Datei | Dokument / Einordnung | Seiten |
|---|---|---:|
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1, Generic Speech Format Implementation | 22 |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1, General requirements for supplementary services | 46 |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5, UICC physical and logical characteristics | 8 |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2, Call Identification, stage 3 | 56 |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1, ISI Short Data Service | 28 |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2, Include Call, stage 2 | 18 |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1, Late Entry, stage 2 | 23 |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1, TSIM application | 139 |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5, UICC physical and logical characteristics | 8 |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1, SIM-ME interface / security aspects | 156 |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1, Call Identification, stage 2 | 44 |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1, Call Authorized by Dispatcher, stage 1 | 20 |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1, Barring of Outgoing Calls, stage 1 | 17 |
| `en_3003921216v010400a.pdf` | Draft EN 300 392-12-16 V1.4.0, 2026-03, Pre-emptive Priority Call, stage 3 | 67 |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1, General network design | 182 |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14, 1997-09, PICS proforma | 61 |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1, Security | 216 |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1, Conformance testing / Radio | 169 |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1, transport-independent ISI Group Call | 191 |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3, TETRA codec | 94 |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1, ISI Group Call | 251 |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1, Peripheral Equipment Interface | 320 |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0, 2026-04, transport-independent ISI Mobility Management | 380 |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1, Air Interface | 1445 |
| `ETSI.pdf` | Umfangreicher Sammelanhang; beginnt mit EN 300 812 V2.1.1; Zusammensetzung nicht vollständig geprüft | 4100 |

Die Dateien über Air Interface und allgemeines Netzdesign können für eine spätere normgerechte Dekodierung angezeigter Funkfelder relevant sein. Sie belegen aber weder das Vorhandensein des Banner-Codes noch Cargo-, GitHub- oder Rustfmt-Verhalten. Die im Inventar als Draft bezeichneten Dateien werden nicht zu endgültig veröffentlichten Normständen umgedeutet. Ihr heutiger normativer Status wurde nicht neu recherchiert.

Es wurden keine unzugänglichen Anhänge festgestellt. Die **Auswertungstiefe** ist dennoch begrenzt: Verfügbarkeit und Metadaten sind geprüft, nicht alle technischen Inhalte und Abbildungen der mehrere tausend Seiten umfassenden Sammlung.

## 14. Ersetztes, offene Aufgaben und Roadmap-Kandidaten

### 14.1 Ersetzte oder überholte Ansätze

| Früherer Ansatz | Spätere Einordnung |
|---|---|
| Separate Banner-/Summary-Hilfsfunktionen | Durch ausdrücklichen Wunsch nach direkt einfügbarem Inline-Code ersetzt. |
| Ursprüngliches BlueStation-Branding | NetCore-Branding gewünscht; heute kompakte NetCore-Fassung implementiert. Upstream-Nachweise und technische Namen sind davon getrennt. |
| Zwei `eprintln!("/n")` | In späteren Chatblöcken durch korrekte Leerzeilen ersetzt. |
| Native x86_64-Release-YAML | Nach Vorlage des vorhandenen ARM64-Workflows nicht mehr die relevante Hauptlösung. |
| Erste Versionssuche über `grep -m1` | Durch gezielteren `awk`-Vorschlag ersetzt, aber weiterhin nicht robustes TOML-Parsing. |
| Beispielversionen `0.0.8` / `0.0.9` / `0.1.0` als heutiger Stand | Historische Nutzerangabe beziehungsweise Beispiele; geprüft ist heute `1.3.0`. |
| Git-Hash mit `g`-Präfix | Für die besprochenen Argumente falsch; nackter Kurz-Hash bestätigt. |
| `-modified` als heutige Ausgabe | Im heutigen Code bewusst entfernt. |
| `edition = "2026"` | Nicht als gültige Projektmigration übernommen; `2024` bleibt stehen. |
| Alter Configstart mit `SharedConfig::from_parts(..., None)` | Nicht auf den heutigen Einstieg übertragen: Fallback und initialer Edge-Policy-Zustand müssen erhalten bleiben. |

### 14.2 Roadmap-Kandidaten aus diesem Chat

Es wurde keine verbindliche Sprintplanung oder Prioritätsnummerierung vereinbart. Die folgende Reihenfolge ist eine **heutige fachliche Empfehlung** für die Fortsetzung; sie darf nicht als rückwirkende Termin- oder Umsetzungszusage gelesen werden.

| Kennung | Aufgabe | Herkunft / Status | Abhängigkeit und Abnahmekriterium |
|---|---|---|---|
| BOOT-01 | Umfang des kompakten heutigen Banners gegenüber dem großen Chatentwurf festlegen | Ursprünglicher Nutzerwunsch, große Variante nicht im Prüfsnapshot | Texte freigeben; statische Scherzzeilen nicht als Healthcheck darstellen. |
| BOOT-02 | Inline-Runtime-Summary an heutigen `main()`-Ablauf anpassen | Nutzerwunsch / geplant, nicht implementiert gefunden | Fallbackpfad und Edge-Policy-Initialisierung erhalten; Kompilierung und Starttest bestehen. |
| BOOT-03 | Rohkonfiguration, effektive Policy, gestartete Komponenten und Live-Verbindungen getrennt anzeigen | Heutiger Review-Kandidat | Keine falschen `ready`-/`open network`-Aussagen; negative Tests mit deaktivierten/fehlgeschlagenen Diensten. |
| BOOT-04 | Dual Carrier, SDR-Center-Frequenzen, Brew2, Control Room und Featureumfang ergänzen | Heute aus Codeerweiterungen abgeleiteter Kandidat, nicht historisch vereinbart | Ausgabe gegen tatsächliche Konfiguration und aktivierte Cargo-Features prüfen. |
| BOOT-05 | Sichere und übersichtliche Ausgabe | Historischer Secret-Hinweis plus heutige Konkretisierung | Keine Credentials; ISSIs standardmäßig gegebenenfalls nur zählen; Ausgabe in Terminal/Journal prüfen. |
| REL-01 | Vorhandensein und gewünschte Rückkehr eines Releaseworkflows klären | Historischer Automatisierungswunsch; im Prüfsnapshot kein Releaseworkflow | Freigegebener Implementierungsbranch und klarer Releaseprozess. |
| REL-02 | ARM64-Buildumgebung und Featurematrix definieren | Historische Zielarchitektur plus heutiger Manifeststand | Native Zielabhängigkeiten, Codec/Audio, Toolchain und Cross-Image erfolgreich bauen. |
| REL-03 | Tag-/Manifestprüfung und sichere Triggerlogik umsetzen | Historischer Entwurf mit offenen Fehlern | Branch-Dispatch veröffentlicht nichts; falsche Version scheitert vor Upload; sichere Dateinamen. |
| REL-04 | Packaging, Prüfsummen und Installationskompatibilität festlegen | Teils historischer Assetwunsch, teils heutiger Review-Kandidat | Eindeutiger Archivname, stabiler interner Binaryname, Zielgerätetest und dokumentierter Updateweg. |
| REL-05 | Reproduzierbarkeit und Buildidentität prüfen | Heutiger Review-Kandidat | Lockfile/Pins bewusst handhaben; Binaryversion entspricht Quellcommit; lokale Dirty-Policy respektieren. |
| FMT-01 | Explizites `style_edition = "2024"` erwägen | Historischer Vorschlag, nicht implementiert gefunden | Format-on-save und CI konsistent; keine ungewollte großflächige Formatierungsänderung. |
| BRAND-01 | Legacy-User-Agents, CLI-Texte und Paketbeschreibungen konsistent prüfen | Historische Nebenidee plus heutige Abweichungen | Kompatibilität erhalten; technische Namen und Attribution nicht blind ersetzen. |
| FUN-01 | Wechselnde Bootzitate / `Monster armed` | Historische optionale Nebenidee | Nur Gestaltung; keine neue notwendige Abhängigkeit oder irreführende Statusaussage. |

### 14.3 Konkrete nächste Schritte

Zuerst in einem gesondert freigegebenen Implementierungsauftrag den aktuellen Einstiegspunkt übernehmen und eine kleine, datensparsame Runtime-Summary ergänzen. Dabei bereits verfügbare `STACK_NAME`-/`STACK_VERSION`-/`STACK_DISPLAY`-Konstanten verwenden und die Config-/Policy-Initialisierung unverändert lassen. Anschließend echte Ausgabe unter deaktivierten und aktivierten Integrationen prüfen.

Danach den ARM64-Releasepfad separat wiederherstellen oder neu festlegen. Zuerst Toolchain, Features und Zielbibliotheken; dann ein manueller Artefaktbuild ohne Veröffentlichung; danach Versionsabweichungs- und Tagtests; zuletzt ein freigegebener Release samt Installationstest auf dem Zielgerät. **Nicht** im Zuge einer reinen Archivablegung ungeprüft eine alte YAML oder eine alte `0.0.9`-Versionsnummer wieder in den Produktcode kopieren.

Humor, User-Agent-Kosmetik und eine explizite Rustfmt-Style-Edition sind nachrangige Ergänzungen. Für diese Aufgaben sind im vorliegenden Chat keine festen Termine vereinbart.

## 15. Quellen und überprüfte Repository-Dateien

### 15.1 Historische Chatquellen

Die direkte Quelle für Anforderungen und historische Ausschnitte ist der verfügbare Dialog selbst: Nutzerlink auf `razvanzeces/flowstation`, beide Nutzer-Bannerfassungen, die Bitte um Inline-Code, die Anfrage nach stärkerem Humor, die Frage nach Systemparametern, der kombinierte `main()`-Ausschnitt, die Pseudoausgabe, die Fragen zu `STACK_VERSION`, die vom Nutzer gezeigte ARM64-YAML, sein Workspace-Ausschnitt mit `0.0.8` und seine Rustfmt-Konfiguration. Dafür liegt kein dauerhafter Original-Chatlink vor.

Historische Fremdlinks:

- [FlowStation](https://github.com/razvanzeces/flowstation)
- [Im Chat referenzierter Einstiegspunkt](https://github.com/razvanzeces/flowstation/blob/main/bins/bluestation-bs/src/main.rs)
- [Ursprünglicher BlueStation-Verweis](https://github.com/MidnightBlueLabs/tetra-bluestation)

Diese Links sind nicht auf einen im historischen Chat aufgezeichneten Commit gepinnt. Eine Übereinstimmung ihres heutigen Inhalts mit der damaligen Betrachtung wird nicht behauptet.

### 15.2 Heutige Repositoryquellen

Alle folgenden Produktcode-Verweise sind auf den **Prüfcommit** `64b381186d02445a3839d737c928ac15d42b4648` fixiert. Sie belegen den dortigen Stand unabhängig davon, welcher Commit dieses Archiv anschließend speichert.

#### R1: Einstiegspunkt und Startreihenfolge

[`bins/bluestation-bs/src/main.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/bins/bluestation-bs/src/main.rs): Banner, Versionsausgabe, CLI-Beschreibung, `load_config_with_fallback`, Edge-Policy-Initialisierung, Logkanal, `build_bs_stack`, Telemetry-/Control-/Control-Room-Worker, Dashboardstart und `router.run_stack`.

#### R2: Versionierung

[`crates/tetra-core/src/lib.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-core/src/lib.rs): `GIT_HASH`, Kommentar zur Dirty-Policy, `STACK_NAME`, `STACK_CODENAME`, `STACK_VERSION`, `STACK_DISPLAY`.

#### R3: Manifeste

- [`Cargo.toml`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/Cargo.toml): Workspace, Version `1.3.0`, Edition, Autoren, Lizenz und gemeinsame Abhängigkeiten.
- [`crates/tetra-core/Cargo.toml`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-core/Cargo.toml): Vererbung der Workspace-Version und Versionsmakro-Abhängigkeiten.
- [`bins/bluestation-bs/Cargo.toml`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/bins/bluestation-bs/Cargo.toml): Paket-/Binaryname, Workspace-Vererbung, Defaultfeatures, Debian-Metadaten, Konfigurationspfad und Systemd-Paketoptionen.

#### R4: Formatierung und Workflows

- [`rustfmt.toml`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/rustfmt.toml)
- [Workflow-Verzeichnis im Prüfsnapshot](https://github.com/JanHG98/netcore-tetra/tree/64b381186d02445a3839d737c928ac15d42b4648/.github/workflows)

#### R5: Konfigurationsstruktur und Funkparameter

- [`crates/tetra-config/src/bluestation/config.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-config/src/bluestation/config.rs): `StackConfig`, weitere heutige Integrationen und Trägerberechnung.
- [`crates/tetra-config/src/bluestation/sec_cell.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-config/src/bluestation/sec_cell.rs): `CfgCellInfo`, DTO, Typen, Timerbeschreibung und Defaults.
- [`crates/tetra-config/src/bluestation/sec_phy_soapy.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-config/src/bluestation/sec_phy_soapy.rs): Soapy-Typen, Center-Frequenzen, DTO-Namen und PPM-Berechnung.

Die weiteren historisch referenzierten Dateien `sec_brew.rs`, `sec_dashboard.rs`, `sec_telemetry.rs` und `sec_control.rs` wurden für dieses Archiv nicht sämtlich im Detail erneut gelesen. Ihre im Chat genannten Einzelparameter sind deshalb in Abschnitt 6 ausdrücklich als **historischer Entwurfsumfang** dokumentiert und nicht pauschal als vollständig aktuell typgeprüft dargestellt.

#### R6: Lokale Security-Konfiguration

[`crates/tetra-config/src/bluestation/sec_security.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-config/src/bluestation/sec_security.rs): `issi_whitelist` und die lokale `is_issi_allowed()`-Entscheidung. Die darüber hinausgehende Policy muss anhand des gesamten heutigen Admissionpfads bewertet werden.

#### R7: Frequenzberechnung

[`crates/tetra-core/src/freqs.rs`](https://github.com/JanHG98/netcore-tetra/blob/64b381186d02445a3839d737c928ac15d42b4648/crates/tetra-core/src/freqs.rs): Band-/Trägerdefinition, Duplex-Tabelle und vorhandene Testdefinition für die Kanalberechnung; keine Ausführung des Rust-Tests in diesem Auftrag.

#### R8: Branchvergleich

[Fixierter Vergleich `main` → `Archiving`](https://github.com/JanHG98/netcore-tetra/compare/6aa9be8f74ab731f72dc133a5f8e90c5018c626d...64b381186d02445a3839d737c928ac15d42b4648). Die in diesem Dokument maßgeblichen Dateien `bins/bluestation-bs/src/main.rs`, Root-/Core-/BS-Manifeste, `tetra-core/src/lib.rs`, die geprüften Konfigurationsdateien und `rustfmt.toml` wurden in diesem Vergleich nicht als geändert aufgeführt. Andere Quellcodebereiche, insbesondere WebUIs, unterscheiden sich durchaus.

### 15.3 Öffentliche Primärdokumentation für die ergänzende Prüfung

#### E1: Rust-Standardbibliothek und Cargo

- [`eprintln!`](https://doc.rust-lang.org/std/macro.eprintln.html): Standardfehlerausgabe und automatisch angehängter Zeilenumbruch.
- [Cargo Workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html): Workspace-Metadaten und explizite Vererbung durch Mitglieder.

#### E2: Git-Versionsermittlung

[Git `describe`](https://git-scm.com/docs/git-describe): Optionen `--always`, `--match`, `--abbrev` und `--dirty`. Die im Archiv konkret dargestellten Argumentkombinationen wurden zusätzlich mit dem isolierten Test aus Abschnitt 11 überprüft.

#### E3: GitHub Actions und Cross

- [Workflow-Auslöser, einschließlich `workflow_dispatch`](https://docs.github.com/en/actions/reference/workflows-and-actions/events-that-trigger-workflows)
- [`softprops/action-gh-release`](https://github.com/softprops/action-gh-release)
- [Cross-Konfiguration](https://github.com/cross-rs/cross/wiki/Configuration)

Diese Quellen erklären Werkzeuge und Trigger. Sie bestätigen keinen erfolgreichen NetCore-Release und keine korrekte native Abhängigkeitsausstattung des vorgeschlagenen Containers.

#### E4: Rust-Edition und Rustfmt

- [Rust Edition Guide: Editions](https://doc.rust-lang.org/edition-guide/editions/index.html)
- [Rustfmt Style Edition 2024](https://doc.rust-lang.org/edition-guide/rust-2024/rustfmt-style-edition.html)

## 16. Übergabestatus

**Historisches Chatergebnis:** Ein detaillierter Inline-Entwurf für Banner und Diagnoseausgabe, eine Erklärung zur Cargo-/Git-Versionierung, ein gezeigter ARM64-Bestandsworkflow sowie ein noch fehlerbehafteter Erweiterungsvorschlag und die Klärung der Rustfmt-Edition.

**Heute als implementiert gefunden:** Kompaktes NetCore-Banner, bestehende `STACK_VERSION`-Kette mit Version `1.3.0`, zusätzliche Produktkonstanten, bewusst unterdrückter Dirty-Zusatz und Rust-Edition `2024`.

**Nicht als implementiert gefunden:** Der große Chat-Diagnoseblock, der besprochene Releaseworkflow im geprüften Workflow-Verzeichnis und eine explizite Rustfmt-Style-Edition.

**Getestet:** Nur die beschriebenen isolierten Git-Fälle sowie Datei-/Repository-Lese- und Strukturprüfungen. **Nicht getestet oder im Betrieb bestätigt:** Produktbuild, ARM64-Artefakt, Releaseautomatik, reale Banner-/Summary-Ausgabe und Funkbetrieb dieser Änderungen.

**Auswertungslücken:** Originaltitel/-link, historische Commitzuordnung des Nutzerarbeitsstands, tatsächliche damalige Umsetzung der Vorschläge, vollständige Release-/Actions-Historie, vollständiger aktueller Typ-/Buildtest und vollständige inhaltliche Auswertung der ETSI-Sammlung.

Die Fortsetzung soll auf dem dann aktuellen Produktstand erfolgen, nicht durch ungeprüfte Wiederherstellung der historischen Snippets. Der Chat selbst wird nach Prüfung durch den Nutzer archiviert.
