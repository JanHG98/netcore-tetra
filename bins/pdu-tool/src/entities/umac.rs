// NETCORE-KOMMENTAR – Was: Enthält die Logik oder Einstellungen für UMAC-Funkzugriffssteuerung.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use tetra_core::BitBuffer;
use tetra_pdus::umac::{
    enums::{broadcast_type::BroadcastType, mac_pdu_type::MacPduType},
    pdus::{
        access_define::AccessDefine,
        // Uplink PDUs
        mac_access::MacAccess,
        mac_d_blck::MacDBlck,
        mac_data::MacData,
        mac_end_dl::MacEndDl,
        mac_end_hu::MacEndHu,
        mac_end_ul::MacEndUl,
        mac_frag_dl::MacFragDl,
        mac_frag_ul::MacFragUl,
        // Downlink PDUs
        mac_resource::MacResource,
        mac_sync::MacSync,
        mac_sysinfo::MacSysinfo,
        mac_u_blck::MacUBlck,
        mac_u_signal::MacUSignal,
    },
};
use tetra_saps::tmv::enums::logical_chans::LogicalChannel;

/// Result of length_ind interpretation
#[derive(Debug)]
// Was: Bündelt die zusammengehörigen Werte für length ind info in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct LengthIndInfo {
    /// PDU payload length in bits (0 for null PDU or fragmentation)
    pub pdu_len_bits: usize,
    /// Whether this is a null PDU
    pub is_null_pdu: bool,
    /// Whether this is fragmentation start (no length known)
    pub is_frag_start: bool,
    /// Whether second half slot is stolen (STCH)
    pub second_half_stolen: bool,
}

/// UMAC parser for standalone PDU debugging
// Was: Bündelt die zusammengehörigen Werte für UMAC-Funkzugriffssteuerung parser in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UmacParser;

