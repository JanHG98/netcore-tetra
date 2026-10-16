//! NetCore SDA V1 text protocol used by the Sepura forms.
//! Parsing and persistence stay available without SDR/native runtime dependencies.

use std::collections::{BTreeMap, VecDeque};
use std::fs::{self, File, OpenOptions};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};

pub mod engine;

#[cfg(feature = "runtime")]
pub mod runtime;

pub const SERVICE_ISSI: u32 = 4_010_001;
pub const INFO_PREFIX: &str = "NC1|INFO|";
pub const MAX_INFO_CHARS: usize = 140;
pub const MAX_RECORDS: usize = 500;
const MAX_JOURNAL_BYTES: u64 = 2 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum SdaCommand {
    Ping { note: String },
    Reception { good: bool, note: String },
    Status { unit: String, status: String },
    Ack { job_id: String, result: String },
    Material { place: String, quantity: u16, description: String },
    Fault { site: String, description: String },
    Query { topic: String },
}

/// The final free-text field may contain pipes. Intermediate fields may not.
pub fn parse_command(text: &str) -> Result<SdaCommand, &'static str> {
    if text.len() > 512 || text.chars().any(char::is_control) {
        return Err("Ungueltige oder zu lange Nachricht");
    }
    let Some(body) = text.strip_prefix("NC1|") else {
        return Err("NC1-Praefix fehlt");
    };
    let Some((verb, rest)) = body.split_once('|') else {
        return Err("Befehl oder Felder fehlen");
    };
    let field = |s: &str, max: usize| !s.trim().is_empty() && s.chars().count() <= max;
    let identifier = |s: &str, max: usize| {
        field(s, max)
            && s.chars()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, ' ' | '_' | '-' | '/' | '.'))
    };
    match verb {
        "PING" | "RX_GOOD" | "RX_BAD" if rest.chars().count() <= 60 => {
            let note = rest.to_owned();
            Ok(if verb == "PING" {
                SdaCommand::Ping { note }
            } else {
                SdaCommand::Reception {
                    good: verb == "RX_GOOD",
                    note,
                }
            })
        }
        "STATUS" => {
            let (unit, status) = rest.split_once('|').ok_or("Einheit oder Status fehlt")?;
            if !identifier(unit, 16) || !matches!(status, "READY" | "ENROUTE" | "ON_SCENE" | "UNAVAILABLE") {
                return Err("Einheit oder Status ungueltig");
            }
            Ok(SdaCommand::Status {
                unit: unit.to_owned(),
                status: status.to_owned(),
            })
        }
        "ACK" => {
            let (job_id, result) = rest.split_once('|').ok_or("Auftragsnummer oder Antwort fehlt")?;
            if job_id.is_empty()
                || job_id.len() > 12
                || !job_id.bytes().all(|c| c.is_ascii_digit())
                || !matches!(result, "ACCEPT" | "BUSY" | "REJECT")
            {
                return Err("Auftragsnummer oder Antwort ungueltig");
            }
            Ok(SdaCommand::Ack {
                job_id: job_id.to_owned(),
                result: result.to_owned(),
            })
        }
        "MATERIAL" => {
            let mut parts = rest.splitn(3, '|');
            let place = parts.next().unwrap_or_default();
            let quantity = parts.next().unwrap_or_default();
            let description = parts.next().unwrap_or_default();
            if !identifier(place, 16)
                || quantity.is_empty()
                || quantity.len() > 3
                || !quantity.bytes().all(|c| c.is_ascii_digit())
                || !field(description, 60)
            {
                return Err("Ort, Menge oder Material fehlt/ungueltig");
            }
            let quantity: u16 = quantity.parse().map_err(|_| "Menge ungueltig")?;
            if quantity == 0 {
                return Err("Menge muss groesser als null sein");
            }
            Ok(SdaCommand::Material {
                place: place.to_owned(),
                quantity,
                description: description.to_owned(),
            })
        }
        "FAULT" => {
            let (site, description) = rest.split_once('|').ok_or("Standort oder Beschreibung fehlt")?;
            if !identifier(site, 16) || !field(description, 100) {
                return Err("Standort oder Beschreibung fehlt/ungueltig");
            }
            Ok(SdaCommand::Fault {
                site: site.to_owned(),
                description: description.to_owned(),
            })
        }
        "REQ" if matches!(rest, "SYSTEM" | "BS" | "VPN" | "SIP") => Ok(SdaCommand::Query { topic: rest.to_owned() }),
        "INFO" | "JOB" => Err("Serverantwort ist kein Uplink-Befehl"),
        _ => Err("Unbekannter NC1-Befehl oder ungueltige Felder"),
    }
}

