# Technische Chat-Abschlussdokumentation: NetCore-Rebranding, Startbanner, Stack-Version und deutsche Projektbeschreibung

> **Historisches Chatarchiv mit separater Repository-Prüfung.** Ein Textentwurf ist kein Commit, Quellcode ist kein erfolgreicher Build und ein Banner ist keine Betriebsbestätigung. Dieses Dokument enthält keine Freigabe für Änderungen außerhalb von `Docs/archive/`.

## 1. Metadaten und Auswertungsumfang

| Merkmal | Wert |
|---|---|
| Projekt / Zielrepository | `JanHG98/netcore-tetra` |
| Thema | Umstellung des sichtbaren TETRA-BlueStation-Auftritts auf NetCore-Tetra; Rust-Startbanner; Versionsquelle; deutsche Projektbeschreibung und Danksagungen |
| Ursprünglicher Chattitel | Nicht verfügbar. Der Dokumenttitel ist eine beschreibende Archivbezeichnung. |
| Ursprünglicher Chatlink | Nicht verfügbar; es wurde kein Link konstruiert. |
| Historischer Zeitraum | Einzelne Nachrichten lassen sich ergänzend dem 31.03.–01.04.2026 zuordnen; kein vollständig datierter Chatexport vorhanden. |
| Erstellungsdatum | **2026-10-04**, Zeitzone `Europe/Berlin` |
| Geprüfter Branch | **`Archiving`** |
| Fixierter technischer Prüfsnapshot | **`3fb792cf4c3653b6ad91652b1265b18ac8c1772e`** |
| Tree des Prüfsnapshots | `241144074993b9be6b0b435384653601a60fd004` |
| Damals genannte Repository-Adresse | `https://github.com/JanHG98/bluestation` |
| Archivdatei | `Docs/archive/2026-10-04_netcore-rebranding-startbanner-stack-version-und-projektbeschreibung.md` |
| Archivindex | `Docs/archive/README.md` |
| Schreibumfang | Nur diese Zusammenfassung und der zugehörige Indexeintrag. Keine Produktcode-, Versions-, Workflow-, Tag-, Release- oder Deploymentänderung. |

**Prüfcommit und Ablagecommit sind zu unterscheiden:** Der oben genannte Commit fixiert den untersuchten Code. Der Commit zur Speicherung dieser Datei entsteht danach und ist über ihre Git-Historie und die Abschlussmeldung nachvollziehbar. Vor der Speicherung muss der dann aktuelle Kopf von `Archiving` erneut gelesen werden; neuere Archivbeiträge bleiben erhalten.

### 1.1 Verfügbare Quellen und Grenzen

Ausgewertet wurde der sichtbare Verlauf vom ursprünglichen `eprintln!`-Banner über die Suche nach `tetra_core::STACK_VERSION`, den vom Nutzer selbst gefundenen Definitionsausdruck bis zur deutschen Anpassung des englischen Projekt-/Dokumentationstextes. Eine ergänzende Verlaufssuche bestätigte Fundstelle und Themen, lieferte aber keinen Originaltitel und keinen Chatlink.

Es liegen **keine damaligen Buildausgaben, Laufzeitlogs, Screenshots der Konsole, Git-Diffs oder Implementierungscommits dieses Chats** vor. Ob der Nutzer Vorschläge damals lokal eingebaut hatte, ist damit nicht feststellbar. Der heutige Code wird separat beschrieben; seine Änderungen werden nicht ohne Beleg diesem Chat zugeschrieben.

Die 25 verfügbaren PDFs sind vom Dateidienst als **Projektquellen** ausgewiesen. Für diesen Auftrag wurden Dateiverfügbarkeit, Seitenzahl, Dateihash und Deckblattinformationen geprüft. Eine vollständige normative Auswertung ihrer insgesamt 8.061 Dateiseiten wurde nicht durchgeführt. Die Summe ist keine Anzahl einzigartiger Normseiten; insbesondere `ETSI.pdf` ist eine umfangreiche Datei mit einem EN-300-812-Deckblatt, deren vollständige Zusammensetzung hier nicht rekonstruiert wurde. Im sichtbaren Gespräch gibt es keine aus diesen PDFs abgeleitete Branding- oder Versionsentscheidung. Sie werden in Abschnitt 12 inventarisiert, nicht als Implementierungsnachweis verwendet.

Die Prüfung betrifft gezielt den genannten `Archiving`-Snapshot. Kein vollständiges Audit aller Repository-Dateien, anderer Branches, des früheren Repositories oder der installierten Anlage wurde vorgenommen. Angaben aus anderen Projektchats wurden nicht als Entscheidungen dieses Gesprächs übernommen.

### 1.2 Statusbegriffe

| Status | Bedeutung |
|---|---|
| **Idee** | Diskutierte Möglichkeit oder Assistentenvorschlag ohne ausdrückliche Auswahl. |
| **Beschlossen/geplant** | Ausdrücklicher Nutzerwunsch bzw. autorisierter Arbeitsauftrag; Umsetzung gesondert nachzuweisen. |
| **Implementiert** | Im ausdrücklich bezeichneten Repository-Snapshot als Code bzw. Datei gefunden. |
| **Getestet** | Konkrete Ausführung mit dokumentiertem Ergebnis. Reine Quellcodelektüre zählt nicht als Laufzeittest. |
| **Im Betrieb bestätigt** | Zuordenbare Beobachtung auf dem tatsächlichen Zielsystem. |

## 2. Ergebnis für die spätere Fortsetzung

Der Nutzer wollte **NetCore-Tetra statt BlueStation als sichtbare Projektidentität**, alternative projektpassende Texte unter dem Startbanner und eine deutsche Projektbeschreibung. Der wichtigste historische technische Befund wurde vom Nutzer selbst geliefert:

```rust
// Historischer, vom Nutzer gezeigter Stand:
// crates/tetra-core/src/lib.rs
pub const STACK_VERSION: &str =
    const_format::formatcp!("{}-{}", env!("CARGO_PKG_VERSION"), GIT_VERSION);
```

Damit war die Fundstelle geklärt. Die Definition von `GIT_VERSION` sowie die damalige konkrete Cargo-Version wurden in diesem Chat jedoch nicht gezeigt. Mehrere vorherige Assistentenaussagen waren lediglich Vermutungen, obwohl der Nutzer ausdrücklich um eine Repository-Prüfung gebeten hatte.

**Der heute geprüfte Stand ist weiterentwickelt:** `STACK_NAME`, `STACK_CODENAME`, `GIT_HASH`, `STACK_VERSION` und `STACK_DISPLAY` existieren bereits in `crates/tetra-core/src/lib.rs`. Die numerische Paketversion wird über Workspace-Vererbung in der **Root-`Cargo.toml` unter `[workspace.package]`** festgelegt und lautet im Prüfsnapshot `1.3.0`. Das Banner zeigt NetCore-Tetra, die echte Repository-URL und weiterhin eine technische Versionszeile. Gleichzeitig bestehen BlueStation-Reste in CLI-Beschreibungen, zwei User-Agent-Vorlagen und dem Core-Moduldokumentationstext. [R1] [R2] [R3] [R4]

