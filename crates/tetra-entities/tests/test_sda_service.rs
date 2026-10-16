#[path = "common/sds.rs"]
mod common;

use common::ComponentTest;
use tetra_config::bluestation::StackMode;
use tetra_core::tetra_entities::TetraEntity;
use tetra_core::{BitBuffer, Sap, SsiType, TetraAddress};
use tetra_pdus::cmce::enums::party_type_identifier::PartyTypeIdentifier;
use tetra_pdus::cmce::pdus::u_sds_data::USdsData;
use tetra_saps::control::enums::sds_user_data::SdsUserData;
use tetra_saps::lcmc::LcmcMleUnitdataInd;
use tetra_saps::{SapMsg, SapMsgInner};

// NC1 SDA regression: the exact 120-bit packet reported by the operator must
// be claimed before WX, while transport receipts must never become requests.
fn sda_uplink(source: u32, dest: u32, bytes: Vec<u8>) -> SapMsg {
    let pdu = USdsData {
        area_selection: 0,
        called_party_type_identifier: PartyTypeIdentifier::Ssi,
        called_party_short_number_address: None,
        called_party_ssi: Some(dest as u64),
        called_party_extension: None,
        user_defined_data: SdsUserData::Type4((bytes.len() * 8) as u16, bytes),
        external_subscriber_number: None,
        dm_ms_address: None,
    };
    let mut sdu = BitBuffer::new_autoexpand(256);
    pdu.to_bitbuf(&mut sdu).unwrap();
    sdu.seek(0);
    SapMsg {
        sap: Sap::LcmcSap,
        src: TetraEntity::Mle,
        dest: TetraEntity::Cmce,
        msg: SapMsgInner::LcmcMleUnitdataInd(LcmcMleUnitdataInd {
            sdu,
            handle: 1,
            endpoint_id: 1,
            link_id: 1,
            received_tetra_address: TetraAddress::new(source, SsiType::Issi),
            chan_change_resp_req: false,
            chan_change_handle: None,
        }),
    }
}

fn sda_text(reference: u8, text: &str) -> Vec<u8> {
    let mut bytes = vec![0x82, 0x06, reference, 0x01];
    bytes.extend_from_slice(text.as_bytes());
    bytes
}

fn sda_setup(
    capacity: usize,
) -> (
    tetra_entities::cmce::subentities::sds_bs::SdsBsSubentity,
    crossbeam_channel::Receiver<tetra_entities::net_sda::engine::SdaRequest>,
) {
    let mut cfg = ComponentTest::get_default_test_config(StackMode::Bs);
    cfg.wx_service.enabled = true;
    cfg.wx_service.service_issi = tetra_entities::net_sda::SERVICE_ISSI;
    let cfg = tetra_config::bluestation::SharedConfig::from_parts(cfg, None);
    let mut sds = tetra_entities::cmce::subentities::sds_bs::SdsBsSubentity::new(cfg);
    let (tx, rx) = crossbeam_channel::bounded(capacity);
    sds.set_sda_service(tx);
    (sds, rx)
}

fn sda_downlink_payloads(queue: &tetra_entities::MessageQueue) -> Vec<Vec<u8>> {
    queue
        .iter()
        .filter_map(|msg| {
            if let SapMsgInner::LcmcMleUnitdataReq(prim) = &msg.msg {
                let mut buf = prim.sdu.clone();
                buf.seek(0);
                Some(
                    tetra_pdus::cmce::pdus::d_sds_data::DSdsData::from_bitbuf(&mut buf)
                        .unwrap()
                        .user_defined_data
                        .to_arr(),
                )
            } else {
                None
            }
        })
        .collect()
}

#[test]
fn test_sda_observed_sip_frame_is_claimed_before_wx_and_receipt_echoes_reference() {
    let (mut sds, rx) = sda_setup(4);
    let mut queue = tetra_entities::MessageQueue::new();
    let frame = vec![
        0x82, 0x06, 0x06, 0x01, 0x4e, 0x43, 0x31, 0x7c, 0x52, 0x45, 0x51, 0x7c, 0x53, 0x49, 0x50,
    ];
    assert_eq!(frame.len() * 8, 120);
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, frame));
    let request = rx.try_recv().expect("NC1 must reach SDA rather than WX");
    assert_eq!(request.source_issi, 5102);
    assert_eq!(request.message_reference, Some(6));
    assert_eq!(request.text, "NC1|REQ|SIP");
    assert_eq!(sda_downlink_payloads(&queue), vec![vec![0x82, 0x10, 0, 6]]);
}

