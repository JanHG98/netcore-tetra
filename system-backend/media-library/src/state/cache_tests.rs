use super::*;
use base64::Engine;
use crate::config::PlayoutStationConfig;

struct Fixture {
    directory: PathBuf,
    config: MediaLibraryConfig,
    library: SharedLibrary,
}

impl Fixture {
    fn new() -> Self {
        let directory = std::env::temp_dir().join(format!("netcore-media-cache-{}", Uuid::new_v4()));
        let mut config = MediaLibraryConfig::default();
        config.storage.root = directory.join("assets");
        config.storage.state_file = directory.join("state.json");
        config.storage.temp_root = directory.join("tmp");
        config.storage.backup_root = directory.join("backups");
        config.storage.archive_root = Some(directory.join("nas/Media-Library"));
        config.storage.recording_archive_root = Some(directory.join("nas/Recordings"));
        config.storage.tts_archive_root = Some(directory.join("nas/TTS-Dateien"));
        config.storage.archive_require_network_mount = false;
        config.storage.local_cache_max_bytes = 0;
        config.storage.min_free_bytes = 0;
        config.runtime.operating_mode = "authoritative".to_string();
        config.playout.default_station = Some("fixture".to_string());
        config.playout.stations = vec![PlayoutStationConfig {
            id: "fixture".to_string(), name: "Fixture".to_string(),
            base_url: "http://127.0.0.1:1".to_string(), ..Default::default()
        }];
        for path in [&config.storage.archive_root, &config.storage.recording_archive_root,
                     &config.storage.tts_archive_root].into_iter().flatten() {
            fs::create_dir_all(path).unwrap();
        }
        let library = SharedLibrary::load(config.clone()).unwrap();
        Self { directory, config, library }
    }

    fn ready(&self, kind: &str) -> AssetRecord {
        let data = [100i16, -200, 300, -400].repeat(120);
        let size = (data.len() * 2) as u32;
        let mut wav = b"RIFF".to_vec();
        wav.extend((36 + size).to_le_bytes());
        wav.extend(b"WAVEfmt ");
        wav.extend(16u32.to_le_bytes());
        wav.extend(1u16.to_le_bytes());
        wav.extend(1u16.to_le_bytes());
        wav.extend(8000u32.to_le_bytes());
        wav.extend(16000u32.to_le_bytes());
        wav.extend(2u16.to_le_bytes());
        wav.extend(16u16.to_le_bytes());
        wav.extend(b"data");
        wav.extend(size.to_le_bytes());
        for sample in data { wav.extend(sample.to_le_bytes()); }
        let input: UploadInput = serde_json::from_value(json!({
            "name":"Archive fixture", "filename":"recording.wav", "media_type":"audio/wav",
            "kind":kind, "approve":true,
            "data_base64":base64::engine::general_purpose::STANDARD.encode(wav),
        })).unwrap();
        let asset = self.library.create_upload(input).unwrap();
        let original = asset.original_path.as_ref().unwrap();
        let processed = media::process_asset(&self.config, original, original.parent().unwrap(), "audio/wav").unwrap();
        self.library.complete_processing(&asset.asset_id, processed).unwrap()
    }

    fn archived(&self) -> AssetRecord {
        let asset = self.ready("recording");
        self.library.archive_asset(&asset.asset_id, ActionInput { actor: None }).unwrap()
    }

    fn stored(&self, id: &str) -> AssetRecord {
        lock(&self.library.inner).state.assets.get(id).unwrap().clone()
    }

    fn nas_payload(&self, asset: &AssetRecord, index: usize) -> PathBuf {
        asset.archive_path.as_ref().unwrap().parent().unwrap().join(&asset.archive_files[index].filename)
    }
}

impl Drop for Fixture {
    fn drop(&mut self) { let _ = fs::remove_dir_all(&self.directory); }
}

#[test]
fn verified_eviction_recall_and_open_http_descriptor_keep_preview_usable() {
    let f = Fixture::new();
    let asset = f.archived();
    let original = fs::read(asset.original_path.as_ref().unwrap()).unwrap();
    assert!(!asset.archive_files.is_empty());
    let result = f.library.maintenance(None).unwrap();
    assert_eq!(result["cache"]["offloaded"], 1);
    assert!(f.stored(&asset.asset_id).local_offloaded);
    assert!(!asset.original_path.as_ref().unwrap().exists());
    let (mut file, content_type, _) = f.library.file_for(&asset.asset_id, "preview").unwrap();
    assert_eq!(content_type, "audio/wav");
    assert!(!f.stored(&asset.asset_id).local_offloaded);
    f.library.maintenance(None).unwrap();
    assert!(f.stored(&asset.asset_id).local_offloaded);
    let mut read = Vec::new();
    file.read_to_end(&mut read).unwrap();
    assert_eq!(read, original);
    assert!(!f.library.waveform(&asset.asset_id, 32).unwrap().is_empty());
}