/// INFO/JOB are server-to-radio messages; never answer them with another INFO.
pub fn is_server_message(text: &str) -> bool {
    text.starts_with(INFO_PREFIX) || text.starts_with("NC1|JOB|")
}

pub fn is_service_message(dest_issi: u32, is_group: bool, protocol_id: u8, is_report: bool, text: &str) -> bool {
    dest_issi == SERVICE_ISSI && !is_group && !is_report && matches!(protocol_id, 0x82 | 0x89 | 0x02 | 0x09) && text.starts_with("NC1|")
}

/// The SDA response field has 140 characters and the bearer uses Latin-1.
/// Use bounded ASCII so UTF-8 bytes cannot be misrendered as Latin-1.
pub fn info_reply(body: &str) -> String {
    let mut out = String::new();
    for c in body.chars() {
        match c {
            'ä' => out.push_str("ae"),
            'ö' => out.push_str("oe"),
            'ü' => out.push_str("ue"),
            'Ä' => out.push_str("Ae"),
            'Ö' => out.push_str("Oe"),
            'Ü' => out.push_str("Ue"),
            'ß' => out.push_str("ss"),
            c if c.is_ascii() && !c.is_ascii_control() => out.push(c),
            c if c.is_whitespace() => out.push(' '),
            _ => out.push('?'),
        }
    }
    out.truncate(MAX_INFO_CHARS);
    format!("{INFO_PREFIX}{out}")
}

