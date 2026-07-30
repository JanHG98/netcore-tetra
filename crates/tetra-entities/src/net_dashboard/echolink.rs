// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Dashboard-side persistence and helpers for EchoLink settings.

use tetra_config::bluestation::EcholinkRuntimeOverride;

// Was: Führt den Arbeitsschritt `mask_secret` für mask secret aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn mask_secret(secret: &str) -> String {
    crate::net_dashboard::dapnet::mask_secret(secret)
}

// Was: Führt den Arbeitsschritt `toml_escape` für toml escape aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn toml_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// Was: Führt den Arbeitsschritt `string_array_toml` für string array toml aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn string_array_toml(values: &[String]) -> String {
    values
        .iter()
        .map(|v| format!("\"{}\"", toml_escape(v)))
        .collect::<Vec<_>>()
        .join(", ")
}

// Was: Führt den Arbeitsschritt `u32_array_toml` für u32 array toml aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn u32_array_toml(values: &[u32]) -> String {
    values.iter().map(|v| v.to_string()).collect::<Vec<_>>().join(", ")
}

// Was: Führt den Arbeitsschritt `routes_toml` für routes toml aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn routes_toml(routes: &std::collections::BTreeMap<String, String>) -> String {
    routes
        .iter()
        .map(|(dial, target)| format!("\"{}\" = \"{}\"", toml_escape(dial), toml_escape(target)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Rewrite (or insert) the `[echolink]` section in the TOML file. A `.echolink.bak` backup is made.
// Was: Diese Funktion schreibt echolink to toml.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
pub fn write_echolink_to_toml(config_path: &str, ov: &EcholinkRuntimeOverride) -> std::io::Result<()> {
    let original = std::fs::read_to_string(config_path)?;
    let section = format!(
        "[echolink]\n\
         enabled = {}\n\
         callsign = \"{}\"\n\
         password = \"{}\"\n\
         location = \"{}\"\n\
         status_text = \"{}\"\n\
         directory_servers = [{}]\n\
         directory_port = {}\n\
         bind_addr = \"{}\"\n\
         audio_port = {}\n\
         control_port = {}\n\n\
         inbound_enabled = {}\n\
         outbound_enabled = {}\n\
         outbound_prefix = \"{}\"\n\
         strip_outbound_prefix = {}\n\
         service_numbers = [{}]\n\n\
         default_tetra_source_issi = {}\n\
         default_tetra_dest_issi = {}\n\
         default_tetra_dest_is_group = {}\n\
         routes = {{{}}}\n\
         allowed_callsigns = [{}]\n\
         allowed_node_ids = [{}]\n\
         auto_connect = \"{}\"\n\
         reconnect_interval_secs = {}\n\
         max_session_secs = {}",
        ov.enabled,
        toml_escape(&ov.callsign),
        toml_escape(&ov.password),
        toml_escape(&ov.location),
        toml_escape(&ov.status_text),
        string_array_toml(&ov.directory_servers),
        ov.directory_port,
        toml_escape(&ov.bind_addr),
        ov.audio_port,
        ov.control_port,
        ov.inbound_enabled,
        ov.outbound_enabled,
        toml_escape(&ov.outbound_prefix),
        ov.strip_outbound_prefix,
        string_array_toml(&ov.service_numbers),
        ov.default_tetra_source_issi,
        ov.default_tetra_dest_issi,
        ov.default_tetra_dest_is_group,
        routes_toml(&ov.routes),
        string_array_toml(&ov.allowed_callsigns),
        u32_array_toml(&ov.allowed_node_ids),
        toml_escape(&ov.auto_connect),
        ov.reconnect_interval_secs.max(1),
        ov.max_session_secs.max(1),
    );

    let lines: Vec<&str> = original.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 24);
    let mut i = 0;
    let mut replaced = false;

    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("[echolink]") {
            out.push(section.clone());
            replaced = true;
            i += 1;
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            while i < lines.len() {
                let t = lines[i].trim_start();
                if t.starts_with('[') && t.contains(']') {
                    break;
                }
                i += 1;
            }
            continue;
        }
        out.push(lines[i].to_string());
        i += 1;
    }

    if !replaced {
        if !out.is_empty() && !out.last().map(|l| l.is_empty()).unwrap_or(true) {
            out.push(String::new());
        }
        out.push(section);
    }

    let mut new_content = out.join("\n");
    if original.ends_with('\n') {
        new_content.push('\n');
    }

    let backup = format!("{config_path}.echolink.bak");
    let _ = std::fs::copy(config_path, &backup);
    std::fs::write(config_path, new_content)
}
