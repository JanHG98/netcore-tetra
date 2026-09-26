// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul d attach detach Gruppe identity in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_attach_detach_group_identity;
// Was: Bindet das Untermodul d attach detach Gruppe identity acknowledgement in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_attach_detach_group_identity_acknowledgement;
// Was: Bindet das Untermodul d location update accept in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_location_update_accept;
// Was: Bindet das Untermodul d location update command in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_location_update_command;
// Was: Bindet das Untermodul d location update proceeding in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_location_update_proceeding;
// Was: Bindet das Untermodul d location update reject in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_location_update_reject;
// Was: Bindet das Untermodul d Mobilitätsverwaltung Status in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_mm_status;
// Was: Bindet das Untermodul Mobilitätsverwaltung Protokollnachricht (PDU) function not supported in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mm_pdu_function_not_supported;
// Was: Bindet das Untermodul u attach detach Gruppe identity in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_attach_detach_group_identity;
// Was: Bindet das Untermodul u attach detach Gruppe identity acknowledgement in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_attach_detach_group_identity_acknowledgement;
// Was: Bindet das Untermodul u itsi detach in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_itsi_detach;
// Was: Bindet das Untermodul u location update demand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_location_update_demand;
// Was: Bindet das Untermodul u Mobilitätsverwaltung Status in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_mm_status;
// Was: Bindet das Untermodul u tei provide in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_tei_provide;
