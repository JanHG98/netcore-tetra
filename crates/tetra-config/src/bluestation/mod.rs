// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Einlesen und Prüfen der TETRA-Konfiguration.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul parsing in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod parsing;
pub use parsing::*;

// Was: Bindet das Untermodul Konfiguration in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod config;
pub use config::*;

// Was: Bindet das Untermodul sec phy in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_phy;
pub use sec_phy::*;

// Was: Bindet das Untermodul sec net in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_net;
pub use sec_net::*;

// Was: Bindet das Untermodul sec cell in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_cell;
pub use sec_cell::*;

// Was: Bindet das Untermodul sec phy soapy in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_phy_soapy;
pub use sec_phy_soapy::*;

// Was: Bindet das Untermodul sec Brew-Verbindung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_brew;
pub use sec_brew::*;

// Was: Bindet das Untermodul sec asterisk in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_asterisk;
pub use sec_asterisk::*;

// Was: Bindet das Untermodul sec dapnet in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_dapnet;
pub use sec_dapnet::*;

// Was: Bindet das Untermodul sec echolink in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_echolink;
pub use sec_echolink::*;

// Was: Bindet das Untermodul sec meshcom in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_meshcom;
pub use sec_meshcom::*;

// Was: Bindet das Untermodul sec geoalarm in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_geoalarm;
pub use sec_geoalarm::*;

// Was: Bindet das Untermodul sec tpg2200 action in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_tpg2200_action;
pub use sec_tpg2200_action::*;

// Was: Bindet das Untermodul sec snom notify in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_snom_notify;
pub use sec_snom_notify::*;

// Was: Bindet das Untermodul sec dashboard in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_dashboard;
pub use sec_dashboard::*;

// Was: Bindet das Untermodul sec recording in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_recording;
pub use sec_recording::*;

// Bidirectional Base Station ↔ Media Library integration.
pub mod sec_media_library;
pub use sec_media_library::*;

// Was: Bindet das Untermodul sec audio player in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_audio_player;
pub use sec_audio_player::*;

// Was: Bindet das Untermodul sec tts in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_tts;
pub use sec_tts::*;

// Was: Bindet das Untermodul sec Telemetrie in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_telemetry;
pub use sec_telemetry::*;

// Was: Bindet das Untermodul sec Steuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_control;
pub use sec_control::*;

// Was: Bindet das Untermodul sec Steuerung room in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_control_room;
pub use sec_control_room::*;

// Was: Bindet das Untermodul sec edge fallback in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_edge_fallback;
pub use sec_edge_fallback::*;

// Was: Bindet das Untermodul sec Sicherheit in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_security;
pub use sec_security::*;

// Was: Bindet das Untermodul sec wx in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_wx;
pub use sec_wx::*;

// Was: Bindet das Untermodul sec telegram in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_telegram;
pub use sec_telegram::*;

// Was: Bindet das Untermodul sec recovery in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_recovery;
pub use sec_recovery::*;

// Was: Bindet das Untermodul sec emergency in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_emergency;
pub use sec_emergency::*;

// Was: Bindet das Untermodul sec health in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sec_health;
pub use sec_health::*;

// Was: Bindet das Untermodul Zustand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod state;
pub use state::*;
