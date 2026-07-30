// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gemeinsame Telemetrie.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Minimal Prometheus text exposition helpers used by NetCore service metrics endpoints.

use std::collections::BTreeMap;

// Was: Führt den Arbeitsschritt `escape_label` für escape label aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn escape_label(value: &str) -> String {
    value.replace('\\', "\\\\").replace('\n', "\\n").replace('"', "\\\"")
}

// Was: Führt den Arbeitsschritt `metric_line` für metric line aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn metric_line(name: &str, labels: &BTreeMap<&str, String>, value: impl std::fmt::Display) -> String {
    let mut line = String::new();
    line.push_str(name);
    if !labels.is_empty() {
        line.push('{');
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (index, (key, label_value)) in labels.iter().enumerate() {
            if index > 0 {
                line.push(',');
            }
            line.push_str(key);
            line.push_str("=\"");
            line.push_str(&escape_label(label_value));
            line.push('"');
        }
        line.push('}');
    }
    line.push(' ');
    line.push_str(&value.to_string());
    line.push('\n');
    line
}

// Was: Führt den Arbeitsschritt `help_and_type` für help and type aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn help_and_type(name: &str, help: &str, metric_type: &str) -> String {
    format!("# HELP {name} {}\n# TYPE {name} {metric_type}\n", help.replace('\n', " "))
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `escapes_prometheus_labels` für escapes prometheus labels aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn escapes_prometheus_labels() {
        assert_eq!(escape_label("a\"b\\c\nd"), "a\\\"b\\\\c\\nd");
    }

    #[test]
    // Was: Führt den Arbeitsschritt `renders_labels_in_stable_order` für renders labels in stable order aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn renders_labels_in_stable_order() {
        let labels = BTreeMap::from([("service", "group-core".to_owned()), ("state", "up".to_owned())]);
        assert_eq!(metric_line("netcore_up", &labels, 1), "netcore_up{service=\"group-core\",state=\"up\"} 1\n");
    }
}
