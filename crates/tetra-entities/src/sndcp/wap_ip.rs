// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! WAP 1.x/2.0 WTP/WSP adapter over IPv4/UDP.

use super::ip::{IPV4_PROTOCOL_UDP, IPV4_UDP_HEADER_BYTES, IpError, build_ipv4_udp_npdu, parse_ipv4_packet, parse_udp_datagram};
use super::wap_portal::{
    WapMarkup, WapPage, WapPortalRoute, parse_portal_path, render_portal_page,
};
use super::wap_status::{WapStatusSnapshot, render_raw_xhtml};

// Was: Legt den festen Wert `WTP_PDU_INVOKE` für wtp Protokollnachricht (PDU) invoke fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WTP_PDU_INVOKE: u8 = 1;
// Was: Legt den festen Wert `WTP_PDU_ACK` für wtp Protokollnachricht (PDU) ack fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WTP_PDU_ACK: u8 = 3;
// Was: Legt den festen Wert `WTP_PDU_ABORT` für wtp Protokollnachricht (PDU) abort fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WTP_PDU_ABORT: u8 = 4;
// Was: Legt den festen Wert `WTP_TID_RESPONSE_FLAG` für wtp tid response flag fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WTP_TID_RESPONSE_FLAG: u16 = 0x8000;
// Was: Legt den festen Wert `WTP_TID_VALUE_MASK` für wtp tid value mask fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WTP_TID_VALUE_MASK: u16 = 0x7fff;
// Was: Legt den festen Wert `WSP_CONNECT` für wsp connect fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_CONNECT: u8 = 0x01;
// Was: Legt den festen Wert `WSP_REPLY` für wsp reply fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_REPLY: u8 = 0x04;
// Was: Legt den festen Wert `WSP_RESUME` für wsp resume fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_RESUME: u8 = 0x09;
// Was: Legt den festen Wert `WSP_GET` für wsp get fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_GET: u8 = 0x40;
// Was: Legt den festen Wert `WSP_CONTENT_WML` für wsp content wml fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_CONTENT_WML: u8 = 0x88;
// Was: Legt den festen Wert `WSP_CONTENT_XHTML` für wsp content xhtml fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_CONTENT_XHTML: u8 = 0xc5;
// Was: Legt den festen Wert `WSP_SDU_CAP` für wsp sdu cap fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const WSP_SDU_CAP: usize = 545;

#[derive(Debug, Clone, Copy)]
// Was: Bündelt die zusammengehörigen Werte für WAP-Dienst endpoint in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct WapEndpoint {
    pub address: [u8; 4],
    pub port: u16,
    pub ttl: u8,
}

#[derive(Debug, Clone, Copy)]
// Was: Bündelt die zusammengehörigen Werte für WAP-Dienst policy in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct WapPolicy {
    pub accept_empty_probe: bool,
    pub accept_root_path: bool,
    pub accept_status_path: bool,
    pub accept_status_wml_path: bool,
    pub max_request_payload_bytes: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für WAP-Dienst error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum WapError {
    Ip(IpError),
    WrongDestination,
    WrongPort,
    PayloadTooLarge,
    UnsupportedPayload,
    UnsupportedPath,
    NoResponseRequired,
}

// Was: Implementiert das zugehörige Verhalten für `From<IpError> for WapError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl From<IpError> for WapError {
    // Was: Wandelt Eingangsdaten in den vorgesehenen Arbeitsschritt um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from(value: IpError) -> Self {
        Self::Ip(value)
    }
}

// Was: Diese Funktion liest uintvar.
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_uintvar(bytes: &[u8], offset: &mut usize) -> Option<usize> {
    let mut value = 0usize;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for _ in 0..5 {
        let b = *bytes.get(*offset)?;
        *offset += 1;
        value = value.checked_shl(7)?.checked_add(usize::from(b & 0x7f))?;
        if b & 0x80 == 0 {
            return Some(value);
        }
    }
    None
}

// Was: Diese Funktion schreibt uintvar.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
fn write_uintvar(mut value: usize, out: &mut Vec<u8>) {
    let mut tmp = [0u8; 5];
    let mut i = tmp.len();
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        i -= 1;
        tmp[i] = (value & 0x7f) as u8;
        value >>= 7;
        if value == 0 {
            break;
        }
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for pos in i..tmp.len() {
        let mut b = tmp[pos];
        if pos + 1 != tmp.len() {
            b |= 0x80;
        }
        out.push(b);
    }
}