Die gewünschte deutsche Projektbeschreibung wurde im Chat als Textentwurf geliefert. Die aktuelle Root-README ist aber eine andere, releasebezogene NetCore-Fassung mit der Kennung `v1.9.0`; `wiki/Home.md` enthält ebenfalls eine weiterentwickelte Systembeschreibung. Diese zwei Versionskennungen dürfen ohne weitere Klärung weder gleichgesetzt noch durch einen automatischen Bump angeglichen werden. [R5] [R6]

## 3. Ziel, Ausgangslage und Gesprächsverlauf

### 3.1 Ursprüngliches Banner

Der Nutzer zeigte folgenden Rust-Ausgabeblock und fragte nach einer Umstellung auf NetCore-Tetra:

```rust
eprintln!("░▀█▀░█▀▀░▀█▀░█▀▄░█▀█░░░░░█▀▄░█░░░█░█░█▀▀░█▀▀░▀█▀░█▀█░▀█▀░▀█▀░█▀█░█▀█");
eprintln!("░░█░░█▀▀░░█░░█▀▄░█▀█░▄▄▄░█▀▄░█░░░█░█░█▀▀░▀▀█░░█░░█▀█░░█░░░█░░█░█░█░█");
eprintln!("░░▀░░▀▀▀░░▀░░▀░▀░▀░▀░░░░░▀▀░░▀▀▀░▀▀▀░▀▀▀░▀▀▀░░▀░░▀░▀░░▀░░▀▀▀░▀▀▀░▀░▀\n");
eprintln!("  Wouter Bokslag / Midnight Blue");
eprintln!("  https://github.com/MidnightBlueLabs/tetra-bluestation");
eprintln!("  Version: {}", tetra_core::STACK_VERSION);
```

Der Assistent lieferte ein dreizeiliges Netcore-Tetra-Banner, setzte jedoch eine Platzhalter-Repository-URL ein und schlug ohne Prüfung alternative Symbolpfade wie `netcore_tetra::STACK_VERSION` oder `crate::STACK_VERSION` vor. **Das war kein Nachweis existierender Crates oder Konstanten.**

Historischer Bannerentwurf; nicht mit dem heutigen, in der Breite abweichenden Banner verwechseln:

```rust
eprintln!("░█▄░█░█▀▀░▀█▀░█▀▀░█▀█░█▀▄░█▀▀░░░░░▀█▀░█▀▀░▀█▀░█▀▄░█▀█");
eprintln!("░█░▀█░█▀▀░░█░░█░░░█░█░█▀▄░█▀▀░▄▄▄░░█░░█▀▀░░█░░█▀▄░█▀█");
eprintln!("░▀░░▀░▀▀▀░░▀░░▀▀▀░▀▀▀░▀░▀░▀▀▀░░░░░░▀░░▀▀▀░░▀░░▀░▀░▀░▀\n");
eprintln!("  Netcore-Tetra");
```

Die sogenannten ASCII-Banner verwenden tatsächlich Unicode-Blockzeichen. Eine Prüfung der Darstellung in Zielterminal, Schriftart oder Journal fand damals nicht statt.

### 3.2 Gewünschte Alternativen für zwei Textzeilen

Der Nutzer wollte **anstelle der Repository-URL und Versionszeile** etwas anderes, das zum Projekt passt. Der Assistent bot mehrere Varianten an, behielt dabei aber jeweils eine Versionszeile bei. Der Wunsch, beide Zeilen zu ersetzen, wurde daher nicht vollständig erfüllt.

| Historischer Vorschlag | Einordnung |
|---|---|
| `Core initialized. All systems nominal.` | Nicht ausgewählter Vorschlag; würde einen Systemzustand behaupten. |
| `Bootstrapping Netcore-Tetra runtime...` | Nicht ausgewählter Initialisierungstext. |
| `Initializing distributed core...` | Nicht ausgewählter Text; kein Beleg einer verteilten Initialisierung. |
| `Node online. Awaiting workload.` | Nicht ausgewählt; würde Onlinezustand suggerieren. |
| `Ready.` | Nicht ausgewählt; Bereitschaft nicht nachgewiesen. |
| `Systems up. Nothing broken (yet).` | Humorvolle Idee, keine bestätigte Auswahl dieses Chats. |
| `Core linked. Network stable.` | Favorit des Assistenten, **nicht** verbindliche Nutzerentscheidung. |

Die weiteren angebotenen Zeilen waren `Stack Version: {}` bzw. `v{}`. Es wurde kein endgültiger Zweizeiler festgelegt. Ein späterer verwandter Bannerchat ist separat archiviert und darf nicht rückwirkend als ausdrückliche Auswahl in diesem Gespräch gelten; siehe Abschnitt 13.

### 3.3 Suche nach der Versionskonstante

Der Nutzer nannte die damalige Adresse `JanHG98/bluestation` und die Stelle `bluestation/bins/bluestation-bs/src/main.rs`. Der Präfix `bluestation/` kann ein damaliges Checkout-Verzeichnis bezeichnen; ein damaliger Repository-Unterordner ist damit nicht bewiesen.

Statt einer tatsächlichen Repository-Lektüre gab der Assistent Suchvorschläge und vermutete `lib.rs`, `version.rs`, eine lokale Cargo-Version oder `CARGO_PKG_NAME`. Als Befehl wurde lediglich vorgeschlagen:

```bash
grep -R "STACK_VERSION" .
```

**Ausführungsstatus:** Im historischen Chat nicht ausgeführt bzw. nicht durch eine Ausgabe belegt.

Anschließend lieferte der Nutzer selbst den oben wiedergegebenen Ausdruck aus `crates/tetra-core/src/lib.rs`. Dieser konkrete Fund ersetzt die vorherigen Vermutungen über die Definitionsstelle.

### 3.4 Ideen zur Trennung von Name und Version

Nach dem Fund wurden folgende Alternativen vorgeschlagen, jedoch nicht ausdrücklich ausgewählt:

```rust
// Historische Idee: separater Produktname, bestehende Versionsbildung beibehalten.
pub const STACK_NAME: &str = "Netcore-Tetra";
pub const STACK_VERSION: &str =
    const_format::formatcp!("{}-{}", env!("CARGO_PKG_VERSION"), GIT_VERSION);

// Historische Ausgabeidee:
eprintln!("  {} v{}", tetra_core::STACK_NAME, tetra_core::STACK_VERSION);
```

Eine zweite Idee war `STACK_FULL`, gebildet mit `const_format::formatcp!("{} v{}-{}", "Netcore-Tetra", env!("CARGO_PKG_VERSION"), GIT_VERSION)`, mit anschließender Ausgabe über `eprintln!("  {}", tetra_core::STACK_FULL);`.

Eine dritte Idee trennte App- und Core-Anzeige:

```rust
// Historische Idee, nicht als aktueller Einbauvorschlag übernehmen:
eprintln!("  Netcore-Tetra v{}", env!("CARGO_PKG_VERSION"));
eprintln!("  Core {}", tetra_core::STACK_VERSION);
```

Die allgemeine Unterscheidung zwischen Paket-/App-Version und Core-Version war nachvollziehbar. Die Behauptung, beide hätten im konkreten Projekt zwingend unabhängige Versionsstände, war jedoch nicht geprüft. Heute erben beide betrachteten Pakete dieselbe Workspace-Version. [R3] [R4]

