# Abschlussdokumentation: Flowstation Wetterdaten und SDS-Abfragedienst

## Metadaten

- **Projekt:** NetCore-Tetra
- **Thema:** Wetterdaten für Flowstation/NetCore-Tetra per SDS und bedarfsgesteuerte Wetterabfrage vom Funkgerät
- **Ursprünglicher Chattitel:** im zugänglichen Verlauf nicht als eigener Titel überliefert; Nutzer-Stichwort: „Flowstation“
- **Chatlink:** im zugänglichen Kontext nicht verfügbar
- **Erstellt:** 2026-10-03
- **Zielbranch:** `Archiving`
- **Vor dem Schreiben geprüfter Branch-HEAD:** `44cfe017ad4610a16a29746c41042eb47fc577f4`
- **Repository:** `JanHG98/netcore-tetra`

> Statusbegriffe werden streng getrennt: **Idee**, **beschlossen/geplant**, **implementiert**, **getestet**, **im Betrieb bestätigt**. Eine Chat-Aussage allein gilt nicht als Implementierungsnachweis.

## 1. Ziel und Ausgangslage

Ausgangspunkt war die Frage, ob Flowstation Wetterdaten online beziehen – insbesondere vom DWD – und als TETRA-SDS verteilen kann. Danach wurde die Idee wesentlich erweitert: Ein Funkgerät soll eine definierte Serviceadresse, beispielhaft `40004`, per SDS ansprechen und automatisch eine Wetter-SDS als Antwort erhalten. Damit entsteht ein SDS-basierter Request/Response-Dienst („WeatherBot“).

Dieser Chat war ein Architektur-/Ideenchat. **Es wurde hier kein WeatherBot-Code geschrieben, kein Deployment durchgeführt und kein Funk-Livetest dokumentiert.**

## 2. Endgültiger fachlicher Wunsch aus dem Chat

**Status: beschlossen/geplant als gewünschte Funktion; noch nicht implementiert nachgewiesen.**

Der gewünschte Ablauf ist:

```text
Funkgerät / MS
   | Uplink-SDS, z.B. "WX HAMBURG"
   v
NetCore SDS-Ingress
   v
SDS Command Router
   v
Weather Service
   | externe Wetterquelle + Cache
   v
SDS Formatter
   v
bestehender SDS-Versandpfad
   v
Antwort-SDS an die anfragende ISSI
```

Beispiel aus dem Chat:

```text
MS -> Serviceadresse 40004
WX HAMBURG
```

Antwortidee:

```text
WX Hamburg 15:42
12C Regen
Wind W 28km/h
DWD: keine Warnung
```

Der Vorteil gegenüber reinem Push ist, dass Wetterdaten gezielt angefordert werden und das Funknetz nicht mit periodischen Wetter-SDS belastet wird.

## 3. Serviceadresse und Kommandos

### 3.1 Beispiel `40004`

**Status: Idee, nicht reserviert nachgewiesen.**

`40004` wurde vom Nutzer beispielhaft genannt. Eine heutige Repository-Suche ergab keinen Treffer für `40004`. Vor Implementierung muss daher eine kollisionsfreie Serviceadresse verbindlich im NetCore-Adressplan reserviert werden.

Im Chat wurde als mögliche spätere Aufteilung skizziert:

```text
40001 = Flowstation System
40002 = StatusBot
40003 = ConfigBot
40004 = WeatherBot
40005 = AlertBot
```

Auch diese komplette Liste ist **nur eine Ideenliste** und kein nachgewiesener aktueller Adressplan.

### 3.2 Vorgeschlagene Kommandos

**Status: Idee.**

```text
WX
WX HAMBURG
WX WARN
WX FULL
WX 48H
```

Mögliche Semantik: `WX` für Default-Kontext, `WX <ORT>` für einen expliziten Ort, `WX WARN` für Wetterwarnungen, `WX FULL` für eine ausführlichere Darstellung und `WX 48H` für einen kompakten Forecast. Syntax, Ortsauflösung, Zeichensatz und Antwortlängen sind noch festzulegen.

## 4. Architektur und Komponenten

### 4.1 SDS Command Router