// Was: Führt den Arbeitsschritt `skip_tpis` für skip tpis aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn skip_tpis(payload: &[u8], mut offset: usize, con: bool) -> Option<usize> {
    if !con {
        return Some(offset);
    }
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let h = *payload.get(offset)?;
        let cont = h & 0x80 != 0;
        if h & 0x04 != 0 {
            let len = usize::from(*payload.get(offset + 1)?);
            offset = offset.checked_add(2 + len)?;
        } else {
            offset = offset.checked_add(1 + usize::from(h & 0x03))?;
        }
        if !cont {
            return Some(offset);
        }
    }
}

// Was: Diese Funktion liest und prüft connect caps.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_connect_caps(wsp: &[u8]) -> Option<Vec<(u8, usize)>> {
    if wsp.first().copied()? != WSP_CONNECT {
        return None;
    }
    let mut off = 2; // type + version
    let caps_len = read_uintvar(wsp, &mut off)?;
    let headers_len = read_uintvar(wsp, &mut off)?;
    if off.checked_add(caps_len + headers_len)? > wsp.len() {
        return None;
    }
    let end = off + caps_len;
    let mut caps = Vec::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while off < end {
        let len = read_uintvar(wsp, &mut off)?;
        if len == 0 || off + len > end {
            return None;
        }
        let id = wsp[off];
        let params = &wsp[off + 1..off + len];
        let mut p = 0;
        let value = if params.is_empty() {
            0
        } else {
            let value = read_uintvar(params, &mut p).unwrap_or(0);
            if p != params.len() {
                off += len;
                continue;
            }
            value
        };
        caps.push((id, value));
        off += len;
    }
    Some(caps)
}

// Was: Diese Funktion verbindet reply.
// Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
fn connect_reply(caps: &[(u8, usize)]) -> Vec<u8> {
    let mut encoded = Vec::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for (id, requested) in caps {
        if !matches!(*id, 0x80 | 0x81) {
            continue;
        }
        let mut params = Vec::new();
        write_uintvar((*requested).min(WSP_SDU_CAP), &mut params);
        write_uintvar(1 + params.len(), &mut encoded);
        encoded.push(*id);
        encoded.extend_from_slice(&params);
    }
    let mut out = vec![0x02, 0x01];
    write_uintvar(encoded.len(), &mut out);
    out.push(0x00);
    out.extend_from_slice(&encoded);
    out
}

// Was: Diese Funktion liest und prüft path from wsp get.
// Warum: Ungültige oder unvollständige Eingaben werden dadurch erkannt, bevor sie den Systemzustand beeinflussen.
fn parse_path_from_wsp_get(wsp: &[u8]) -> Option<String> {
    let first = *wsp.first()?;
    if first != WSP_GET && !(0x50..=0x5f).contains(&first) {
        return None;
    }
    // WSP GET encodes the URI as: method octet, uintvar URI length, raw URI bytes.
    let mut off = 1;
    let uri_len = read_uintvar(wsp, &mut off)?;
    let end = off.checked_add(uri_len)?;
    let uri = std::str::from_utf8(wsp.get(off..end)?).ok()?;
    Some(uri.trim().to_string())
}

// Was: Führt den Arbeitsschritt `uri_path` für uri path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn uri_path(uri: &str) -> &str {
    let uri = uri.trim();
    if let Some(rest) = uri.strip_prefix("http://").or_else(|| uri.strip_prefix("https://")) {
        return rest.find('/').map(|idx| &rest[idx..]).unwrap_or("/");
    }
    uri
}

// Was: Führt den Arbeitsschritt `normalize_path` für normalize path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn normalize_path(uri: &str) -> String {
    let path = uri_path(uri).trim();
    let path = path.split(['?', '#']).next().unwrap_or(path).trim();
    let path = path.strip_prefix("./").unwrap_or(path);
    if path.is_empty() {
        "/".to_string()
    } else if path.starts_with('/') {
        path.to_string()
    } else {
        format!("/{path}")
    }
}

