// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul CMCE-Rufsteuerung function not supported in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod cmce_function_not_supported;
// Was: Bindet das Untermodul d alert in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_alert;
// Was: Bindet das Untermodul d Ruf proceeding in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_call_proceeding;
// Was: Bindet das Untermodul d Ruf Wiederherstellung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_call_restore;
// Was: Bindet das Untermodul d connect in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_connect;
// Was: Bindet das Untermodul d connect acknowledge in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_connect_acknowledge;
// Was: Bindet das Untermodul d disconnect in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_disconnect;
// Was: Bindet das Untermodul d facility in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_facility;
// Was: Bindet das Untermodul d info in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_info;
// Was: Bindet das Untermodul d release in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_release;
// Was: Bindet das Untermodul d TETRA-Kurznachricht (SDS) data in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_sds_data;
// Was: Bindet das Untermodul d setup in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_setup;
// Was: Bindet das Untermodul d Status in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_status;
// Was: Bindet das Untermodul d tx ceased in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_tx_ceased;
// Was: Bindet das Untermodul d tx continue in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_tx_continue;
// Was: Bindet das Untermodul d tx granted in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_tx_granted;
// Was: Bindet das Untermodul d tx interrupt in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_tx_interrupt;
// Was: Bindet das Untermodul d tx wait in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod d_tx_wait;
// Was: Bindet das Untermodul u alert in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_alert;
// Was: Bindet das Untermodul u Ruf Wiederherstellung in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_call_restore;
// Was: Bindet das Untermodul u connect in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_connect;
// Was: Bindet das Untermodul u disconnect in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_disconnect;
// Was: Bindet das Untermodul u facility in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_facility;
// Was: Bindet das Untermodul u info in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_info;
// Was: Bindet das Untermodul u release in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_release;
// Was: Bindet das Untermodul u TETRA-Kurznachricht (SDS) data in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_sds_data;
// Was: Bindet das Untermodul u setup in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_setup;
// Was: Bindet das Untermodul u Status in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_status;
// Was: Bindet das Untermodul u tx ceased in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_tx_ceased;
// Was: Bindet das Untermodul u tx demand in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod u_tx_demand;