/// Preserve the complete TL packet and use the local RF-delivery command.
/// SendSds would wrap it again and may divert an unlisted requester to Brew.
pub fn reply_command(dest_issi: u32, mr: u8, reply: &str) -> crate::net_control::ControlCommand {
    let mut payload = vec![0x82, 0x00, mr, 0x01]; // TL transfer, no delivery-report request, Latin-1
    payload.extend_from_slice(reply.as_bytes());
    crate::net_control::ControlCommand::DeliverSds {
        handle: 0,
        source_ssi: SERVICE_ISSI,
        dest_ssi: dest_issi,
        dest_is_group: false,
        sds_type: 4,
        len_bits: (payload.len() * 8) as u16,
        payload,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SdaRecord {
    pub id: u64,
    pub received_at: String,
    pub source_issi: u32,
    pub command: SdaCommand,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Journal {
    version: u8,
    next_id: u64,
    pub records: VecDeque<SdaRecord>,
}

impl Default for Journal {
    fn default() -> Self {
        Self {
            version: 1,
            next_id: 1,
            records: VecDeque::new(),
        }
    }
}

pub fn journal_path(config_path: &Path) -> PathBuf {
    config_path.parent().unwrap_or_else(|| Path::new(".")).join("sda_log.json")
}

impl Journal {
    pub fn load(path: &Path) -> io::Result<Self> {
        let file = match File::open(path) {
            Ok(f) => f,
            Err(e) if e.kind() == io::ErrorKind::NotFound => return Ok(Self::default()),
            Err(e) => return Err(e),
        };
        if file.metadata()?.len() > MAX_JOURNAL_BYTES {
            return Err(io::Error::other("SDA journal too large"));
        }
        let mut data = String::new();
        file.take(MAX_JOURNAL_BYTES + 1).read_to_string(&mut data)?;
        if data.len() as u64 > MAX_JOURNAL_BYTES {
            return Err(io::Error::other("SDA journal too large"));
        }
        let journal: Self = serde_json::from_str(&data).map_err(io::Error::other)?;
        if journal.version != 1
            || journal.next_id == 0
            || journal.records.len() > MAX_RECORDS
            || journal.records.iter().any(|r| r.id >= journal.next_id)
        {
            return Err(io::Error::other("Invalid SDA journal version or sequence"));
        }
        Ok(journal)
    }

    /// Commit to disk before returning a successful application acknowledgement.
    pub fn record(&mut self, path: &Path, source_issi: u32, command: SdaCommand) -> io::Result<SdaRecord> {
        let mut next = self.clone();
        let id = next.next_id;
        next.next_id = id.checked_add(1).ok_or_else(|| io::Error::other("SDA sequence exhausted"))?;
        let record = SdaRecord {
            id,
            received_at: chrono::Utc::now().to_rfc3339(),
            source_issi,
            command,
        };
        next.records.push_back(record.clone());
        while next.records.len() > MAX_RECORDS {
            next.records.pop_front();
        }
        let tmp = path.with_extension("json.tmp");
        let mut options = OpenOptions::new();
        options.write(true).create(true).truncate(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let result = (|| {
            let mut file = options.open(&tmp)?;
            serde_json::to_writer(&mut file, &next).map_err(io::Error::other)?;
            file.write_all(b"\n")?;
            file.sync_all()?;
            drop(file);
            fs::rename(&tmp, path)
        })();
        if let Err(e) = result {
            let _ = fs::remove_file(&tmp);
            return Err(e);
        }
        *self = next;
        Ok(record)
    }

    pub fn snapshot(&self) -> serde_json::Value {
        let mut statuses = BTreeMap::new();
        for record in &self.records {
            if let SdaCommand::Status { unit, .. } = &record.command {
                statuses.insert((record.source_issi, unit.clone()), record);
            }
        }
        serde_json::json!({
            "scope": "local", "max_records": MAX_RECORDS,
            "records": self.records.iter().rev().collect::<Vec<_>>(),
            "unit_statuses": statuses.values().collect::<Vec<_>>(),
        })
    }
}

pub fn recorded_reply(record: &SdaRecord) -> String {
    let detail = match &record.command {
        SdaCommand::Ping { .. } => "Funkprobe empfangen".to_owned(),
        SdaCommand::Reception { good, .. } => format!("Empfangsmeldung {}", if *good { "gut" } else { "schlecht" }),
        SdaCommand::Status { unit, status } => format!("Status {unit}: {status}"),
        SdaCommand::Ack { job_id, result } => format!("Auftragsantwort {job_id}: {result}"),
        SdaCommand::Material { .. } => "Materialanforderung".to_owned(),
        SdaCommand::Fault { .. } => "Stoerungsmeldung".to_owned(),
        SdaCommand::Query { .. } => "Abfrage".to_owned(),
    };
    info_reply(&format!("{detail}; lokal erfasst #{}", record.id))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn path(label: &str) -> PathBuf {
        static ID: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(1);
        let p = std::env::temp_dir().join(format!(
            "netcore-sda-{label}-{}-{}",
            std::process::id(),
            ID.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&p).unwrap();
        p.join("sda_log.json")
    }

    #[test]
    fn parses_all_template_commands_and_final_free_text() {
        for s in [
            "NC1|PING|",
            "NC1|RX_GOOD|Ad",
            "NC1|RX_BAD|schwach",
            "NC1|STATUS|TRUPP1|READY",
            "NC1|STATUS|TRUPP1|ENROUTE",
            "NC1|STATUS|TRUPP1|ON_SCENE",
            "NC1|STATUS|TRUPP1|UNAVAILABLE",
            "NC1|STATUS|UNIT123456789012|READY",
            "NC1|ACK|123456789012|ACCEPT",
            "NC1|ACK|00042|ACCEPT",
            "NC1|ACK|42|BUSY",
            "NC1|ACK|42|REJECT",
            "NC1|MATERIAL|TOR1|2|Seil|Karabiner",
            "NC1|FAULT|BS03|Strom|Luefter",
            "NC1|REQ|SYSTEM",
            "NC1|REQ|BS",
            "NC1|REQ|VPN",
            "NC1|REQ|SIP",
        ] {
            assert!(parse_command(s).is_ok(), "{s}");
        }
        assert_eq!(
            parse_command("NC1|MATERIAL|TOR1|2|Seil|Karabiner").unwrap(),
            SdaCommand::Material {
                place: "TOR1".into(),
                quantity: 2,
                description: "Seil|Karabiner".into()
            }
        );
    }

    #[test]
    fn rejects_missing_fields_unknown_tokens_and_invalid_quantities() {
        for s in [
            "NC1|",
            "NC1|REQ|SIP|EXTRA",
            "NC1|STATUS||READY",
            "NC1|STATUS|TRUPP1|BAD",
            "NC1|ACK|abc|ACCEPT",
            "NC1|ACK|1234567890123|ACCEPT",
            "NC1|MATERIAL|TOR1|0|Seil",
            "NC1|MATERIAL|TOR1|9999|Seil",
            "NC1|MATERIAL|TOR1|+1|Seil",
            "NC1|FAULT|BS03|",
            "NC1|INFO|OK",
            "NC1|JOB|1|Test",
            "NC1|REQ|SIP\n",
        ] {
            assert!(parse_command(s).is_err(), "{s:?}");
        }
        assert!(parse_command(&format!("NC1|PING|{}", "x".repeat(61))).is_err());
        assert!(parse_command(&format!("NC1|FAULT|BS03|{}", "x".repeat(101))).is_err());
    }

    #[test]
    fn only_claims_nc1_text_for_local_individual_service() {
        assert!(is_service_message(SERVICE_ISSI, false, 0x82, false, "NC1|REQ|SIP"));
        assert!(!is_service_message(9998, false, 0x82, false, "NC1|REQ|SIP"));
        assert!(!is_service_message(SERVICE_ISSI, true, 0x82, false, "NC1|REQ|SIP"));
        assert!(!is_service_message(SERVICE_ISSI, false, 0x82, true, "NC1|REQ|SIP"));
        assert!(!is_service_message(SERVICE_ISSI, false, 0x0a, false, "NC1|REQ|SIP"));
        assert!(!is_service_message(SERVICE_ISSI, false, 0x82, false, "METAR EDDV"));
        assert!(is_server_message("NC1|INFO|OK"));
        assert!(is_server_message("NC1|JOB|1|Test"));
    }

    #[test]
    fn replies_fit_template_and_preserve_the_complete_local_rf_payload() {
        let text = info_reply(&format!("Grüße\n{}", "ä".repeat(150)));
        assert!(text.is_ascii());
        assert_eq!(text.strip_prefix(INFO_PREFIX).unwrap().len(), 140);
        let crate::net_control::ControlCommand::DeliverSds {
            source_ssi,
            dest_ssi,
            sds_type,
            len_bits,
            payload,
            ..
        } = reply_command(5102, 6, &text)
        else {
            panic!("must use local RF delivery");
        };
        assert_eq!((source_ssi, dest_ssi, sds_type), (SERVICE_ISSI, 5102, 4));
        assert_eq!(&payload[..4], &[0x82, 0, 6, 1]);
        assert_eq!(&payload[4..], text.as_bytes());
        assert_eq!(len_bits as usize, payload.len() * 8);
    }

    #[test]
    fn journal_survives_restart_preserves_issuer_and_latest_unit_status() {
        let p = path("restart");
        let mut log = Journal::load(&p).unwrap();
        for (issi, status) in [(5102, "READY"), (5103, "READY"), (5102, "ENROUTE")] {
            log.record(&p, issi, parse_command(&format!("NC1|STATUS|TRUPP1|{status}")).unwrap())
                .unwrap();
        }
        let mut reopened = Journal::load(&p).unwrap();
        assert_eq!(reopened.records.len(), 3);
        let snap = reopened.snapshot();
        assert_eq!(snap["unit_statuses"].as_array().unwrap().len(), 2);
        assert_eq!(snap["records"][0]["command"]["status"], "ENROUTE");
        let record = reopened.record(&p, 5102, parse_command("NC1|FAULT|BS03|Luefter").unwrap()).unwrap();
        assert_eq!(record.id, 4);
        assert!(recorded_reply(&record).contains("lokal erfasst #4"));
        fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }

    #[test]
    fn persistence_failure_does_not_commit_or_report_success() {
        let p = path("failure");
        let mut log = Journal::default();
        let bad = p.parent().unwrap().join("missing").join("sda_log.json");
        assert!(log.record(&bad, 5102, parse_command("NC1|PING|").unwrap()).is_err());
        assert!(log.records.is_empty());
        assert_eq!(log.next_id, 1);
        fs::write(&p, b"invalid journal").unwrap();
        assert!(Journal::load(&p).is_err());
        assert_eq!(fs::read(&p).unwrap(), b"invalid journal");
        fs::remove_dir_all(p.parent().unwrap()).unwrap();
    }
}