### 3.5 Deutsche Projektbeschreibung

Der Nutzer stellte einen englischen Text mit Projektbeschreibung, Dokumentationsworkflow und Danksagungen bereit und bat um eine deutsche Anpassung an das NetCore-Universum. Der Ausgangstext beschrieb:

- einen freien, erweiterbaren TETRA-Stack für Experimente und Forschung im Alpha-Stadium;
- die Aussendung eines Basisstations-Downlinks, Empfang durch korrekt konfigurierte MS, Verbindung zur Basisstation und Talkgroup-Anmeldung;
- teilweise implementierte Sprachrufe, optionale Brew-/BrandMeister-Anbindung und umfangreich vorhandene Parser für TETRA-Protokollnachrichten;
- ein Dokumentationsrepository mit Pull Requests und behaupteter automatischer Wiki-Aktualisierung aus `main`;
- die unten dokumentierten ursprünglichen Beiträge und Danksagungen.

Der Assistent übersetzte und strukturierte diese Inhalte unter Netcore-Tetra, Dokumentation und Danksagungen. **Es wurde dabei kein Code geprüft und keine Datei gespeichert.** Aussagen wie „Sprachübertragung teilweise implementiert“ sind deshalb als historischer Beschreibungsstand zu lesen, nicht als heutiger Funktionstest.

## 4. Endgültige Anforderungen und nicht getroffene Entscheidungen

| Gegenstand | Historischer Status | Begründung / Grenze |
|---|---|---|
| Sichtbares Branding auf NetCore-Tetra umstellen | **Beschlossen/geplant** | Ausdrücklicher Nutzerwunsch. Kein Auftrag zur vollständigen technischen Umbenennung aller Pakete. |
| Zwei Texte unter dem Banner projektpassend ersetzen | **Beschlossen/geplant**, Textauswahl offen | Nutzerwunsch eindeutig; keine konkrete Variante ausgewählt. |
| Definition von `STACK_VERSION` finden | **Geklärt im Chat** | Konkrete Datei und Formel vom Nutzer geliefert. |
| Deutsche Projektbeschreibung erstellen | **Beschlossen/geplant; Textentwurf geliefert** | Gewünschte Sprache ausdrücklich Deutsch. Keine Veröffentlichung nachgewiesen. |
| `STACK_NAME`, `STACK_FULL` oder App/Core-Doppelanzeige verwenden | **Idee** | Damals Alternativen des Assistenten, keine endgültige Auswahl. |
| `tetra-core` technisch umbenennen | **Nicht beschlossen** | Für sichtbares Branding nicht automatisch erforderlich. |
| Neue konkrete Versionsnummer festlegen | **Nicht beschlossen** | In diesem Chat kein Zielwert gewählt. |
| Wiki automatisch aus `main` aktualisieren | **Historische Textaussage, Umsetzung unbestätigt** | Aus Vorlage übernommen, kein damaliger Workflow-Nachweis. |
| Archiv auf GitHub speichern | **Beschlossen/geplant** | Neuer ausdrücklicher Auftrag: vorhandener Branch `Archiving`, nur `Docs/archive/`. |

Es wurden keine technischen Prioritätsstufen, Termine, verantwortlichen Bearbeiter oder Implementierungs-PRs für das Rebranding vereinbart. Die später aufgeführten Reihenfolgen sind Vorschläge dieser Abschlussdokumentation, keine rückwirkenden Beschlüsse.

## 5. Zusätzlich geprüfter Repository-Stand am 04.10.2026

Alle Befunde dieses Abschnitts beziehen sich auf `3fb792cf4c3653b6ad91652b1265b18ac8c1772e`.

### 5.1 Tatsächliche Pfade und Workspace

Die Repository-Wurzel enthält direkt `bins/`, `crates/`, `Cargo.toml`, `Docs/`, `wiki/` und `.github/`. Der zunächst ausprobierte Pfad `bluestation/crates/tetra-core/src/lib.rs` lieferte beim heutigen Lesen HTTP 404; der korrekte Pfad ohne vorangestelltes `bluestation/` war zugänglich. Das war ein **Pfadproblem der Archivprüfung**, kein Build- oder Funkfehler.

Die einschlägigen Manifeste lauten auszugsweise:

```toml
# Cargo.toml an der Repository-Wurzel
[workspace.package]
version = "1.3.0"
edition = "2024"
authors = ["JanHG98 / NetCore-Tetra"]
license = "MIT"

# crates/tetra-core/Cargo.toml
[package]
name = "tetra-core"
version.workspace = true
edition.workspace = true

# bins/bluestation-bs/Cargo.toml
[package]
name = "bluestation-bs"
version.workspace = true
edition.workspace = true
license.workspace = true

[[bin]]
name = "bluestation-bs"
path = "src/main.rs"
```

**Die aktuelle numerische Versionsquelle ist somit Root-`Cargo.toml`, nicht eine eigenständige `version = ...`-Zeile im Core-Manifest.** Cargo stellt dem kompilierten Paket die geerbte Version als `CARGO_PKG_VERSION` bereit. Die Workspace-Felder sind opt-in; aus einer Root-Metadatenzeile folgt nicht, dass jedes Paket jedes Feld erbt. Die Lizenzzeile wird hier nur als Manifestinhalt dokumentiert; eine Prüfung der Lizenz-/Attributionslage aller übernommenen Komponenten wurde nicht vorgenommen. [R3] [R4] [E1] [E2]

### 5.2 Aktuelle Branding- und Versionskonstanten

In `crates/tetra-core/src/lib.rs` stehen, ohne die zusätzlichen erläuternden Kommentare wiederzugeben:

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

Die relevante Abhängigkeitsdeklaration in `crates/tetra-core/Cargo.toml` nennt `git-version = "0.3.9"` und `const_format = "0.2.35"`. Dies sind die gelesenen Manifestanforderungen, keine Behauptung über die vollständig aufgelösten Lockfile-Versionen. [R1] [R3]

Die Verarbeitungskette ist:

```text
Root-Cargo.toml: workspace.package.version
  -> tetra-core/Cargo.toml: version.workspace = true
  -> env!("CARGO_PKG_VERSION")
                                     + Git-Kennung beim Kompilieren
                                     -> GIT_HASH oder "unknown"
  -> STACK_VERSION = "v<Paketversion>-<Git-Kennung>"
  -> STACK_DISPLAY = "NetCore-Tetra <STACK_VERSION>"
  -> Verwendung durch aufrufenden Code, unter anderem das Startbanner
```

`const_format::formatcp!` erzeugt die Zeichenkette zur Kompilierzeit. `git_version!` verwendet die angegebenen `git describe`-Argumente. Der vorgefundene Code verfolgt bewusst eine commitbasierte Darstellung ohne Tag-Namen und ohne `-modified`-Zusatz; der Kommentar nennt lokale Operator-Patches und den OTA-Vergleich als Motivation. Der OTA-Pfad selbst wurde in diesem Auftrag nicht geprüft. [R1] [E3] [E4] [E5]

