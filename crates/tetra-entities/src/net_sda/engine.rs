use std::collections::VecDeque;
use std::path::PathBuf;
use std::time::{Duration, Instant};

use super::{Journal, SdaCommand, info_reply, is_server_message, parse_command, recorded_reply};

#[derive(Debug, Clone)]
pub struct SdaRequest {
    pub source_issi: u32,
    pub message_reference: Option<u8>,
    pub text: String,
}

struct CachedReply {
    source_issi: u32,
    reference: u8,
    text: String,
    reply: String,
    at: Instant,
}

/// Called only by the SDA worker, never by the radio timing thread.
pub struct Engine {
    path: PathBuf,
    journal: Option<Journal>,
    cache: VecDeque<CachedReply>,
}

impl Engine {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            journal: None,
            cache: VecDeque::new(),
        }
    }

    pub fn process(&mut self, request: &SdaRequest, query: impl FnOnce(&str) -> String) -> Option<String> {
        if request.source_issi == 0 || request.source_issi > 0xff_ffff || is_server_message(&request.text) {
            return None;
        }
        self.cache.retain(|entry| entry.at.elapsed() < Duration::from_secs(60));
        if let Some(reference) = request.message_reference
            && let Some(cached) = self
                .cache
                .iter()
                .find(|entry| entry.source_issi == request.source_issi && entry.reference == reference && entry.text == request.text)
        {
            return Some(cached.reply.clone());
        }
        let (reply, cacheable) = match parse_command(&request.text) {
            Err(error) => (info_reply(&format!("Fehler: {error}")), true),
            Ok(SdaCommand::Query { topic }) => (info_reply(&query(&topic)), true),
            Ok(command) => {
                if self.journal.is_none() {
                    match Journal::load(&self.path) {
                        Ok(log) => self.journal = Some(log),
                        Err(e) => tracing::error!("SDA: cannot read {}: {e}", self.path.display()),
                    }
                }
                let result = self
                    .journal
                    .as_mut()
                    .ok_or_else(|| std::io::Error::other("SDA journal unavailable"))
                    .and_then(|log| log.record(&self.path, request.source_issi, command));
                match result {
                    Ok(record) => (recorded_reply(&record), true),
                    Err(e) => {
                        tracing::error!("SDA: request from ISSI {} was NOT saved: {e}", request.source_issi);
                        (info_reply("Fehler: Meldung nicht gespeichert; bitte erneut senden"), false)
                    }
                }
            }
        };
        if cacheable && let Some(reference) = request.message_reference {
            self.cache.push_back(CachedReply {
                source_issi: request.source_issi,
                reference,
                text: request.text.clone(),
                reply: reply.clone(),
                at: Instant::now(),
            });
            while self.cache.len() > 256 {
                self.cache.pop_front();
            }
        }
        Some(reply)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::net_sda::SERVICE_ISSI;

    fn engine() -> Engine {
        let id = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("netcore-sda-engine-{}-{id}", std::process::id()));
        std::fs::create_dir(&path).unwrap();
        Engine::new(path.join("sda_log.json"))
    }

    fn request(issi: u32, mr: u8, text: &str) -> SdaRequest {
        SdaRequest {
            source_issi: issi,
            message_reference: Some(mr),
            text: text.to_owned(),
        }
    }

    #[test]
    fn duplicate_transport_reference_replays_ack_without_duplicate_records() {
        let mut e = engine();
        let q = request(5102, 6, "NC1|STATUS|TRUPP1|ENROUTE");
        let reply = e.process(&q, |_| unreachable!()).unwrap();
        assert_eq!(e.process(&q, |_| unreachable!()).unwrap(), reply);
        e.process(&request(5103, 6, &q.text), |_| unreachable!());
        e.process(&request(5102, 7, &q.text), |_| unreachable!());
        assert_eq!(Journal::load(&e.path).unwrap().records.len(), 3);
        assert!(reply.starts_with("NC1|INFO|Status TRUPP1: ENROUTE"));
        std::fs::remove_dir_all(e.path.parent().unwrap()).unwrap();
    }

    #[test]
    fn same_reference_with_different_payload_is_a_new_request() {
        let mut e = engine();
        e.process(&request(5102, 6, "NC1|RX_GOOD|Ad"), |_| unreachable!());
        e.process(&request(5102, 6, "NC1|RX_BAD|Schwach"), |_| unreachable!());
        assert_eq!(Journal::load(&e.path).unwrap().records.len(), 2);
        std::fs::remove_dir_all(e.path.parent().unwrap()).unwrap();
    }

    #[test]
    fn expired_reference_can_be_reused_and_server_messages_never_loop() {
        let mut e = engine();
        let q = request(5102, 6, "NC1|PING|");
        e.process(&q, |_| unreachable!());
        e.cache[0].at = Instant::now() - Duration::from_secs(61);
        e.process(&q, |_| unreachable!());
        assert_eq!(Journal::load(&e.path).unwrap().records.len(), 2);
        for text in ["NC1|INFO|OK", "NC1|JOB|42|Test"] {
            assert!(e.process(&request(SERVICE_ISSI, 6, text), |_| unreachable!()).is_none());
        }
        std::fs::remove_dir_all(e.path.parent().unwrap()).unwrap();
    }

    #[test]
    fn live_query_has_no_journal_dependency_and_invalid_request_has_error_reply() {
        let mut e = engine();
        std::fs::write(&e.path, b"broken").unwrap();
        let result = e
            .process(&request(5102, 6, "NC1|REQ|SIP"), |topic| {
                assert_eq!(topic, "SIP");
                "SIP lokal: registered; Dialoge 0".into()
            })
            .unwrap();
        assert_eq!(result, "NC1|INFO|SIP lokal: registered; Dialoge 0");
        assert!(
            e.process(&request(5102, 7, "NC1|MATERIAL|X|0|Seil"), |_| unreachable!())
                .unwrap()
                .contains("Fehler:")
        );
        assert!(
            e.process(&request(5102, 8, "NC1|FAULT|BS03|Luefter"), |_| unreachable!())
                .unwrap()
                .contains("nicht gespeichert")
        );
        assert_eq!(std::fs::read(&e.path).unwrap(), b"broken");
        std::fs::remove_dir_all(e.path.parent().unwrap()).unwrap();
    }
}
