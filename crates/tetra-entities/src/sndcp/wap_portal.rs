// NETCORE-KOMMENTAR – Was: Rendert das kompakte NetCore-WAP-Portal in XHTML und WML.
// NETCORE-KOMMENTAR – Warum: Alte Openwave-Terminals benötigen sehr kleine, vollständig verlinkte Seiten ohne Desktop-HTML-Ballast.

//! Compact, terminal-friendly NetCore portal pages for WAP/WSP.

use super::wap_status::{WapStatusSnapshot, escape_xhtml_text_limited};

pub const XHTML_INDEX_MAX_BYTES: usize = 104;
pub const XHTML_PAGE_MAX_BYTES: usize = 144;
pub const WML_PAGE_MAX_BYTES: usize = 144;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WapMarkup {
    Xhtml,
    Wml,
}

impl WapMarkup {
    pub fn content_type_token(self) -> u8 {
        match self {
            Self::Xhtml => 0xc5,
            Self::Wml => 0x88,
        }
    }

    fn route_prefix(self) -> &'static str {
        match self {
            Self::Xhtml => "/x",
            Self::Wml => "/w",
        }
    }

    fn wrap(self, body: &str) -> String {
        match self {
            Self::Xhtml => format!("<html><body>{body}</body></html>"),
            Self::Wml => format!("<wml><card><p>{body}</p></card></wml>"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WapPage {
    Home,
    Status,
    Subscribers,
    Groups,
    Calls,
    Sds,
    ControlRoom,
    Health,
    Radio,
    PacketData,
    Gateway,
    Services,
    Diagnostics,
    MediaLibrary,
    Recorder,
    Tts,
    Tests,
    Help,
    About,
}

pub const ALL_PAGES: [WapPage; 19] = [
    WapPage::Home,
    WapPage::Status,
    WapPage::Subscribers,
    WapPage::Groups,
    WapPage::Calls,
    WapPage::Sds,
    WapPage::ControlRoom,
    WapPage::Health,
    WapPage::Radio,
    WapPage::PacketData,
    WapPage::Gateway,
    WapPage::Services,
    WapPage::Diagnostics,
    WapPage::MediaLibrary,
    WapPage::Recorder,
    WapPage::Tts,
    WapPage::Tests,
    WapPage::Help,
    WapPage::About,
];

impl WapPage {
    pub fn short_slug(self) -> &'static str {
        match self {
            Self::Home => "",
            Self::Status => "st",
            Self::Subscribers => "ms",
            Self::Groups => "gr",
            Self::Calls => "ca",
            Self::Sds => "sd",
            Self::ControlRoom => "cr",
            Self::Health => "he",
            Self::Radio => "ra",
            Self::PacketData => "pd",
            Self::Gateway => "gw",
            Self::Services => "sv",
            Self::Diagnostics => "dg",
            Self::MediaLibrary => "me",
            Self::Recorder => "re",
            Self::Tts => "tt",
            Self::Tests => "te",
            Self::Help => "hl",
            Self::About => "ab",
        }
    }

    pub fn long_slug(self) -> &'static str {
        match self {
            Self::Home => "index",
            Self::Status => "status",
            Self::Subscribers => "subscribers",
            Self::Groups => "groups",
            Self::Calls => "calls",
            Self::Sds => "sds",
            Self::ControlRoom => "control-room",
            Self::Health => "health",
            Self::Radio => "radio",
            Self::PacketData => "packet-data",
            Self::Gateway => "gateway",
            Self::Services => "services",
            Self::Diagnostics => "diagnostics",
            Self::MediaLibrary => "media-library",
            Self::Recorder => "recorder",
            Self::Tts => "tts",
            Self::Tests => "tests",
            Self::Help => "help",
            Self::About => "about",
        }
    }

    fn title(self) -> &'static str {
        match self {
            Self::Home => "NetCore",
            Self::Status => "Status",
            Self::Subscribers => "Teilnehmer",
            Self::Groups => "Gruppen",
            Self::Calls => "Rufe",
            Self::Sds => "SDS",
            Self::ControlRoom => "Control Room",
            Self::Health => "Health",
            Self::Radio => "Funkzelle",
            Self::PacketData => "Paketdaten",
            Self::Gateway => "IP Gateway",
            Self::Services => "Dienste",
            Self::Diagnostics => "Diagnose",
            Self::MediaLibrary => "Media Library",
            Self::Recorder => "Recorder",
            Self::Tts => "TTS Piper",
            Self::Tests => "Testseiten",
            Self::Help => "Hilfe",
            Self::About => "NetCore-Tetra",
        }
    }

    fn previous(self) -> Option<Self> {
        match self {
            Self::Home => None,
            Self::Status => None,
            Self::Subscribers => Some(Self::Status),
            Self::Groups => Some(Self::Subscribers),
            Self::Calls => Some(Self::Groups),
            Self::Sds => Some(Self::Calls),
            Self::ControlRoom => Some(Self::Sds),
            Self::Health => Some(Self::ControlRoom),
            Self::Radio => None,
            Self::PacketData => Some(Self::Radio),
            Self::Gateway => Some(Self::PacketData),
            Self::Services => Some(Self::Gateway),
            Self::Diagnostics => Some(Self::Services),
            Self::MediaLibrary => None,
            Self::Recorder => Some(Self::MediaLibrary),
            Self::Tts => Some(Self::Recorder),
            Self::Tests => Some(Self::Tts),
            Self::Help => Some(Self::Tests),
            Self::About => Some(Self::Help),
        }
    }

    fn next(self) -> Option<Self> {
        match self {
            Self::Home => None,
            Self::Status => Some(Self::Subscribers),
            Self::Subscribers => Some(Self::Groups),
            Self::Groups => Some(Self::Calls),
            Self::Calls => Some(Self::Sds),
            Self::Sds => Some(Self::ControlRoom),
            Self::ControlRoom => Some(Self::Health),
            Self::Health => None,
            Self::Radio => Some(Self::PacketData),
            Self::PacketData => Some(Self::Gateway),
            Self::Gateway => Some(Self::Services),
            Self::Services => Some(Self::Diagnostics),
            Self::Diagnostics => None,
            Self::MediaLibrary => Some(Self::Recorder),
            Self::Recorder => Some(Self::Tts),
            Self::Tts => Some(Self::Tests),
            Self::Tests => Some(Self::Help),
            Self::Help => Some(Self::About),
            Self::About => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WapPortalRoute {
    pub page: WapPage,
    pub markup: WapMarkup,
}

fn short_path(markup: WapMarkup, page: WapPage) -> String {
    if page == WapPage::Home {
        markup.route_prefix().to_string()
    } else {
        format!("{}/{}", markup.route_prefix(), page.short_slug())
    }
}

fn link(markup: WapMarkup, page: WapPage, label: &str) -> String {
    format!("<a href=\"{}\">{label}</a>", short_path(markup, page))
}

fn compact_uptime(secs: u64) -> String {
    let days = (secs / 86_400).min(99);
    let hours = (secs % 86_400) / 3_600;
    let minutes = (secs % 3_600) / 60;
    if days > 0 {
        format!("{days}d{hours}h")
    } else if hours > 0 {
        format!("{hours}h{minutes}m")
    } else {
        format!("{minutes}m")
    }
}

fn navigation(markup: WapMarkup, page: WapPage, include_previous: bool) -> String {
    let mut links = Vec::new();
    if include_previous {
        if let Some(previous) = page.previous() {
            links.push(link(markup, previous, "P"));
        }
    }
    if let Some(next) = page.next() {
        links.push(link(markup, next, "N"));
    }
    links.push(link(markup, WapPage::Home, "H"));
    links.join(" ")
}

fn page_line(page: WapPage, snapshot: &WapStatusSnapshot) -> String {
    let state = escape_xhtml_text_limited(&snapshot.state, 12);
    let health = escape_xhtml_text_limited(&snapshot.health, 12);
    let version = escape_xhtml_text_limited(snapshot.version.trim_start_matches('v'), 18);
    let last = escape_xhtml_text_limited(&snapshot.last_activity, 18);
    match page {
        WapPage::Home => format!("{} {state}", snapshot.title),
        WapPage::Status => format!(
            "Status {state}<br/>MS:{} G:{} R:{}",
            snapshot.registered_ms, snapshot.attached_groups, snapshot.active_calls
        ),
        WapPage::Subscribers => format!("Teilnehmer<br/>Registriert:{}", snapshot.registered_ms),
        WapPage::Groups => format!("Gruppen<br/>Affiliiert:{}", snapshot.attached_groups),
        WapPage::Calls => format!("Rufe<br/>Aktiv:{}", snapshot.active_calls),
        WapPage::Sds => format!("SDS<br/>Wartend:{}", snapshot.queued_sds),
        WapPage::ControlRoom => "Control Room<br/>Steuerung/Status".to_string(),
        WapPage::Health => format!("Health {health}<br/>Up:{}", compact_uptime(snapshot.uptime_secs)),
        WapPage::Radio => format!("Funkzelle<br/>{state} TETRA"),
        WapPage::PacketData => "Paketdaten<br/>SNDCP IPv4 MTU576".to_string(),
        WapPage::Gateway => "IP Gateway<br/>10.0.0.1:9200".to_string(),
        WapPage::Services => "Dienste<br/>Voice SDS WAP IP".to_string(),
        WapPage::Diagnostics => format!("Diagnose<br/>{health} {last}"),
        WapPage::MediaLibrary => "Media Library<br/>Audioarchiv".to_string(),
        WapPage::Recorder => "Recorder<br/>Rufaufzeichnung".to_string(),
        WapPage::Tts => "TTS Piper<br/>Text zu Audio".to_string(),
        WapPage::Tests => "Testseiten<br/>XHTML/WML OK".to_string(),
        WapPage::Help => "Hilfe<br/>N=weiter P=zurueck".to_string(),
        WapPage::About => format!("NetCore-Tetra<br/>Version:{version}"),
    }
}

fn first_fitting(markup: WapMarkup, bodies: impl IntoIterator<Item = String>, max: usize) -> String {
    let mut shortest: Option<String> = None;
    for body in bodies {
        let document = markup.wrap(&body);
        if shortest.as_ref().map_or(true, |current| document.len() < current.len()) {
            shortest = Some(document.clone());
        }
        if document.len() <= max {
            return document;
        }
    }
    shortest.unwrap_or_else(|| markup.wrap("NetCore"))
}

pub fn render_portal_page(route: WapPortalRoute, snapshot: &WapStatusSnapshot) -> String {
    let markup = route.markup;
    let page = route.page;
    let max = match (markup, page) {
        (WapMarkup::Xhtml, WapPage::Status) => XHTML_INDEX_MAX_BYTES,
        (WapMarkup::Xhtml, _) => XHTML_PAGE_MAX_BYTES,
        (WapMarkup::Wml, _) => WML_PAGE_MAX_BYTES,
    };

    if page == WapPage::Home {
        let title = escape_xhtml_text_limited(&snapshot.title, 14);
        return first_fitting(
            markup,
            [
                format!(
                    "{title}<br/>{} {} {}",
                    link(markup, WapPage::Status, "Betrieb"),
                    link(markup, WapPage::Radio, "Netz"),
                    link(markup, WapPage::MediaLibrary, "Info")
                ),
                format!(
                    "{title}<br/>{} {} {}",
                    link(markup, WapPage::Status, "B"),
                    link(markup, WapPage::Radio, "N"),
                    link(markup, WapPage::MediaLibrary, "I")
                ),
                format!("{title}<br/>{}", link(markup, WapPage::Status, "Start")),
            ],
            max,
        );
    }

    let line = page_line(page, snapshot);
    first_fitting(
        markup,
        [
            format!("{line}<br/>{}", navigation(markup, page, true)),
            format!("{line}<br/>{}", navigation(markup, page, false)),
            format!("{}<br/>{}", page.title(), link(markup, WapPage::Home, "H")),
        ],
        max,
    )
}

fn find_page_by_short_slug(slug: &str) -> Option<WapPage> {
    ALL_PAGES.iter().copied().find(|page| page.short_slug() == slug)
}

fn find_page_by_long_slug(slug: &str) -> Option<WapPage> {
    ALL_PAGES.iter().copied().find(|page| page.long_slug() == slug)
}

/// Parse compact routes (`/x/st`, `/w/st`) and readable aliases
/// (`/status.xhtml`, `/status.wml`). Query and fragment must already be removed.
pub fn parse_portal_path(path: &str) -> Option<WapPortalRoute> {
    if matches!(path, "/" | "/index" | "/index.xhtml") {
        return Some(WapPortalRoute { page: WapPage::Home, markup: WapMarkup::Xhtml });
    }
    if path == "/index.wml" {
        return Some(WapPortalRoute { page: WapPage::Home, markup: WapMarkup::Wml });
    }
    if let Some(rest) = path.strip_prefix("/x") {
        let slug = rest.strip_prefix('/').unwrap_or(rest);
        return find_page_by_short_slug(slug)
            .map(|page| WapPortalRoute { page, markup: WapMarkup::Xhtml });
    }
    if let Some(rest) = path.strip_prefix("/w") {
        let slug = rest.strip_prefix('/').unwrap_or(rest);
        return find_page_by_short_slug(slug)
            .map(|page| WapPortalRoute { page, markup: WapMarkup::Wml });
    }
    if let Some(slug) = path.strip_prefix('/').and_then(|value| value.strip_suffix(".xhtml")) {
        return find_page_by_long_slug(slug)
            .map(|page| WapPortalRoute { page, markup: WapMarkup::Xhtml });
    }
    if let Some(slug) = path.strip_prefix('/').and_then(|value| value.strip_suffix(".wml")) {
        return find_page_by_long_slug(slug)
            .map(|page| WapPortalRoute { page, markup: WapMarkup::Wml });
    }
    if path == "/status" {
        return Some(WapPortalRoute { page: WapPage::Status, markup: WapMarkup::Xhtml });
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::{HashSet, VecDeque};

    fn snapshot() -> WapStatusSnapshot {
        WapStatusSnapshot {
            title: "NetCore-Tetra".into(),
            state: "ONLINE".into(),
            version: "v1.3.0".into(),
            registered_ms: 12,
            attached_groups: 4,
            active_calls: 2,
            queued_sds: 1,
            uptime_secs: 93_784,
            last_activity: "SDS 4010001>4010002".into(),
            health: "OK".into(),
        }
    }

    fn hrefs(document: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut rest = document;
        while let Some(start) = rest.find("href=\"") {
            rest = &rest[start + 6..];
            let Some(end) = rest.find('"') else { break };
            out.push(rest[..end].to_string());
            rest = &rest[end + 1..];
        }
        out
    }

    #[test]
    fn all_pages_have_both_readable_aliases() {
        for page in ALL_PAGES {
            let xhtml = format!("/{}.xhtml", page.long_slug());
            let wml = format!("/{}.wml", page.long_slug());
            assert_eq!(parse_portal_path(&xhtml), Some(WapPortalRoute { page, markup: WapMarkup::Xhtml }));
            assert_eq!(parse_portal_path(&wml), Some(WapPortalRoute { page, markup: WapMarkup::Wml }));
        }
    }

    #[test]
    fn rendered_pages_stay_inside_openwave_caps() {
        for page in ALL_PAGES {
            let xhtml = render_portal_page(WapPortalRoute { page, markup: WapMarkup::Xhtml }, &snapshot());
            let wml = render_portal_page(WapPortalRoute { page, markup: WapMarkup::Wml }, &snapshot());
            let xhtml_max = if page == WapPage::Status { XHTML_INDEX_MAX_BYTES } else { XHTML_PAGE_MAX_BYTES };
            assert!(xhtml.len() <= xhtml_max, "XHTML {:?} is {} bytes", page, xhtml.len());
            assert!(wml.len() <= WML_PAGE_MAX_BYTES, "WML {:?} is {} bytes", page, wml.len());
            assert!(xhtml.starts_with("<html><body>"));
            assert!(wml.starts_with("<wml><card><p>"));
        }
    }

    #[test]
    fn each_format_is_fully_navigable_without_cross_format_links() {
        for markup in [WapMarkup::Xhtml, WapMarkup::Wml] {
            let mut visited = HashSet::new();
            let mut queue = VecDeque::new();
            queue.push_back(WapPage::Home);
            while let Some(page) = queue.pop_front() {
                if !visited.insert(page) {
                    continue;
                }
                let document = render_portal_page(WapPortalRoute { page, markup }, &snapshot());
                for href in hrefs(&document) {
                    assert!(href.starts_with(markup.route_prefix()), "cross-format link {href}");
                    let route = parse_portal_path(&href).expect("generated portal link must parse");
                    assert_eq!(route.markup, markup);
                    queue.push_back(route.page);
                }
            }
            assert_eq!(visited.len(), ALL_PAGES.len(), "not all pages reachable in {:?}", markup);
        }
    }
}