**Wichtig für spätere Änderungen:** Das heutige `STACK_VERSION` enthält das `v` bereits. Die historische Ausgabe `"{} v{}"` würde damit ein doppeltes `v` erzeugen. Außerdem heißt die heutige Git-Konstante `GIT_HASH`; ein alter Block mit `GIT_VERSION` ist nicht ungeprüft über den aktuellen Stand zu kopieren. `STACK_FULL` ist im geprüften Core-Definitionsblock nicht vorhanden; die entsprechende aktuelle kombinierte Anzeige heißt `STACK_DISPLAY`.

Die fehlende Dirty-Markierung bedeutet nicht, dass ein Build unveränderten Quellen entspricht. Zwei Builds können denselben Commitbezug zeigen und trotzdem lokale Änderungen enthalten. Ein eingebetteter Hash ist keine Aussage über die augenblickliche Repository-Spitze, einen laufenden Dienst oder die Herkunft eines bereits installierten Binaries. [R1] [E3] [E5]

### 5.3 Aktuelles Startbanner und Reihenfolge

`bins/bluestation-bs/src/main.rs` enthält bereits ein dreizeiliges NetCore-Banner. Darunter stehen:

```rust
eprintln!("  NetCore-Tetra Systems");
eprintln!("  https://github.com/JanHG98/netcore-tetra");
eprintln!("  Version: {}", tetra_core::STACK_VERSION);
eprintln!("  Radio runtime: MAIN-COMPAT (local MM/MLE/CMCE state machines)");
```

Der aktuelle Ablauf in `main()` ist: **Banner ausgeben → `Args::parse()` → Konfiguration laden → weitere Initialisierung**. Daraus folgt unmittelbar: Das Banner kann weder eine erfolgreiche Konfigurationsprüfung noch einen erfolgreichen Funk-/Netzwerkstart belegen. Die vorgeschlagenen Texte `Network stable`, `Ready` oder `All systems nominal` wären an dieser frühen Stelle ohne zusätzliche Prüfung irreführend. [R2]

Der ursprüngliche Wunsch nach zwei anderen Texten ist im Prüfsnapshot nicht als gewählter Zweizeiler erkennbar: URL und Versionszeile bestehen weiterhin. Das ist eine Abweichung zwischen Wunsch und aktuellem Code, aber kein Nachweis, dass ein späterer Entschluss des Nutzers missachtet wurde; ein solcher liegt für diesen Chat nicht vor.

### 5.4 Verbleibende BlueStation-Bezeichnungen

| Fundstelle | Gelesener Inhalt | Technische Bedeutung |
|---|---|---|
| `crates/tetra-core/src/lib.rs` | `//! Core utilities for TETRA BlueStation` | Dokumentationsrest; Produktkonstante daneben bereits NetCore. |
| `bins/bluestation-bs/src/main.rs`, Clap-Attribute | `about = "TETRA BlueStation base station stack"` und entsprechendes `long_about` | Noch nicht umgestellte CLI-Hilfebeschreibung. |
| `start_telemetry_worker()` | `user_agent: format!("BlueStation/{}", tetra_core::STACK_VERSION)` | Übertragene Softwarekennung der Telemetrie-Verbindung. |
| `start_control_worker()` | Dieselbe `BlueStation/{}`-Vorlage | Softwarekennung der Legacy-Control-Verbindung. |
| `start_control_room_worker()` | `NetCore-Tetra/{} ({})` mit Version und Node-ID | Hier ist die NetCore-Kennung bereits umgesetzt. |
| Binärpaket und Binärtarget | `bluestation-bs` | Technischer Name; nicht mit einer sichtbaren Produktbezeichnung gleichsetzen. |

Diese Tabelle ist **keine vollständige Suche nach jeder Altbezeichnung im Repository**. User-Agent-Änderungen betreffen eine Schnittstelle und sollten mit den Gegenstellen geprüft werden; aus einem neuen Banner folgt keine automatische Freigabe zum Umbenennen von Paket-, Protokoll-, Unit- oder Dateinamen. [R1] [R2] [R4]

### 5.5 Heutige Dokumentation und Wiki-Automatik

Die Root-README trägt `NetCore-TETRA` und beschreibt einen Releasekontext `v1.9.0` mit Warnfunktionen und SIP-/Fallback-Themen. Sie ist nicht der kurze Alpha-Text aus diesem Chat. Diese README-Aussagen wurden nicht als Ende-zu-Ende-Tests der genannten Funktionen übernommen. [R5]

`wiki/Home.md` beschreibt NetCore als TETRA-Basisstation mit verteilten Netzdiensten und Leitstellenwerkzeugen. Die Seite benennt selbst als Dokumentationsbasis einen älteren `main`-Stand vom 26.09.2026, weist auf abweichende Installationen hin und trennt Quellcode von abgenommenem Funkbetrieb. Ihre Inhalte belegen einen heutigen Dokumentationstext, nicht den kompletten gegenwärtigen Betriebszustand. [R6]

Der vollständig gelesene `.github/`-Tree enthält sechs Workflow-Dateien: `alert-service-tests.yml`, `asterisk-installer-tests.yml`, `dashboard-ui-tests.yml`, `phy-slotter-tests.yml`, `service-ui-tests.yml` und `tmp-v170-dual-umac.yml`. Eine dedizierte Wiki-Synchronisationsdatei ist darin nicht erkennbar. Die Inhalte sämtlicher Workflows, externe Automationen und ein etwaiges separates Dokumentationsrepository wurden nicht vollständig geprüft. **Die historische Aussage „Wiki wird automatisch aus main aktualisiert“ bleibt deshalb unbestätigt; sie darf nicht ungeprüft als aktueller Workflow veröffentlicht werden.** [R7]

## 6. Architektur, Schnittstellen und technische Parameter

Der Gegenstand dieses Chats ist in erster Linie die **Darstellungs- und Build-Metadatenebene**, nicht eine Umgestaltung der Funkarchitektur.

| Komponente / Parameter | Rolle und belegter Stand |
|---|---|
| `tetra-core` / Rust-Pfad `tetra_core` | Gemeinsame Hilfstypen sowie Branding-/Versionskonstanten; Beibehaltung des technischen Namens ist mit NetCore-Branding vereinbar. |
| `bluestation-bs` | Konsument der Konstanten; eigenes Binärtarget und Clap-basierte Argumentverarbeitung. |
| `eprintln!` | Konsolenausgabe des Banners; keine Zustandsprüfung. |
| `CARGO_PKG_VERSION` | Versionsmetadatum des jeweils kompilierten Pakets; im betrachteten Core und Binary aktuell aus dem Workspace geerbt. |
| `const_format` | Kompilierzeit-Formatierung; zusätzliche Namen sind definierte Konstanten, keine automatisch verfügbaren Symbole. |
| `git-version` | Liefert Kompilierzeit-Git-Kennung nach den konfigurierten Argumenten bzw. Fallback. |
| CLI `--help` / `--version` | Durch Clap vorbereitet; die nackte `version`-Ableitung und die manuelle Stack-Versionszeile sind getrennte Ausgabepfade. Ihre tatsächlichen Ausgaben wurden nicht gestartet. |
| WebSocket-User-Agent | Zusätzliche Branding-Verbraucher in Telemetrie, Legacy-Control und Control Room. |
| `config.toml` | Keine im Chat beschlossene Branding-/Versionsoption darin; das betrachtete Branding liegt im Rust-Code. |
| Ports, Frequenzen, MCC/MNC/ISSI, IP-Adressen | In diesem Einzelchat keine konkreten Werte festgelegt. Keine Übernahme aus anderen Projektchats. |
| Brew / BrandMeister | Bestandteil der historischen Beschreibung; keine Verbindungsparameter oder Live-Abnahme dieses Chats. |
| TETRA Downlink, MS, Talkgroups, Sprachrufe, PDU-Parser | Historischer Projektumfang der bereitgestellten Vorlage; keine neue Protokollimplementierung durch das Rebranding. |