#[test]
fn test_sda_does_not_claim_weather_or_another_destination() {
    let (mut sds, rx) = sda_setup(4);
    let mut queue = tetra_entities::MessageQueue::new();
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(1, "METAR EDDV")));
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 5103, sda_text(2, "NC1|REQ|SIP")));
    assert!(rx.is_empty());
}

#[test]
fn test_sda_reports_and_server_messages_never_create_reply_loops() {
    let (mut sds, rx) = sda_setup(4);
    let mut queue = tetra_entities::MessageQueue::new();
    let mut report = vec![0x82, 0x10, 0, 6];
    report.extend_from_slice(b"NC1|REQ|SIP");
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, report));
    assert!(queue.is_empty());
    for text in ["NC1|INFO|OK", "NC1|JOB|42|Test"] {
        sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(7, text)));
    }
    assert!(rx.is_empty());
    let packets = sda_downlink_payloads(&queue);
    assert_eq!(packets.len(), 2);
    assert!(packets.iter().all(|packet| packet.len() == 4 && packet[1] == 0x10));
}

#[test]
fn test_sda_full_queue_reports_failure_instead_of_acknowledging_acceptance() {
    let (mut sds, rx) = sda_setup(1);
    let mut queue = tetra_entities::MessageQueue::new();
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(1, "NC1|PING|")));
    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(2, "NC1|PING|")));
    assert_eq!(rx.len(), 1);
    assert_eq!(sda_downlink_payloads(&queue), vec![vec![0x82, 0x10, 0, 1], vec![0x82, 0x10, 2, 2]]);
}

#[test]
fn test_sda_reply_reaches_unlisted_radio_without_second_tl_header() {
    let (mut sds, _) = sda_setup(1);
    let mut queue = tetra_entities::MessageQueue::new();
    let text = tetra_entities::net_sda::info_reply("SIP lokal: registered; Dialoge 0");
    assert!(sds.rx_sds_from_control(&mut queue, tetra_entities::net_sda::reply_command(5102, 9, &text)));
    let packets = sda_downlink_payloads(&queue);
    assert_eq!(packets.len(), 1);
    assert_eq!(&packets[0][..4], &[0x82, 0, 9, 1]);
    assert_eq!(&packets[0][4..], text.as_bytes());
}

#[test]
fn test_sda_worker_answers_sip_and_persists_status_before_replying_over_local_rf() {
    use std::time::{Duration, SystemTime, UNIX_EPOCH};
    use tetra_entities::net_sda::{Journal, journal_path, runtime::start_worker};

    let id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos();
    let dir = std::env::temp_dir().join(format!("netcore-sda-rf-worker-{}-{id}", std::process::id()));
    std::fs::create_dir(&dir).unwrap();
    let path = journal_path(&dir.join("config.toml"));
    let mut cfg = ComponentTest::get_default_test_config(StackMode::Bs);
    cfg.wx_service.enabled = true;
    cfg.wx_service.service_issi = 4010001;
    cfg.asterisk.enabled = false;
    let cfg = tetra_config::bluestation::SharedConfig::from_parts(cfg, None);
    let (replies, rx) = crossbeam_channel::bounded(4);
    let requests = start_worker(cfg.clone(), replies, path.clone()).unwrap();
    let mut sds = tetra_entities::cmce::subentities::sds_bs::SdsBsSubentity::new(cfg);
    sds.set_sda_service(requests);
    let mut queue = tetra_entities::MessageQueue::new();

    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(6, "NC1|REQ|SIP")));
    let reply = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("SIP application reply from actual worker");
    assert!(sds.rx_sds_from_control(&mut queue, reply));
    let packets = sda_downlink_payloads(&queue);
    assert_eq!(packets.len(), 2);
    assert_eq!(&packets[1][4..], b"NC1|INFO|SIP lokal: deaktiviert");

    sds.route_rf_deliver(&mut queue, sda_uplink(5102, 4010001, sda_text(7, "NC1|STATUS|TRUPP1|ENROUTE")));
    let reply = rx
        .recv_timeout(Duration::from_secs(5))
        .expect("persisted status reply from actual worker");
    let snapshot = Journal::load(&path).unwrap().snapshot();
    assert_eq!(snapshot["records"].as_array().unwrap().len(), 1);
    assert_eq!(snapshot["records"][0]["source_issi"], 5102);
    assert_eq!(snapshot["unit_statuses"][0]["command"]["status"], "ENROUTE");
    assert!(sds.rx_sds_from_control(&mut queue, reply));
    let packets = sda_downlink_payloads(&queue);
    assert_eq!(packets.len(), 4);
    assert_eq!(&packets[3][4..], b"NC1|INFO|Status TRUPP1: ENROUTE; lokal erfasst #1");
    drop(sds);
    std::fs::remove_dir_all(dir).unwrap();
}
