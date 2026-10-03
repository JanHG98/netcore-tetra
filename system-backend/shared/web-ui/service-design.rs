//! Standalone, dependency-free shell for NetCore service WebUIs.
//! Existing markup and service JavaScript remain owned by their service.

const STYLE: &str = include_str!("assets/service-design.css");
const SCRIPT: &str = include_str!("assets/service-design.js");
const THEME_INIT: &str = include_str!("assets/service-theme-init.js");
const LOGO: &str = include_str!("assets/netcore-logo.data-uri");

fn json_string(value: &str) -> String {
    let mut out = String::from("\"");
    for c in value.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            '\t' => out.push_str("\\t"),
            '<' => out.push_str("\\u003c"),
            '>' => out.push_str("\\u003e"),
            '&' => out.push_str("\\u0026"),
            c if c.is_control() => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

/// Inline shared assets so a deployed binary needs no external web asset paths.
/// `access` describes the existing authentication mode; it does not grant access.
pub fn render(html: &str, service: &str, access: &str) -> String {
    if html.contains("id=\"netcore-service-design\"") {
        return html.to_owned();
    }
    let config = format!("{{\"name\":{},\"access\":{},\"logo\":{}}}", json_string(service), json_string(access), json_string(LOGO.trim()));
    let head = format!("<style id=\"netcore-service-design\">{STYLE}</style><script type=\"application/json\" id=\"netcore-service-config\">{config}</script>");
    // Apply the saved palette before styles or service scripts begin.
    let init = format!("<script id=\"netcore-service-init\">{THEME_INIT}</script>");
    let html = if let Some(start) = html.find("<head") {
        if let Some(offset) = html[start..].find('>') {
            let end = start + offset + 1;
            format!("{}{init}{}", &html[..end], &html[end..])
        } else {
            format!("{init}{html}")
        }
    } else {
        format!("{init}{html}")
    };
    let html = if html.contains("</head>") {
        html.replacen("</head>", &format!("{head}</head>"), 1)
    } else {
        format!("{head}{html}")
    };
    let script = format!("<script id=\"netcore-service-shell\">{SCRIPT}</script>");
    if html.contains("</body>") {
        html.replacen("</body>", &format!("{script}</body>"), 1)
    } else {
        format!("{html}{script}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_existing_content_and_injects_once() {
        let source = "<!doctype html><head></head><body><button id=\"save\" onclick=\"save()\">Save</button><script>fetch('/api/v1/status')</script></body>";
        let result = render(source, "Subscriber Core", "open-lab");
        assert!(result.contains("<button id=\"save\" onclick=\"save()\">Save</button>"));
        assert!(result.contains("fetch('/api/v1/status')"));
        assert_eq!(result.matches("id=\"netcore-service-design\"").count(), 1);
        assert_eq!(render(&result, "Subscriber Core", "open-lab"), result);
    }

    #[test]
    fn configuration_cannot_close_its_script() {
        let result = render("<head></head><body></body>", "</script><script>alert(1)</script>", "access-key");
        assert!(!result.contains("</script><script>alert(1)</script>"));
        assert!(result.contains("\\u003c/script\\u003e"));
    }
}