Quellen: Chatabschnitte 3.1–3.5, [R1]–[R4], [E1]–[E4]. Die Standardsammlung ergänzt Hintergrundmaterial, nicht die projektspezifische Versionslogik.

## 7. Deutsche Beschreibung und Danksagungen: bewahrter Inhalt

### 7.1 Historisch gelieferter Beschreibungsstand

Der deutsche Entwurf bezeichnete Netcore-Tetra als freien, modularen und erweiterbaren TETRA-Stack für Experimente, Forschung und Entwicklung digitaler Bündelfunksysteme. Er kennzeichnete den damaligen Stand als Alpha, nicht vollständig ausgebaut und nicht produktionsreif. Ein korrekt konfiguriertes MS könne den ausgesendeten Downlink empfangen, sich verbinden und an Talkgroups anmelden. Sprachübertragung sei teilweise implementiert; Brew/BrandMeister optional. Viele Funktionen seien noch in Entwicklung, umfangreicher Parsercode jedoch vorhanden.

Der Dokumentationsteil übernahm: Änderungen im Dokumentationsrepository, Einbringung per Pull Request, automatische Wiki-Erzeugung aus `main`, weitere Hinweise zu Beiträgen und Issues in der Dokumentation. **Diese Formulierungen sind historische Textarbeit, kein verifizierter Prozessbeschluss für das heutige Monorepository.**

### 7.2 Ursprüngliche Beiträge und Attribution

Die vom Nutzer vorgelegte Danksagung würdigte konkret:

| Genannte Personen / Projekte | Beitrag laut bereitgestellter Vorlage |
|---|---|
| Harald Welte und Osmocom-Team | Grundlagenarbeit an `osmocom-tetra`. |
| Tatu Peltola | Erweiterung von `rust-soapysdr` um Timestamping für robuste RX/TX-Verarbeitung und Rust-basierter Viterbi-Encoder/Decoder für den LMAC. |
| BlueStation-Contributors | Beiträge zu Stabilität, Ausgestaltung und Funktionsumfang des Ursprungsprojekts. |
| Stichting NLnet / RETETRA3 | In der Vorlage genannte Förderung der Implementierung freier TETRA-Software aus dem RETETRA3-Kontext. |
| Wouter Bokslag / Midnight Blue | Im ursprünglichen Startbanner ausdrücklich genannte Herkunft. |

Der damalige deutsche Assistententext erwähnte die ersten vier Gruppen, ersetzte bei den Contributors BlueStation durch Netcore-Tetra und nahm die Bannerherkunft nicht zusätzlich auf. Für eine spätere Veröffentlichung sollten **Ursprungsbeiträge und NetCore-Weiterentwicklung getrennt erkennbar bleiben**. Das ist hier ein Dokumentationsvorschlag; kein Auftrag zur Änderung der Lizenz oder sämtlicher Urheberangaben.

Die Vorlage belegt insbesondere **keine direkte NLnet-Förderzusage an NetCore-Tetra**. Ein späterer NetCore-Text sollte die Finanzierung dem beschriebenen Ursprungskontext zuordnen, statt eine eigene Förderung zu suggerieren. Der verlinkte RETETRA3-Verweis wird als historische Quelle bewahrt; eine zusätzliche Förderprüfung wurde nicht durchgeführt.

### 7.3 Korrektur der früheren Positionierungsempfehlung

Die frühere Assistentenaussage, der Text solle möglichst wenig nach Fork und mehr nach eigenständigem Stack aussehen, ist keine technische Entscheidung des Nutzers und kein geeigneter Beleg für eigenständige Urheberschaft. **Eigene Produktidentität und transparente Upstream-Herkunft schließen einander nicht aus.**

Eine neue, an den aktuellen Funktionsstand angepasste README ist ein offener Folgeauftrag. Die Alpha-Vorlage sollte dabei nicht einfach über die heutige README oder Systemwiki-Seite kopiert werden.

## 8. Fehler, Diagnose, überholte Ansätze und verbleibende Probleme

| Problem / Aussage | Diagnose / Korrektur | Status |
|---|---|---|
| Erbetene Repo-Suche wurde historisch durch Vermutungen ersetzt | Damals keine Tool- oder Datei-Lektüre belegt; Nutzer fand die Definition selbst. Heutige Prüfung ist nachgeholt und separat referenziert. | Historische Auskunftslücke behoben, keine rückwirkende Implementierung. |
| Platzhalter in der vorgeschlagenen Repository-URL | Nicht als endgültige Projektadresse verwenden. Heutiges Banner enthält `JanHG98/netcore-tetra`. | Im heutigen Banner korrigiert. |
| Vermutete reine Cargo-Version | Tatsächliche historische Formel kombinierte Cargo-Version und `GIT_VERSION`. | Durch Nutzerfund ersetzt. |
| Aussage „BlueStation steckt gar nicht in der Version“ | Kein BlueStation-Literal im gezeigten Formatstring; der damalige Inhalt von `GIT_VERSION` war aber unbekannt. Ein Tag-/Präfixbezug ließ sich damals nicht ausschließen. | Frühere Gewissheit nicht gerechtfertigt. |
| `tetra_core` als automatisch zu beseitigende Altlast | Technischer Cratename und Produktname sind getrennt. Heutiger Code demonstriert diese Trennung bereits. | Kein Rename-Beschluss. |
| Vorschläge `netcore_tetra::...` / `crate::...` | Ohne passende Definition oder Re-Export keine austauschbaren Symbolpfade. | Nicht ungeprüft übernehmen. |
| „Version in Core-Cargo.toml ändern“ | Heute steht dort `version.workspace = true`; numerische Quelle ist Root-Manifest. | Konkrete aktuelle Fundstelle geklärt. |
| Historische `v{}`-Ausgabe | Heute bereits `v` in `STACK_VERSION`. | Doppelpräfix bei Übernahme vermeiden. |
| Statische „Ready/Network stable“-Zeile vor Initialisierung | Kein gemessener Zustand; aktuelles Banner läuft vor Argument- und Konfigurationsprüfung. | Textauswahl weiterhin offen. |
| Automatischer Wiki-Sync als Tatsache | Historisch nur Textvorlage; aktueller tatsächlicher Sync nicht belegt. | Offen. |
| Root-README `v1.9.0`, Cargo `1.3.0` | Verschiedene gelesene Kennungen; ihre beabsichtigte Beziehung in diesem Chat nicht definiert. | Versionsmodell dokumentieren, nicht blind angleichen. |
| Frühere Pfadangabe mit `bluestation/` | Heutiger Root-Tree enthält `crates/` und `bins/` direkt. | Lesepfad korrigiert. |

