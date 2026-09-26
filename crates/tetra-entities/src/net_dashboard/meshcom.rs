// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Dashboard-side persistence helpers for MeshCom UDP settings.

use tetra_config::bluestation::MeshcomRuntimeOverride;

// Was: Führt den Arbeitsschritt `toml_escape` für toml escape aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn toml_escape(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

// Was: Führt den Arbeitsschritt `string_set_toml` für string set toml aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn string_set_toml(values: &std::collections::BTreeSet<String>) -> String {
    values
        .iter()
        .map(|v| format!("\"{}\"", toml_escape(v)))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Rewrite (or insert) the `[meshcom]` section in the TOML file. A `.meshcom.bak` backup is made.
// Was: Diese Funktion schreibt meshcom to toml.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
pub fn write_meshcom_to_toml(config_path: &str, ov: &MeshcomRuntimeOverride) -> std::io::Result<()> {
    let original = std::fs::read_to_string(config_path)?;
    let section = format!(
        "[meshcom]\n\
         enabled = {}\n\
         bind_addr = \"{}\"\n\
         bind_port = {}\n\
         tx_host = \"{}\"\n\
         tx_port = {}\n\
         allow_broadcast = {}\n\
         max_messages = {}\n\
         max_nodes = {}\n\n\
         forward_sds = {}\n\
         forward_sip = {}\n\
         forward_telegram = {}\n\n\
         sds_source_issi = {}\n\
         sds_dest_issi = {}\n\
         sds_dest_is_group = {}\n\
         sds_allowed_sources = [{}]\n\n\
         sip_title_prefix = \"{}\"\n\
         sip_allowed_sources = [{}]\n\n\
         telegram_prefix = \"{}\"\n\
         telegram_allowed_sources = [{}]",
        ov.enabled,
        toml_escape(&ov.bind_addr),
        nonzero_u16(ov.bind_port, 1799),
        toml_escape(&ov.tx_host),
        nonzero_u16(ov.tx_port, 1799),
        ov.allow_broadcast,
        ov.max_messages.clamp(10, 10_000),
        ov.max_nodes.clamp(10, 65_535),
        ov.forward_sds,
        ov.forward_sip,
        ov.forward_telegram,
        ov.sds_source_issi.max(1),
        ov.sds_dest_issi,
        ov.sds_dest_is_group,
        string_set_toml(&ov.sds_allowed_sources),
        toml_escape(&ov.sip_title_prefix),
        string_set_toml(&ov.sip_allowed_sources),
        toml_escape(&ov.telegram_prefix),
        string_set_toml(&ov.telegram_allowed_sources),
    );

    let lines: Vec<&str> = original.lines().collect();
    let mut out: Vec<String> = Vec::with_capacity(lines.len() + 12);
    let mut i = 0;
    let mut replaced = false;

    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while i < lines.len() {
        let trimmed = lines[i].trim_start();
        if trimmed.starts_with("[meshcom]") {
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

    let backup = format!("{config_path}.meshcom.bak");
    let _ = std::fs::copy(config_path, &backup);
    std::fs::write(config_path, new_content)
}

// Was: Führt den Arbeitsschritt `nonzero_u16` für nonzero u16 aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn nonzero_u16(value: u16, fallback: u16) -> u16 {
    if value == 0 { fallback } else { value }
}
