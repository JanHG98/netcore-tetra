// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für die Kopplung von TETRA-Paketdaten an IP-Netze.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::collections::{BTreeSet, VecDeque};
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::{Duration, Instant};

use crate::config::{IpGatewayConfig, MODE_AUTHORITATIVE};
use crate::kernel::{self, KernelPlan};
use crate::packet_core::PacketCoreClient;
use crate::protocol::{bytes_to_hex, DownlinkNpduInput, PacketCoreContext};
use crate::state::{KernelStateSnapshot, SharedGateway};
use crate::tun::TunDevice;

// Was: Listet die möglichen Varianten für Laufzeit command auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum RuntimeCommand {
    Reconcile {
        response: Sender<Result<KernelPlan, String>>,
    },
}

#[derive(Clone)]
// Was: Bündelt die zusammengehörigen Werte für Laufzeit handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RuntimeHandle {
    tx: Sender<RuntimeCommand>,
}

// Was: Implementiert das zugehörige Verhalten für `RuntimeHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl RuntimeHandle {
    // Was: Führt den Arbeitsschritt `reconcile` für reconcile aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn reconcile(&self) -> Result<KernelPlan, String> {
        let (tx, rx) = mpsc::channel();
        self.tx
            .send(RuntimeCommand::Reconcile { response: tx })
            .map_err(|error| format!("runtime command channel closed: {error}"))?;
        rx.recv_timeout(Duration::from_secs(30))
            .map_err(|error| format!("runtime reconcile timed out: {error}"))?
    }
}

// Was: Bündelt die zusammengehörigen Werte für pending Downlink (Netz zum Funkgerät) in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct PendingDownlink {
    packet: Vec<u8>,
    context: PacketCoreContext,
    attempts: u8,
    next_attempt: Instant,
}

// Was: Diese Funktion startet Laufzeit.
// Warum: Länger laufende Arbeit blockiert dadurch nicht den aufrufenden Ablauf.
pub fn spawn_runtime(config: IpGatewayConfig, gateway: SharedGateway) -> RuntimeHandle {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || run(config, gateway, rx));
    RuntimeHandle { tx }
}

// Was: Diese Funktion führt den vorgesehenen Arbeitsschritt.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
fn run(config: IpGatewayConfig, gateway: SharedGateway, rx: Receiver<RuntimeCommand>) {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let client = match PacketCoreClient::new(&config.packet_core) {
        Ok(client) => client,
        Err(error) => {
            gateway.packet_core_disconnected(error);
            return;
        }
    };
    let mut tun: Option<TunDevice> = None;
    let mut previous_snapshot: Option<KernelStateSnapshot> = None;
    let mut last_context_refresh = Instant::now() - Duration::from_secs(60);
    let mut last_kernel_reconcile = Instant::now() - Duration::from_secs(60);
    let mut last_persist = Instant::now();
    let mut pending_downlinks = VecDeque::new();
    let mut pending_uplink_deletes = BTreeSet::new();
    let poll = Duration::from_millis(config.packet_core.poll_interval_ms);
    let context_interval = Duration::from_millis(config.packet_core.context_refresh_ms);
    let kernel_interval = Duration::from_secs(config.routing.reconcile_interval_secs);

    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while let Ok(command) = rx.try_recv() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match command {
                RuntimeCommand::Reconcile { response } => {
                    let result = reconcile_now(&config, &gateway, &mut previous_snapshot);
                    let _ = response.send(result);
                }
            }
        }

        if config.interface.mode == MODE_AUTHORITATIVE && tun.is_none() {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match TunDevice::open(
                &config.interface.name,
                Some(&config.interface.owner_user),
                !config.interface.delete_on_exit,
            ) {
                Ok(device) => {
                    gateway.tun_opened(device.name().to_string());
                    tun = Some(device);
                    let _ = reconcile_now(&config, &gateway, &mut previous_snapshot);
                    last_kernel_reconcile = Instant::now();
                }
                Err(error) => {
                    gateway.tun_closed(Some(error));
                    thread::sleep(Duration::from_secs(2));
                    continue;
                }
            }
        }

        if last_kernel_reconcile.elapsed() >= kernel_interval {
            let _ = reconcile_now(&config, &gateway, &mut previous_snapshot);
            last_kernel_reconcile = Instant::now();
        }

        if last_context_refresh.elapsed() >= context_interval {
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match client.status() {
                Ok(status) => {
                    gateway.packet_core_connected(status.mode);
                    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                    match client.contexts() {
                        Ok(contexts) => gateway.replace_contexts(contexts),
                        Err(error) => gateway.packet_core_disconnected(error),
                    }
                }
                Err(error) => gateway.packet_core_disconnected(error),
            }
            last_context_refresh = Instant::now();
        }

        if config.interface.mode == MODE_AUTHORITATIVE {
            let healthy = if let Some(device) = tun.as_mut() {
                process_uplink(
                    &config,
                    &client,
                    &gateway,
                    device,
                    &mut pending_uplink_deletes,
                )
                    && read_downlink(&config, &gateway, device, &mut pending_downlinks)
            } else {
                false
            };
            if !healthy {
                tun = None;
            }
            process_pending_downlinks(&client, &gateway, &mut pending_downlinks);
        }
        if last_persist.elapsed() >= Duration::from_secs(10) {
            if let Err(error) = gateway.persist() {
                tracing::warn!("IP Gateway state persistence failed: {error}");
            }
            last_persist = Instant::now();
        }
        thread::sleep(poll);
    }
}

