# Teilnehmerzulassung: zentrale Richtlinie und aktive TBS

**Abgleich: 9. Oktober 2026, Quellstand `c3ccdb4`.** Grundlage: [Policy-Bildung und Sync](../../../system-backend/subscriber-core/src/state.rs), [aktive TBS-Capabilities](../../../crates/tetra-entities/src/net_control_room/protocol.rs) und [MM-Registrierung](../../../crates/tetra-entities/src/mm/mm_bs.rs). Die vorhandene Core-Schnittstelle ist von der fehlenden aktiven MM-Anbindung zu unterscheiden.

## Was der Subscriber Core erzeugt

Der Core übersetzt Profile in eine versionierte Zulassungsrichtlinie. Im Modus `allow_list` nimmt `profile_authorized` nur Profile mit `enabled = true` und `registration_allowed = true` auf. Eine leere zentrale Liste erzeugt `allow_all = false` und keine erlaubten ISSIs, also deny-all im Policy-Modell. `open_network` erzeugt `allow_all = true`.

`auto_sync`, `disconnect_unauthorized` und `sync_timeout_secs` stehen im Abschnitt `[access_policy]`. `default_groups` bleibt ein Profilfeld und erzeugt hier keine Group-Core-Mitgliedschaft.

## Vorhandener Verteil- und Bestätigungspfad

Für eine verbundene, nicht veraltete TBS mit angekündigter `subscriber_policy`-Capability plant der Core `ControlCommand::SubscriberAccessPolicyApply` über `/ws/backend` des Node Gateway. Request-ID, Command-ID und die fachliche Response können korreliert werden. Eine Bestätigung kann Revision, erlaubte Teilnehmerzahl und erzwungene Disconnects in den Sync-Datensatz übernehmen.

Dieser Pfad ist im zentralen Dienst und in den Transportdatentypen vorhanden. Die Gateway-Annahme allein ist keine Policy-Anwendung auf der TBS.

## Der aktive Stack ist noch nicht angeschlossen

`ControlRoomNodeCapabilities::from_stack_config` setzt `subscriber_policy = false`. `schedule_sync_locked` im Subscriber Core markiert solche Nodes als `unsupported` und sendet kein Policy-Kommando. Der aktive MM-Registrierungspfad liest die lokale Konfigurations-/Dashboard-Whitelist, nicht die hier erzeugte zentrale Teilnehmerliste.

Daraus folgen für diesen Quellstand:

- Ein zentral gesperrtes oder gelöschtes Profil sperrt die aktive TBS-Registrierung noch nicht automatisch.
- Eine leere zentrale `allow_list` schließt das tatsächliche Funknetz noch nicht.
- `disconnect_unauthorized = true` ist ohne aktive Policy-Anwendung keine belegte Re-Registrierungs-/Disconnect-Funktion.
- Ein erfolgreiches Mock-TBS-ACK beweist die aktive MM-Durchsetzung nicht.

## Lokale TBS-Whitelist

MM bevorzugt `issi_whitelist_override` aus dem Laufzeitzustand und fällt sonst auf die konfigurierte ISSI-Whitelist zurück. Eine leere lokale Liste bedeutet im aktiven Pfad **offenes Netz**. Dieser Unterschied zur zentralen Closed-Empty-Policy muss bei der Anlagenprüfung sichtbar bleiben.

Die Dashboard-Liste ist derzeit daher die tatsächlich wirksame lokale Zulassungsebene. Sie kann nicht als bloßer Override einer bereits aktiven Subscriber-Core-Richtlinie beschrieben werden.

## Home-ISSI und migrierte Teilnehmer

Die zentrale Richtlinie wird aus den Home-ISSIs der Teilnehmerprofile gebildet. Eine korrekte Behandlung lokaler Migrationsadressen muss Bestandteil der späteren aktiven Policy-Anbindung sein. Aus dem vorhandenen Core-Modell folgt noch keine nachgewiesene Home-ISSI-/VASSI-Abbildung für jede lokale MM-Zulassungs- oder Disconnect-Entscheidung.

## Prüfung vor einer zentralen Zulassungsabnahme

Über `/api/v1/nodes` und `/api/v1/syncs` Capability und Phase prüfen. Im jetzigen aktiven Stack ist `unsupported` der erwartbare Hinweis auf die Integrationsgrenze. Für eine spätere Abnahme müssen aktive TBS-Capability, MM-Anwendung, fachlich bestätigte Revision und echte erlaubte beziehungsweise abgewiesene Registrierung zusammen belegt werden.