#[test]
fn offline_nas_preserves_all_local_payloads_and_readiness() {
    let f = Fixture::new();
    let asset = f.archived();
    let root = f.config.storage.recording_archive_root.as_ref().unwrap();
    fs::rename(root, root.with_extension("offline")).unwrap();
    let result = f.library.maintenance(None).unwrap();
    assert_eq!(result["cache"]["offloaded"], 0);
    assert!(!f.stored(&asset.asset_id).local_offloaded);
    assert!(asset.original_path.as_ref().unwrap().is_file());
    assert!(asset.preview_path.as_ref().unwrap().is_file());
    assert!(f.library.readiness().ready);
    assert!(f.library.status().archive_last_error.is_some());
}

#[test]
fn corrupt_or_missing_nas_payload_never_authorizes_local_deletion() {
    for missing in [false, true] {
        let f = Fixture::new();
        let asset = f.archived();
        let path = f.nas_payload(&asset, 0);
        if missing { fs::remove_file(path).unwrap(); }
        else { fs::write(path, b"corrupt NAS copy").unwrap(); }
        f.library.maintenance(None).unwrap();
        assert!(!f.stored(&asset.asset_id).local_offloaded);
        assert!(asset.original_path.as_ref().unwrap().is_file());
        assert!(asset.preview_path.as_ref().unwrap().is_file());
    }
}

#[test]
fn editable_manifest_cannot_replace_private_local_receipt() {
    let f = Fixture::new();
    let asset = f.archived();
    let path = asset.archive_path.as_ref().unwrap();
    let mut manifest: Value = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
    manifest["files"][0]["sha256"] = json!("attacker-supplied-hash");
    fs::write(path, serde_json::to_vec(&manifest).unwrap()).unwrap();
    f.library.maintenance(None).unwrap();
    assert!(asset.original_path.as_ref().unwrap().is_file());
    assert!(!f.stored(&asset.asset_id).local_offloaded);
}

#[test]
fn missing_local_original_cannot_produce_partial_successful_archive() {
    let f = Fixture::new();
    let asset = f.ready("recording");
    fs::remove_file(asset.original_path.as_ref().unwrap()).unwrap();
    assert!(f.library.archive_asset(&asset.asset_id, ActionInput { actor: None }).is_err());
    assert!(!f.stored(&asset.asset_id).archived);
    assert!(f.stored(&asset.asset_id).archive_files.is_empty());
}

#[test]
fn failed_marker_commit_keeps_originals_even_as_root() {
    let f = Fixture::new();
    let asset = f.archived();
    fs::rename(&f.config.storage.state_file, f.directory.join("saved-state.json")).unwrap();
    fs::create_dir(&f.config.storage.state_file).unwrap();
    assert!(f.library.maintenance(None).is_err());
    assert!(asset.original_path.as_ref().unwrap().is_file());
    assert!(asset.preview_path.as_ref().unwrap().is_file());
    assert!(!f.stored(&asset.asset_id).local_offloaded);
    assert!(!f.library.readiness().ready);
}

#[test]
fn restart_between_marker_commit_and_unlink_recalls_all_payloads() {
    let f = Fixture::new();
    let asset = f.archived();
    {
        let mut inner = lock(&f.library.inner);
        inner.state.assets.get_mut(&asset.asset_id).unwrap().local_offloaded = true;
        persist_locked(&mut inner).unwrap();
    }
    fs::remove_file(asset.original_path.as_ref().unwrap()).unwrap();
    let restarted = SharedLibrary::load(f.config.clone()).unwrap();
    restarted.file_for(&asset.asset_id, "preview").unwrap();
    assert!(asset.original_path.as_ref().unwrap().is_file());
    assert!(asset.preview_path.as_ref().unwrap().is_file());
    assert_eq!(media::sha256_file(asset.original_path.as_ref().unwrap()).unwrap(), asset.sha256.unwrap());
}

#[test]
fn legacy_archived_record_needs_fresh_receipt_before_first_eviction() {
    let f = Fixture::new();
    let mut asset = f.archived();
    {
        let mut inner = lock(&f.library.inner);
        inner.state.assets.get_mut(&asset.asset_id).unwrap().archive_files.clear();
        persist_locked(&mut inner).unwrap();
    }
    asset.archive_files.clear();
    assert!(crate::archive::automatic(&f.config, &asset));
    f.library.maintenance(None).unwrap();
    assert!(asset.original_path.as_ref().unwrap().is_file());
    let verified = f.library.archive_asset(&asset.asset_id, ActionInput { actor: None }).unwrap();
    assert!(!verified.archive_files.is_empty());
    f.library.maintenance(None).unwrap();
    assert!(!asset.original_path.as_ref().unwrap().exists());
}

