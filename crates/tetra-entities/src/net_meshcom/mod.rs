// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! MeshCom external UDP integration.
//!
//! MeshCom nodes can expose an external-client UDP interface that exchanges JSON packets,
//! commonly on UDP/1799. FlowStation listens for received `msg`, `pos`, and `tele` packets
//! and keeps a small runtime directory/log for the dashboard. Outbound text messages are sent
//! by the dashboard API using the same documented JSON format.

use std::collections::{BTreeSet, VecDeque};
use std::net::UdpSocket;
use std::thread;
use std::time::Duration;

use serde_json::Value;
use tetra_config::bluestation::{CfgMeshcom, MeshcomMessageStatus, MeshcomNodeStatus, MeshcomRuntimeStatus, SharedConfig};

use crate::net_control::commands::ControlCommand;
use crate::net_geoalarm::GeoAlarmSink;
use crate::net_snom::SnomNotifySink;
use crate::net_telegram::TelegramAlertSink;
use crate::net_telemetry::{TelemetryEvent, TelemetrySink};
use crate::tpg2200::build_sds_text_payload;

// Was: Vergibt für cmd sender einen fachlich verständlichen Typnamen.
// Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
type CmdSender = crossbeam_channel::Sender<ControlCommand>;

// Was: Legt den festen Wert `UDP_READ_TIMEOUT` für UDP read timeout fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const UDP_READ_TIMEOUT: Duration = Duration::from_secs(1);
// Was: Legt den festen Wert `DISABLED_SLEEP` für disabled sleep fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const DISABLED_SLEEP: Duration = Duration::from_secs(1);
// Was: Legt den festen Wert `ERROR_SLEEP` für error sleep fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const ERROR_SLEEP: Duration = Duration::from_secs(5);

// Was: Diese Funktion startet meshcom Hintergrundverarbeitung.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_meshcom_worker(
    cfg: SharedConfig,
    cmce_cmd_tx: Option<CmdSender>,
    telegram_sink: Option<TelegramAlertSink>,
    snom_sink: Option<SnomNotifySink>,
    geoalarm_sink: Option<GeoAlarmSink>,
    telemetry_sink: Option<TelemetrySink>,
) -> Option<thread::JoinHandle<()>> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match thread::Builder::new()
        .name("meshcom-worker".into())
        .spawn(move || MeshcomWorker::new(cfg, cmce_cmd_tx, telegram_sink, snom_sink, geoalarm_sink, telemetry_sink).run())
    {
        Ok(handle) => Some(handle),
        Err(err) => {
            tracing::warn!("MeshCom: failed to spawn worker thread: {}", err);
            None
        }
    }
}

// Was: Bündelt die zusammengehörigen Werte für meshcom Hintergrundverarbeitung in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct MeshcomWorker {
    cfg: SharedConfig,
    cmce_cmd_tx: Option<CmdSender>,
    telegram_sink: Option<TelegramAlertSink>,
    snom_sink: Option<SnomNotifySink>,
    geoalarm_sink: Option<GeoAlarmSink>,
    telemetry_sink: Option<TelemetrySink>,
    socket: Option<UdpSocket>,
    bind_key: String,
    rx_packets: u64,
    nodes: Vec<MeshcomNodeStatus>,
    messages: VecDeque<MeshcomMessageStatus>,
    last_enabled: Option<bool>,
}