// Was: Implementiert das zugehörige Verhalten für `UmacParser`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl UmacParser {
    /// Parse an uplink MAC PDU and print the result
    /// Follows the structure of UmacBs::rx_tmv_unitdata_ind and rx_tmv_sch
    // Was: Diese Funktion liest und prüft ul.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    pub fn parse_ul(mut pdu: BitBuffer, logical_channel: LogicalChannel) {
        println!("=== UMAC UL Parser ===");
        println!("Logical channel: {:?}", logical_channel);
        println!("Input bits: {}", pdu.dump_bin());
        println!();

        // Iterate until no more messages left in mac block
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            let Some(bits) = pdu.peek_bits(3) else {
                println!("[!] Insufficient bits remaining: {}", pdu.dump_bin());
                return;
            };
            let orig_start = pdu.get_raw_start();

            // Clause 21.4.1; handling differs between SCH_HU and others
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match logical_channel {
                LogicalChannel::SchF | LogicalChannel::Stch => {
                    // First two bits are MAC PDU type
                    let Ok(pdu_type) = MacPduType::try_from(bits >> 1) else {
                        println!("[!] Invalid PDU type: {}", bits >> 1);
                        return;
                    };
                    println!("MAC PDU Type: {:?} ({})", pdu_type, bits >> 1);

                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match pdu_type {
                        MacPduType::MacResourceMacData => {
                            // On uplink this is MAC-DATA
                            Self::parse_mac_data(&mut pdu);
                        }
                        MacPduType::MacFragMacEnd => {
                            // Third bit distinguishes mac-frag (0) from mac-end (1)
                            if bits & 1 == 0 {
                                Self::parse_mac_frag_ul(&mut pdu);
                            } else {
                                Self::parse_mac_end_ul(&mut pdu);
                            }
                        }
                        MacPduType::SuppMacUSignal => {
                            // STCH determines which subtype is relevant
                            if logical_channel == LogicalChannel::Stch {
                                Self::parse_mac_u_signal(&mut pdu);
                            } else {
                                // Supplementary MAC PDU - third bit distinguishes
                                if bits & 1 == 0 {
                                    Self::parse_mac_u_blck(&mut pdu);
                                } else {
                                    println!("[!] Unexpected supplementary PDU subtype");
                                    return;
                                }
                            }
                        }
                        MacPduType::Broadcast => {
                            println!("[!] Broadcast PDU not expected on uplink");
                            return;
                        }
                    }
                }
                LogicalChannel::SchHu => {
                    // Only 1 bit needed for subtype distinction on SCH/HU
                    let pdu_type = (bits >> 2) & 1;
                    println!(
                        "SCH/HU PDU Type: {} ({})",
                        if pdu_type == 0 { "MAC-ACCESS" } else { "MAC-END-HU" },
                        pdu_type
                    );

                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match pdu_type {
                        0 => Self::parse_mac_access(&mut pdu),
                        1 => Self::parse_mac_end_hu(&mut pdu),
                        _ => unreachable!(),
                    }
                }
                _ => {
                    println!("[!] Unknown/unsupported logical channel for UL: {:?}", logical_channel);
                    return;
                }
            }

            // Check if more PDUs remain in MAC block
            if !Self::check_continue(&pdu, orig_start) {
                break;
            }
        }
    }

    /// Parse a downlink MAC PDU and print the result
    /// Follows the structure of UmacMs::rx_tmv_unitdata_ind and rx_tmv_sch
    // Was: Diese Funktion liest und prüft dl.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    pub fn parse_dl(mut pdu: BitBuffer, logical_channel: LogicalChannel) {
        println!("=== UMAC DL Parser ===");
        println!("Logical channel: {:?}", logical_channel);
        println!("Input bits: {}", pdu.dump_bin());
        println!();

        // Handle special channels first
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match logical_channel {
            LogicalChannel::Aach => {
                println!("--- AACH (Access Assignment Channel) ---");
                println!("[!] AACH parsing not implemented in standalone tool");
                return;
            }
            LogicalChannel::Bsch => {
                Self::parse_mac_sync(&mut pdu);
                return;
            }
            _ => {}
        }

        assert!(
            logical_channel == LogicalChannel::SchF || logical_channel == LogicalChannel::SchHd,
            "Unsupported logical channel for DL: {:?}",
            logical_channel
        );

        // Iterate until no more messages left in mac block
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            let Some(bits) = pdu.peek_bits(3) else {
                println!("[!] Insufficient bits remaining: {}", pdu.dump_bin());
                return;
            };
            let orig_start = pdu.get_raw_start();

            // First two bits are MAC PDU type
            let Ok(pdu_type) = MacPduType::try_from(bits >> 1) else {
                println!("[!] Invalid PDU type: {}", bits >> 1);
                return;
            };
            println!("MAC PDU Type: {:?} ({})", pdu_type, bits >> 1);

            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match pdu_type {
                MacPduType::MacResourceMacData => {
                    // On downlink this is MAC-RESOURCE
                    Self::parse_mac_resource(&mut pdu);
                }
                MacPduType::MacFragMacEnd => {
                    // Third bit distinguishes mac-frag (0) from mac-end (1)
                    if bits & 1 == 0 {
                        Self::parse_mac_frag_dl(&mut pdu);
                    } else {
                        Self::parse_mac_end_dl(&mut pdu);
                    }
                }
                MacPduType::Broadcast => {
                    Self::parse_broadcast(&mut pdu);
                }
                MacPduType::SuppMacUSignal => {
                    if logical_channel == LogicalChannel::Stch {
                        // U-SIGNAL on stealing channel
                        Self::parse_mac_u_signal(&mut pdu);
                    } else {
                        // Supplementary PDU - third bit distinguishes
                        if bits & 1 == 0 {
                            Self::parse_mac_d_blck(&mut pdu);
                        } else {
                            println!("[!] Unexpected supplementary PDU subtype on DL");
                            return;
                        }
                    }
                }
            }

            // Check if more PDUs remain in MAC block
            if !Self::check_continue(&pdu, orig_start) {
                break;
            }
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung data.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_data(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-DATA ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacData::from_bitbuf(pdu) {
            Ok(mac_data) => {
                println!("{:#?}", mac_data);

                // Print SDU preview if we have a length
                if let Some(len) = mac_data.length_ind {
                    let info = Self::interpret_length_ind(len, pdu.get_len_remaining());
                    if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                        Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                    } else if info.is_frag_start {
                        println!("Fragment data: {} bits remaining", pdu.get_len_remaining());
                    }
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, mac_data.length_ind, mac_data.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-DATA: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung access.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_access(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-ACCESS ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacAccess::from_bitbuf(pdu) {
            Ok(mac_access) => {
                println!("{:#?}", mac_access);

                // Print SDU preview if we have a length
                if let Some(len) = mac_access.length_ind {
                    let info = Self::interpret_length_ind(len, pdu.get_len_remaining());
                    if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                        Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                    } else if info.is_frag_start {
                        println!("Fragment data: {} bits remaining", pdu.get_len_remaining());
                    }
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, mac_access.length_ind, mac_access.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-ACCESS: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung frag ul.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_frag_ul(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-FRAG (UL) ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacFragUl::from_bitbuf(pdu) {
            Ok(mac_frag) => {
                println!("{:#?}", mac_frag);
                let remaining = pdu.get_len_remaining();
                println!("TM-SDU fragment: {} bits remaining", remaining);
                if remaining > 0 && remaining <= 64 {
                    if let Some(frag_bits) = pdu.peek_bits(remaining) {
                        println!("Fragment data: {:0width$b}", frag_bits, width = remaining);
                    }
                }
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-FRAG: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung end ul.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_end_ul(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-END (UL) ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacEndUl::from_bitbuf(pdu) {
            Ok(mac_end) => {
                println!("{:#?}", mac_end);

                // Print SDU preview if we have a length
                if let Some(len) = mac_end.length_ind {
                    let info = Self::interpret_length_ind(len, pdu.get_len_remaining());
                    if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                        Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                    } else if info.is_frag_start {
                        println!("Fragment data: {} bits remaining", pdu.get_len_remaining());
                    }
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, mac_end.length_ind, mac_end.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-END: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung end hu.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_end_hu(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-END-HU ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacEndHu::from_bitbuf(pdu) {
            Ok(mac_end) => {
                println!("{:#?}", mac_end);

                // Print SDU preview if we have a length
                if let Some(len) = mac_end.length_ind {
                    let info = Self::interpret_length_ind(len, pdu.get_len_remaining());
                    if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                        Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                    } else if info.is_frag_start {
                        println!("Fragment data: {} bits remaining", pdu.get_len_remaining());
                    }
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, mac_end.length_ind, mac_end.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-END-HU: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung u blck.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_u_blck(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-U-BLCK ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacUBlck::from_bitbuf(pdu) {
            Ok(mac_u_blck) => {
                println!("{:#?}", mac_u_blck);
                let remaining = pdu.get_len_remaining();
                println!("TM-SDU: {} bits remaining", remaining);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-U-BLCK: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung u signal.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_u_signal(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-U-SIGNAL ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacUSignal::from_bitbuf(pdu) {
            Ok(mac_u_signal) => {
                println!("{:#?}", mac_u_signal);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-U-SIGNAL: {:?}", e),
        }
    }

    // ========================================================================
    // DOWNLINK PDU PARSERS
    // ========================================================================

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung resource.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_resource(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-RESOURCE ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacResource::from_bitbuf(pdu) {
            Ok(mac_res) => {
                println!("{:#?}", mac_res);

                // Print SDU preview
                let info = Self::interpret_length_ind(mac_res.length_ind, pdu.get_len_remaining());
                if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                    Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                } else if info.is_frag_start {
                    let remaining = pdu.get_len_remaining();
                    println!("Fragment data: {} bits remaining", remaining);
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, Some(mac_res.length_ind), mac_res.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-RESOURCE: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung frag dl.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_frag_dl(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-FRAG (DL) ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacFragDl::from_bitbuf(pdu) {
            Ok(mac_frag) => {
                println!("{:#?}", mac_frag);
                let remaining = pdu.get_len_remaining();
                println!("TM-SDU fragment: {} bits remaining", remaining);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-FRAG (DL): {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung end dl.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_end_dl(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-END (DL) ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacEndDl::from_bitbuf(pdu) {
            Ok(mac_end) => {
                println!("{:#?}", mac_end);

                // Print SDU preview
                let info = Self::interpret_length_ind(mac_end.length_ind, pdu.get_len_remaining());
                if !info.is_null_pdu && !info.is_frag_start && !info.second_half_stolen && info.pdu_len_bits > 0 {
                    Self::print_sdu(pdu, info.pdu_len_bits.min(pdu.get_len_remaining()), "TM-SDU");
                } else if info.is_frag_start {
                    println!("Fragment data: {} bits remaining", pdu.get_len_remaining());
                }

                // Apply PDU association
                Self::apply_pdu_association(pdu, Some(mac_end.length_ind), mac_end.fill_bits);
            }
            Err(e) => println!("[!] Failed to parse MAC-END (DL): {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung d blck.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_d_blck(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-D-BLCK ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacDBlck::from_bitbuf(pdu) {
            Ok(mac_d_blck) => {
                println!("{:#?}", mac_d_blck);
                let remaining = pdu.get_len_remaining();
                println!("TM-SDU: {} bits remaining", remaining);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-D-BLCK: {:?}", e),
        }
    }

    // ========================================================================
    // BROADCAST PDU PARSERS
    // ========================================================================

    // Was: Diese Funktion liest und prüft broadcast.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_broadcast(pdu: &mut BitBuffer) {
        // Peek broadcast type (bits 2-3 after MAC PDU type)
        let Some(bits) = pdu.peek_bits_posoffset(2, 2) else {
            println!("[!] Insufficient bits for broadcast type");
            return;
        };

        let Ok(bcast_type) = BroadcastType::try_from(bits) else {
            println!("[!] Invalid broadcast type: {}", bits);
            return;
        };
        println!("Broadcast Type: {:?}", bcast_type);

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match bcast_type {
            BroadcastType::Sysinfo => Self::parse_mac_sysinfo(pdu),
            BroadcastType::AccessDefine => Self::parse_access_define(pdu),
            BroadcastType::SysinfoDa => {
                println!("[!] SYSINFO-DA parsing not implemented");
            }
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung sync.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_sync(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-SYNC (BSCH) ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacSync::from_bitbuf(pdu) {
            Ok(mac_sync) => {
                println!("{:#?}", mac_sync);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-SYNC: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft MAC-Funkzugriffssteuerung sysinfo.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_mac_sysinfo(pdu: &mut BitBuffer) {
        println!("--- Parsing MAC-SYSINFO ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match MacSysinfo::from_bitbuf(pdu) {
            Ok(mac_sysinfo) => {
                println!("{:#?}", mac_sysinfo);
                let remaining = pdu.get_len_remaining();
                if remaining > 0 {
                    println!("MLE PDU follows: {} bits remaining", remaining);
                }
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse MAC-SYSINFO: {:?}", e),
        }
    }

    // Was: Diese Funktion liest und prüft access define.
    // Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
    fn parse_access_define(pdu: &mut BitBuffer) {
        println!("--- Parsing ACCESS-DEFINE ---");
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match AccessDefine::from_bitbuf(pdu) {
            Ok(access_def) => {
                println!("{:#?}", access_def);
                println!("BitBuffer: {}", pdu.dump_bin());
            }
            Err(e) => println!("[!] Failed to parse ACCESS-DEFINE: {:?}", e),
        }
    }

    // ========================================================================
    // HELPER FUNCTIONS
    // ========================================================================

    /// Interpret length_ind value according to ETSI EN 300 392-2 clause 21.4.3.1
    // Was: Führt den Arbeitsschritt `interpret_length_ind` für interpret length ind aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn interpret_length_ind(length_ind: u8, remaining_bits: usize) -> LengthIndInfo {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match length_ind {
            0b000000 => {
                // Null PDU
                LengthIndInfo {
                    pdu_len_bits: 0,
                    is_null_pdu: true,
                    is_frag_start: false,
                    second_half_stolen: false,
                }
            }
            0b000001 => {
                // Reserved
                println!("[!] Reserved length_ind value: 1");
                LengthIndInfo {
                    pdu_len_bits: 0,
                    is_null_pdu: false,
                    is_frag_start: false,
                    second_half_stolen: false,
                }
            }
            0b000010..=0b111001 => {
                // Valid length: 2-57 octets (16-456 bits)
                let len_bits = length_ind as usize * 8;
                LengthIndInfo {
                    pdu_len_bits: len_bits.min(remaining_bits),
                    is_null_pdu: false,
                    is_frag_start: false,
                    second_half_stolen: false,
                }
            }
            0b111010..=0b111101 => {
                // Reserved
                println!("[!] Reserved length_ind value: {}", length_ind);
                LengthIndInfo {
                    pdu_len_bits: 0,
                    is_null_pdu: false,
                    is_frag_start: false,
                    second_half_stolen: false,
                }
            }
            0b111110 => {
                // Second half slot stolen (STCH)
                LengthIndInfo {
                    pdu_len_bits: remaining_bits,
                    is_null_pdu: false,
                    is_frag_start: false,
                    second_half_stolen: true,
                }
            }
            0b111111 => {
                // Start of TL-SDU which extends in one or more subsequent MAC PDUs (fragmentation)
                LengthIndInfo {
                    pdu_len_bits: remaining_bits,
                    is_null_pdu: false,
                    is_frag_start: true,
                    second_half_stolen: false,
                }
            }
            _ => {
                // Should not happen for 6-bit value
                println!("[!] Invalid length_ind value: {}", length_ind);
                LengthIndInfo {
                    pdu_len_bits: 0,
                    is_null_pdu: false,
                    is_frag_start: false,
                    second_half_stolen: false,
                }
            }
        }
    }

    /// Count fill bits at the end of a PDU segment.
    /// Fill bits are: a single '1' followed by zero or more '0' bits.
    /// Returns the number of fill bits (including the leading '1').
    // Was: Führt den Arbeitsschritt `count_fill_bits` für count fill bits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn count_fill_bits(pdu: &BitBuffer, pdu_len_bits: usize) -> usize {
        let start = pdu.get_raw_start();
        let mut index = pdu_len_bits as isize - 1;

        // Walk backwards from end looking for the '1' that marks start of fill
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while index >= 0 {
            let bit = pdu.peek_bits_startoffset(start + index as usize, 1);
            if let Some(bit) = bit {
                if bit == 0 {
                    index -= 1;
                } else {
                    // Found the '1' bit - fill bits are from here to end
                    return (pdu_len_bits as isize - index) as usize;
                }
            } else {
                break;
            }
        }

        // No fill bits found (all zeros or empty)
        0
    }

    /// Apply PDU association: truncate buffer to PDU boundary, strip fill bits,
    /// and check for remaining data that could be another PDU.
    ///
    /// Returns the remaining bits string if there's a "next block", None otherwise.
    // Was: Diese Funktion wendet Protokollnachricht (PDU) association.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    pub fn apply_pdu_association(pdu: &mut BitBuffer, length_ind: Option<u8>, has_fill_bits: bool) -> Option<String> {
        let Some(length_ind) = length_ind else {
            println!("    [No length_ind present - cannot apply PDU association]");
            return None;
        };

        let remaining_bits = pdu.get_len_remaining();
        let info = Self::interpret_length_ind(length_ind, remaining_bits);

        println!();
        println!("=== PDU Association ===");
        println!("length_ind: {} (0b{:06b})", length_ind, length_ind);

        if info.is_null_pdu {
            println!("    Null PDU");
            return None;
        }

        if info.is_frag_start {
            println!("    Fragmentation start (TL-SDU extends to subsequent MAC PDUs)");
            println!("    Remaining {} bits are fragment data", remaining_bits);
            return None;
        }

        if info.second_half_stolen {
            println!("    Second half slot stolen (STCH signalling)");
            return None;
        }

        let pdu_len_bits = info.pdu_len_bits;
        println!("    TM-SDU length: {} bits ({} bytes)", pdu_len_bits, length_ind);

        // Calculate fill bits if requested
        let fill_bits = if has_fill_bits {
            let fb = Self::count_fill_bits(pdu, pdu_len_bits);
            if fb > 0 {
                println!("    Fill bits detected: {} bits", fb);
            }
            fb
        } else {
            0
        };

        let sdu_len_bits = pdu_len_bits.saturating_sub(fill_bits);
        println!("    Effective SDU length: {} bits", sdu_len_bits);

        // Check what's left after this PDU
        let orig_end = pdu.get_raw_end();
        let pdu_start = pdu.get_raw_start();
        let next_pdu_start = pdu_start + pdu_len_bits;
        let remaining_after_pdu = orig_end.saturating_sub(next_pdu_start);

        if remaining_after_pdu >= 16 {
            // Minimum MAC PDU is ~16 bits (null PDU), so there could be another
            println!();
            println!("=== Next Block Available ===");
            println!("    {} bits remaining after this PDU", remaining_after_pdu);

            // Extract the remaining bits as a string
            let mut next_block = String::new();
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for i in 0..remaining_after_pdu {
                if let Some(bit) = pdu.peek_bits_startoffset(next_pdu_start + i, 1) {
                    next_block.push(if bit == 1 { '1' } else { '0' });
                }
            }

            println!("    Next block: {}", next_block);
            println!();
            println!("To decode next PDU, run:");
            println!("    pdu-tool <direction> tmv umac \"{}\"", next_block);

            return Some(next_block);
        } else if remaining_after_pdu > 0 {
            println!();
            println!("    {} bits remaining (too few for another PDU)", remaining_after_pdu);
        }

        None
    }

    /// Check if we should continue parsing more PDUs in this MAC block
    // Was: Diese Funktion prüft continue.
    // Warum: Fehler oder unzulässige Zustände werden dadurch früh erkannt.
    fn check_continue(pdu: &BitBuffer, orig_start: usize) -> bool {
        // If start was not updated, we also consider it end of message
        // If 16 or more bits remain (len of null pdu), we continue parsing
        if pdu.get_raw_start() != orig_start && pdu.get_len() >= 16 {
            println!();
            println!("--- Remaining {} bits, continuing parse ---", pdu.get_len_remaining());
            println!("Remaining: {}", pdu.dump_bin_full(true));
            println!();
            true
        } else {
            println!();
            println!("=== End of MAC block ===");
            false
        }
    }

    /// Print SDU data if available
    // Was: Führt den Arbeitsschritt `print_sdu` für print sdu aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn print_sdu(pdu: &mut BitBuffer, bit_len: usize, label: &str) {
        println!("{} length: {} bits ({} bytes)", label, bit_len, bit_len / 8);

        if let Some(sdu_bits) = pdu.peek_bits(bit_len) {
            println!("{}: {:0width$b}", label, sdu_bits, width = bit_len);
        }
        pdu.read_bits(bit_len); // Advance past SDU
    }
}
