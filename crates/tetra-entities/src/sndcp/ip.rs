// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Strict IPv4/transport primitives used by SNDCP, WAP and the Linux packet gateway.

// Was: Legt den festen Wert `IPV4_PROTOCOL_ICMP` für ipv4 protocol icmp fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const IPV4_PROTOCOL_ICMP: u8 = 1;
// Was: Legt den festen Wert `IPV4_PROTOCOL_TCP` für ipv4 protocol tcp fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const IPV4_PROTOCOL_TCP: u8 = 6;
// Was: Legt den festen Wert `IPV4_PROTOCOL_UDP` für ipv4 protocol UDP fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const IPV4_PROTOCOL_UDP: u8 = 17;
// Was: Legt den festen Wert `IPV4_HEADER_BYTES` für ipv4 header bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const IPV4_HEADER_BYTES: usize = 20;
// Was: Legt den festen Wert `UDP_HEADER_BYTES` für UDP header bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const UDP_HEADER_BYTES: usize = 8;
// Was: Legt den festen Wert `IPV4_UDP_HEADER_BYTES` für ipv4 UDP header bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const IPV4_UDP_HEADER_BYTES: usize = IPV4_HEADER_BYTES + UDP_HEADER_BYTES;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für IP-Daten error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum IpError {
    TooShort,
    UnsupportedVersion(u8),
    UnsupportedProtocol(u8),
    InvalidHeaderLength,
    InvalidTotalLength,
    InvalidChecksum,
    Fragmented,
    PacketTooLarge,
    MtuTooSmall,
    DontFragment,
    OverlappingFragments,
    InvalidFragment,
    InvalidOptions,
    InvalidUdpLength,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für ipv4 Datenpaket in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct Ipv4Packet<'a> {
    pub source: [u8; 4],
    pub destination: [u8; 4],
    pub protocol: u8,
    pub identification: u16,
    pub ttl: u8,
    pub header_len: usize,
    pub total_len: usize,
    pub dont_fragment: bool,
    pub more_fragments: bool,
    pub fragment_offset_bytes: usize,
    pub payload: &'a [u8],
    packet: &'a [u8],
}

// Was: Implementiert das zugehörige Verhalten für `Ipv4Packet<'a>`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl<'a> Ipv4Packet<'a> {
    // Was: Führt den Arbeitsschritt `dscp` für dscp aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn dscp(&self) -> u8 {
        self.packet[1] >> 2
    }

    // Was: Prüft, ob fragmented zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_fragmented(&self) -> bool {
        self.more_fragments || self.fragment_offset_bytes != 0
    }

    // Was: Führt den Arbeitsschritt `octets` für octets aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn octets(&self) -> &'a [u8] {
        &self.packet[..self.total_len]
    }
}

#[derive(Debug, Clone, Copy)]
// Was: Bündelt die zusammengehörigen Werte für UDP datagram in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct UdpDatagram<'a> {
    pub source_port: u16,
    pub destination_port: u16,
    pub payload: &'a [u8],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für transport ports in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TransportPorts {
    pub source: u16,
    pub destination: u16,
}

// Was: Führt den Arbeitsschritt `internet_checksum` für internet checksum aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn internet_checksum(data: &[u8]) -> u16 {
    let mut sum = 0u32;
    let mut chunks = data.chunks_exact(2);
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for chunk in &mut chunks {
        sum = sum.wrapping_add(u16::from_be_bytes([chunk[0], chunk[1]]) as u32);
    }
    if let Some(last) = chunks.remainder().first() {
        sum = sum.wrapping_add((*last as u32) << 8);
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while sum >> 16 != 0 {
        sum = (sum & 0xffff) + (sum >> 16);
    }
    !(sum as u16)
}