// Was: Implementiert das zugehörige Verhalten für `MeshcomWorker`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl MeshcomWorker {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    fn new(
        cfg: SharedConfig,
        cmce_cmd_tx: Option<CmdSender>,
        telegram_sink: Option<TelegramAlertSink>,
        snom_sink: Option<SnomNotifySink>,
        geoalarm_sink: Option<GeoAlarmSink>,
        telemetry_sink: Option<TelemetrySink>,
    ) -> Self {
        Self {
            cfg,
            cmce_cmd_tx,
            telegram_sink,
            snom_sink,
            geoalarm_sink,
            telemetry_sink,
            socket: None,
            bind_key: String::new(),
            rx_packets: 0,
            nodes: Vec::new(),
            messages: VecDeque::new(),
            last_enabled: None,
        }
    }

    // Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
    // Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
    fn run(&mut self) {
        // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
        // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
        loop {
            let meshcom = self.cfg.effective_meshcom();
            if !meshcom.enabled {
                if self.last_enabled != Some(false) {
                    tracing::info!("MeshCom UDP integration disabled");
                    self.last_enabled = Some(false);
                }
                self.socket = None;
                self.bind_key.clear();
                self.publish_status(&meshcom, None, None);
                thread::sleep(DISABLED_SLEEP);
                continue;
            }

            if self.last_enabled != Some(true) {
                tracing::info!(
                    "MeshCom UDP integration enabled (bind={}:{} tx={}:{} forward_sds={} forward_sip={} forward_telegram={})",
                    meshcom.bind_addr,
                    meshcom.bind_port,
                    meshcom.tx_host,
                    meshcom.tx_port,
                    meshcom.forward_sds,
                    meshcom.forward_sip,
                    meshcom.forward_telegram
                );
                self.last_enabled = Some(true);
            }

            if let Err(err) = self.ensure_socket(&meshcom) {
                tracing::warn!("MeshCom: {}", err);
                self.publish_status(&meshcom, None, Some(err));
                thread::sleep(ERROR_SLEEP);
                continue;
            }

            let mut buf = [0u8; 8192];
            let Some(socket) = self.socket.as_ref() else {
                thread::sleep(ERROR_SLEEP);
                continue;
            };
            let recv = socket.recv_from(&mut buf);

            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match recv {
                Ok((len, from)) => {
                    let text = String::from_utf8_lossy(&buf[..len]).trim().to_string();
                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match self.handle_packet(&meshcom, &text) {
                        Ok(()) => {
                            tracing::debug!("MeshCom: received {} bytes from {}", len, from);
                            self.publish_status(&meshcom, Some(now_stamp()), None);
                        }
                        Err(err) => {
                            tracing::warn!("MeshCom: dropping invalid UDP packet from {}: {}", from, err);
                            self.publish_status(&meshcom, None, Some(err));
                        }
                    }
                }
                Err(err) if err.kind() == std::io::ErrorKind::WouldBlock || err.kind() == std::io::ErrorKind::TimedOut => {}
                Err(err) => {
                    let msg = format!("UDP receive failed: {err}");
                    tracing::warn!("MeshCom: {}", msg);
                    self.publish_status(&meshcom, None, Some(msg));
                    self.socket = None;
                    self.bind_key.clear();
                    thread::sleep(ERROR_SLEEP);
                }
            }
        }
    }

    // Was: Diese Funktion stellt socket.
    // Warum: So wird die notwendige Voraussetzung hergestellt, bevor abhängiger Code weiterläuft.
    fn ensure_socket(&mut self, meshcom: &CfgMeshcom) -> Result<(), String> {
        let key = format!("{}:{}", meshcom.bind_addr.trim(), meshcom.bind_port);
        if self.socket.is_some() && self.bind_key == key {
            return Ok(());
        }

        self.socket = None;
        let socket = UdpSocket::bind(&key).map_err(|e| format!("UDP bind {key} failed: {e}"))?;
        socket
            .set_read_timeout(Some(UDP_READ_TIMEOUT))
            .map_err(|e| format!("UDP set_read_timeout failed: {e}"))?;
        if let Err(err) = socket.set_broadcast(meshcom.allow_broadcast) {
            tracing::warn!(
                "MeshCom: failed to set UDP broadcast={} on {}: {}",
                meshcom.allow_broadcast,
                key,
                err
            );
        }
        self.bind_key = key;
        self.socket = Some(socket);
        self.publish_status(meshcom, None, None);
        Ok(())
    }

    // Was: Diese Funktion verarbeitet Datenpaket.
    // Warum: Die Reaktion auf dieses Ereignis bleibt damit an einer Stelle nachvollziehbar.
    fn handle_packet(&mut self, meshcom: &CfgMeshcom, text: &str) -> Result<(), String> {
        if text.is_empty() {
            return Err("empty packet".to_string());
        }
        let value: Value = serde_json::from_str(text).map_err(|e| format!("invalid JSON: {e}"))?;
        let msg_type = string_field(&value, "type").unwrap_or_else(|| "unknown".to_string());
        let src = string_field(&value, "src");
        let (src, via) = split_meshcom_source(src);
        let dst = string_field(&value, "dst");
        let src_type = string_field(&value, "src_type");
        let msg = string_field(&value, "msg").map(|s| truncate_chars(&s, 512));
        let msg_id = string_field(&value, "msg_id");
        let lat = signed_coord(f64_field(&value, "lat"), string_field(&value, "lat_dir").as_deref());
        let lon = signed_coord(f64_field(&value, "long"), string_field(&value, "long_dir").as_deref());
        let alt = f64_field(&value, "alt");
        let batt = f64_field(&value, "batt");
        let rssi = i64_field(&value, "rssi");
        let snr = i64_field(&value, "snr");
        let firmware = string_field(&value, "firmware");
        let fw_sub = string_field(&value, "fw_sub");
        let hw_id = string_field(&value, "hw_id");
        let ts = now_stamp();
        let paths = self.forward_message(
            meshcom,
            &msg_type,
            src.as_deref(),
            dst.as_deref(),
            msg.as_deref(),
            msg_id.as_deref(),
        );

        if let (Some(sink), Some(source), Some(lat), Some(lon)) = (&self.geoalarm_sink, src.as_deref(), lat, lon) {
            sink.send_meshcom_position_with_via(source.to_string(), via.clone(), lat, lon);
        }

        self.rx_packets = self.rx_packets.saturating_add(1);
        let event = MeshcomMessageStatus {
            ts: ts.clone(),
            direction: "rx".to_string(),
            msg_type: msg_type.clone(),
            src_type,
            src: src.clone(),
            via: via.clone(),
            dst,
            msg,
            msg_id,
            paths,
            lat,
            lon,
            alt,
            batt,
            rssi,
            snr,
        };

        if let Some(source) = src {
            let node = MeshcomNodeStatus {
                src: source,
                via,
                last_seen: ts.clone(),
                last_type: msg_type.clone(),
                lat,
                lon,
                alt,
                batt,
                rssi,
                snr,
                firmware,
                fw_sub,
                hw_id,
            };
            self.emit_node_update(&node);
            self.upsert_node(meshcom, node);
        }
        self.emit_message_log(&event);
        self.messages.push_front(event);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.messages.len() > meshcom.max_messages {
            self.messages.pop_back();
        }
        Ok(())
    }

    // Was: Diese Funktion gibt Nachricht log.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn emit_message_log(&self, event: &MeshcomMessageStatus) {
        if let Some(sink) = &self.telemetry_sink {
            sink.send(TelemetryEvent::MeshcomMessageLog {
                ts: event.ts.clone(),
                direction: event.direction.clone(),
                msg_type: event.msg_type.clone(),
                src_type: event.src_type.clone(),
                src: event.src.clone(),
                dst: event.dst.clone(),
                msg: event.msg.clone(),
                msg_id: event.msg_id.clone(),
                paths: event.paths.clone(),
                lat: event.lat,
                lon: event.lon,
                alt: event.alt,
                batt: event.batt,
                rssi: event.rssi,
                snr: event.snr,
                via: event.via.clone(),
            });
        }
    }

    // Was: Diese Funktion gibt Netzknoten update.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn emit_node_update(&self, node: &MeshcomNodeStatus) {
        if let Some(sink) = &self.telemetry_sink {
            sink.send(TelemetryEvent::MeshcomNodeUpdate {
                src: node.src.clone(),
                last_seen: node.last_seen.clone(),
                last_type: node.last_type.clone(),
                lat: node.lat,
                lon: node.lon,
                alt: node.alt,
                batt: node.batt,
                rssi: node.rssi,
                snr: node.snr,
                firmware: node.firmware.clone(),
                fw_sub: node.fw_sub.clone(),
                hw_id: node.hw_id.clone(),
                via: node.via.clone(),
            });
        }
    }

    // Was: Führt den Arbeitsschritt `upsert_node` für upsert Netzknoten aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn upsert_node(&mut self, meshcom: &CfgMeshcom, update: MeshcomNodeStatus) {
        if let Some(node) = self.nodes.iter_mut().find(|node| node.src == update.src) {
            node.last_seen = update.last_seen;
            node.last_type = update.last_type;
            node.via = update.via;
            if update.lat.is_some() {
                node.lat = update.lat;
            }
            if update.lon.is_some() {
                node.lon = update.lon;
            }
            if update.alt.is_some() {
                node.alt = update.alt;
            }
            if update.batt.is_some() {
                node.batt = update.batt;
            }
            if update.rssi.is_some() {
                node.rssi = update.rssi;
            }
            if update.snr.is_some() {
                node.snr = update.snr;
            }
            if update.firmware.is_some() {
                node.firmware = update.firmware;
            }
            if update.fw_sub.is_some() {
                node.fw_sub = update.fw_sub;
            }
            if update.hw_id.is_some() {
                node.hw_id = update.hw_id;
            }
            return;
        }
        self.nodes.push(update);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.nodes.len() > meshcom.max_nodes {
            self.nodes.remove(0);
        }
    }

    // Was: Diese Funktion leitet Nachricht.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn forward_message(
        &self,
        meshcom: &CfgMeshcom,
        msg_type: &str,
        src: Option<&str>,
        dst: Option<&str>,
        msg: Option<&str>,
        msg_id: Option<&str>,
    ) -> Vec<String> {
        let mut paths = Vec::new();
        if !msg_type.eq_ignore_ascii_case("msg") {
            return paths;
        }
        let Some(text) = msg.map(str::trim).filter(|s| !s.is_empty()) else {
            return paths;
        };
        let src = src.map(str::trim).filter(|s| !s.is_empty()).unwrap_or("unknown");

        if meshcom.forward_sds && source_allowed(&meshcom.sds_allowed_sources, src) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match self.forward_sds(meshcom, src, text) {
                Ok(()) => paths.push("sds".to_string()),
                Err(err) => tracing::warn!("MeshCom: SDS forwarding failed src={}: {}", src, err),
            }
        }
        if meshcom.forward_sip && source_allowed(&meshcom.sip_allowed_sources, src) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match self.forward_sip(meshcom, src, dst, text, msg_id) {
                Ok(()) => paths.push("sip".to_string()),
                Err(err) => tracing::warn!("MeshCom: SIP notify forwarding failed src={}: {}", src, err),
            }
        }
        if meshcom.forward_telegram && source_allowed(&meshcom.telegram_allowed_sources, src) {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match self.forward_telegram(meshcom, src, text) {
                Ok(()) => paths.push("telegram".to_string()),
                Err(err) => tracing::warn!("MeshCom: Telegram forwarding failed src={}: {}", src, err),
            }
        }

        if paths.is_empty() && (meshcom.forward_sds || meshcom.forward_sip || meshcom.forward_telegram) {
            tracing::info!(
                "MeshCom: received message src={} dst={} with no successful forwarding target",
                src,
                dst.unwrap_or("-")
            );
        } else if !paths.is_empty() {
            tracing::info!(
                "MeshCom: forwarded message src={} dst={} paths={}",
                src,
                dst.unwrap_or("-"),
                paths.join(",")
            );
        }
        paths
    }

    // Was: Diese Funktion leitet TETRA-Kurznachricht (SDS).
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn forward_sds(&self, meshcom: &CfgMeshcom, src: &str, text: &str) -> Result<(), String> {
        if meshcom.sds_dest_issi == 0 {
            return Err("sds_dest_issi is 0".to_string());
        }
        let Some(tx) = &self.cmce_cmd_tx else {
            return Err("CMCE control sender unavailable".to_string());
        };
        let body = format_plain_meshcom_message(src, text);
        let (len_bits, payload) = build_sds_text_payload(&body);
        tx.send(ControlCommand::SendSds {
            handle: 0,
            source_ssi: meshcom.sds_source_issi,
            dest_ssi: meshcom.sds_dest_issi,
            dest_is_group: meshcom.sds_dest_is_group,
            len_bits,
            payload,
        })
        .map_err(|e| format!("send to CMCE failed: {}", e))
    }

    // Was: Diese Funktion leitet sip.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn forward_sip(&self, meshcom: &CfgMeshcom, src: &str, dst: Option<&str>, text: &str, msg_id: Option<&str>) -> Result<(), String> {
        let Some(sink) = &self.snom_sink else {
            return Err("Snom notify sink unavailable".to_string());
        };
        sink.send_meshcom(
            meshcom.sip_title_prefix.clone(),
            src.to_string(),
            dst.map(ToString::to_string),
            text.to_string(),
            msg_id.map(ToString::to_string),
        );
        Ok(())
    }

    // Was: Diese Funktion leitet telegram.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn forward_telegram(&self, meshcom: &CfgMeshcom, src: &str, text: &str) -> Result<(), String> {
        let Some(sink) = &self.telegram_sink else {
            return Err("Telegram alert sink unavailable".to_string());
        };
        sink.send_meshcom(meshcom.telegram_prefix.clone(), src.to_string(), text.to_string());
        Ok(())
    }

    // Was: Diese Funktion veröffentlicht Status.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn publish_status(&self, meshcom: &CfgMeshcom, last_rx: Option<String>, last_error: Option<String>) {
        let mut state = self.cfg.state_write();
        let previous_tx_packets = state.meshcom_status.tx_packets;
        let previous_last_tx = state.meshcom_status.last_tx.clone();
        let previous_last_rx = state.meshcom_status.last_rx.clone();
        let mut messages: Vec<MeshcomMessageStatus> = self.messages.iter().cloned().collect();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for msg in state.meshcom_status.messages.iter().filter(|msg| msg.direction == "tx") {
            if messages.len() >= meshcom.max_messages {
                break;
            }
            messages.push(msg.clone());
        }
        state.meshcom_status = MeshcomRuntimeStatus {
            configured: true,
            enabled: meshcom.enabled,
            bind: format!("{}:{}", meshcom.bind_addr, meshcom.bind_port),
            tx: format!("{}:{}", meshcom.tx_host, meshcom.tx_port),
            rx_packets: self.rx_packets,
            tx_packets: previous_tx_packets,
            last_rx: last_rx.or(previous_last_rx),
            last_tx: previous_last_tx,
            last_error,
            forward_sds: meshcom.forward_sds,
            forward_sip: meshcom.forward_sip,
            forward_telegram: meshcom.forward_telegram,
            nodes: self.nodes.clone(),
            messages,
        };
    }
}