// Was: Ermittelt eine erlaubte XHTML-/WML-Portalseite aus URI und Richtlinie.
// Warum: Alle Portalpfade nutzen dieselbe Zugriffskontrolle und die alten Status-Sektor-Links bleiben kompatibel.
fn portal_route(uri: &str, policy: WapPolicy) -> Option<WapPortalRoute> {
    let base = normalize_path(uri);
    let mut route = parse_portal_path(&base)?;

    // Kompatibilität zum bisherigen Status-Sektor: ?s=1 zeigt die Health-Seite.
    if route.page == WapPage::Status && uri.split('#').next().unwrap_or(uri).contains("?s=1") {
        route.page = WapPage::Health;
    }

    let allowed = match (route.markup, route.page) {
        (WapMarkup::Xhtml, WapPage::Home) => policy.accept_root_path,
        (WapMarkup::Xhtml, _) => policy.accept_status_path,
        (WapMarkup::Wml, _) => policy.accept_status_wml_path,
    };
    allowed.then_some(route)
}

// Was: Erzeugt die WSP-Antwort für eine NetCore-Portalseite.
// Warum: Content-Type, Seitenformat und Openwave-Größenlimit bleiben dadurch immer synchron.
fn wsp_status_reply(path: &str, policy: WapPolicy, snapshot: &WapStatusSnapshot) -> Result<Vec<u8>, WapError> {
    let route = portal_route(path, policy).ok_or(WapError::UnsupportedPath)?;
    let body = render_portal_page(route, snapshot);
    let ct = match route.markup {
        WapMarkup::Wml => WSP_CONTENT_WML,
        WapMarkup::Xhtml => WSP_CONTENT_XHTML,
    };
    let mut out = vec![WSP_REPLY, 0x20, 0x01, ct];
    out.extend_from_slice(body.as_bytes());
    Ok(out)
}

// Was: Führt den Arbeitsschritt `wtp_result` für wtp result aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn wtp_result(tid: u16, wsp: &[u8]) -> Vec<u8> {
    let tid = (tid & WTP_TID_VALUE_MASK) | WTP_TID_RESPONSE_FLAG;
    let mut out = vec![0x12];
    out.extend_from_slice(&tid.to_be_bytes());
    out.extend_from_slice(wsp);
    out
}

// Was: Diese Funktion verarbeitet wtp.
// Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
fn handle_wtp(payload: &[u8], policy: WapPolicy, snapshot: &WapStatusSnapshot) -> Result<Option<Vec<u8>>, WapError> {
    if payload.len() < 3 {
        return Err(WapError::UnsupportedPayload);
    }
    let pdu_type = (payload[0] >> 3) & 0x0f;
    let tid = u16::from_be_bytes([payload[1], payload[2]]) & WTP_TID_VALUE_MASK;
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match pdu_type {
        WTP_PDU_ACK | WTP_PDU_ABORT => return Ok(None),
        WTP_PDU_INVOKE if payload.len() >= 4 => {}
        WTP_PDU_INVOKE => return Err(WapError::UnsupportedPayload),
        _ => return Err(WapError::UnsupportedPayload),
    }
    let invoke = payload[3];
    if invoke & 0xc0 != 0 || invoke & 0x0c != 0 || invoke & 0x03 != 2 {
        return Err(WapError::UnsupportedPayload);
    }
    let con = payload[0] & 0x80 != 0;
    let wsp_start = skip_tpis(payload, 4, con).ok_or(WapError::UnsupportedPayload)?;
    let wsp = payload.get(wsp_start..).ok_or(WapError::UnsupportedPayload)?;
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let response = match wsp.first().copied() {
        Some(WSP_CONNECT) => connect_reply(&parse_connect_caps(wsp).ok_or(WapError::UnsupportedPayload)?),
        Some(WSP_RESUME) => vec![WSP_REPLY, 0x20, 0x00],
        Some(method) if method == WSP_GET || (0x50..=0x5f).contains(&method) => {
            let path = parse_path_from_wsp_get(wsp).ok_or(WapError::UnsupportedPayload)?;
            wsp_status_reply(&path, policy, snapshot)?
        }
        _ => return Err(WapError::UnsupportedPayload),
    };
    Ok(Some(wtp_result(tid, &response)))
}

// Was: Führt den Arbeitsschritt `plain_get_path` für plain get path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn plain_get_path(payload: &[u8]) -> Option<&str> {
    let text = std::str::from_utf8(payload).ok()?;
    let mut parts = text.split_whitespace();
    if parts.next()? != "GET" {
        return None;
    }
    parts.next()
}