Nicht beobachtet wurden in diesem Chat Compilerfehler, Verbindungsabbrüche, fehlgeschlagene Deployments oder Funkstörungen. Solche Probleme dürfen nicht aus allgemeinen Projektinformationen ergänzt werden.

## 9. Befehle, Abläufe und Tests

### 9.1 Tatsächlich ausgeführte Arbeit dieses Archivauftrags

Die GitHub-Verbindung wurde verwendet, um Branchkopf, Root-Tree, Archivverzeichnis, Archivindex und die in Abschnitt 13 genannten Dateien am fixierten Commit zu lesen. Zusätzlich wurde die vorhandene benachbarte Banner-/Release-Zusammenfassung zur Abgrenzung geprüft. Die verfügbaren Projektanhänge wurden über den Dateidienst inventarisiert; lokale PDF-Dateien wurden für Seitenzahlen, Deckblatttext und SHA-256 gelesen. **Es wurde kein OCR durchgeführt und kein Funkdienst gestartet.**

Diese Tätigkeiten sind erfolgreiche **Quellen- und Strukturprüfungen**, keine Software-/HF-Funktionstests. Die angekündigte Speicherung erfolgt als gesonderter Archivschritt; nur ein tatsächlich aktualisierter Branch mit anschließendem Rücklesen gilt als gespeicherter Auftrag.

### 9.2 Suchbefehle für die Fortsetzung — nur vorgeschlagen

Im richtigen lokalen Repository-Checkout:

```bash
# Ursprünglicher Chatvorschlag; kann auch Buildverzeichnisse durchsuchen:
grep -R "STACK_VERSION" .

# Präzisere, hier zusätzlich vorgeschlagene Suche in versionierten Quelldateien:
git grep -n -E 'STACK_VERSION|STACK_NAME|STACK_DISPLAY|STACK_CODENAME|GIT_VERSION|GIT_HASH' -- crates bins

git grep -n -E 'BlueStation|Midnight Blue|Net[Cc]ore|NetCore-TETRA' -- crates bins README.md wiki

git grep -n -E '^\[workspace\.package\]|^version(\.workspace)? *=' -- Cargo.toml crates/tetra-core/Cargo.toml bins/bluestation-bs/Cargo.toml
```

**Ausführungsstatus:** Diese Shell-Suchen wurden nicht auf einer lokalen vollständigen Repository-Kopie ausgeführt. Die oben dokumentierten Befunde stammen aus GitHub-Dateilesen, nicht aus erfundenen `grep`-Ausgaben.

### 9.3 Build- und Anzeigeprüfung — noch durchzuführen

Nach einem separat autorisierten Produktcode-Änderungsauftrag und in einer eingerichteten Buildumgebung wären beispielsweise zu prüfen:

```bash
cargo check --locked -p tetra-core
cargo check --locked -p bluestation-bs
cargo build --locked --release -p bluestation-bs

# Mit dem entsprechend gebauten Binary:
./target/release/bluestation-bs --help
./target/release/bluestation-bs --version
```

**Nicht ausgeführt.** Native Abhängigkeiten, Target-Auswahl, vorhandene Cargo-Konfiguration und ein gegebenenfalls abweichendes Ausgabeverzeichnis sind auf dem tatsächlichen Buildhost zu berücksichtigen. Das Binary aktiviert laut gelesenem Manifest standardmäßig `asterisk`, `recording` und `audio-player`; deshalb ist ein vollständiger Build nicht mit einer reinen Zeichenkettenprüfung gleichzusetzen. [R4]

Zu erwartende Abnahmekriterien, nicht vorliegende Testergebnisse: korrekte Unicode-Darstellung; NetCore-Bezeichnung in Banner und Hilfe; genau ein Versionspräfix; erklärbares Verhalten bei fehlenden Git-Metadaten; konsistente Kennungen in den gewünschten Ausgabepfaden; keine unberechtigten Bereitschaftsaussagen. Vor einem späteren User-Agent-Wechsel ist zusätzlich die Akzeptanz der Gegenstellen zu testen.

Ein normaler Start mit Funkkonfiguration wird hier **nicht** als Test vorgeschlagen oder ausgeführt. Für dieses Rebranding ist zunächst eine Prüfung ohne HF-Inbetriebnahme ausreichend.

### 9.4 Test- und Betriebsstatus

| Prüfung | Ergebnis / Grenze |
|---|---|
| Historischer Rust-Build | Kein Ergebnis vorhanden. |
| Heutige Pfad- und Konstantenprüfung | Definierte Dateien und Werte am Prüfsnapshot gelesen. |
| Workspace-Versionskette | Root-Version sowie Vererbung in beiden Manifesten gelesen; keine Cargo-Metadaten- oder Compilerausführung. |
| Banner-Startreihenfolge | Durch Quellcode gelesen; keine reale Terminalausgabe. |
| Wiki-Veröffentlichung | Nicht nachgewiesen. |
| Zielgerät / TBS / Funkgeräte | Nicht geprüft. |
| Brew-/BrandMeister-Verbindung | Nicht geprüft. |
| Im Betrieb bestätigtes Rebranding | In diesem Chat kein zuordenbarer Nachweis. |

## 10. Roadmap-Kandidaten, offene Wünsche und nächste Schritte

Die Reihenfolge ist ein **Vorschlag dieser Abschlussdokumentation**. Keine dieser Produktänderungen gehört zum aktuellen Schreibauftrag.

| ID | Kandidat / offener Wunsch | Ausgangsstatus | Nächster konkreter Schritt / Abhängigkeit |
|---|---|---|---|
| RB-01 | Einheitliche sichtbare Schreibweise `NetCore-Tetra` | Wunsch beschlossen; aktueller Code teilweise umgesetzt | Banner, CLI-Hilfe, Moduldokumentation und veröffentlichte Texte gezielt abgleichen; `NetCore-TETRA`/`Netcore-Tetra` bewusst vereinheitlichen oder begründen. |
| RB-02 | Projektpassender Zweizeiler statt URL und Version | Wunsch beschlossen, Text offen | Nutzerentscheidung nachholen, ob beide Zeilen entfallen oder die Diagnoseversion an anderer Stelle bleibt. Keine der sieben Ideen als gewählt behandeln. |
| RB-03 | Zentrale Konstanten statt verstreuter Produktliterale | Heute teilweise implementiert | Vorhandene `STACK_NAME`/`STACK_DISPLAY` wiederverwenden statt `STACK_FULL` parallel neu einzuführen; gewünschte Ausgabestellen einzeln prüfen. |
| RB-04 | CLI-Version und Banner-Version nachvollziehbar machen | Idee / neu erkannter Abgleichbedarf | Prüfen, ob `--version` ebenfalls Stack-/Git-Kennung zeigen soll; Core und Binary erben aktuell dieselbe Nummer. |
| RB-05 | Verbleibende User-Agent-Altkennungen prüfen | Konkreter heutiger Befund | Kompatibilität mit Telemetrie-/Control-Gegenstellen klären; erst danach gezielte Änderung. |
| RB-06 | Deutsche Projektbeschreibung auf heutigen Stand bringen | Historischer Entwurf vorhanden | Aktuelle Funktionsmatrix und Betriebsgrenzen zugrunde legen; Ursprungstext, heutigen Code und reale Abnahme trennen. |
| RB-07 | Credits und Herkunft korrekt erhalten | Historischer Quellinhalt vorhanden | Wouter/Midnight Blue, BlueStation/Osmocom, Tatu und Contributors passend zuordnen; NLnet nicht als eigene Förderzusage darstellen. |
| RB-08 | Wiki-Workflow belastbar beschreiben | Unbestätigte historische Aussage | Tatsächliche Pflegequelle, Branch, Sync-Auslöser und Veröffentlichungsnachweis ermitteln. |
| RB-09 | Releasekennung versus Rust-Workspace-Version erklären | README `v1.9.0` / Cargo `1.3.0` gelesen | Beabsichtigte Versionsebenen dokumentieren; kein automatisches Gleichziehen ohne Entscheidung. |
| RB-10 | Buildherkunft trotz fehlender Dirty-Anzeige erfassen | Heutige Git-Policy implementiert | Bedarf an separaten Build-/Patch-Metadaten bewerten; bestehende Anzeige-/OTA-Policy nicht unbemerkt ändern. |
| RB-11 | Ursprünglichen Chattitel/Chatlink ergänzen | Metadaten fehlen | Nur bei verlässlicher Bereitstellung durch Nutzer oder Export ergänzen. |