**Status: geplant/Architekturvorschlag.**

Der Router soll Transport und Fachlogik trennen. Er nimmt eine bereits dekodierte eingehende SDS entgegen, ermittelt Absender und Ziel, validiert Berechtigungen/Rate-Limits und delegiert an einen Handler. Wetter-, Status- oder spätere Bot-Funktionen sollten nicht direkt in der PDU-Schicht implementiert werden.

### 4.2 Weather Service

**Status: Idee.**

Im Chat wurde sinngemäß folgendes internes Modell vorgeschlagen:

```rust
struct WeatherSdsMessage {
    location: String,
    temperature_c: Option<f32>,
    condition: Option<String>,
    wind_kmh: Option<f32>,
    warning: Option<String>,
    valid_until: Option<String>,
}
```

Das ist Pseudocode, kein vorhandener Repository-Typ. Der Dienst soll externe Daten abrufen, cachen, kompakt formatieren und über den vorhandenen SDS-Pfad an den ursprünglichen Absender antworten.

### 4.3 Datenquellen

**DWD/Open Data – Status: Idee.** DWD wurde als fachlich naheliegende Primärquelle genannt.

**Bright Sky – Status: Idee.** Bright Sky wurde als einfacher nutzbare API-Schicht für DWD-Daten vorgeschlagen, um die Integration gegenüber rohen DWD-Dateidaten zu vereinfachen.

Im Chat wurde **keine endgültige Datenquelle beschlossen**. Vor Implementierung müssen konkrete Endpunkte, Datenformate, Verfügbarkeit und Nutzungsbedingungen aktuell geprüft werden.

## 5. Heutiger Repository-Abgleich

Der Branch `Archiving` wurde vor dem Schreiben geprüft; Ausgangs-HEAD war `44cfe017ad4610a16a29746c41042eb47fc577f4`.

### 5.1 SDS-Grundlage ist bereits implementiert

**Status: implementiert im Repository; kein neuer Live-Test in diesem Chat.**

Vorhanden sind unter anderem:

- `crates/tetra-saps/src/control/enums/sds_user_data.rs`
- `crates/tetra-pdus/src/cmce/pdus/u_sds_data.rs`
- `crates/tetra-pdus/src/cmce/pdus/d_sds_data.rs`

`SdsUserData` modelliert Type 1 (16 Bit), Type 2 (32 Bit), Type 3 (64 Bit) und Type 4 (variable Länge). U-SDS-DATA und D-SDS-DATA besitzen Parsing-/Serialisierungslogik. Der vorhandene Type-4-Code verwendet einen 11-Bit-Längenindikator; die Codekommentare nennen bis zu 2.047 Bit und empfehlen, möglichst den kürzesten geeigneten SDS-Typ zu verwenden.

Damit ist eine wesentliche technische Grundlage für Request/Response bereits vorhanden.

### 5.2 Bestehender Warn-/SDS-Pfad

**Status: implementiert/dokumentiert; Betriebszustand hier nicht erneut live bestätigt.**

Im Repository existiert `system-backend/alert-service/`. Dessen README beschreibt eine NetCore-Warnzentrale, die öffentliche BBK-Feeds verarbeitet und individuelle SDS über den SDS Router versendet. Als Standardquellen sind dort `mowas`, `katwarn`, `biwapp`, `dwd` und `lhp` aufgeführt.

Damit existieren bereits wiederverwendbare Muster für:

- externen HTTP-Datenabruf,
- Polling und Stale-/Fehlerbehandlung,
- SDS Router als Versandpfad,
- konfigurierbare Source-ISSI,
- Funktext-Kürzung und ASCII-Normalisierung,
- Health-/Ready-Endpunkte,
- LXC-Dienst und WebUI,
- authentifizierte Verwaltungs-API.

Der bestehende Alert Service nutzt standardmäßig Port `8310`. Dieser Port gehört **nicht automatisch** zum geplanten WeatherBot.

Die README des Alert Service nennt standardmäßig 120 Zeichen Funktext. Das ist eine bestehende Entscheidung dieses Dienstes und nicht automatisch die verbindliche WeatherBot-Grenze.

### 5.3 DWD ist bereits teilweise integriert