// Was: Führt den Arbeitsschritt `reconcile_now` für reconcile now aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn reconcile_now(
    config: &IpGatewayConfig,
    gateway: &SharedGateway,
    previous_snapshot: &mut Option<KernelStateSnapshot>,
) -> Result<KernelPlan, String> {
    let snapshot = gateway.kernel_snapshot();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match kernel::reconcile(config, &snapshot, previous_snapshot.as_ref()) {
        Ok(plan) => {
            gateway.kernel_reconciled(
                snapshot.revision,
                None,
                config.interface.mode == MODE_AUTHORITATIVE,
            );
            if config.interface.mode == MODE_AUTHORITATIVE {
                *previous_snapshot = Some(snapshot);
            }
            Ok(plan)
        }
        Err(error) => {
            gateway.kernel_reconciled(
                snapshot.revision,
                Some(error.clone()),
                config.interface.mode == MODE_AUTHORITATIVE,
            );
            Err(error)
        }
    }
}

// Was: Diese Funktion verarbeitet Uplink (Funkgerät zum Netz).
// Warum: Die einzelnen Verarbeitungsschritte bleiben damit gebündelt und leichter testbar.
fn process_uplink(
    config: &IpGatewayConfig,
    client: &PacketCoreClient,
    gateway: &SharedGateway,
    tun: &mut TunDevice,
    pending_deletes: &mut BTreeSet<String>,
) -> bool {
    retry_uplink_deletes(client, gateway, pending_deletes);
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let npdus = match client.npdu_outbox(config.packet_core.outbox_batch) {
        Ok(npdus) => npdus,
        Err(error) => {
            gateway.packet_core_disconnected(error);
            return true;
        }
    };
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for npdu in npdus {
        if pending_deletes.contains(&npdu.id) {
            continue;
        }
        if npdu.direction != "uplink" {
            gateway.record_drop(
                "unsupported_npdu_direction",
                serde_json::json!({"id":npdu.id,"direction":npdu.direction}),
            );
            acknowledge_uplink_npdu(client, gateway, pending_deletes, &npdu.id);
            continue;
        }
        if npdu.payload.len() > config.limits.max_packet_bytes {
            gateway.record_drop(
                "uplink_packet_too_large",
                serde_json::json!({"id":npdu.id,"bytes":npdu.payload.len()}),
            );
            acknowledge_uplink_npdu(client, gateway, pending_deletes, &npdu.id);
            continue;
        }
        let context = gateway
            .contexts()
            .into_iter()
            .find(|context| context.issi == npdu.issi && context.nsapi == npdu.nsapi);
        if let Err(error) = gateway.record_packet("uplink", &npdu.payload, context.as_ref()) {
            gateway.record_drop(
                "invalid_uplink_ipv4",
                serde_json::json!({"id":npdu.id,"error":error}),
            );
            acknowledge_uplink_npdu(client, gateway, pending_deletes, &npdu.id);
            continue;
        }
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match tun.write_packet(&npdu.payload) {
            Ok(()) => acknowledge_uplink_npdu(
                client,
                gateway,
                pending_deletes,
                &npdu.id,
            ),
            Err(error) => {
                gateway.tun_closed(Some(error));
                return false;
            }
        }
    }
    true
}

