#![allow(dead_code)]
// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Nachrichten an den Schnittstellen zwischen TETRA-Protokollschichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.


/// Shared typed values for local MLE/LLC/MAC service primitives.
// Was: Bindet das Untermodul common in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod common;

/// Custom definitions for stack control
// Was: Bindet das Untermodul Steuerung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod control;
// Was: Bindet das Untermodul tmd in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tmd;

// Was: Bindet das Untermodul lcmc in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod lcmc;
// Was: Bindet das Untermodul lmm in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod lmm;
// Was: Bindet das Untermodul TETRA-Paketdatentransport in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod ltpd;
// Was: Bindet das Untermodul sapmsg in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod sapmsg;
// Was: Bindet das Untermodul tla in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tla;
// Was: Bindet das Untermodul tle in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tle;
// Was: Bindet das Untermodul tlmb in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tlmb;
// Was: Bindet das Untermodul tlmc in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tlmc;
// Was: Bindet das Untermodul tma in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tma;
// Was: Bindet das Untermodul tmv in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tmv;
// Was: Bindet das Untermodul tp in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tp;
// Was: Bindet das Untermodul tpc in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tpc;

// Was: Bindet das Untermodul tnmm in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod tnmm;

pub use sapmsg::*;