Die ursprüngliche Annahme „DWD müsste vollständig neu angebunden werden“ ist teilweise überholt: DWD-Warnungen sind über den bestehenden BBK/NINA-Pfad bereits Teil des Alert Service.

Weiterhin offen und Gegenstand dieses Chats ist ein **interaktiver Abruf allgemeiner Wetter-/Forecastdaten per SDS**.

### 5.4 WeatherBot fehlt weiterhin

Repository-Suchen nach `WeatherService`, `WeatherBot`, `Bright Sky`, `brightsky` und `40004` ergaben keine entsprechende Implementierung.

**Aktueller Status des Kernfeatures: nicht implementiert nachgewiesen.**

## 6. Relevanter Standardsbezug

Für die spätere Umsetzung sind insbesondere die im Projekt vorhandenen ETSI-Unterlagen relevant:

- ETSI EN 300 392-3-4: ANF-ISISDS, einschließlich Called/Calling Party SSI, Short Data Type Identifier und User Defined Data.
- ETSI EN 300 392-1: allgemeines TETRA-Netzdesign; enthält einen Abschnitt zur technischen Realisierung von SDS.
- ETSI EN 300 392-5: PEI; behandelt SDS Message Stacks, Status, SDS Typ 1–4 und SDS User Data.

Diese Dokumentation führt **keine vollständige Normkonformitätsprüfung** des WeatherBot durch.

## 7. Sicherheit und Betriebsgrenzen

**Status: geplant/empfohlen; konkrete Policy offen.**

Im Chat wurden vorgeschlagen:

- SDS-Kommandos nicht unbeschränkt für jede ISSI freigeben,
- Weather-Request separat aktivierbar machen,
- Rate Limit, beispielhaft 1 Request / 60 s / ISSI,
- Wetterdaten cachen, beispielhaft 10–15 Minuten,
- keine periodische Wetter-Spam-SDS,
- automatischen Push eher auf Warnlagen beschränken.

Die genannten Zeitwerte sind Vorschläge, keine beschlossene Konfiguration.

Für die Umsetzung zusätzlich wichtig: Eingaben strikt begrenzen, keine SDS-gesteuerten beliebigen URLs zulassen, HTTP-Timeout/Response-Limits setzen, Ortsparameter validieren, Antwort an die tatsächliche Absender-ISSI binden und Telemetrie für Requests, Cache-Hits, API-Fehler und SDS-Versandstatus erfassen.

## 8. Entwicklungs- und Betriebsstand

| Baustein | Status | Befund |
|---|---|---|
| SDS Type 1–4 | implementiert | Repository-Code vorhanden |
| U-SDS-DATA | implementiert | Parsing/Serialisierung vorhanden |
| D-SDS-DATA | implementiert | Parsing/Serialisierung vorhanden |
| SDS Router / zentraler Versand | implementiert/dokumentiert | wird vom Alert Service genutzt |
| Alert Service | implementiert/dokumentiert | `system-backend/alert-service/` |
| DWD-Warnquelle | implementiert im Alert-Kontext | BBK/NINA-Quelle `dwd` |
| allgemeine Wetterdaten | nicht nachgewiesen | kein WeatherService gefunden |
| WeatherBot | nicht implementiert | keine Treffer |
| Service-ISSI `40004` | Idee | keine Reservierung nachgewiesen |
| SDS Command Router für WX | nicht implementiert nachgewiesen | Roadmap-Kandidat |
| Funk-Livetest Request -> Response | nicht getestet | kein Test im Chat |
| DWD/Bright-Sky-Livetest | nicht getestet | kein Test im Chat |

## 9. Befehle, Deployment, Fehler und Tests

In diesem Chat wurden **keine Shell-Befehle, Installationen, Deployments oder Reparaturen ausgeführt**. Es traten daher keine WeatherBot-Laufzeitfehler auf.

Es wurden auch keine produktiven WeatherBot-Konfigurationsdateien angelegt.

Der heutige Repository-Abgleich bestätigt nur vorhandene SDS- und Alert-Service-Bausteine. Er bestätigt **nicht**, dass ein Funkgerät aktuell eine SDS an eine Service-ISSI senden und darauf dynamisch erzeugte Wetterdaten empfangen kann.

