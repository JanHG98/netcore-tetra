// NETCORE-KOMMENTAR – Was: Bindet die Oberflächen der Basisstation als Build-Assets ein.
// NETCORE-KOMMENTAR – Warum: Gemeinsame Design-Tokens bleiben unabhängig von Protokoll- und Bedienlogik wartbar.

pub const DASHBOARD_HTML: &str = include_str!("ui/dashboard.html");
pub const LOGIN_HTML: &str = include_str!("ui/login.html");
pub const NETCORE_CSS: &str = include_str!("ui/netcore.css");
pub const NETCORE_JS: &str = include_str!("ui/netcore.js");
pub const NETCORE_RF_CSS: &str = include_str!("ui/netcore-rf.css");
pub const NETCORE_RF_JS: &str = include_str!("ui/netcore-rf.js");
pub const NETCORE_LOGIN_JS: &str = include_str!("ui/netcore-login.js");
pub const NETCORE_LOGO: &[u8] = include_bytes!("ui/netcore-logo.png");