**Sinnvolle Arbeitsfolge:** zuerst RB-02 und RB-09 entscheiden; danach die kleinen Darstellungsänderungen RB-01/RB-03/RB-04 bearbeiten; Schnittstellenänderungen RB-05 separat abnehmen; Dokumentation und Credits RB-06 bis RB-08 aktualisieren. RB-10 bleibt eine optionale Nachvollziehbarkeitsverbesserung. Termine wurden nicht vereinbart.

Kleine Nebenideen bleiben ausdrücklich erhalten: humorvolle Bootmeldungen; kombiniertes Name-/Versionsfeld; getrennte App-/Core-Anzeige; Debug-/Release-Buildinformationen. Letztere wurden nur als mögliche spätere Vertiefung angeboten und nicht ausgearbeitet oder beauftragt.

## 11. Archivierung, Bilder und Schreibgrenzen

Der vorhandene Archivindex wurde gelesen. Die bereits bestehende Datei `2026-10-03_startbanner-runtime-versionierung-arm64-release-rustfmt.md` gehört zu einem **anderen** Gespräch mit FlowStation-Einstieg, Runtime-Summary, ARM64-Workflow und Rustfmt. Sie wird nicht überschrieben. Der neue Eintrag dokumentiert den hier behandelten ursprünglichen Rebranding-/Versionsfund-/Übersetzungschat.

**Bildprüfung:** Im verfügbaren Verlauf sind keine eigenständigen Fotos, Screenshots oder generierten Projektbilder dieses Chats vorhanden. Der Dateidienst führt ausschließlich 25 PDF-Projektquellen auf; auch unter den bereitgestellten Originaldateien wurde kein eigenständiges Bild gefunden. Die sichtbaren ETSI-Deckblattvorschauen sind Bestandteile dieser PDFs, keine separat erstellten NetCore-Bilder. Deshalb wird für diesen Chat kein Bildasset hochgeladen und kein Ersatzbild erfunden. Die Rust-Unicode-Banner sind oben als Text erhalten.

Die Norm-PDFs werden für diesen thematisch engen Auftrag **inventarisiert, nicht als vollständige Kopien oder seitenweise Bildexporte ins Archiv hochgeladen**. Es gibt keine im Chat besprochene Normgrafik, die für das Verständnis des Rebrandings zusätzlich gesichert werden müsste. Das ist eine bewusste Umfangsabgrenzung, keine Behauptung, die PDFs seien unzugänglich.

Beim Speichern gelten: ausschließlich `Archiving`; nur Zusammenfassung und `Docs/archive/README.md`; bestehende Indexzeilen erhalten; kein Force-Push; kein Merge. Der spätere Ablagecommit darf außerhalb dieses Pfadbereichs keine Änderungen enthalten. Ein separater Abschlusscheck liest Zusammenfassung und Index vom Zielbranch zurück und prüft den tatsächlichen Commitumfang. Zugangsdaten werden nicht übernommen.

## 12. Anhanginventar

Die Angaben beziehen sich auf die **bereitgestellten Ausgaben**, nicht auf einen behaupteten aktuellen ETSI-Publikationsstatus. Entwürfe bleiben als solche gekennzeichnet. Erfasst wurden Deckblatt, Seitenzahl und ein auf zwölf Hexzeichen verkürzter SHA-256 zur Zuordnung; kein Volltext-/Konformitätstest.

| Datei | Bereitgestellte Ausgabe / Thema | Seiten | SHA-256, gekürzt |
|---|---|---:|---|
| `ETSI.pdf` | 4.100-seitige Datei; erstes Deckblatt EN 300 812 V2.1.1 (2001-12); Gesamtzusammensetzung hier nicht rekonstruiert | 4100 | `9434dad1e7bc` |
| `en_30039201v010601p.pdf` | EN 300 392-1 V1.6.1 (2020-04): General network design | 182 | `788722e56609` |
| `en_30039202v030801p.pdf` | EN 300 392-2 V3.8.1 (2016-08): Air Interface | 1445 | `3f07b1e4ad73` |
| `en_3003920303v010301p.pdf` | EN 300 392-3-3 V1.3.1 (2011-11): ANF-ISIGC | 251 | `94ca61038b3f` |
| `en_3003920304v010301p.pdf` | EN 300 392-3-4 V1.3.1 (2010-08): ANF-ISISDS | 28 | `8a38cc6238c6` |
| `en_3003920308v010401p.pdf` | EN 300 392-3-8 V1.4.1 (2020-04): Generic Speech Format Implementation | 22 | `4d993a25e35d` |
| `en_3003920313v010201p.pdf` | EN 300 392-3-13 V1.2.1 (2020-04): transportunabhängiges ANF-ISIGC | 191 | `b47d63853f83` |
| `en_3003920315v010500a.pdf` | Draft EN 300 392-3-15 V1.5.0 (2026-04): transportunabhängiges ANF-ISIMM | 380 | `e95970966719` |
| `en_30039205v020701p.pdf` | EN 300 392-5 V2.7.1 (2020-04): PEI | 320 | `10aaf78988ae` |
| `en_30039207v030501p.pdf` | EN 300 392-7 V3.5.1 (2019-07): Security | 216 | `df47af9a642b` |
| `en_30039209v010701p.pdf` | EN 300 392-9 V1.7.1 (2020-04): allgemeine Anforderungen an Supplementary Services | 46 | `cad45938bcdf` |
| `en_3003921006v010401p.pdf` | EN 300 392-10-6 V1.4.1 (2006-08): CAD, Stage 1 | 20 | `32b6ee43602e` |
| `en_3003921018v010301p.pdf` | EN 300 392-10-18 V1.3.1 (2003-10): BOC, Stage 1 | 17 | `4cc10bcd94d3` |
| `en_3003921101v010201p.pdf` | EN 300 392-11-1 V1.2.1 (2004-01): Call Identification, Stage 2 | 44 | `852c17ea5667` |
| `en_3003921114v010101p.pdf` | EN 300 392-11-14 V1.1.1 (2002-07): Late Entry, Stage 2 | 23 | `ba1882df71db` |
| `en_3003921117v010102p.pdf` | EN 300 392-11-17 V1.1.2 (2002-01): Include Call, Stage 2 | 18 | `69ce800e352b` |
| `en_3003921201v010202p.pdf` | EN 300 392-12-1 V1.2.2 (2007-08): Call Identification, Stage 3 | 56 | `4d5b56a188fb` |
| `en_3003921216v010400a.pdf` | DRAFT EN 300 392-12-16 V1.4.0 (2026-03): PPC, Stage 3 | 67 | `c0ee7859f448` |
| `en_30039401v030301p.pdf` | EN 300 394-1 V3.3.1 (2015-04): Conformance testing, Radio | 169 | `2d9c327e5ccf` |
| `en_30039502v010303p.pdf` | EN 300 395-2 V1.3.3 (2025-02): TETRA codec | 94 | `ac716ca18082` |
| `en_300812v020101p.pdf` | EN 300 812 V2.1.1 (2001-12): Security aspects, SIM-ME | 156 | `196effeaae47` |
| `es_20081201v020205p.pdf` | ES 200 812-1 V2.2.5 (2003-12): UICC, physikalische/logische Eigenschaften | 8 | `346dc545049a` |
| `es_20081202v020401m.pdf` | Final draft ES 200 812-2 V2.4.1 (2005-08): TSIM application | 139 | `330f045a908e` |
| `ets_30039214e01v.pdf` | Final draft prETS 300 392-14 (1997-09): PICS proforma | 61 | `2b703f3ebb88` |
| `ts_10081201v020205p.pdf` | TS 100 812-1 V2.2.5 (2003-10): UICC, physikalische/logische Eigenschaften | 8 | `96df75f5ddf1` |