## 10. Verworfene oder präzisierte Ansätze

- **„DWD komplett neu anbinden“:** teilweise überholt, weil DWD-Warnungen bereits im Alert Service auftauchen. Allgemeine Wetterdaten/Forecast bleiben offen.
- **„40004 ist WeatherBot“:** nicht als bestehende Festlegung behandeln; es war ein Beispiel.
- **„Bright Sky als Startpunkt“:** bleibt Vorschlag, ist weder beschlossen noch implementiert.
- **Reiner Push:** nicht verworfen, aber durch Pull/Request-Response ergänzt. Automatischer Push bleibt besonders für Warnlagen sinnvoll.

## 11. Roadmap-Kandidaten und nächste Schritte

1. **Serviceadressplan festlegen:** WeatherBot-Adresse verbindlich reservieren; `40004` nur bei Kollisionsfreiheit.
2. **SDS-Ingress verfolgen:** konkreten Anwendungshook bestimmen, an dem eingehende U-SDS-DATA inklusive Absender-ISSI an einen Command Router übergeben werden kann.
3. **Generischen Command Router bauen:** Handler-Registry, Auth/Allowlist, Rate-Limit, Fehlerantworten und Telemetrie.
4. **Datenquelle entscheiden:** DWD direkt vs. geeignete API-Schicht; aktuelle API-/Nutzungsbedingungen prüfen.
5. **Doppelstrukturen vermeiden:** Alert Service und WeatherBot auf gemeinsame Wetter-/Warnprovider prüfen.
6. **Weather Handler implementieren:** `WX`, `WX <ORT>`, anschließend optional `WX WARN`, `WX FULL`, `WX 48H`.
7. **Caching implementieren:** API-Schonung und schnelle SDS-Antworten.
8. **Formatter definieren:** ASCII/Zeichensatz, harte Längenlimits, Kurz-/Detailformat.
9. **Antwortpfad:** vorhandenen SDS Router verwenden und Antwort gezielt an die anfragende ISSI senden.
10. **Tests:** Unit-Tests Parser/Router/Formatter, Mock-API, Rate-Limit, Timeout, ungültige Orte, anschließend kontrollierter TBS/MS-Livetest.
11. **WebUI optional:** manueller Wetterabruf und „Als SDS senden“ kann später ergänzend eingebaut werden.
12. **Betrieb:** Health/Ready, Metriken, Logs und externe API-Ausfälle dokumentieren.

**Priorität:** Zuerst Adressierung + Ingress/Router festziehen; erst danach Provider und UI. Dadurch wird der Command Router auch für spätere SDS-Dienste wiederverwendbar.

## 12. Relevante Dateien und Quellen

Repository:

- `crates/tetra-saps/src/control/enums/sds_user_data.rs`
- `crates/tetra-pdus/src/cmce/pdus/u_sds_data.rs`
- `crates/tetra-pdus/src/cmce/pdus/d_sds_data.rs`
- `system-backend/alert-service/README.md`
- `system-backend/alert-service/main.py`

ETSI-Unterlagen aus dem Projektkontext:

- EN 300 392-3-4 – Additional Network Feature Short Data Service (ANF-ISISDS)
- EN 300 392-1 – General network design
- EN 300 392-5 – Peripheral Equipment Interface (PEI)

## 13. Auswertungslücken

Der in dieser Unterhaltung sichtbare Verlauf zum Thema Flowstation/Wetter wurde vollständig berücksichtigt. Ein eigener ursprünglicher Chatlink und ein expliziter UI-Chattitel waren nicht verfügbar.

Die zahlreichen im Projektkontext vorhandenen ETSI-PDFs wurden **nicht vollständig Seite für Seite ausgewertet**, weil dieser Chat keine vollständige SDS-Normimplementierung oder Konformitätsanalyse zum Gegenstand hatte. Herangezogen wurden die für das Thema unmittelbar relevanten SDS-Bezüge.

Es wurden keine Passwörter, Tokens, Schlüssel oder sonstigen Zugangsdaten in diese Archivdokumentation übernommen.