#[test]
fn queued_and_playing_jobs_pin_recalled_payloads() {
    let f = Fixture::new();
    let asset = f.archived();
    f.library.maintenance(None).unwrap();
    let input: DispatchInput = serde_json::from_value(json!({
        "asset_id":asset.asset_id, "destination_kind":"group",
        "destination_id":1234, "station_id":"fixture",
    })).unwrap();
    let job = f.library.create_dispatch(input).unwrap();
    assert!(!f.stored(&asset.asset_id).local_offloaded);
    f.library.maintenance(None).unwrap();
    assert!(asset.preview_path.as_ref().unwrap().is_file());
    let claim = f.library.claim_dispatch().unwrap();
    assert_eq!(claim.job.job_id, job.job_id);
    f.library.maintenance(None).unwrap();
    assert!(asset.preview_path.as_ref().unwrap().is_file());
    f.library.complete_dispatch(&job.job_id);
    f.library.maintenance(None).unwrap();
    assert!(!asset.preview_path.as_ref().unwrap().exists());
}

#[test]
fn insufficient_recall_space_preserves_nas_receipt_and_offloaded_state() {
    let f = Fixture::new();
    let asset = f.archived();
    f.library.maintenance(None).unwrap();
    lock(&f.library.inner).config.storage.min_free_bytes = u64::MAX;
    assert!(f.library.file_for(&asset.asset_id, "preview").is_err());
    assert!(f.stored(&asset.asset_id).local_offloaded);
    assert!(f.nas_payload(&asset, 0).is_file());
    assert!(!asset.original_path.as_ref().unwrap().exists());
}

#[test]
fn corrupt_recall_never_becomes_playable() {
    let f = Fixture::new();
    let asset = f.archived();
    f.library.maintenance(None).unwrap();
    fs::write(f.nas_payload(&asset, 0), b"corrupt").unwrap();
    assert!(f.library.file_for(&asset.asset_id, "preview").is_err());
    assert!(f.stored(&asset.asset_id).local_offloaded);
    assert!(!asset.original_path.as_ref().unwrap().exists());
}

#[test]
fn aged_payloads_leave_cache_and_auto_offload_can_be_disabled() {
    let f = Fixture::new();
    let asset = f.archived();
    {
        let mut inner = lock(&f.library.inner);
        inner.config.storage.local_cache_max_bytes = u64::MAX;
        inner.state.assets.get_mut(&asset.asset_id).unwrap().local_last_accessed_at =
            Some(Utc::now() - chrono::Duration::days(2));
        inner.config.storage.auto_offload = false;
    }
    f.library.maintenance(None).unwrap();
    assert!(asset.original_path.as_ref().unwrap().exists());
    lock(&f.library.inner).config.storage.auto_offload = true;
    f.library.maintenance(None).unwrap();
    assert!(!asset.original_path.as_ref().unwrap().exists());
}

#[test]
fn reprocessing_recalls_original_and_invalidates_previous_receipt() {
    let f = Fixture::new();
    let asset = f.archived();
    f.library.maintenance(None).unwrap();
    let result = f.library.reprocess_asset(&asset.asset_id, ActionInput { actor: None }).unwrap();
    assert!(result.original_path.as_ref().unwrap().is_file());
    assert_eq!(result.state, "processing");
    assert!(!result.archived);
    assert!(result.archive_files.is_empty());
}

#[test]
fn changing_kind_recalls_old_root_before_switching_archive() {
    let f = Fixture::new();
    let asset = f.archived();
    f.library.maintenance(None).unwrap();
    let input = serde_json::from_value(json!({"kind":"tts"})).unwrap();
    let changed = f.library.update_asset(&asset.asset_id, input).unwrap();
    assert_eq!(changed.kind, "tts");
    assert!(changed.original_path.as_ref().unwrap().is_file());
    assert!(!changed.archived);
    let archived = f.library.archive_asset(&asset.asset_id, ActionInput { actor: None }).unwrap();
    assert!(archived.archive_path.unwrap().starts_with(f.config.storage.tts_archive_root.as_ref().unwrap()));
}

