// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Alert message formatting.
//!
//! Every alert is a compact, professional HTML message (Telegram `parse_mode=HTML`): a bold
//! titled header with an at-a-glance emoji, a couple of detail lines, then a footer that names
//! the station and stamps the local time. Dynamic fields are HTML-escaped.

use tetra_config::bluestation::SharedConfig;

/// Cached, human-readable identity of this station, shown in every alert footer.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für station info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct StationInfo {
    /// e.g. "FlowStation · MCC 901 / MNC 9999 · LA 1 · CC 3"
    pub label: String,
}

// Was: Implementiert das zugehörige Verhalten für `StationInfo`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl StationInfo {
    /// Build the label once from immutable config (network + cell identity + optional name).
    // Was: Wandelt Eingangsdaten in Konfiguration um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn from_config(cfg: &SharedConfig) -> Self {
        let c = cfg.config();
        let name = c.service_name.clone().unwrap_or_else(|| "FlowStation".to_string());
        // Bound the (operator-controlled) name so the footer can't blow past Telegram's limit.
        let name = truncate_chars(&name, STATION_LABEL_MAX_CHARS);
        let label = format!(
            "{} · MCC {} / MNC {} · LA {} · CC {}",
            name, c.net.mcc, c.net.mnc, c.cell.location_area, c.cell.colour_code
        );
        StationInfo { label }
    }
}

/// Minimal HTML escaping for Telegram's HTML parse mode (only &, <, > are special).
// Was: Führt den Arbeitsschritt `escape_html` für escape html aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn escape_html(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// Truncate to at most `max` characters, always on a char boundary. Unlike `String::truncate`
/// (which panics if the byte index is mid-character), this is safe for arbitrary UTF-8 — log
/// lines and decoded SDS/LIP text can contain multibyte characters and emoji.
// Was: Führt den Arbeitsschritt `truncate_chars` für truncate chars aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn truncate_chars(s: &str, max: usize) -> String {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match s.char_indices().nth(max) {
        Some((idx, _)) => s[..idx].to_string(),
        None => s.to_string(),
    }
}

/// Telegram rejects messages longer than 4096 characters; stay safely under it.
// Was: Legt den festen Wert `TELEGRAM_MAX_CHARS` für telegram max chars fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TELEGRAM_MAX_CHARS: usize = 4000;
/// Cap on the station label length (mostly bounds an over-long configured service_name).
// Was: Legt den festen Wert `STATION_LABEL_MAX_CHARS` für station label max chars fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const STATION_LABEL_MAX_CHARS: usize = 80;

/// Assemble a framed alert: emoji + bold title, detail lines, then the station/time footer.
// Was: Führt den Arbeitsschritt `frame` für Funkrahmen aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn frame(emoji: &str, title: &str, lines: &[String], station: &StationInfo) -> String {
    let now = chrono::Local::now().format("%Y-%m-%d %H:%M:%S");
    let mut out = format!("{emoji} <b>{}</b>\n", escape_html(title));
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for l in lines {
        out.push_str(l);
        out.push('\n');
    }
    out.push_str(&format!("🛰 <i>{}</i>\n", escape_html(&station.label)));
    out.push_str(&format!("🕒 {now}"));
    // Defensive backstop: every dynamic field is already individually bounded, so this only ever
    // trims a pathological input. Trimming on a char boundary keeps the payload valid UTF-8.
    if out.chars().count() > TELEGRAM_MAX_CHARS {
        out = truncate_chars(&out, TELEGRAM_MAX_CHARS);
    }
    out
}

/// A radio raised an emergency (emergency status PDU — pre-coded status Emergency).
// Was: Führt den Arbeitsschritt `emergency` für emergency aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn emergency(station: &StationInfo, source_issi: u32, dest_ssi: u32) -> String {
    frame(
        "🆘",
        "EMERGENCY",
        &[
            format!("From ISSI: <code>{source_issi}</code>"),
            format!("To: <code>{dest_ssi}</code>"),
        ],
        station,
    )
}

/// A radio attached to the cell.
// Was: Diese Funktion verbindet den vorgesehenen Arbeitsschritt.
// Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
pub fn connect(station: &StationInfo, issi: u32) -> String {
    frame("🟢", "Radio connected", &[format!("ISSI: <code>{issi}</code>")], station)
}

/// A radio detached / deregistered.
// Was: Diese Funktion trennt den vorgesehenen Arbeitsschritt.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn disconnect(station: &StationInfo, issi: u32) -> String {
    frame("🔴", "Radio disconnected", &[format!("ISSI: <code>{issi}</code>")], station)
}

