//! Persisted neighbour-cell editing. Lists are small (at most seven entries)
//! and applied through the same controlled restart as RF carrier changes.

use serde::Deserialize;
use tetra_config::bluestation::{CfgNeighborCellCa, parsing};
use toml_edit::{ArrayOfTables, Item};

#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NeighboursRequest {
    pub revision: String,
    pub neighbors: Vec<CfgNeighborCellCa>,
}

pub struct EditError {
    pub status: u16,
    pub message: String,
}

impl EditError {
    fn invalid(message: impl Into<String>) -> Self {
        Self {
            status: 400,
            message: message.into(),
        }
    }
    fn unavailable(message: impl Into<String>) -> Self {
        Self {
            status: 503,
            message: message.into(),
        }
    }
}

pub struct SavedNeighbours {
    pub changed: bool,
    pub neighbors: Vec<CfgNeighborCellCa>,
}

pub fn revision(original: &str) -> String {
    format!("{:x}", md5::compute(original.as_bytes()))
}

pub fn snapshot(
    original: &str,
    running: &[CfgNeighborCellCa],
    running_neighbor_cell_broadcast: u8,
) -> Result<serde_json::Value, String> {
    let cfg = parsing::from_toml_str(original).map_err(|e| e.to_string())?;
    let configured = serde_json::to_value(&cfg.cell.neighbor_cells_ca).map_err(|e| e.to_string())?;
    let running = serde_json::to_value(running).map_err(|e| e.to_string())?;
    Ok(serde_json::json!({
        "neighbors": configured,
        "running_neighbors": running,
        "revision": revision(original),
        "restart_required": configured != running || cfg.cell.neighbor_cell_broadcast != running_neighbor_cell_broadcast,
        "max_neighbors": 7,
        "neighbor_cell_broadcast": cfg.cell.neighbor_cell_broadcast,
        "running_neighbor_cell_broadcast": running_neighbor_cell_broadcast,
    }))
}

/// Produces one validated candidate, preserving unrelated tables and comments.
/// Typed deserialization rejects unknown row/service fields instead of dropping
/// them on save. StackConfig validates every transmitted bit width and duplicate.
pub fn compute_toml(original: &str, neighbors: &[CfgNeighborCellCa]) -> Result<String, String> {
    if neighbors.len() > 7 {
        return Err("Es sind höchstens sieben Nachbarzellen erlaubt.".to_string());
    }
    let configured = parsing::from_toml_str(original).map_err(|e| e.to_string())?;
    // ETSI EN 300 392-2 V3.8.1, 18.5.16/table 18.61: the first/MSB indicates support for
    // D-NWRK-BROADCAST (0b10); the second/LSB is the separate enquiry bit.
    // The MLE scheduler also broadcasts time without neighbours. Only manage
    // the broadcast bit here; retain the explicitly configured enquiry bit.
    let broadcasts = !neighbors.is_empty() || configured.cell.timezone.is_some();
    let neighbor_cell_broadcast =
        (configured.cell.neighbor_cell_broadcast & !0b10) | if broadcasts { 0b10 } else { 0 };
    let mut doc = super::config_edit::document(original)?;
    let mut entries = ArrayOfTables::new();
    for cell in neighbors {
        let encoded = toml::to_string(cell).map_err(|e| e.to_string())?;
        entries.push(super::config_edit::document(&encoded)?.as_table().clone());
    }
    // Inline table values avoid reusing the per-document table positions from
    // independently serialized rows (notably bs_service_details sub-tables).
    // This representation is valid for both normal and inline cell_info tables.
    let replacement = Item::Value(entries.into_array().into());
    let cell_table = super::config_edit::cell_table(&mut doc)?;
    cell_table.insert("neighbor_cells_ca", replacement);
    super::config_edit::set_scalar(
        cell_table,
        "neighbor_cell_broadcast",
        i64::from(neighbor_cell_broadcast).into(),
    );
    let candidate = doc.to_string();
    let cfg = parsing::from_toml_str(&candidate).map_err(|e| e.to_string())?;
    cfg.validate().map_err(str::to_string)?;
    Ok(candidate)
}

