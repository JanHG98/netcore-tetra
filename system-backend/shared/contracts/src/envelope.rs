// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für envelope.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
// Was: Bündelt die zusammengehörigen Werte für API-Schnittstelle version in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct ApiVersion(String);

// Was: Implementiert das zugehörige Verhalten für `ApiVersion`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl ApiVersion {
    // Was: Legt den festen Wert `V1` für v1 fest.
    // Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
    pub const V1: &'static str = "netcore.v1";

    // Was: Führt den Arbeitsschritt `v1` für v1 aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn v1() -> Self {
        Self(Self::V1.to_owned())
    }

    // Was: Wandelt den vorhandenen Wert in str um oder stellt ihn in dieser Form bereit.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

// Was: Implementiert das zugehörige Verhalten für `Default for ApiVersion`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for ApiVersion {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self::v1()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für Nachricht kind auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum MessageKind {
    Command,
    Event,
    Query,
    Reply,
    Snapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für delivery semantics auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum DeliverySemantics {
    AtMostOnce,
    AtLeastOnce,
    IdempotentAtLeastOnce,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
// Was: Bündelt die zusammengehörigen Werte für trace Kontext in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TraceContext {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub span_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für envelope meta in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct EnvelopeMeta {
    pub message_id: Uuid,
    #[serde(default)]
    pub api_version: ApiVersion,
    pub kind: MessageKind,
    pub source: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,
    pub created_at: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub correlation_id: Option<Uuid>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub causation_id: Option<Uuid>,
    #[serde(default)]
    pub trace: TraceContext,
    pub delivery: DeliverySemantics,
    #[serde(default)]
    pub idempotency_key: Option<String>,
}

// Was: Implementiert das zugehörige Verhalten für `EnvelopeMeta`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EnvelopeMeta {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(kind: MessageKind, source: impl Into<String>) -> Self {
        Self {
            message_id: Uuid::new_v4(),
            api_version: ApiVersion::v1(),
            kind,
            source: source.into(),
            destination: None,
            created_at: Utc::now(),
            expires_at: None,
            correlation_id: None,
            causation_id: None,
            trace: TraceContext::default(),
            delivery: DeliverySemantics::IdempotentAtLeastOnce,
            idempotency_key: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
// Was: Bündelt die zusammengehörigen Werte für envelope in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Envelope<T> {
    pub meta: EnvelopeMeta,
    pub payload_type: String,
    pub payload: T,
}

// Was: Implementiert das zugehörige Verhalten für `Envelope<T>`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl<T> Envelope<T> {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(
        kind: MessageKind,
        source: impl Into<String>,
        payload_type: impl Into<String>,
        payload: T,
    ) -> Self {
        Self {
            meta: EnvelopeMeta::new(kind, source),
            payload_type: payload_type.into(),
            payload,
        }
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use serde_json::json;

    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `envelope_uses_v1_contract_by_default` für envelope uses v1 contract by default aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn envelope_uses_v1_contract_by_default() {
        let envelope = Envelope::new(MessageKind::Event, "group-core", "group.affiliation", json!({"gssi": 2000}));
        assert_eq!(envelope.meta.api_version.as_str(), ApiVersion::V1);
        assert_eq!(envelope.meta.delivery, DeliverySemantics::IdempotentAtLeastOnce);
    }
}
