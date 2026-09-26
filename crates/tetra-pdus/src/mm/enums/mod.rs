// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul Mobilitätsverwaltung Protokollnachricht (PDU) type dl in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mm_pdu_type_dl;
// Was: Bindet das Untermodul Mobilitätsverwaltung Protokollnachricht (PDU) type ul in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mm_pdu_type_ul;

// Was: Bindet das Untermodul energy saving mode in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod energy_saving_mode;
// Was: Bindet das Untermodul location update accept type in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod location_update_accept_type;
// Was: Bindet das Untermodul location update type in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod location_update_type;

// Was: Bindet das Untermodul Status Downlink (Netz zum Funkgerät) in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod status_downlink;
// Was: Bindet das Untermodul Status Uplink (Funkgerät zum Netz) in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod status_uplink;

// Was: Bindet das Untermodul reject cause in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod reject_cause;

// Was: Bindet das Untermodul type34 elem Kennung dl in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod type34_elem_id_dl;
// Was: Bindet das Untermodul type34 elem Kennung ul in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod type34_elem_id_ul;
