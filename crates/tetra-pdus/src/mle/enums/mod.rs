// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

// Was: Bindet das Untermodul MLE-Verbindungssteuerung Protokollnachricht (PDU) type dl in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mle_pdu_type_dl;
// Was: Bindet das Untermodul MLE-Verbindungssteuerung Protokollnachricht (PDU) type ul in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mle_pdu_type_ul;

// Was: Bindet das Untermodul MLE-Verbindungssteuerung protocol discriminator in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
pub mod mle_protocol_discriminator;