#[test]
fn retry_recalls_both_station_and_direct_tetra_playout_before_queueing() {
    for direct in [false, true] {
        let f = Fixture::new();
        let mut asset = f.ready("recording");
        if direct {
            let tetra = f.config.storage.root.join(&asset.asset_id).join("audio.tacelp");
            fs::write(&tetra, [0x5a; 35 * 3]).unwrap();
            {
                let mut inner = lock(&f.library.inner);
                inner.config.playout.mode = MEDIA_SWITCH_PLAYOUT_MODE.to_string();
                let stored = inner.state.assets.get_mut(&asset.asset_id).unwrap();
                stored.tetra_path = Some(tetra.clone());
                stored.tetra_sha256 = Some(media::sha256_file(&tetra).unwrap());
                stored.metadata.tetra_frame_count = Some(3);
                stored.broadcast_ready = true;
            }
        }
        asset = f.library.archive_asset(&asset.asset_id, ActionInput { actor: None }).unwrap();
        let input = serde_json::from_value(json!({
            "asset_id":asset.asset_id,"destination_kind":"group","destination_id":1234,
            "station_id":"fixture","session_id":"fixture-session","target_logical_ts":1,
        })).unwrap();
        let job = f.library.create_dispatch(input).unwrap();
        f.library.claim_dispatch().unwrap();
        f.library.fail_dispatch(&job.job_id, "fixture failure".to_string());
        f.library.maintenance(None).unwrap();
        assert!(f.stored(&asset.asset_id).local_offloaded);
        let archive_root = f.config.storage.recording_archive_root.as_ref().unwrap();
        let offline = archive_root.with_extension("offline");
        fs::rename(archive_root, &offline).unwrap();
        assert!(f.library.retry_job(&job.job_id, ActionInput { actor: None }).is_err());
        assert_eq!(f.library.job(&job.job_id).unwrap().state, "failed");
        fs::rename(&offline, archive_root).unwrap();
        let retry = f.library.retry_job(&job.job_id, ActionInput { actor: None }).unwrap();
        assert_eq!(retry.state, "queued");
        assert!(!f.stored(&asset.asset_id).local_offloaded);
        let claim = f.library.claim_dispatch().unwrap();
        if direct {
            assert!(claim.tetra_path.unwrap().is_file());
        } else {
            assert!(asset.preview_path.as_ref().unwrap().is_file());
        }
    }
}

#[test]
fn a_bad_archive_does_not_starve_other_assets_and_retries_are_bounded() {
    let f = Fixture::new();
    let bad = f.ready("recording");
    let good = f.ready("recording");
    fs::write(bad.original_path.as_ref().unwrap(), b"changed original").unwrap();
    assert_eq!(f.library.next_auto_archive().unwrap().asset_id, bad.asset_id);
    let error = f.library.archive_asset(&bad.asset_id, ActionInput { actor: None }).unwrap_err();
    f.library.record_archive_failure(error);
    assert_eq!(f.library.next_auto_archive().unwrap().asset_id, good.asset_id);
    f.library.archive_asset(&good.asset_id, ActionInput { actor: None }).unwrap();
    assert!(f.library.next_auto_archive().is_none());
    lock(&f.library.inner).archive_attempts.insert(bad.asset_id.clone(),
        Utc::now() - chrono::Duration::seconds(61));
    assert_eq!(f.library.next_auto_archive().unwrap().asset_id, bad.asset_id);
    assert!(bad.original_path.as_ref().unwrap().is_file());
}

#[test]
fn recall_evicts_another_idle_asset_to_make_room_for_requested_payloads() {
    let f = Fixture::new();
    let wanted = f.archived();
    f.library.maintenance(None).unwrap();
    let idle = f.archived();
    assert!(f.stored(&wanted.asset_id).local_offloaded);
    assert!(!f.stored(&idle.asset_id).local_offloaded);
    f.library.file_for(&wanted.asset_id, "preview").unwrap();
    assert!(!f.stored(&wanted.asset_id).local_offloaded);
    assert!(f.stored(&idle.asset_id).local_offloaded);
    assert!(wanted.preview_path.as_ref().unwrap().is_file());
    assert!(!idle.preview_path.as_ref().unwrap().exists());
}

#[test]
fn complete_valid_local_crash_copy_needs_neither_nas_nor_duplicate_disk_space() {
    let f = Fixture::new();
    let asset = f.archived();
    let original = asset.original_path.as_ref().unwrap();
    use std::os::unix::fs::MetadataExt;
    let inode = fs::metadata(original).unwrap().ino();
    {
        let mut inner = lock(&f.library.inner);
        inner.state.assets.get_mut(&asset.asset_id).unwrap().local_offloaded = true;
        inner.config.storage.min_free_bytes = u64::MAX;
        persist_locked(&mut inner).unwrap();
    }
    let root = f.config.storage.recording_archive_root.as_ref().unwrap();
    fs::rename(root, root.with_extension("offline")).unwrap();
    f.library.file_for(&asset.asset_id, "preview").unwrap();
    assert!(!f.stored(&asset.asset_id).local_offloaded);
    assert_eq!(fs::metadata(original).unwrap().ino(), inode);
}

