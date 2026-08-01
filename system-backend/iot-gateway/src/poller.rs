use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use netcore_contracts::NetCoreEvent;
use reqwest::blocking::Client;

use crate::config::{EventSourceConfig, IotGatewayConfig};
use crate::state::{EnqueueOutcome, SharedGateway};

#[derive(Clone)]
pub struct PollControl {
    sender: mpsc::Sender<()>,
}

impl PollControl {
    pub fn poll_now(&self) -> Result<(), String> {
        self.sender
            .send(())
            .map_err(|_| "event poller is unavailable".to_string())
    }
}

pub fn spawn_poller(
    config: IotGatewayConfig,
    state: SharedGateway,
) -> Result<(PollControl, thread::JoinHandle<()>), String> {
    let client = Client::builder()
        .timeout(Duration::from_millis(config.polling.request_timeout_ms))
        .build()
        .map_err(|error| error.to_string())?;
    let (sender, receiver) = mpsc::channel();
    let control = PollControl { sender };
    let handle = thread::spawn(move || {
        loop {
            poll_all(&client, &config, &state);
            let wait = Duration::from_millis(config.polling.interval_ms);
            match receiver.recv_timeout(wait) {
                Ok(()) | Err(mpsc::RecvTimeoutError::Timeout) => {}
                Err(mpsc::RecvTimeoutError::Disconnected) => break,
            }
        }
    });
    Ok((control, handle))
}

pub fn poll_all(client: &Client, config: &IotGatewayConfig, state: &SharedGateway) {
    state.mark_poll_started();
    for source in config.sources.iter().filter(|source| source.enabled) {
        poll_source(client, config, state, source);
    }
}

fn poll_source(
    client: &Client,
    config: &IotGatewayConfig,
    state: &SharedGateway,
    source: &EventSourceConfig,
) {
    let separator = if source.url.contains('?') { '&' } else { '?' };
    let url = format!(
        "{}{}limit={}",
        source.url, separator, config.polling.batch_limit
    );
    let response = match client.get(&url).send() {
        Ok(response) => response,
        Err(error) => {
            state.record_source_failure(&source.id, error.to_string());
            return;
        }
    };
    if !response.status().is_success() {
        state.record_source_failure(
            &source.id,
            format!("HTTP {} from {}", response.status(), source.url),
        );
        return;
    }
    let mut events = match response.json::<Vec<NetCoreEvent>>() {
        Ok(events) => events,
        Err(error) => {
            state.record_source_failure(&source.id, format!("invalid event JSON: {error}"));
            return;
        }
    };

    // Phase-2 APIs return newest-first. MQTT consumers receive the batch in
    // chronological order, while event_id based deduplication prevents replay.
    events.reverse();
    let mut seen = 0_u64;
    let mut enqueued = 0_u64;
    let mut duplicates = 0_u64;
    let mut invalid = 0_u64;
    for event in events {
        seen = seen.saturating_add(1);
        match state.enqueue_event(&event) {
            Ok(EnqueueOutcome::Enqueued) => enqueued = enqueued.saturating_add(1),
            Ok(EnqueueOutcome::Duplicate) => duplicates = duplicates.saturating_add(1),
            Err(error) => {
                invalid = invalid.saturating_add(1);
                state.record_invalid_event(&source.id, error);
            }
        }
    }
    state.record_source_success(&source.id, seen, enqueued, duplicates, invalid);
}