// Was: Führt den Arbeitsschritt `acknowledge_uplink_npdu` für acknowledge Uplink (Funkgerät zum Netz) npdu aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn acknowledge_uplink_npdu(
    client: &PacketCoreClient,
    gateway: &SharedGateway,
    pending_deletes: &mut BTreeSet<String>,
    id: &str,
) {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match client.delete_npdu(id) {
        Ok(()) => {
            pending_deletes.remove(id);
            gateway.record_packet_core_delete();
        }
        Err(error) => {
            pending_deletes.insert(id.to_string());
            gateway.packet_core_disconnected(error);
        }
    }
}

// Was: Führt den Arbeitsschritt `retry_uplink_deletes` für retry Uplink (Funkgerät zum Netz) deletes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn retry_uplink_deletes(
    client: &PacketCoreClient,
    gateway: &SharedGateway,
    pending_deletes: &mut BTreeSet<String>,
) {
    let ids: Vec<String> = pending_deletes.iter().take(256).cloned().collect();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for id in ids {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match client.delete_npdu(&id) {
            Ok(()) => {
                pending_deletes.remove(&id);
                gateway.record_packet_core_delete();
            }
            Err(error) => gateway.packet_core_disconnected(error),
        }
    }
}

// Was: Diese Funktion liest Downlink (Netz zum Funkgerät).
// Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
fn read_downlink(
    config: &IpGatewayConfig,
    gateway: &SharedGateway,
    tun: &mut TunDevice,
    queue: &mut VecDeque<PendingDownlink>,
) -> bool {
    let mut buffer = vec![0u8; config.limits.max_packet_bytes];
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for _ in 0..128 {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let size = match tun.read_packet(&mut buffer) {
            Ok(Some(size)) => size,
            Ok(None) => break,
            Err(error) => {
                gateway.tun_closed(Some(error));
                return false;
            }
        };
        let packet = buffer[..size].to_vec();
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let observation = match gateway.record_packet("downlink", &packet, None) {
            Ok(observation) => observation,
            Err(error) => {
                gateway.record_drop(
                    "invalid_downlink_ipv4",
                    serde_json::json!({"error":error,"bytes":size}),
                );
                continue;
            }
        };
        let Some(context) = gateway.context_by_ipv4(observation.destination) else {
            gateway.record_drop(
                "downlink_no_pdp_context",
                serde_json::json!({"destination":observation.destination.to_string()}),
            );
            continue;
        };
        if !context.available {
            gateway.record_drop(
                "downlink_context_unavailable",
                serde_json::json!({"context_id":context.id,"destination":context.ipv4}),
            );
            continue;
        }
        if queue.len() >= 1_024 {
            queue.pop_front();
            gateway.record_drop("downlink_retry_queue_overflow", serde_json::json!({}));
        }
        queue.push_back(PendingDownlink {
            packet,
            context,
            attempts: 0,
            next_attempt: Instant::now(),
        });
    }
    true
}

// Was: Diese Funktion verarbeitet pending downlinks.
// Warum: Die einzelnen Verarbeitungsschritte bleiben damit gebündelt und leichter testbar.
fn process_pending_downlinks(
    client: &PacketCoreClient,
    gateway: &SharedGateway,
    queue: &mut VecDeque<PendingDownlink>,
) {
    let now = Instant::now();
    let mut remaining = VecDeque::new();
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for _ in 0..64 {
        let Some(mut pending) = queue.pop_front() else {
            break;
        };
        if pending.next_attempt > now {
            remaining.push_back(pending);
            continue;
        }
        let input = DownlinkNpduInput {
            issi: pending.context.issi,
            nsapi: pending.context.nsapi,
            payload_hex: bytes_to_hex(&pending.packet),
            acknowledged: false,
            priority: Some(pending.context.priority),
        };
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match client.queue_downlink(&input) {
            Ok(()) => gateway.record_packet_core_downlink(),
            Err(error) => {
                pending.attempts = pending.attempts.saturating_add(1);
                if pending.attempts >= 5 {
                    gateway.record_drop(
                        "downlink_packet_core_failed",
                        serde_json::json!({"context_id":pending.context.id,"error":error}),
                    );
                } else {
                    pending.next_attempt = Instant::now()
                        + Duration::from_millis(250 * u64::from(pending.attempts));
                    remaining.push_back(pending);
                }
            }
        }
    }
    remaining.append(queue);
    *queue = remaining;
}
