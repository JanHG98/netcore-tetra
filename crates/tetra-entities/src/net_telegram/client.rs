// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Minimal Telegram Bot API client (blocking).
//!
//! Three operations are all we need:
//! - [`TelegramClient::get_me`] — validate a bot token and learn the bot's @username.
//! - [`TelegramClient::get_updates`] — list the chats that recently messaged the bot, so the
//!   owner can pick their chat ID with one click instead of hunting for it.
//! - [`TelegramClient::send_message_html`] — deliver an alert (or a test message).
//!
//! Blocking HTTP, exactly like the built-in WX/METAR fetch. Always call from a worker thread
//! (the alerter) or the dashboard's per-connection thread — never from the stack loop.

use std::time::Duration;

// Was: Legt den festen Wert `API_BASE` für API-Schnittstelle base fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const API_BASE: &str = "https://api.telegram.org";
// Was: Legt den festen Wert `USER_AGENT` für Benutzer agent fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const USER_AGENT: &str = "FlowStation-TelegramAlerts";
// Was: Legt den festen Wert `HTTP_TIMEOUT` für HTTP timeout fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const HTTP_TIMEOUT: Duration = Duration::from_secs(10);

/// Bot identity returned by `getMe`.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für bot info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct BotInfo {
    /// Bot username without the leading '@' (e.g. "MyStationBot").
    pub username: String,
}

/// A chat that recently messaged the bot, surfaced by `getUpdates` for one-click pickup.
#[derive(Debug, Clone)]
// Was: Bündelt die zusammengehörigen Werte für detected chat in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct DetectedChat {
    /// Telegram chat ID (negative for groups/channels, positive for private chats).
    pub id: i64,
    /// Friendly label: a person's name, or a group/channel title, or "@username".
    pub name: String,
    /// Chat kind reported by Telegram: "private", "group", "supergroup", or "channel".
    pub kind: String,
}

// Was: Bündelt die zusammengehörigen Werte für telegram client in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TelegramClient {
    http: reqwest::blocking::Client,
}

// Was: Implementiert das zugehörige Verhalten für `Default for TelegramClient`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Default for TelegramClient {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default() -> Self {
        Self::new()
    }
}

// Was: Implementiert das zugehörige Verhalten für `TelegramClient`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TelegramClient {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        // If the client fails to build (should never happen with rustls-tls), fall back to a
        // default client so callers still get a clean per-request error instead of a panic.
        let http = reqwest::blocking::Client::builder()
            .timeout(HTTP_TIMEOUT)
            .user_agent(USER_AGENT)
            .build()
            .unwrap_or_default();
        Self { http }
    }

    // Was: Führt den Arbeitsschritt `method_url` für method url aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn method_url(token: &str, method: &str) -> String {
        format!("{API_BASE}/bot{token}/{method}")
    }

    /// Validate `token` and return the bot's identity. Err carries a human-readable reason.
    // Was: Diese Funktion liest me.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_me(&self, token: &str) -> Result<BotInfo, String> {
        let url = Self::method_url(token, "getMe");
        let json = self.get_json(&url)?;
        let result = ok_result(&json)?;
        let username = result.get("username").and_then(|v| v.as_str()).unwrap_or("").to_string();
        Ok(BotInfo { username })
    }

    /// List the distinct chats that have recently messaged the bot. Telegram buffers updates
    /// for ~24h when no webhook is set, so the owner messages the bot once, then clicks detect.
    // Was: Diese Funktion liest updates.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_updates(&self, token: &str) -> Result<Vec<DetectedChat>, String> {
        let url = format!("{}?timeout=0&limit=100", Self::method_url(token, "getUpdates"));
        let json = self.get_json(&url)?;
        let result = ok_result(&json)?;
        let updates = result.as_array().ok_or_else(|| "unexpected getUpdates response".to_string())?;

        let mut seen: Vec<DetectedChat> = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for upd in updates {
            // A chat can show up under several update kinds; check the common ones.
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for key in ["message", "edited_message", "channel_post", "my_chat_member"] {
                if let Some(chat) = upd.get(key).and_then(|m| m.get("chat"))
                    && let Some(detected) = chat_to_detected(chat)
                    && !seen.iter().any(|c| c.id == detected.id)
                {
                    seen.push(detected);
                }
            }
        }
        Ok(seen)
    }

    /// Send an HTML-formatted message to `chat_id`. Used for both alerts and the test button.
    // Was: Diese Funktion sendet Nachricht html.
    // Warum: Ausgehende Daten werden so einheitlich aufgebaut, geprüft und übertragen.
    pub fn send_message_html(&self, token: &str, chat_id: i64, html: &str) -> Result<(), String> {
        let url = Self::method_url(token, "sendMessage");
        let body = serde_json::json!({
            "chat_id": chat_id,
            "text": html,
            "parse_mode": "HTML",
            "disable_web_page_preview": true,
        });
        let resp = self
            .http
            .post(&url)
            .json(&body)
            .send()
            .map_err(|e| format!("request failed: {e}"))?;
        let json: serde_json::Value = resp.json().map_err(|e| format!("read failed: {e}"))?;
        ok_result(&json).map(|_| ())
    }

    // Was: Diese Funktion liest JSON-Daten.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    fn get_json(&self, url: &str) -> Result<serde_json::Value, String> {
        self.http
            .get(url)
            .send()
            .map_err(|e| format!("request failed: {e}"))?
            .json::<serde_json::Value>()
            .map_err(|e| format!("read failed: {e}"))
    }
}

