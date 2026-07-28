// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für Kodierung und Dekodierung von TETRA-Protokollnachrichten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::frames;

// Timers as defined in Annex A.1 LLC timers
// Was: Legt den festen Wert `T251_SENDER_RETRY_TIMER` für t251 sender retry Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T251_SENDER_RETRY_TIMER: u32 = frames!(4); // 4 signalling frames
// Was: Legt den festen Wert `T252_ACK_WAITING_TIMER` für t252 ack waiting Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T252_ACK_WAITING_TIMER: u32 = frames!(9);
// Was: Legt den festen Wert `T261_SETUP_WAITING_TIMER` für t261 setup waiting Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T261_SETUP_WAITING_TIMER: u32 = frames!(4);
// Was: Legt den festen Wert `T263_DISCONNECT_WAITING_TIMER` für t263 disconnect waiting Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T263_DISCONNECT_WAITING_TIMER: u32 = frames!(4);
// Was: Legt den festen Wert `T265_RECONNECT_WAITING_TIMER` für t265 reconnect waiting Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T265_RECONNECT_WAITING_TIMER: u32 = frames!(4);
// Was: Legt den festen Wert `T271_RECEIVER_NOT_READY_FOR_TX_TIMER` für t271 receiver not ready for tx Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T271_RECEIVER_NOT_READY_FOR_TX_TIMER: u32 = frames!(36);
// Was: Legt den festen Wert `T272_RECEIVER_NOT_READY_FOR_RX_TIMER` für t272 receiver not ready for rx Zeitüberwachung fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const T272_RECEIVER_NOT_READY_FOR_RX_TIMER: u32 = frames!(18);
