// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul bl ack in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod bl_ack;
// Was: Bindet das Untermodul bl adata in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod bl_adata;
// Was: Bindet das Untermodul bl data in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod bl_data;
// Was: Bindet das Untermodul bl udata in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod bl_udata;
// mod al_setup;
// mod al_data; // and possibly AL-DATA-AR/AL-FINAL/AL-FINAL-AR
// mod al_udata; // and AL-UFINAL
// mod al_ack // and AL-RNR
// mod al_reconnect;
// mod supp_llc_pdu;
// mod l2_sig_pdu;
// mod al_disc;