/// Parse an IPv4 packet including fragments. Header checksum and total length are strict.
// Was: Diese Funktion liest und prüft ipv4 Datenpaket any.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
pub fn parse_ipv4_packet_any(packet: &[u8]) -> Result<Ipv4Packet<'_>, IpError> {
    if packet.len() < 20 {
        return Err(IpError::TooShort);
    }
    let version = packet[0] >> 4;
    if version != 4 {
        return Err(IpError::UnsupportedVersion(version));
    }
    let header_len = usize::from(packet[0] & 0x0f) * 4;
    if header_len < 20 || header_len > packet.len() {
        return Err(IpError::InvalidHeaderLength);
    }
    let total_len = usize::from(u16::from_be_bytes([packet[2], packet[3]]));
    if total_len < header_len || total_len > packet.len() {
        return Err(IpError::InvalidTotalLength);
    }
    if internet_checksum(&packet[..header_len]) != 0 {
        return Err(IpError::InvalidChecksum);
    }
    let flags_offset = u16::from_be_bytes([packet[6], packet[7]]);
    if flags_offset & 0x8000 != 0 {
        return Err(IpError::InvalidFragment);
    }
    let dont_fragment = flags_offset & 0x4000 != 0;
    let more_fragments = flags_offset & 0x2000 != 0;
    let fragment_offset_bytes = usize::from(flags_offset & 0x1fff) * 8;
    let payload_len = total_len - header_len;
    if more_fragments && payload_len % 8 != 0 {
        return Err(IpError::InvalidFragment);
    }
    if fragment_offset_bytes.checked_add(payload_len).is_none_or(|end| end > u16::MAX as usize) {
        return Err(IpError::InvalidFragment);
    }
    Ok(Ipv4Packet {
        source: [packet[12], packet[13], packet[14], packet[15]],
        destination: [packet[16], packet[17], packet[18], packet[19]],
        protocol: packet[9],
        identification: u16::from_be_bytes([packet[4], packet[5]]),
        ttl: packet[8],
        header_len,
        total_len,
        dont_fragment,
        more_fragments,
        fragment_offset_bytes,
        payload: &packet[header_len..total_len],
        packet,
    })
}

/// Parse only a complete, unfragmented IPv4 packet.
// Was: Diese Funktion liest und prüft ipv4 Datenpaket.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
pub fn parse_ipv4_packet(packet: &[u8]) -> Result<Ipv4Packet<'_>, IpError> {
    let parsed = parse_ipv4_packet_any(packet)?;
    if parsed.is_fragmented() {
        return Err(IpError::Fragmented);
    }
    Ok(parsed)
}

// Was: Diese Funktion liest und prüft UDP datagram.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
pub fn parse_udp_datagram(segment: &[u8]) -> Result<UdpDatagram<'_>, IpError> {
    if segment.len() < 8 {
        return Err(IpError::TooShort);
    }
    let length = usize::from(u16::from_be_bytes([segment[4], segment[5]]));
    if length < 8 || length > segment.len() {
        return Err(IpError::InvalidUdpLength);
    }
    Ok(UdpDatagram {
        source_port: u16::from_be_bytes([segment[0], segment[1]]),
        destination_port: u16::from_be_bytes([segment[2], segment[3]]),
        payload: &segment[8..length],
    })
}

/// Extract TCP/UDP ports from an initial (offset-zero) fragment or full packet.
// Was: Führt den Arbeitsschritt `transport_ports` für transport ports aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn transport_ports(packet: &Ipv4Packet<'_>) -> Option<TransportPorts> {
    if packet.fragment_offset_bytes != 0 || !matches!(packet.protocol, IPV4_PROTOCOL_TCP | IPV4_PROTOCOL_UDP) || packet.payload.len() < 4 {
        return None;
    }
    Some(TransportPorts {
        source: u16::from_be_bytes([packet.payload[0], packet.payload[1]]),
        destination: u16::from_be_bytes([packet.payload[2], packet.payload[3]]),
    })
}

