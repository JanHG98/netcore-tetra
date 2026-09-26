// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_config::bluestation::{CfgBrew, SharedConfig};
use tetra_core::tetra_entities::TetraEntity;

// Was: Legt den festen Wert `BREW_ENTITIES` für Brew-Verbindung entities fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const BREW_ENTITIES: [TetraEntity; 2] = [TetraEntity::Brew, TetraEntity::Brew2];

#[inline]
// Was: Prüft, ob Brew-Verbindung entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_entity(entity: TetraEntity) -> bool {
    matches!(entity, TetraEntity::Brew | TetraEntity::Brew2)
}

#[inline]
// Was: Führt den Arbeitsschritt `brew_config_for_entity` für Brew-Verbindung Konfiguration for entity aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn brew_config_for_entity(config: &SharedConfig, entity: TetraEntity) -> Option<CfgBrew> {
    let cfg = config.config();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match entity {
        TetraEntity::Brew => cfg.brew.clone(),
        TetraEntity::Brew2 => cfg.brew2.clone(),
        _ => None,
    }
}

/// Returns true if the Brew component is active
#[inline]
// Was: Prüft, ob active zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_active(config: &SharedConfig) -> bool {
    config.config().brew.is_some() || config.config().brew2.is_some()
}

#[inline]
// Was: Prüft, ob active for entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_active_for_entity(config: &SharedConfig, entity: TetraEntity) -> bool {
    brew_config_for_entity(config, entity).is_some()
}

/// Returns true if the SDS over Brew feature is enabled
#[inline]
// Was: Führt den Arbeitsschritt `feature_sds_enabled` für feature TETRA-Kurznachricht (SDS) enabled aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn feature_sds_enabled(config: &SharedConfig) -> bool {
    BREW_ENTITIES
        .iter()
        .copied()
        .any(|entity| feature_sds_enabled_for_entity(config, entity))
}

#[inline]
// Was: Führt den Arbeitsschritt `feature_sds_enabled_for_entity` für feature TETRA-Kurznachricht (SDS) enabled for entity aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn feature_sds_enabled_for_entity(config: &SharedConfig, entity: TetraEntity) -> bool {
    brew_config_for_entity(config, entity).is_some_and(|brew| brew.feature_sds_enabled)
}

/// Returns true if the configured Brew server is TetraPack (core.tetrapack.online)
// Was: Prüft, ob tetrapack server zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
fn is_tetrapack_server(brew_config: &CfgBrew) -> bool {
    brew_config.host == "core.tetrapack.online"
}

// Was: Prüft, ob pbx Gateway Teilnehmerkennung (ISSI) zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
fn is_pbx_gateway_issi(brew_config: &CfgBrew, issi: u32) -> bool {
    brew_config
        .pbx_gateway_issis
        .as_ref()
        .is_some_and(|allowed| allowed.contains(&issi))
}

#[inline]
// Was: Prüft, ob Brew-Verbindung local Teilnehmerkennung (ISSI) allowed for entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_local_issi_allowed_for_entity(config: &SharedConfig, entity: TetraEntity, issi: u32) -> bool {
    brew_config_for_entity(config, entity).is_some_and(|brew| brew.local_issi_allowed(issi))
}

/// Pick the one Brew entity that may represent this local TETRA ISSI.
///
/// Returning `None` on ambiguity is deliberate: a local terminal must never be registered or
/// forwarded through two Brew backhauls at the same time.
// Was: Diese Funktion leitet entity for local Teilnehmerkennung (ISSI).
// Warum: Nachrichten und Daten gelangen dadurch nachvollziehbar an das richtige Ziel.
pub fn route_entity_for_local_issi(config: &SharedConfig, issi: u32) -> Option<TetraEntity> {
    let mut routed = None;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for entity in BREW_ENTITIES {
        if is_brew_local_issi_allowed_for_entity(config, entity, issi) {
            if routed.is_some() {
                return None;
            }
            routed = Some(entity);
        }
    }
    routed
}

