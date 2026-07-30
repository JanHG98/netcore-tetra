#![allow(dead_code)]
// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.


// Protocol-only modules.  These are intentionally available without the
// `runtime` feature so external tools such as `netcore-control-room` can share
// the same wire structs without linking SDR/audio/native runtime libraries.
// Was: Bindet das Untermodul health in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod health;
// Was: Bindet das Untermodul legacy WAP-Dienst in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod legacy_wap;
// Was: Bindet das Untermodul net Steuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_control;
// Was: Bindet das Untermodul net Steuerung room in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_control_room;
// Was: Bindet das Untermodul net Audio- und Mediendaten in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_media;
// Was: Bindet das Untermodul net Telemetrie in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_telemetry;

// Full base-station/entity runtime.  Kept behind `runtime` so the Control Room
// Core can build in a lean LXC without SoapySDR, libgsm or libtetra-codec.
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul CMCE-Rufsteuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod cmce;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul entity trait in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod entity_trait;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul LLC-Verbindungsschicht in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod llc;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul lmac in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod lmac;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul messagerouter in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod messagerouter;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul MLE-Verbindungssteuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mle;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Mobilitätsverwaltung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mm;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul phy in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod phy;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul SNDCP-Paketdaten in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sndcp;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul UMAC-Funkzugriffssteuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod umac;

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul network in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod network;

#[cfg(feature = "tetra-codec")]
// Was: Bindet das Untermodul net audio in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_audio;
#[cfg(feature = "asterisk")]
// Was: Bindet das Untermodul net asterisk in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_asterisk;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net Brew-Verbindung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_brew;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net dapnet in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_dapnet;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net dashboard in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_dashboard;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net echolink in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_echolink;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net geoalarm in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_geoalarm;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net meshcom in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_meshcom;
#[cfg(feature = "recording")]
// Was: Bindet das Untermodul net Aufzeichnung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_recorder;
#[cfg(feature = "audio-player")]
// Was: Bindet das Untermodul net audio player in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_audio_player;
#[cfg(feature = "audio-player")]
// Was: Bindet das Untermodul net tts in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_tts;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net snom in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_snom;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul net telegram in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod net_telegram;

#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul backlight in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod backlight;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul Dienst Steuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod service_control;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul sys Telemetrie in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sys_telemetry;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul tpg2200 in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tpg2200;
#[cfg(feature = "runtime")]
// Was: Bindet das Untermodul wifi in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod wifi;

// Re-export commonly used runtime items from router.
#[cfg(feature = "runtime")]
pub use entity_trait::TetraEntityTrait;
#[cfg(feature = "runtime")]
pub use messagerouter::{MessagePrio, MessageQueue, MessageRouter};