## 13. Quellen, verwandte Archive und Nachvollziehbarkeit

### Historische Chatquellen

**H1:** Nutzeranfrage zum originalen dreizeiligen BlueStation-Banner mit Wouter Bokslag/Midnight Blue und `tetra_core::STACK_VERSION`.

**H2:** Nutzerwunsch nach einem Ersatz der zwei Zeilen für Repository-URL und Version; sieben nachfolgende Assistentenideen ohne endgültige Auswahl.

**H3:** Anfrage zur Repository-Prüfung unter `JanHG98/bluestation` und der danach vom Nutzer selbst gelieferte Ausdruck aus `crates/tetra-core/src/lib.rs`.

**H4:** Assistentenvorschläge `STACK_NAME`, `STACK_FULL`, App-/Core-Anzeige; keine gespeicherten Änderungen belegt.

**H5:** Englische Projekt-/Dokumentationsvorlage einschließlich Danksagungen und der daraufhin gelieferte deutsche Entwurf. Historischer Förderverweis: [RETETRA3 bei NLnet](https://nlnet.nl/project/RETETRA3/), in diesem Auftrag nicht zusätzlich inhaltlich überprüft.

### Verifizierte Repository-Quellen

Die folgenden Links sind auf den technischen Prüfsnapshot gepinnt, nicht auf einen veränderlichen Branchnamen:

- **R1:** [Core-Konstanten][R1], Blob `7062c759096d94213edb308fde9f1a47ba04269a`.
- **R2:** [Basisstations-Einstiegspunkt][R2], Blob `2e37254ce396a99765f499b211454436456693fa`; relevante Bereiche 1–460, 490–670 und 800–Dateiende gelesen, keine vollständige semantische Prüfung jeder Funktion.
- **R3:** [Core-Manifest][R3], Blob `e31970bff244a0e07631fec6c7c3095e582da14c`, sowie [Workspace-Manifest][R3a], Blob `fb22c40ed26b08778f7b256b0aad87588ad6574f`.
- **R4:** [Binary-Manifest][R4], Blob `db65cf2714bcfda02987b3e5e0fea81837afb073`, relevanter Bereich 1–72 gelesen.
- **R5:** [Root-README][R5], Blob `80dbc3d5f5ff94083ed1b90c4d0256d40fb126fe`.
- **R6:** [Wiki-Startseite][R6], Blob `0f199507580aaca3d5deadf4fd735897854634ce`.
- **R7:** [GitHub-Konfigurationsbaum][R7], Tree `2eda89e96a014080ccf50c6f72bc63b3c80fbd84`, rekursiv ohne serverseitige Kürzung gelesen.

Das Datum des Prüfsnapshot-Commits ist im GitHub-Objekt `2026-10-03T22:35:04Z`, entsprechend bereits 04.10.2026 in `Europe/Berlin`. Die Zusammenfassung verwendet deshalb den tatsächlichen lokalen Erstellungstag **2026-10-04**.

### Externe technische Primärquellen zur heutigen Einordnung

Diese Quellen ergänzen die historische Darstellung; sie ersetzen keine Chatfestlegung und keinen Repository-Test:

- **E1:** [Cargo: Environment Variables][E1] — Bedeutung von `CARGO_PKG_VERSION`.
- **E2:** [Cargo: Workspaces][E2] — Vererbung über `version.workspace = true`.
- **E3:** [git-version: Makro-Dokumentation][E3] — `args`, `fallback` und Standardverhalten.
- **E4:** [const_format 0.2.35][E4] — Kompilierzeit-Formatierung.
- **E5:** [Git: git-describe][E5] — Commit-/Tag-Darstellung, Abkürzung und Dirty-Option.

### Verwandtes, getrennt zu behandelndes Archiv

[Startbanner, Runtime-Diagnose, Versionierung, ARM64-Releases und rustfmt](2026-10-03_startbanner-runtime-versionierung-arm64-release-rustfmt.md) ist ein bestehender Nachbarbeitrag. Er wurde nur zur Abgrenzung gelesen und bleibt unverändert. Seine Nutzerentscheidungen, Testaussagen oder historischen Codeblöcke sind nicht automatisch Bestandteil dieses Chats.

**Historische Implementierungscommits / PRs dieses Chats:** nicht verfügbar. **Laufender TBS-Build:** nicht ermittelt. **Archiv-Ablagecommit:** über Git-Historie dieser Datei und Abschlussmeldung feststellbar; nicht mit dem oben gepinnten Prüfcommit verwechseln.

[R1]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/crates/tetra-core/src/lib.rs
[R2]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/bins/bluestation-bs/src/main.rs
[R3]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/crates/tetra-core/Cargo.toml
[R3a]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/Cargo.toml
[R4]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/bins/bluestation-bs/Cargo.toml
[R5]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/README.md
[R6]: https://github.com/JanHG98/netcore-tetra/blob/3fb792cf4c3653b6ad91652b1265b18ac8c1772e/wiki/Home.md
[R7]: https://api.github.com/repos/JanHG98/netcore-tetra/git/trees/2eda89e96a014080ccf50c6f72bc63b3c80fbd84?recursive=1
[E1]: https://doc.rust-lang.org/cargo/reference/environment-variables.html
[E2]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[E3]: https://docs.rs/git-version/latest/git_version/macro.git_version.html
[E4]: https://docs.rs/crate/const_format/0.2.35
[E5]: https://git-scm.com/docs/git-describe
