// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Dashboard-side persistence and helpers for Telegram alerts.
//!
//! Telegram's semantic TOML editor preserves image-generated quoted headers,
//! unrelated settings and comments, and publishes edits atomically. Helpers
//! expose setup gaps and mask the bot token before responding to the browser.

use tetra_config::bluestation::TelegramRuntimeOverride;

/// Configuration gaps are setup information, not a failed bot connection.
pub fn setup_required(token: &str, chat_ids: &[i64]) -> Vec<&'static str> {
    let mut missing = Vec::new();
    if token.trim().is_empty() {
        missing.push("bot_token");
    }
    if chat_ids.is_empty() {
        missing.push("chat_ids");
    }
    missing
}

pub fn missing_token_response() -> String {
    serde_json::json!({
        "ok": false,
        "code": "token_missing",
        "error": "Kein Bot-Token hinterlegt. Trage den Token von BotFather ein und speichere die Einstellungen."
    })
    .to_string()
}

pub fn token_acceptable(t: &str) -> bool {
    t.is_empty() || (t.contains(':') && t.chars().all(|c| !c.is_whitespace() && !c.is_control()))
}

/// Empty fields preserve the stored secret; display masks never become tokens.
pub fn resolve_token(incoming: Option<&str>, current: &str) -> String {
    match incoming.map(str::trim) {
        Some(value) if !value.is_empty() && !value.contains('…') && !value.chars().all(|c| c == '•') => value.to_string(),
        _ => current.to_string(),
    }
}

/// Mask a bot token for display: keep the numeric bot id and the last few chars, hide the rest.
/// Returns an empty string for an empty token. e.g. "123456:ABCdef...WXYZ" → "123456:AB…WXYZ".
// Was: Führt den Arbeitsschritt `mask_token` für mask Zugriffsschlüssel aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn mask_token(token: &str) -> String {
    let token = token.trim();
    if token.is_empty() {
        return String::new();
    }
    let chars: Vec<char> = token.chars().collect();
    if chars.len() <= 10 {
        return "•".repeat(chars.len());
    }
    let head: String = chars[..8].iter().collect();
    let tail: String = chars[chars.len() - 4..].iter().collect();
    format!("{head}…{tail}")
}