// Was: Diese Funktion erstellt ipv4 Datenpaket.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_ipv4_packet(
    source: [u8; 4],
    destination: [u8; 4],
    protocol: u8,
    identification: u16,
    ttl: u8,
    dont_fragment: bool,
    payload: &[u8],
) -> Result<Vec<u8>, IpError> {
    let total_len = 20usize.checked_add(payload.len()).ok_or(IpError::PacketTooLarge)?;
    if total_len > u16::MAX as usize {
        return Err(IpError::PacketTooLarge);
    }
    let mut packet = vec![0u8; total_len];
    packet[0] = 0x45;
    packet[2..4].copy_from_slice(&(total_len as u16).to_be_bytes());
    packet[4..6].copy_from_slice(&identification.to_be_bytes());
    packet[6..8].copy_from_slice(&(if dont_fragment { 0x4000u16 } else { 0 }).to_be_bytes());
    packet[8] = ttl.max(1);
    packet[9] = protocol;
    packet[12..16].copy_from_slice(&source);
    packet[16..20].copy_from_slice(&destination);
    packet[20..].copy_from_slice(payload);
    let checksum = internet_checksum(&packet[..20]);
    packet[10..12].copy_from_slice(&checksum.to_be_bytes());
    Ok(packet)
}

// Was: Diese Funktion erstellt ipv4 UDP npdu.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_ipv4_udp_npdu(
    source: [u8; 4],
    destination: [u8; 4],
    source_port: u16,
    destination_port: u16,
    identification: u16,
    ttl: u8,
    payload: &[u8],
) -> Result<Vec<u8>, IpError> {
    let udp_len = 8usize.checked_add(payload.len()).ok_or(IpError::PacketTooLarge)?;
    if udp_len > u16::MAX as usize {
        return Err(IpError::PacketTooLarge);
    }
    let mut udp = vec![0u8; udp_len];
    udp[0..2].copy_from_slice(&source_port.to_be_bytes());
    udp[2..4].copy_from_slice(&destination_port.to_be_bytes());
    udp[4..6].copy_from_slice(&(udp_len as u16).to_be_bytes());
    // UDP checksum zero is legal for IPv4 and preserves legacy WAP compatibility.
    udp[6..8].copy_from_slice(&0u16.to_be_bytes());
    udp[8..].copy_from_slice(payload);
    build_ipv4_packet(source, destination, IPV4_PROTOCOL_UDP, identification, ttl, false, &udp)
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Führt den Arbeitsschritt `hex` für hex aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn hex(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }

    #[test]
    // Was: Führt den Arbeitsschritt `builds_reference_ipv4_udp_packet` für builds reference ipv4 UDP Datenpaket aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn builds_reference_ipv4_udp_packet() {
        let p = build_ipv4_udp_npdu([192, 0, 2, 1], [192, 0, 2, 2], 49152, 9200, 7, 32, b"wap").unwrap();
        assert_eq!(hex(&p), "4500001f00070000201116c4c0000201c0000202c00023f0000b0000776170");
        let ip = parse_ipv4_packet(&p).unwrap();
        let udp = parse_udp_datagram(ip.payload).unwrap();
        assert_eq!(udp.payload, b"wap");
        assert_eq!(&p[26..28], &[0, 0]);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `options_and_fragment_metadata_parse` für options and fragment metadata parse aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn options_and_fragment_metadata_parse() {
        let mut p = build_ipv4_packet([10, 0, 0, 2], [10, 0, 0, 1], IPV4_PROTOCOL_UDP, 9, 32, false, b"12345678").unwrap();
        p[6..8].copy_from_slice(&0x2001u16.to_be_bytes());
        p[10..12].copy_from_slice(&0u16.to_be_bytes());
        let checksum = internet_checksum(&p[..20]);
        p[10..12].copy_from_slice(&checksum.to_be_bytes());
        let ip = parse_ipv4_packet_any(&p).unwrap();
        assert!(ip.more_fragments);
        assert_eq!(ip.fragment_offset_bytes, 8);
        assert_eq!(parse_ipv4_packet(&p), Err(IpError::Fragmented));
    }
}