/// A radio was dropped for not answering the periodic registration (T351).
// Was: Führt den Arbeitsschritt `t351_drop` für t351 drop aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn t351_drop(station: &StationInfo, issi: u32) -> String {
    frame(
        "📴",
        "Radio dropped (no T351 response)",
        &[
            format!("ISSI: <code>{issi}</code>"),
            "Reason: did not answer the periodic re-registration request.".to_string(),
        ],
        station,
    )
}

/// A radio beaconed its position over LIP/APRS. `text` is the best-effort decoded body (often
/// empty for binary LIP payloads).
// Was: Führt den Arbeitsschritt `lip_beacon` für lip beacon aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn lip_beacon(station: &StationInfo, source_issi: u32, dest_issi: u32, text: &str) -> String {
    let mut lines = vec![
        format!("From ISSI: <code>{source_issi}</code>"),
        format!("To: <code>{dest_issi}</code>"),
    ];
    let trimmed = text.trim();
    if trimmed.is_empty() {
        lines.push("Position: binary LIP payload (undecoded).".to_string());
    } else {
        let body = truncate_chars(trimmed, 200);
        lines.push(format!("Position: {}", escape_html(&body)));
    }
    frame("📍", "LIP/APRS position beacon", &lines, station)
}

/// The Brew/TetraPack backhaul connected or disconnected.
// Was: Führt den Arbeitsschritt `backhaul` für backhaul aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn backhaul(station: &StationInfo, connected: bool, server_version: u8) -> String {
    if connected {
        frame(
            "🛰️",
            "Brew backhaul connected",
            &[format!("Server version: v{server_version}")],
            station,
        )
    } else {
        frame(
            "🛰️",
            "Brew backhaul disconnected",
            &["Station running in fallback mode (local).".to_string()],
            station,
        )
    }
}

/// An external ISSI registered through a Brew/TetraPack backhaul.
// Was: Führt den Arbeitsschritt `brew_register` für Brew-Verbindung register aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn brew_register(station: &StationInfo, prefix: &str, source: &str, issi: u32) -> String {
    let prefix = truncate_chars(prefix.trim(), 32);
    let source = truncate_chars(source.trim(), 160);
    frame(
        "🟢",
        if prefix.is_empty() { "Brew REGISTER" } else { &prefix },
        &[
            format!("ISSI: <code>{issi}</code>"),
            format!("Source: <code>{}</code>", escape_html(&source)),
        ],
        station,
    )
}

/// One or more WARN/ERROR log lines, coalesced into a single message. `extra` is how many lines
/// were dropped beyond the ones shown.
// Was: Führt den Arbeitsschritt `critical_logs` für critical logs aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn critical_logs(station: &StationInfo, lines: &[(String, String)], extra: usize) -> String {
    let title = if lines.iter().any(|(lvl, _)| lvl == "ERROR") {
        "Station error"
    } else {
        "Station warning"
    };
    let mut body: Vec<String> = lines
        .iter()
        .map(|(lvl, msg)| {
            let icon = if lvl == "ERROR" { "⛔" } else { "⚠️" };
            let m = truncate_chars(msg.trim(), 220);
            format!("{icon} {}", escape_html(&m))
        })
        .collect();
    if extra > 0 {
        body.push(format!("… and {extra} more message(s)."));
    }
    frame("🚨", title, &body, station)
}

/// The "send test alert" button payload.
// Was: Prüft automatisch den Fall Nachricht.
// Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
pub fn test_message(station: &StationInfo) -> String {
    frame(
        "✅",
        "Telegram alert test",
        &["FlowStation alerts are configured correctly. You'll receive notifications here.".to_string()],
        station,
    )
}

/// A DAPNET message forwarded by the DAPNET worker.
// Was: Führt den Arbeitsschritt `dapnet` für dapnet aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn dapnet(station: &StationInfo, prefix: &str, callsign: &str, text: &str) -> String {
    let prefix = truncate_chars(prefix.trim(), 32);
    let callsign = truncate_chars(callsign.trim(), 64);
    let text = truncate_chars(text.trim(), 900);
    frame(
        "📟",
        if prefix.is_empty() { "DAPNET" } else { &prefix },
        &[
            format!("Callsign/recipient: <code>{}</code>", escape_html(&callsign)),
            format!("Message: {}", escape_html(&text)),
        ],
        station,
    )
}