/// JSON-escape a string for manual JSON assembly. Unlike a bare quote/backslash replace, this
/// also escapes control characters (newline, tab, etc.) — Telegram chat titles and reqwest error
/// strings are external data that can contain them, and a raw control char would make the
/// hand-assembled JSON response unparseable in the browser.
// Was: Führt den Arbeitsschritt `json_escape` für JSON-Daten escape aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn json_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len() + 2);
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for c in s.chars() {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match c {
            '\\' => out.push_str("\\\\"),
            '"' => out.push_str("\\\""),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '\u{08}' => out.push_str("\\b"),
            '\u{0C}' => out.push_str("\\f"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out
}

/// Update Telegram settings by the semantic TOML key, including quoted image
/// headers. Keep all other fields (such as alert_health), comments and tables.
pub fn write_telegram_to_toml(config_path: &str, ov: &TelegramRuntimeOverride) -> std::io::Result<()> {
    use toml_edit::{Array, Item, Table, Value};
    let _guard = super::config_edit::CONFIG_EDIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original = std::fs::read_to_string(config_path)?;
    let invalid = |reason: String| std::io::Error::new(std::io::ErrorKind::InvalidData, reason);
    // Parser diagnostics may quote a token-bearing source line. Return a
    // generic reason rather than leaking config secrets into HTTP/log errors.
    let mut doc = super::config_edit::document(&original)
        .map_err(|_| invalid("Die bestehende TOML-Konfiguration ist ungültig; keine Änderungen gespeichert".into()))?;
    if !doc.contains_key("telegram_alerts") {
        doc.insert("telegram_alerts", Item::Table(Table::new()));
    }
    let table = doc
        .get_mut("telegram_alerts")
        .and_then(Item::as_table_like_mut)
        .ok_or_else(|| invalid("telegram_alerts must be a TOML table".into()))?;
    for (key, value) in [
        ("enabled", ov.enabled),
        ("alert_connect", ov.alert_connect),
        ("alert_disconnect", ov.alert_disconnect),
        ("alert_t351", ov.alert_t351),
        ("alert_lip", ov.alert_lip),
        ("alert_backhaul", ov.alert_backhaul),
        ("alert_critical_logs", ov.alert_critical_logs),
        ("alert_brew_register", ov.alert_brew_register),
    ] {
        super::config_edit::set_scalar(table, key, value.into());
    }
    super::config_edit::set_scalar(table, "bot_token", ov.bot_token.clone().into());
    super::config_edit::set_scalar(table, "brew_register_prefix", ov.brew_register_prefix.clone().into());
    for (key, values) in [
        ("chat_ids", ov.chat_ids.clone()),
        (
            "brew_register_issi_whitelist",
            ov.brew_register_issi_whitelist.iter().map(|value| i64::from(*value)).collect(),
        ),
        (
            "brew_register_issi_blacklist",
            ov.brew_register_issi_blacklist.iter().map(|value| i64::from(*value)).collect(),
        ),
    ] {
        let mut array = Array::new();
        for value in values {
            array.push(value);
        }
        super::config_edit::set_scalar(table, key, Value::Array(array));
    }
    let replacement = doc.to_string();
    // Check the entire candidate before atomic publication of that exact content.
    super::config_edit::document(&replacement)
        .map_err(|_| invalid("Die Telegram-Änderung erzeugt keine gültige TOML-Konfiguration".into()))?;
    super::config_edit::write_checked(config_path, &original, &replacement)
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    fn missing_settings_are_setup_gaps_without_secrets() {
        assert_eq!(setup_required("", &[]), vec!["bot_token", "chat_ids"]);
        assert_eq!(setup_required("123:secret", &[]), vec!["chat_ids"]);
        assert!(setup_required("123:secret", &[42]).is_empty());
        let response: serde_json::Value = serde_json::from_str(&missing_token_response()).unwrap();
        assert_eq!(response["code"], "token_missing");
        assert!(response["error"].as_str().unwrap().contains("BotFather"));
        assert!(!response.to_string().contains("123:secret"));
    }

    #[test]
    fn blank_and_masked_inputs_preserve_token_but_fresh_input_replaces_it() {
        for incoming in [None, Some(""), Some("  "), Some("123456:A…WXYZ"), Some("•••••")] {
            assert_eq!(resolve_token(incoming, "123:stored"), "123:stored");
        }
        assert_eq!(resolve_token(Some(" 456:fresh "), "123:stored"), "456:fresh");
        assert!(!token_acceptable("123:bad\nvalue"));
        assert!(!token_acceptable("invalid"));
    }

    // Was: Führt den Arbeitsschritt `ov` für ov aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn ov() -> TelegramRuntimeOverride {
        TelegramRuntimeOverride {
            enabled: true,
            bot_token: "123456:ABC-DEF".to_string(),
            chat_ids: vec![987654321, -1001234567890],
            alert_connect: true,
            alert_disconnect: false,
            alert_t351: true,
            alert_lip: true,
            alert_backhaul: false,
            alert_critical_logs: true,
            alert_brew_register: true,
            brew_register_prefix: "Brew REGISTER".to_string(),
            brew_register_issi_whitelist: [2632585].into_iter().collect(),
            brew_register_issi_blacklist: [9999].into_iter().collect(),
        }
    }

    #[test]
    // Was: Führt den Arbeitsschritt `mask_short_and_long` für mask short and long aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn mask_short_and_long() {
        assert_eq!(mask_token(""), "");
        assert_eq!(mask_token("short"), "•••••");
        assert_eq!(mask_token("123456:ABCdefGHIjklMNOpqrsWXYZ"), "123456:A…WXYZ");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `json_escape_handles_control_chars` für JSON-Daten escape handles Steuerung chars aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn json_escape_handles_control_chars() {
        let out = json_escape("line1\nline2\t\"q\"\\x");
        assert_eq!(out, "line1\\nline2\\t\\\"q\\\"\\\\x");
        // A raw control byte becomes a \u escape, never a literal that breaks JSON.
        assert_eq!(json_escape("\u{1}"), "\\u0001");
        // The escaped output parses back as the original string.
        let wrapped = format!("\"{}\"", json_escape("a\nb\"c"));
        let v: serde_json::Value = serde_json::from_str(&wrapped).unwrap();
        assert_eq!(v.as_str().unwrap(), "a\nb\"c");
    }

    #[test]
    // Was: Diese Funktion schreibt toml replace section.
    // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
    fn write_toml_replace_section() {
        let cfg = "[cell]\nfoo = 1\n\n[telegram_alerts]\nenabled = false\nbot_token = \"old\"\n\n[security]\nbar = 2\n";
        let dir = std::env::temp_dir();
        let path = dir.join("fs_tg_test_replace.toml");
        std::fs::write(&path, cfg).unwrap();
        write_telegram_to_toml(path.to_str().unwrap(), &ov()).unwrap();
        let out = std::fs::read_to_string(&path).unwrap();
        assert!(out.contains("enabled = true"));
        assert!(out.contains("bot_token = \"123456:ABC-DEF\""));
        assert!(out.contains("chat_ids = [987654321, -1001234567890]"));
        assert!(out.contains("alert_disconnect = false"));
        assert!(out.contains("alert_brew_register = true"));
        assert!(out.contains("brew_register_issi_whitelist = [2632585]"));
        assert!(out.contains("brew_register_issi_blacklist = [9999]"));
        // Other sections preserved, old value gone.
        assert!(out.contains("[cell]"));
        assert!(out.contains("[security]"));
        assert!(out.contains("bar = 2"));
        assert!(!out.contains("bot_token = \"old\""));
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    // Was: Diese Funktion schreibt toml append when missing.
    // Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
    fn write_toml_append_when_missing() {
        let cfg = "[cell]\nfoo = 1\n";
        let dir = std::env::temp_dir();
        let path = dir.join("fs_tg_test_append.toml");
        std::fs::write(&path, cfg).unwrap();
        write_telegram_to_toml(path.to_str().unwrap(), &ov()).unwrap();
        let out = std::fs::read_to_string(&path).unwrap();
        assert!(out.contains("[telegram_alerts]"));
        assert!(out.contains("[cell]"));
        let _ = std::fs::remove_file(&path);
    }

    /// The written section must round-trip through the real config parser.
    #[test]
    // Was: Führt den Arbeitsschritt `written_section_parses_back` für written section parses back aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn written_section_parses_back() {
        let cfg = "config_version = \"0.6\"\nstack_mode = \"Bs\"\n\n[phy_io]\nbackend = \"None\"\n\n[net_info]\nmcc = 901\nmnc = 9999\n\n[cell_info]\nmain_carrier = 1584\nfreq_band = 4\nfreq_offset = 0\nduplex_spacing = 4\nreverse_operation = false\nlocation_area = 1\n";
        let dir = std::env::temp_dir();
        let path = dir.join("fs_tg_test_roundtrip.toml");
        // Use valid paths on both Windows and Linux rather than depending on
        // Linux-only media defaults in an unrelated section of the parser.
        let cfg = format!(
            "{cfg}\n[audio_player]\ndirectory = {}\ncache_directory = {}\n\n[tts]\ncache_directory = {}\ntemplate_directory = {}\n",
            toml::Value::String(dir.join("tg-audio").to_string_lossy().into_owned()),
            toml::Value::String(dir.join("tg-cache").to_string_lossy().into_owned()),
            toml::Value::String(dir.join("tg-tts-cache").to_string_lossy().into_owned()),
            toml::Value::String(dir.join("tg-tts-templates").to_string_lossy().into_owned()),
        );
        std::fs::write(&path, cfg).unwrap();
        write_telegram_to_toml(path.to_str().unwrap(), &ov()).unwrap();
        let parsed = tetra_config::bluestation::parsing::from_file(path.to_str().unwrap()).expect("written telegram section must parse");
        let tg = parsed.telegram.expect("telegram present");
        assert!(tg.enabled);
        assert_eq!(tg.chat_ids, vec![987654321, -1001234567890]);
        assert!(!tg.alert_disconnect);
        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn prefix_with_newline_and_quotes_cannot_corrupt_toml() {
        let mut values = ov();
        values.brew_register_prefix = "Station \"A\"\nZeile zwei".into();
        let path = std::env::temp_dir().join("fs_tg_test_escaped_prefix.toml");
        std::fs::write(&path, "[other]\nkeep = true\n").unwrap();
        write_telegram_to_toml(path.to_str().unwrap(), &values).unwrap();
        let out = std::fs::read_to_string(&path).unwrap();
        let parsed: toml::Value = toml::from_str(&out).unwrap();
        assert_eq!(
            parsed["telegram_alerts"]["brew_register_prefix"].as_str(),
            Some(values.brew_register_prefix.as_str())
        );
        assert_eq!(parsed["other"]["keep"].as_bool(), Some(true));
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_file(format!("{}.telegram.bak", path.display()));
    }

    #[test]
    fn image_quoted_header_is_updated_once_and_preserves_comments_and_health() {
        let path = std::env::temp_dir().join("fs_tg_test_quoted_image.toml");
        let original = "# Image configuration\n[\"telegram_alerts\"]\nenabled = false # operator switch\nbot_token = \"old\"\nalert_health = false # keep this value\n\n[\"cell_info\"]\nmain_carrier = 720 # RF setting\n";
        std::fs::write(&path, original).unwrap();
        for _ in 0..2 {
            write_telegram_to_toml(path.to_str().unwrap(), &ov()).unwrap();
            let out = std::fs::read_to_string(&path).unwrap();
            let parsed: toml::Value = toml::from_str(&out).expect("quoted table must not be duplicated");
            assert_eq!(parsed["telegram_alerts"]["enabled"].as_bool(), Some(true));
            assert_eq!(parsed["telegram_alerts"]["bot_token"].as_str(), Some("123456:ABC-DEF"));
            assert_eq!(parsed["telegram_alerts"]["alert_health"].as_bool(), Some(false));
            assert_eq!(parsed["cell_info"]["main_carrier"].as_integer(), Some(720));
            assert_eq!(out.matches("[\"telegram_alerts\"]").count(), 1);
            assert!(out.contains("# operator switch"));
            assert!(out.contains("# keep this value"));
            assert!(out.contains("# RF setting"));
        }
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn invalid_source_is_not_modified_or_disclosed_in_errors() {
        let path = std::env::temp_dir().join("fs_tg_test_invalid_source.toml");
        let original = "[\"telegram_alerts\"]\nbot_token = \"123:secret-never-in-errors\n";
        std::fs::write(&path, original).unwrap();
        let error = write_telegram_to_toml(path.to_str().unwrap(), &ov()).unwrap_err();
        assert_eq!(error.kind(), std::io::ErrorKind::InvalidData);
        assert!(!error.to_string().contains("secret-never-in-errors"));
        assert_eq!(std::fs::read_to_string(&path).unwrap(), original);
        std::fs::remove_file(path).unwrap();
    }
}
