// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul access assign in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod access_assign;
// Was: Bindet das Untermodul access assign fr18 in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod access_assign_fr18;

// Was: Bindet das Untermodul access define in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod access_define;

// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung access in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_access;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung d blck in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_d_blck;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung data in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_data;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung end dl in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_end_dl;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung end hu in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_end_hu;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung end ul in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_end_ul;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung frag dl in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_frag_dl;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung frag ul in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_frag_ul;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung resource in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_resource;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung u blck in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_u_blck;

// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung u signal in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_u_signal;

// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung sync in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_sync;
// Was: Bindet das Untermodul MAC-Funkzugriffssteuerung sysinfo in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mac_sysinfo;
