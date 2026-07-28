// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Legacy WAP delivery through SDS Type 4.
//!
//! EN 300 392-2 assigns protocol identifier 0x04 to WAP/WDP carried directly
//! in SDS Type 4 and 0x84 to WAP carried through the SDS-TL transfer service.
//! The latter therefore includes an SDS-TL transfer header after the PID.

use bitcode::{Decode, Encode};
use serde::{Deserialize, Serialize};

// Was: Legt den festen Wert `WAP_WDP_PROTOCOL_ID` für WAP-Dienst wdp protocol Kennung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const WAP_WDP_PROTOCOL_ID: u8 = 0x04;
// Was: Legt den festen Wert `WAP_SDS_TL_PROTOCOL_ID` für WAP-Dienst TETRA-Kurznachricht (SDS) tl protocol Kennung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const WAP_SDS_TL_PROTOCOL_ID: u8 = 0x84;
// Was: Legt den festen Wert `SDS_TYPE4_MAX_BYTES` für TETRA-Kurznachricht (SDS) type4 max bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SDS_TYPE4_MAX_BYTES: usize = 255;
// Was: Legt den festen Wert `SDS_TL_NO_REPORT_FLAGS` für TETRA-Kurznachricht (SDS) tl no report flags fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const SDS_TL_NO_REPORT_FLAGS: u8 = 0x00;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Encode, Decode, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
// Was: Listet die möglichen Varianten für legacy WAP-Dienst transport auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LegacyWapTransport {
    /// PID 0x04 followed by the application WAP/WML payload.
    Wdp,
    /// PID 0x84 followed by a minimal SDS-TL TRANSFER header and payload.
    SdsTl,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für legacy WAP-Dienst error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LegacyWapError {
    EmptyPayload,
    PayloadTooLarge { len: usize, max: usize },
}

// Was: Diese Funktion erstellt type4 payload.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_type4_payload(
    payload: &[u8],
    transport: LegacyWapTransport,
    message_reference: u8,
) -> Result<Vec<u8>, LegacyWapError> {
    if payload.is_empty() {
        return Err(LegacyWapError::EmptyPayload);
    }
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let header_len = match transport {
        LegacyWapTransport::Wdp => 1,
        LegacyWapTransport::SdsTl => 3,
    };
    let total = header_len + payload.len();
    if total > SDS_TYPE4_MAX_BYTES {
        return Err(LegacyWapError::PayloadTooLarge {
            len: total,
            max: SDS_TYPE4_MAX_BYTES,
        });
    }
    let mut out = Vec::with_capacity(total);
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match transport {
        LegacyWapTransport::Wdp => out.push(WAP_WDP_PROTOCOL_ID),
        LegacyWapTransport::SdsTl => {
            out.push(WAP_SDS_TL_PROTOCOL_ID);
            out.push(SDS_TL_NO_REPORT_FLAGS);
            out.push(message_reference);
        }
    }
    out.extend_from_slice(payload);
    Ok(out)
}

// Was: Diese Funktion erzeugt compact wml.
// Warum: Darstellung und Fachdaten bleiben dadurch voneinander getrennt.
pub fn render_compact_wml(title: &str, message: &str, target_url: Option<&str>) -> String {
    let title = escape_xml(title.trim());
    let message = escape_xml(message.trim());
    let link = target_url
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(|value| format!("<br/><a href=\"{}\">Oeffnen</a>", escape_xml(value)))
        .unwrap_or_default();
    format!(
        "<?xml version=\"1.0\"?><!DOCTYPE wml PUBLIC \"-//WAPFORUM//DTD WML 1.1//EN\" \"http://www.wapforum.org/DTD/wml_1.1.xml\"><wml><card title=\"{title}\"><p><b>{title}</b><br/>{message}{link}</p></card></wml>"
    )
}

/// Render a WML card and shrink the human text until the complete SDS Type 4
/// payload fits. XML framing and the optional URL are never cut mid-token.
// Was: Diese Funktion erstellt compact wml type4.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_compact_wml_type4(
    title: &str,
    message: &str,
    target_url: Option<&str>,
    transport: LegacyWapTransport,
    message_reference: u8,
) -> Result<Vec<u8>, LegacyWapError> {
    let chars = message.chars().collect::<Vec<_>>();
    // Validate the fixed XML/title/URL framing before searching the largest
    // message prefix. This also rejects a URL/title combination that can never
    // fit, instead of spinning through the entire message.
    let empty_wml = render_compact_wml(title, "", target_url);
    build_type4_payload(empty_wml.as_bytes(), transport, message_reference)?;

    let mut low = 0usize;
    let mut high = chars.len();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while low < high {
        let mid = low + (high - low + 1) / 2;
        let current = chars[..mid].iter().collect::<String>();
        let wml = render_compact_wml(title, &current, target_url);
        if build_type4_payload(wml.as_bytes(), transport, message_reference).is_ok() {
            low = mid;
        } else {
            high = mid - 1;
        }
    }
    let current = chars[..low].iter().collect::<String>();
    let wml = render_compact_wml(title, &current, target_url);
    build_type4_payload(wml.as_bytes(), transport, message_reference)
}

// Was: Führt den Arbeitsschritt `escape_xml` für escape xml aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn escape_xml(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for ch in value.chars() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match ch {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&apos;"),
            _ => out.push(ch),
        }
    }
    out
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `direct_wdp_starts_with_pid_04` für direct wdp starts with pid 04 aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn direct_wdp_starts_with_pid_04() {
        let payload = build_type4_payload(b"<wml/>", LegacyWapTransport::Wdp, 7).unwrap();
        assert_eq!(payload[0], WAP_WDP_PROTOCOL_ID);
        assert_eq!(&payload[1..], b"<wml/>");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `sds_tl_has_transfer_header` für TETRA-Kurznachricht (SDS) tl has transfer header aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn sds_tl_has_transfer_header() {
        let payload = build_type4_payload(b"<wml/>", LegacyWapTransport::SdsTl, 0x42).unwrap();
        assert_eq!(&payload[..3], &[WAP_SDS_TL_PROTOCOL_ID, SDS_TL_NO_REPORT_FLAGS, 0x42]);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `compact_renderer_never_exceeds_type4_limit` für compact renderer never exceeds type4 limit aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn compact_renderer_never_exceeds_type4_limit() {
        let message = "A".repeat(1000);
        let payload = build_compact_wml_type4(
            "NetCore",
            &message,
            Some("http://10.0.0.1:9200/"),
            LegacyWapTransport::Wdp,
            1,
        )
        .unwrap();
        assert!(payload.len() <= SDS_TYPE4_MAX_BYTES);
    }
}