/// Station health changed level. Lists the domains that are not Ok, plus the last action taken.
// Was: Führt den Arbeitsschritt `health` für health aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn health(station: &StationInfo, snap: &crate::health::HealthSnapshot) -> String {
    use crate::health::HealthLevel;
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let (emoji, title) = match snap.overall {
        HealthLevel::Ok => ("🟢", "Station healthy"),
        HealthLevel::Degraded => ("🟠", "Station degraded"),
        HealthLevel::Critical => ("🔴", "Station CRITICAL"),
    };
    let mut lines: Vec<String> = snap
        .domains
        .iter()
        .filter(|d| !matches!(d.level, HealthLevel::Ok))
        .map(|d| {
            format!(
                "⚠️ <b>{}</b> ({}): {}",
                escape_html(d.domain.as_str()),
                d.level.as_str(),
                escape_html(&truncate_chars(&d.detail, 120))
            )
        })
        .collect();
    if lines.is_empty() {
        lines.push("All domains nominal again.".to_string());
    }
    if let Some(action) = &snap.last_action {
        lines.push(format!("🔧 {}", escape_html(&truncate_chars(action, 160))));
    }
    frame(emoji, title, &lines, station)
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Führt den Arbeitsschritt `station` für station aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn station() -> StationInfo {
        StationInfo {
            label: "FlowStation · MCC 901 / MNC 9999 · LA 1 · CC 3".to_string(),
        }
    }

    #[test]
    // Was: Diese Funktion verbindet has title Teilnehmerkennung (ISSI) and footer.
    // Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    fn connect_has_title_issi_and_footer() {
        let m = connect(&station(), 2260571);
        assert!(m.contains("<b>Radio connected</b>"));
        assert!(m.contains("<code>2260571</code>"));
        assert!(m.contains("MCC 901"));
        assert!(m.starts_with("🟢"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `html_is_escaped_in_dynamic_fields` für html is escaped in dynamic fields aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn html_is_escaped_in_dynamic_fields() {
        // A log message with HTML-special chars must not break the markup.
        let m = critical_logs(&station(), &[("ERROR".to_string(), "panic in <Foo> & bar".to_string())], 0);
        assert!(m.contains("panic in &lt;Foo&gt; &amp; bar"));
        assert!(!m.contains("<Foo>"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `critical_logs_picks_error_title_and_counts_extra` für critical logs picks error title and counts und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn critical_logs_picks_error_title_and_counts_extra() {
        let lines = vec![("WARN".to_string(), "w1".to_string()), ("ERROR".to_string(), "e1".to_string())];
        let m = critical_logs(&station(), &lines, 3);
        assert!(m.contains("<b>Station error</b>"));
        assert!(m.contains("and 3 more"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `lip_handles_empty_and_textual_payload` für lip handles empty and textual payload aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn lip_handles_empty_and_textual_payload() {
        let empty = lip_beacon(&station(), 1, 2, "");
        assert!(empty.contains("binary"));
        let textual = lip_beacon(&station(), 1, 2, "4426.12N 02606.55E");
        assert!(textual.contains("4426.12N"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `dapnet_message_contains_prefix_callsign_and_text` für dapnet Nachricht contains prefix callsign and text aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn dapnet_message_contains_prefix_callsign_and_text() {
        let m = dapnet(&station(), "DAPNET", "DL1ABC", "Test <msg>");
        assert!(m.contains("<b>DAPNET</b>"));
        assert!(m.contains("<code>DL1ABC</code>"));
        assert!(m.contains("Test &lt;msg&gt;"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `truncate_never_panics_on_multibyte_boundary` für truncate never panics on multibyte boundary aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn truncate_never_panics_on_multibyte_boundary() {
        // The byte that would be cut lands mid-character — String::truncate would panic here.
        let s = "ab日".repeat(200); // 3-byte chars; byte 220 is mid-character
        let t = truncate_chars(&s, 220);
        assert_eq!(t.chars().count(), 220);
        // And the alert builders that truncate must not panic on a long multibyte payload/log.
        let _ = lip_beacon(&station(), 1, 2, &"😀".repeat(500));
        let _ = critical_logs(&station(), &[("ERROR".to_string(), "ăîâ".repeat(500))], 0);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `message_stays_under_telegram_limit_even_with_huge_inputs` für Nachricht stays under telegram limit even with und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn message_stays_under_telegram_limit_even_with_huge_inputs() {
        let big = StationInfo { label: "X".repeat(10_000) };
        let m = critical_logs(&big, &[("ERROR".to_string(), "e".repeat(10_000))], 0);
        assert!(m.chars().count() <= TELEGRAM_MAX_CHARS);
    }
}
