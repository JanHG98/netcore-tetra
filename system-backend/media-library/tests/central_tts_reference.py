#!/usr/bin/env python3
"""Static regression checks for central Media-Library TTS ownership."""
from pathlib import Path
import tomllib

ROOT = Path(__file__).resolve().parents[3]


def read(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def main() -> None:
    media_http = read("system-backend/media-library/src/http.rs")
    media_main = read("system-backend/media-library/src/main.rs")
    media_tts = read("system-backend/media-library/src/tts.rs")
    bs_html = read("crates/tetra-entities/src/net_dashboard/html.rs")
    bs_main = read("bins/bluestation-bs/src/main.rs")
    bs_updater = read("install/update-basisstation.sh")
    player = read("crates/tetra-entities/src/net_audio_player/service.rs")
    media_state = read("system-backend/media-library/src/state.rs")
    archive_migration = read("system-backend/media-library/install/migrate-archive-layout.py")

    for route in (
        "/api/v1/tts/status",
        "/api/v1/tts/voices",
        "/api/v1/tts/templates",
        "/api/v1/tts/templates/save",
        "/api/v1/tts/templates/delete",
        "/api/v1/tts/generate",
    ):
        assert route in media_http, route

    assert "mod tts;" in media_main
    assert "TtsService::new" in media_main
    assert 'kind: Some("tts".to_string())' in media_tts
    assert "create_upload" in media_tts

    # The basis-station Audio Centre must no longer expose local Piper controls.
    for obsolete in (
        'id="tts-card"',
        'id="tts-template-card"',
        'id="tts-preview-card"',
        "/api/audio/tts/status",
        "generateTtsRecording()",
    ):
        assert obsolete not in bs_html, obsolete
    assert "Local basis-station TTS is deprecated and disabled" in bs_main
    assert "remove-local-tts-config.py" in bs_updater
    assert "disable --now netcore-piper.service" in bs_updater

    # Media Library catalogue intentionally has no kind filter: ready TTS assets
    # use the exact same tree browser/cache/playout path as recordings and uploads.
    assert 'query.append_pair("kind"' not in player
    assert 'format!("FREIGEGEBEN · {}", item.asset.kind.to_ascii_uppercase())' in player
    assert 'source_id: Some("media-library".to_string())' in player
    assert '"tts" => "TTS-Dateien"' in player
    assert '"recording" => "Recordings"' in player
    assert '"TTS-Dateien" | "Media-Library"' in player
    assert '"tts" => inner.config.storage.tts_archive_root.clone()' in media_state
    assert '"recording" => inner.config.storage.recording_archive_root.clone()' in media_state
    assert 'relocate_archived_asset' in archive_migration
    assert 'falsch einsortierte Archive verschoben' in archive_migration

    config = tomllib.loads(
        read("system-backend/media-library/config/media-library.example.toml")
    )
    assert config["tts"]["enabled"] is True
    assert config["tts"]["endpoint"] == "http://127.0.0.1:5005"
    assert config["storage"]["tts_archive_root"] == "/mnt/nfs-share/TTS-Dateien"
    assert config["runtime"]["auto_archive_tts"] is True
    assert len(config["tts"]["voices"]) >= 5

    for path in (
        "Docs/basisstation.config.sanitized.example.toml",
        "wiki/basisstation.config.sanitized.example.toml",
    ):
        cfg = tomllib.loads(read(path))
        assert "tts" not in cfg, path
        assert "media_library" in cfg, path

    print("central Media-Library TTS regression checks: OK")


if __name__ == "__main__":
    main()