/// Determine if a given GSSI should be routed over Brew, or is restricted to local handling
// Was: Prüft, ob Brew-Verbindung Gruppenkennung (GSSI) routable zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_gssi_routable(config: &SharedConfig, ssi: u32) -> bool {
    BREW_ENTITIES
        .iter()
        .copied()
        .any(|entity| is_brew_gssi_routable_for_entity(config, entity, ssi))
}

// Was: Prüft, ob Brew-Verbindung Gruppenkennung (GSSI) routable for entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_gssi_routable_for_entity(config: &SharedConfig, entity: TetraEntity, ssi: u32) -> bool {
    let Some(brew_config) = brew_config_for_entity(config, entity) else {
        return false;
    };
    if config.config().cell.local_ssi_ranges.contains(ssi) {
        // Range overridden as local
        return false;
    }

    // Check if whitelist is present and if so, check
    if let Some(whitelist) = &brew_config.whitelisted_ssis {
        if whitelist.contains(&ssi) {
            // Range explicitly whitelisted for routing to Brew
            return true;
        } else {
            // Not in whitelist - block routing to Brew
            return false;
        }
    }

    // No whitelist present, default to allow
    true
}

/// Determine whether a Brew-originated INBOUND call/SDS for a given GSSI may be admitted locally.
///
/// This is deliberately weaker than [`is_brew_gssi_routable`]. That predicate governs OUTBOUND
/// forwarding of *local* traffic to Brew and therefore honours `whitelisted_ssis` — which is
/// documented as "allow only calls for selected SSIs to be **forwarded through Brew**", i.e. an
/// outbound concept. Applying the whitelist to inbound admission wrongly blocks a bridging/foreign
/// GSSI that is absent from the whitelist (FH-FEAT-032 R3): a network call legitimately arriving
/// from an authenticated Brew connection must still reach the local MS camped on that group.
///
/// The `local_ssi_ranges` override is still honoured — those ranges are documented as local-only
/// ("Incoming brew traffic on these ranges will also be rejected"), so inbound traffic to them stays
/// rejected.
#[inline]
// Was: Prüft, ob Brew-Verbindung inbound allowed zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_inbound_allowed(config: &SharedConfig, ssi: u32) -> bool {
    is_active(config) && !config.config().cell.local_ssi_ranges.contains(ssi)
}

#[inline]
// Was: Prüft, ob Brew-Verbindung inbound allowed for entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_inbound_allowed_for_entity(config: &SharedConfig, entity: TetraEntity, ssi: u32) -> bool {
    is_active_for_entity(config, entity) && !config.config().cell.local_ssi_ranges.contains(ssi)
}

/// Determine if a given ISSI should be sent to the Brew server.
/// On TetraPack, subscriber ISSIs must be 7 digits (1_000_000..=9_999_999).
/// Special service ISSIs (e.g. 600 echo, short numbers) are always forwarded to Brew —
/// TetraPack Core handles them internally; blocking them here causes "Service Denied".
// Was: Prüft, ob Brew-Verbindung Teilnehmerkennung (ISSI) routable zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_issi_routable(config: &SharedConfig, issi: u32) -> bool {
    BREW_ENTITIES
        .iter()
        .copied()
        .any(|entity| is_brew_issi_routable_for_entity(config, entity, issi))
}

// Was: Prüft, ob Brew-Verbindung Teilnehmerkennung (ISSI) routable for entity zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_brew_issi_routable_for_entity(config: &SharedConfig, entity: TetraEntity, issi: u32) -> bool {
    let Some(brew_config) = brew_config_for_entity(config, entity) else {
        return false;
    };

    if is_tetrapack_server(&brew_config) {
        // 7-digit subscriber ISSIs are always routable.
        // Short ISSIs (< 1_000_000) are service numbers handled by TetraPack Core —
        // let them through so the core can respond (echo test 600, etc.)
        (issi >= 1_000_000 && issi <= 9_999_999) || issi < 1_000_000 || is_pbx_gateway_issi(&brew_config, issi)
    } else {
        true
    }
}