// Was: Führt den Arbeitsschritt `now_stamp` für now stamp aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn now_stamp() -> String {
    chrono::Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
}

// Was: Führt den Arbeitsschritt `string_field` für string field aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn string_field(value: &Value, key: &str) -> Option<String> {
    value
        .get(key)
        .and_then(|v| v.as_str())
        .map(str::trim)
        .filter(|s| !s.is_empty())
        .map(ToString::to_string)
}

// Was: Führt den Arbeitsschritt `split_meshcom_source` für split meshcom source aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn split_meshcom_source(src: Option<String>) -> (Option<String>, Vec<String>) {
    let Some(src) = src else {
        return (None, Vec::new());
    };
    let mut parts = src.split(',').map(str::trim).filter(|s| !s.is_empty());
    let Some(origin) = parts.next() else {
        return (None, Vec::new());
    };
    (Some(origin.to_string()), parts.map(ToString::to_string).collect())
}

// Was: Führt den Arbeitsschritt `f64_field` für f64 field aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn f64_field(value: &Value, key: &str) -> Option<f64> {
    value
        .get(key)
        .and_then(|v| v.as_f64().or_else(|| v.as_str().and_then(|s| s.trim().parse::<f64>().ok())))
}

// Was: Führt den Arbeitsschritt `i64_field` für i64 field aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn i64_field(value: &Value, key: &str) -> Option<i64> {
    value.get(key).and_then(|v| {
        v.as_i64()
            .or_else(|| v.as_u64().and_then(|n| i64::try_from(n).ok()))
            .or_else(|| v.as_str().and_then(|s| s.trim().parse::<i64>().ok()))
    })
}

// Was: Führt den Arbeitsschritt `signed_coord` für signed coord aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn signed_coord(value: Option<f64>, dir: Option<&str>) -> Option<f64> {
    let mut value = value?;
    if matches!(dir.map(|d| d.trim().to_ascii_uppercase()), Some(d) if d == "S" || d == "W") {
        value = -value.abs();
    }
    Some(value)
}

// Was: Führt den Arbeitsschritt `truncate_chars` für truncate chars aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn truncate_chars(text: &str, max_chars: usize) -> String {
    if text.chars().count() <= max_chars {
        return text.to_string();
    }
    text.chars().take(max_chars).collect()
}

// Was: Führt den Arbeitsschritt `source_allowed` für source allowed aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn source_allowed(allowed: &BTreeSet<String>, src: &str) -> bool {
    if allowed.is_empty() {
        return true;
    }
    let src = src.trim().to_ascii_uppercase();
    allowed.contains(&src)
}

// Was: Führt den Arbeitsschritt `format_plain_meshcom_message` für format plain meshcom Nachricht aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn format_plain_meshcom_message(src: &str, text: &str) -> String {
    format!("MeshCom: {} - {}", src.trim(), text.trim())
}