/// Extract `result` from a `{ ok, result }` envelope, or turn `{ ok:false, description }` into an Err.
// Was: Führt den Arbeitsschritt `ok_result` für ok result aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn ok_result(json: &serde_json::Value) -> Result<serde_json::Value, String> {
    if json.get("ok").and_then(|v| v.as_bool()).unwrap_or(false) {
        Ok(json.get("result").cloned().unwrap_or(serde_json::Value::Null))
    } else {
        let desc = json
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown Telegram API error");
        Err(desc.to_string())
    }
}

/// Build a `DetectedChat` from a Telegram `chat` object, deriving a friendly display name.
// Was: Führt den Arbeitsschritt `chat_to_detected` für chat to detected aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn chat_to_detected(chat: &serde_json::Value) -> Option<DetectedChat> {
    let id = chat.get("id").and_then(|v| v.as_i64())?;
    let kind = chat.get("type").and_then(|v| v.as_str()).unwrap_or("").to_string();

    let title = chat.get("title").and_then(|v| v.as_str());
    let first = chat.get("first_name").and_then(|v| v.as_str());
    let last = chat.get("last_name").and_then(|v| v.as_str());
    let username = chat.get("username").and_then(|v| v.as_str());

    let name = if let Some(t) = title {
        t.to_string()
    } else if first.is_some() || last.is_some() {
        [first.unwrap_or(""), last.unwrap_or("")].join(" ").trim().to_string()
    } else if let Some(u) = username {
        format!("@{u}")
    } else {
        format!("Chat {id}")
    };

    Some(DetectedChat { id, name, kind })
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `method_url_builds` für method url builds aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn method_url_builds() {
        assert_eq!(
            TelegramClient::method_url("123:ABC", "getMe"),
            "https://api.telegram.org/bot123:ABC/getMe"
        );
    }

    #[test]
    // Was: Führt den Arbeitsschritt `ok_result_extracts` für ok result extracts aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn ok_result_extracts() {
        let j = serde_json::json!({"ok": true, "result": {"username": "Bot"}});
        let r = ok_result(&j).unwrap();
        assert_eq!(r.get("username").unwrap(), "Bot");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `ok_result_surfaces_error_description` für ok result surfaces error description aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn ok_result_surfaces_error_description() {
        let j = serde_json::json!({"ok": false, "description": "Unauthorized"});
        assert_eq!(ok_result(&j).unwrap_err(), "Unauthorized");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `detect_private_chat_name` für detect private chat name aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn detect_private_chat_name() {
        let chat = serde_json::json!({"id": 42, "type": "private", "first_name": "Ana", "last_name": "Pop"});
        let d = chat_to_detected(&chat).unwrap();
        assert_eq!(d.id, 42);
        assert_eq!(d.name, "Ana Pop");
        assert_eq!(d.kind, "private");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `detect_group_chat_title` für detect Gruppe chat title aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn detect_group_chat_title() {
        let chat = serde_json::json!({"id": -100123, "type": "supergroup", "title": "BTS Ops"});
        let d = chat_to_detected(&chat).unwrap();
        assert_eq!(d.id, -100123);
        assert_eq!(d.name, "BTS Ops");
    }

    #[test]
    // Was: Diese Funktion liest updates dedupes by chat.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    fn get_updates_dedupes_by_chat() {
        // Validate the dedup/parse logic against a representative getUpdates result without network.
        let json = serde_json::json!({
            "ok": true,
            "result": [
                {"update_id": 1, "message": {"chat": {"id": 7, "type": "private", "first_name": "Ed"}}},
                {"update_id": 2, "message": {"chat": {"id": 7, "type": "private", "first_name": "Ed"}}},
                {"update_id": 3, "message": {"chat": {"id": -9, "type": "group", "title": "Net"}}}
            ]
        });
        let result = ok_result(&json).unwrap();
        let mut seen: Vec<DetectedChat> = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for upd in result.as_array().unwrap() {
            if let Some(chat) = upd.get("message").and_then(|m| m.get("chat"))
                && let Some(d) = chat_to_detected(chat)
                && !seen.iter().any(|c| c.id == d.id)
            {
                seen.push(d);
            }
        }
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0].id, 7);
        assert_eq!(seen[1].id, -9);
    }
}