pub fn save(config_path: &str, body: &str) -> Result<SavedNeighbours, EditError> {
    let request: NeighboursRequest = serde_json::from_str(body).map_err(|e| EditError::invalid(format!("Ungültige Nachbarzellen: {e}")))?;
    let _guard = super::config_edit::CONFIG_EDIT_LOCK.lock().unwrap_or_else(|e| e.into_inner());
    let original = std::fs::read_to_string(config_path)
        .map_err(|e| EditError::unavailable(format!("Konfiguration kann nicht gelesen werden: {e}")))?;
    if revision(&original) != request.revision {
        return Err(EditError {
            status: 409,
            message: "Die Konfiguration wurde zwischenzeitlich geändert. Bitte neu laden.".to_string(),
        });
    }
    let prospective = compute_toml(&original, &request.neighbors).map_err(EditError::invalid)?;
    let before = parsing::from_toml_str(&original).map_err(|e| EditError::invalid(e.to_string()))?;
    let after = parsing::from_toml_str(&prospective).map_err(|e| EditError::invalid(e.to_string()))?;
    let changed = serde_json::to_value(&before.cell.neighbor_cells_ca).unwrap() != serde_json::to_value(&after.cell.neighbor_cells_ca).unwrap()
        || before.cell.neighbor_cell_broadcast != after.cell.neighbor_cell_broadcast;
    if changed {
        super::config_edit::write_checked(config_path, &original, &prospective).map_err(|e| EditError {
            status: if e.kind() == std::io::ErrorKind::WouldBlock { 409 } else { 503 },
            message: format!("Nachbarzellen konnten nicht gespeichert werden: {e}"),
        })?;
    }
    Ok(SavedNeighbours {
        changed,
        neighbors: request.neighbors,
    })
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;

    pub(crate) const IMAGE_CONFIG: &str = r#"# image/discovery generated table names
config_version = "0.6"
stack_mode = "Bs"
["phy_io"]
backend = "None"
["net_info"]
mcc = 901
mnc = 1510
["cell_info"] # same legal table as bare [cell_info]
main_carrier = 720
secondary_carrier = 721
dual_carrier_enabled = false # preserve this flag
freq_band = 4
freq_offset = 0
duplex_spacing = 4
reverse_operation = false
location_area = 1
local_ssi_ranges = [
    [0, 90],
]
["cell_info"."sds_command_control"]
authorized_issis = [2010001]
"#;

    pub(crate) fn image_config() -> String {
        let directory = std::env::temp_dir().join("netcore-config-fixture").to_string_lossy().to_string();
        format!(
            "{IMAGE_CONFIG}
[\"audio_player\"]
directory = {directory:?}
cache_directory = {directory:?}
[\"recording\"]
directory = {directory:?}
archive_directory = {directory:?}
tts_archive_directory = {directory:?}
[\"tts\"]
cache_directory = {directory:?}
template_directory = {directory:?}
"
        )
    }

    fn row(id: u8, carrier: u16) -> CfgNeighborCellCa {
        serde_json::from_value(serde_json::json!({
            "cell_identifier_ca":id, "main_carrier_number":carrier,
            "cell_reselection_types_supported":0,
            "neighbor_cell_synchronized":false,"cell_load_ca":0
        }))
        .unwrap()
    }

    #[test]
    fn create_edit_delete_roundtrip_keeps_other_image_config() {
        let mut neighbors = vec![row(1, 722), row(2, 723)];
        neighbors[0].mcc = Some(901);
        neighbors[0].mnc = Some(1510);
        neighbors[0].location_area = Some(2);
        neighbors[0].tdma_frame_offset = Some(15);
        neighbors[0].maximum_ms_transmit_power = Some(5);
        neighbors[0].minimum_rx_access_level = Some(9);
        neighbors[0].subscriber_class = Some(0xA101);
        neighbors[0].timeshare_cell_information_or_security_parameters = Some(31);
        neighbors[0].bs_service_details = Some(
            serde_json::from_value(serde_json::json!({
                "voice_service": true, "registration":true
            }))
            .unwrap(),
        );
        let created = compute_toml(&image_config(), &neighbors).unwrap();
        let cfg = parsing::from_toml_str(&created).unwrap();
        cfg.validate().unwrap();
        assert_eq!(cfg.cell.neighbor_cell_broadcast, 2);
        assert_eq!(
            serde_json::to_value(&cfg.cell.neighbor_cells_ca).unwrap(),
            serde_json::to_value(&neighbors).unwrap()
        );
        assert!(created.contains("dual_carrier_enabled = false # preserve this flag"));
        assert!(created.contains("[0, 90]"));
        assert!(created.contains("authorized_issis = [2010001]"));
        let before: toml::Value = image_config().parse().unwrap();
        let mut after: toml::Value = created.parse().unwrap();
        let after_cell = after
            .get_mut("cell_info")
            .unwrap()
            .as_table_mut()
            .unwrap();
        after_cell.remove("neighbor_cells_ca");
        after_cell.remove("neighbor_cell_broadcast");
        assert_eq!(before, after);

        neighbors[1].main_carrier_number = 724;
        let edited = compute_toml(&created, &neighbors).unwrap();
        assert_eq!(
            parsing::from_toml_str(&edited).unwrap().cell.neighbor_cells_ca[1].main_carrier_number,
            724
        );
        let deleted = compute_toml(&edited, &[]).unwrap();
        let cfg = parsing::from_toml_str(&deleted).unwrap();
        assert!(cfg.cell.neighbor_cells_ca.is_empty());
        assert_eq!(cfg.cell.neighbor_cell_broadcast, 0);
        assert!(!deleted.contains("[[cell_info.neighbor_cells_ca]]"));
    }

    #[test]
    fn edit_rejects_invalid_ranges_duplicates_and_excess_count() {
        let mut invalid = row(32, 722);
        assert!(compute_toml(&image_config(), &[invalid.clone()]).unwrap_err().contains("0-31"));
        invalid.cell_identifier_ca = 1;
        invalid.main_carrier_number = 4096;
        assert!(compute_toml(&image_config(), &[invalid.clone()]).unwrap_err().contains("0-4095"));
        invalid.main_carrier_number = 722;
        invalid.tdma_frame_offset = Some(64);
        assert!(compute_toml(&image_config(), &[invalid]).unwrap_err().contains("0-63"));
        assert!(
            compute_toml(&image_config(), &[row(1, 722), row(1, 723)])
                .unwrap_err()
                .contains("duplicate cell_identifier")
        );
        assert!(
            compute_toml(&image_config(), &[row(1, 722), row(2, 722)])
                .unwrap_err()
                .contains("duplicate main_carrier")
        );
        let too_many = (0..8).map(|id| row(id, 722 + u16::from(id))).collect::<Vec<_>>();
        assert!(compute_toml(&image_config(), &too_many).is_err());
    }

    #[test]
    fn request_rejects_unknown_fields_instead_of_silently_losing_them() {
        let mut neighbor = serde_json::to_value(row(1, 722)).unwrap();
        neighbor["unexpected"] = serde_json::json!(42);
        let body = serde_json::json!({"revision":"x","neighbors":[neighbor]}).to_string();
        assert!(serde_json::from_str::<NeighboursRequest>(&body).is_err());
        let mut neighbor = serde_json::to_value(row(1, 722)).unwrap();
        neighbor["bs_service_details"] = serde_json::json!({"unsupported_flag": true});
        let body = serde_json::json!({"revision":"x","neighbors":[neighbor]}).to_string();
        assert!(serde_json::from_str::<NeighboursRequest>(&body).is_err());
    }

    fn with_capability(capability: u8, timezone: bool) -> String {
        let timezone = if timezone { "timezone = \"Europe/Berlin\"\n" } else { "" };
        image_config().replace(
            "dual_carrier_enabled = false # preserve this flag\n",
            &format!("dual_carrier_enabled = false # preserve this flag\nneighbor_cell_broadcast = {capability} # keep capability comment\n{timezone}"),
        )
    }

    #[test]
    fn broadcast_bit_tracks_periodic_neighbour_or_time_broadcasts_and_preserves_enquiry() {
        for capability in 0..=3 {
            for timezone in [false, true] {
                let source = with_capability(capability, timezone);
                for neighbors in [vec![], vec![row(1, 722)]] {
                    let written = compute_toml(&source, &neighbors).unwrap();
                    let cfg = parsing::from_toml_str(&written).unwrap();
                    let expected = (capability & 1) | if timezone || !neighbors.is_empty() { 2 } else { 0 };
                    assert_eq!(cfg.cell.neighbor_cell_broadcast, expected);
                    assert!(written.contains("# keep capability comment"));
                    assert_eq!(cfg.cell.timezone.is_some(), timezone);
                }
            }
        }
        let invalid = with_capability(4, false);
        assert!(compute_toml(&invalid, &[row(1, 722)]).unwrap_err().contains("neighbor_cell_broadcast must be 0-3"));
    }

    #[test]
    fn capability_edit_preserves_bare_quoted_and_inline_cell_tables() {
        let original = with_capability(1, false);
        let mut sources = ["[cell_info]", "[\"cell_info\"]", "['cell_info']"]
            .map(|header| original.replace("[\"cell_info\"]", header))
            .to_vec();
        let mut inline = super::super::config_edit::document(&original).unwrap();
        let table = inline.remove("cell_info").unwrap().as_table().unwrap().clone();
        inline.insert("cell_info", Item::Value(table.into_inline_table().into()));
        sources.push(inline.to_string());
        for (index, source) in sources.iter().enumerate() {
            let written = compute_toml(source, &[row(1, 722)]).unwrap();
            let cfg = parsing::from_toml_str(&written).unwrap();
            cfg.validate().unwrap();
            assert_eq!(cfg.cell.neighbor_cell_broadcast, 3);
            let before: toml::Value = source.parse().unwrap();
            let mut after: toml::Value = written.parse().unwrap();
            after["cell_info"]["neighbor_cell_broadcast"] = toml::Value::Integer(1);
            after["cell_info"].as_table_mut().unwrap().remove("neighbor_cells_ca");
            assert_eq!(before, after);
            if index < 3 {
                assert!(written.contains("neighbor_cell_broadcast = 3 # keep capability comment"));
            } else {
                assert!(super::super::config_edit::document(&written).unwrap()["cell_info"].as_inline_table().is_some());
            }
        }
    }

    #[test]
    fn unchanged_list_repairs_stale_broadcast_capability_once() {
        let neighbors = vec![row(1, 722)];
        for enquiry in 0..=1 {
            let configured = compute_toml(&with_capability(enquiry, false), &neighbors).unwrap();
            let stale = configured.replace(
                &format!("neighbor_cell_broadcast = {}", enquiry | 2),
                &format!("neighbor_cell_broadcast = {enquiry}"),
            );
            assert_eq!(parsing::from_toml_str(&stale).unwrap().cell.neighbor_cell_broadcast, enquiry);
            let path = std::env::temp_dir().join(format!("netcore-neighbours-capability-{}-{enquiry}.toml", std::process::id()));
            let path_str = path.to_str().unwrap();
            std::fs::write(&path, &stale).unwrap();
            let request = serde_json::json!({"revision":revision(&stale),"neighbors":neighbors}).to_string();
            assert!(save(path_str, &request).unwrap_or_else(|e| panic!("{}", e.message)).changed);
            let written = std::fs::read_to_string(&path).unwrap();
            assert_eq!(parsing::from_toml_str(&written).unwrap().cell.neighbor_cell_broadcast, enquiry | 2);
            assert_eq!(serde_json::to_value(parsing::from_toml_str(&written).unwrap().cell.neighbor_cells_ca).unwrap(), serde_json::to_value(&neighbors).unwrap());
            assert_eq!(save(path_str, &request).err().unwrap().status, 409);
            let request = serde_json::json!({"revision":revision(&written),"neighbors":neighbors}).to_string();
            assert!(!save(path_str, &request).unwrap_or_else(|e| panic!("{}", e.message)).changed);
            assert_eq!(std::fs::read_to_string(&path).unwrap(), written);
            std::fs::remove_file(path).unwrap();
        }
    }

    #[test]
    fn saving_last_neighbour_removal_keeps_time_broadcast_and_enquiry_capabilities() {
        for enquiry in 0..=1 {
            for timezone in [false, true] {
                let source = compute_toml(&with_capability(enquiry, timezone), &[row(1, 722)]).unwrap();
                let path = std::env::temp_dir().join(format!("netcore-neighbours-remove-{}-{enquiry}-{timezone}.toml", std::process::id()));
                let path_str = path.to_str().unwrap();
                std::fs::write(&path, &source).unwrap();
                let request = serde_json::json!({"revision":revision(&source),"neighbors":[]}).to_string();
                assert!(save(path_str, &request).unwrap_or_else(|e| panic!("{}", e.message)).changed);
                let written = std::fs::read_to_string(&path).unwrap();
                let cfg = parsing::from_toml_str(&written).unwrap();
                assert!(cfg.cell.neighbor_cells_ca.is_empty());
                assert_eq!(cfg.cell.neighbor_cell_broadcast, enquiry | if timezone { 2 } else { 0 });
                assert_eq!(cfg.cell.timezone.is_some(), timezone);
                let request = serde_json::json!({"revision":revision(&written),"neighbors":[]}).to_string();
                assert!(!save(path_str, &request).unwrap_or_else(|e| panic!("{}", e.message)).changed);
                assert_eq!(std::fs::read_to_string(&path).unwrap(), written);
                std::fs::remove_file(path).unwrap();
            }
        }
    }

    #[test]
    fn saved_list_detects_stale_edits_and_does_not_restart_itself() {
        let path = std::env::temp_dir().join(format!("netcore-neighbours-save-{}.toml", std::process::id()));
        let path_str = path.to_str().unwrap();
        std::fs::write(&path, &image_config()).unwrap();
        let request = serde_json::json!({"revision":revision(&image_config()),"neighbors":[row(1,722)]}).to_string();
        let saved = save(path_str, &request).unwrap_or_else(|e| panic!("{}", e.message));
        assert!(saved.changed);
        let written = std::fs::read_to_string(&path).unwrap();
        assert_eq!(parsing::from_toml_str(&written).unwrap().cell.neighbor_cells_ca.len(), 1);
        assert_eq!(parsing::from_toml_str(&written).unwrap().cell.neighbor_cell_broadcast, 2);
        assert_eq!(save(path_str, &request).err().unwrap().status, 409);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), written);
        let invalid = serde_json::json!({"revision":revision(&written),"neighbors":[row(32,722)]}).to_string();
        assert_eq!(save(path_str, &invalid).err().unwrap().status, 400);
        assert_eq!(std::fs::read_to_string(&path).unwrap(), written);
        let noop = serde_json::json!({"revision":revision(&written),"neighbors":[row(1,722)]}).to_string();
        assert!(!save(path_str, &noop).unwrap_or_else(|e| panic!("{}", e.message)).changed);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn snapshot_distinguishes_saved_from_running_configuration() {
        let cfg = compute_toml(&image_config(), &[row(1, 722)]).unwrap();
        let snapshot = snapshot(&cfg, &[], 0).unwrap();
        assert_eq!(snapshot["neighbors"].as_array().unwrap().len(), 1);
        assert_eq!(snapshot["running_neighbors"].as_array().unwrap().len(), 0);
        assert_eq!(snapshot["restart_required"], true);
    }

    #[test]
    fn snapshot_detects_pending_restart_when_only_capability_differs() {
        let neighbors = vec![row(1, 722)];
        let cfg = compute_toml(&image_config(), &neighbors).unwrap();
        let pending = snapshot(&cfg, &neighbors, 0).unwrap();
        assert_eq!(pending["neighbor_cell_broadcast"], 2);
        assert_eq!(pending["running_neighbor_cell_broadcast"], 0);
        assert_eq!(pending["restart_required"], true);
        assert_eq!(snapshot(&cfg, &neighbors, 2).unwrap()["restart_required"], false);
    }
}
