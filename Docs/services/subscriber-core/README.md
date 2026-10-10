# Subscriber Core

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [src/state.rs](../../../system-backend/subscriber-core/src/state.rs) · [src/http.rs](../../../system-backend/subscriber-core/src/http.rs). Beschreibt den implementierten Umfang; eine Live- oder Funkabnahme wird damit nicht belegt.

## Zweck

Der Subscriber Core ist die zentrale Teilnehmerdatenbank und erzeugt eine versionierte Zulassungsrichtlinie für dafür kompatible TBS. Die aktive TBS dieses Quellstands setzt diese zentrale Richtlinie noch nicht in MM durch.

## Aktueller Funktionsumfang

- persistente Teilnehmerprofile als atomar geschriebene JSON-Datenbank
- ISSI, Home-MCC/MNC, Name, Organisation und Gerätezuordnung
- Freigabe, Sperre, Rufpriorität und Dienstberechtigungen
- Standardgruppen als Profilfelder; die verbindlichen Gruppenmitgliedschaften verwaltet der bereits vorhandene Group Core
- Live-Sicht auf registrierte Funkgeräte aus der TBS-Telemetrie
- vorhandener Verteil- und Bestätigungspfad für TBS mit `subscriber_policy = true`; aktive TBS melden derzeit `false` und erhalten `unsupported`
- expliziter Closed-Empty-Modus im zentralen Policy-Modell: eine leere `allow_list` erzeugt **deny all**; dies sperrt die aktive TBS ohne Policy-Anbindung noch nicht
- JSON-Import/Export und CSV-Export
- eigene WebUI, REST-API, Metriken und OpenAPI

## WebUI

```text
http://<LXC-IP>:8100/
```

Die WebUI bietet Teilnehmer-CRUD, Import/Export, Live-Registrierungen, TBS-Synchronisationsstatus und Ereignisprotokoll.

## Offener Testmodus

Diese Stufe arbeitet absichtlich ohne Tokens, Login, Passwörter, TLS oder Client-Zertifikate. Jeder erreichbare Client kann Teilnehmer und Zugangsregeln ändern. Nur in einem isolierten Testnetz betreiben.

## Zugangsmodi

- `allow_list`: Die zentrale Richtlinie enthält nur Profile mit `enabled = true` und `registration_allowed = true`.
- `open_network`: Die zentrale Richtlinie setzt `allow_all = true`; Profile dienen als Stammdaten.

Diese Modi beschreiben die Policy-Erzeugung im Core. Auf der aktiven TBS wird die Registrierung weiterhin anhand der lokalen Konfigurations-/Dashboard-Whitelist geprüft; eine leere lokale Whitelist bedeutet dort offenes Netz.

Bei `auto_sync = true` plant der Core die Synchronisation für verbundene TBS. Ohne `subscriber_policy`-Capability endet sie als `unsupported`, ohne Policy-Kommando. Der aktive Stack setzt diese Capability in `from_stack_config` auf `false`; die zentrale Sperre und `disconnect_unauthorized` sind damit im vorhandenen MM-Pfad noch nicht wirksam.

## Architekturgrenze

Aktueller Aufenthaltsort und Context Transfer verbleiben im Mobility Core. Gruppenautorität liegt im Group Core. Kryptografische Schlüssel gehören niemals in diesen Dienst.

## Profilfelder und Durchsetzung

`call_priority`, `emergency_allowed`, `sds_allowed`, `packet_data_allowed` und `default_groups` sind gespeicherte Profilinformationen. Die **zentrale Policy-Bildung** filtert anhand von `enabled` und `registration_allowed`; dies ist keine Prüfung dieser Profilfelder in der aktiven TBS-MM-Zulassung. Ein gespeichertes Dienstrecht ist allein kein Nachweis seiner Durchsetzung in Call Control, SDS Router oder Packet Core. Gruppenmitgliedschaften deshalb zusätzlich im Group Core verwalten.

## Fehlende aktive TBS-Anbindung

Quellen: [aktive Capability-Erzeugung](../../../crates/tetra-entities/src/net_control_room/protocol.rs), [Core-Sync und Policy-Bildung](../../../system-backend/subscriber-core/src/state.rs) und [aktive MM-Registrierung](../../../crates/tetra-entities/src/mm/mm_bs.rs). Der vorhandene Kommandodatentyp `SubscriberAccessPolicyApply` und die Response-Korrelation belegen einen Schnittstellenpfad, aber keine aktive MM-Durchsetzung. Vor einer behaupteten zentralen Netzsperre sind Capability, MM-Anwendung und bestätigte Revision einschließlich realer Registrierungsprüfung nachzuweisen.