// Was: Diese Funktion erstellt response npdu.
// Warum: Die Erzeugung bleibt damit reproduzierbar und von der restlichen Verarbeitung getrennt.
pub fn build_response_npdu(
    request_npdu: &[u8],
    endpoint: WapEndpoint,
    policy: WapPolicy,
    snapshot: &WapStatusSnapshot,
) -> Result<Option<Vec<u8>>, WapError> {
    let ip = parse_ipv4_packet(request_npdu)?;
    if ip.protocol != IPV4_PROTOCOL_UDP {
        return Err(WapError::Ip(IpError::UnsupportedProtocol(ip.protocol)));
    }
    if ip.destination != endpoint.address {
        return Err(WapError::WrongDestination);
    }
    let udp = parse_udp_datagram(ip.payload)?;
    if udp.destination_port != endpoint.port {
        return Err(WapError::WrongPort);
    }
    if udp.payload.len() > policy.max_request_payload_bytes {
        return Err(WapError::PayloadTooLarge);
    }

    let response_payload = if udp.payload.is_empty() {
        if !policy.accept_empty_probe {
            return Err(WapError::UnsupportedPayload);
        }
        Some(render_raw_xhtml(snapshot, 576 - IPV4_UDP_HEADER_BYTES).into_bytes())
    } else {
        // Binary WTP first. Byte 0 encodes a known WTP type in bits 6..3.
        let wtp_type = (udp.payload[0] >> 3) & 0x0f;
        if matches!(wtp_type, WTP_PDU_INVOKE | WTP_PDU_ACK | WTP_PDU_ABORT) {
            handle_wtp(udp.payload, policy, snapshot)?
        } else if let Some(path) = plain_get_path(udp.payload) {
            let route = portal_route(path, policy).ok_or(WapError::UnsupportedPath)?;
            Some(render_portal_page(route, snapshot).into_bytes())
        } else {
            return Err(WapError::UnsupportedPayload);
        }
    };

    let Some(response_payload) = response_payload else {
        return Ok(None);
    };
    let npdu = build_ipv4_udp_npdu(
        endpoint.address,
        ip.source,
        endpoint.port,
        udp.source_port,
        ip.identification.wrapping_add(1),
        endpoint.ttl,
        &response_payload,
    )?;
    if npdu.len() > 576 {
        return Err(WapError::PayloadTooLarge);
    }
    Ok(Some(npdu))
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    // Was: Diese Funktion erzeugt den vorgesehenen Arbeitsschritt.
    // Warum: Die Oberfläche und andere Dienste erhalten dadurch eine in sich stimmige Momentaufnahme.
    fn snapshot() -> WapStatusSnapshot {
        WapStatusSnapshot {
            title: "NetCore-Tetra".into(),
            state: "ONLINE".into(),
            version: "v1.3.0".into(),
            registered_ms: 2,
            attached_groups: 1,
            active_calls: 0,
            queued_sds: 0,
            uptime_secs: 93784,
            last_activity: "WAP 4010001".into(),
            health: "OK".into(),
        }
    }

    #[test]
    // Was: Diese Funktion verbindet reply matches reference vector.
    // Warum: Der Verbindungsaufbau wird dadurch zentral überwacht und kann sauber fehlschlagen.
    fn connect_reply_matches_reference_vector() {
        let caps = vec![(0x80, 327_680), (0x81, 327_680), (0x82, 0xf0)];
        assert_eq!(
            wtp_result(0x13cc, &connect_reply(&caps)),
            vec![0x12, 0x93, 0xcc, 0x02, 0x01, 0x08, 0x00, 0x03, 0x80, 0x84, 0x21, 0x03, 0x81, 0x84, 0x21]
        );
    }



    #[test]
    // Was: Führt den Arbeitsschritt `full_connect_invoke_produces_reference_reply` für full connect invoke produces reference reply aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn full_connect_invoke_produces_reference_reply() {
        // Invoke, TID 0x13cc, class 2, WSP Connect v1.0. Only the two SDU
        // capabilities are echoed; both are clamped from 327680 to 545.
        let request = [
            0x0b, 0x13, 0xcc, 0x12, 0x01, 0x10, 0x0a, 0x00,
            0x04, 0x80, 0x94, 0x80, 0x00,
            0x04, 0x81, 0x94, 0x80, 0x00,
        ];
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        assert_eq!(
            handle_wtp(&request, policy, &snapshot()).unwrap().unwrap(),
            vec![0x12, 0x93, 0xcc, 0x02, 0x01, 0x08, 0x00, 0x03, 0x80, 0x84, 0x21, 0x03, 0x81, 0x84, 0x21]
        );
    }

    #[test]
    // Was: Führt den Arbeitsschritt `wsp_get_uses_uintvar_uri_and_returns_xhtml` für wsp get uses uintvar uri and returns und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn wsp_get_uses_uintvar_uri_and_returns_xhtml() {
        let uri = b"http://10.0.0.1:9200/status.xhtml";
        let mut request = vec![0x0b, 0x12, 0x34, 0x12, WSP_GET, uri.len() as u8];
        request.extend_from_slice(uri);
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        let response = handle_wtp(&request, policy, &snapshot()).unwrap().unwrap();
        assert_eq!(&response[..7], &[0x12, 0x92, 0x34, 0x04, 0x20, 0x01, WSP_CONTENT_XHTML]);
        assert!(response.len() <= 3 + 4 + 104);
    }

    #[test]
    fn readable_wml_alias_returns_wml_content_type() {
        let uri = b"/media-library.wml";
        let mut request = vec![0x0b, 0x12, 0x35, 0x12, WSP_GET, uri.len() as u8];
        request.extend_from_slice(uri);
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        let response = handle_wtp(&request, policy, &snapshot()).unwrap().unwrap();
        assert_eq!(response[6], WSP_CONTENT_WML);
        assert!(std::str::from_utf8(&response[7..]).unwrap().starts_with("<wml><card><p>"));
    }

    #[test]
    fn legacy_status_sector_query_maps_to_health_page() {
        let uri = b"/status.xhtml?s=1";
        let mut request = vec![0x0b, 0x12, 0x36, 0x12, WSP_GET, uri.len() as u8];
        request.extend_from_slice(uri);
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        let response = handle_wtp(&request, policy, &snapshot()).unwrap().unwrap();
        assert_eq!(response[6], WSP_CONTENT_XHTML);
        assert!(std::str::from_utf8(&response[7..]).unwrap().contains("Health"));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `endpoint_response_swaps_addresses_ports_and_increments_id` für endpoint response swaps addresses ports and increments und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn endpoint_response_swaps_addresses_ports_and_increments_id() {
        let endpoint = WapEndpoint { address: [10, 0, 0, 1], port: 9200, ttl: 32 };
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        let request = build_ipv4_udp_npdu(
            [10, 0, 0, 226],
            endpoint.address,
            49152,
            endpoint.port,
            0x2222,
            64,
            b"GET /status.xhtml HTTP/1.0\r\n\r\n",
        )
        .unwrap();
        let response = build_response_npdu(&request, endpoint, policy, &snapshot()).unwrap().unwrap();
        let ip = parse_ipv4_packet(&response).unwrap();
        let udp = parse_udp_datagram(ip.payload).unwrap();
        assert_eq!(ip.source, [10, 0, 0, 1]);
        assert_eq!(ip.destination, [10, 0, 0, 226]);
        assert_eq!(ip.identification, 0x2223);
        assert_eq!(ip.ttl, 32);
        assert_eq!(udp.source_port, 9200);
        assert_eq!(udp.destination_port, 49152);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `three_octet_invoke_is_rejected_without_panicking` für three octet invoke is rejected without panicking aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn three_octet_invoke_is_rejected_without_panicking() {
        let policy = WapPolicy {
            accept_empty_probe: true,
            accept_root_path: true,
            accept_status_path: true,
            accept_status_wml_path: true,
            max_request_payload_bytes: 1024,
        };
        assert_eq!(handle_wtp(&[0x08, 0x00, 0x01], policy, &snapshot()), Err(WapError::UnsupportedPayload));
    }

    #[test]
    // Was: Führt den Arbeitsschritt `ack_needs_no_response` für ack needs no response aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn ack_needs_no_response() {
        let policy = WapPolicy { accept_empty_probe: true, accept_root_path: true, accept_status_path: true, accept_status_wml_path: true, max_request_payload_bytes: 1024 };
        assert_eq!(handle_wtp(&[0x18, 0x13, 0xcc], policy, &snapshot()).unwrap(), None);
    }
}
