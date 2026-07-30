// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für gespeicherte Aufzeichnungen, TTS- und Mediendateien.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::fs::{self, File};
use std::io::{Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;

use base64::Engine;
use sha2::{Digest, Sha256};

use crate::config::MediaLibraryConfig;
use crate::model::{AudioMetadata, ProcessResult};

// Was: Legt den festen Wert `TETRA_FRAME_BYTES` für TETRA Funkrahmen bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_FRAME_BYTES: usize = 35;

// Was: Diese Funktion dekodiert base64.
// Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
pub fn decode_base64(data: &str) -> Result<Vec<u8>, String> {
    let payload = data
        .split_once(',')
        .map(|(_, value)| value)
        .unwrap_or(data)
        .trim();
    base64::engine::general_purpose::STANDARD
        .decode(payload)
        .map_err(|error| format!("invalid base64 payload: {error}"))
}

// Was: Führt den Arbeitsschritt `sha256_bytes` für sha256 bytes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn sha256_bytes(bytes: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    format!("{:x}", hasher.finalize())
}

// Was: Führt den Arbeitsschritt `sha256_file` für sha256 file aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn sha256_file(path: &Path) -> Result<String, String> {
    let mut file = File::open(path).map_err(|error| format!("cannot open {}: {error}", path.display()))?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 64 * 1024];
    // Was: Startet eine bewusst dauerhaft laufende Verarbeitungsschleife.
    // Warum: Dienste und Empfänger müssen fortlaufend auf neue Ereignisse reagieren, bis sie ausdrücklich beendet werden.
    loop {
        let read = file.read(&mut buffer).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
        if read == 0 {
            break;
        }
        hasher.update(&buffer[..read]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

// Was: Führt den Arbeitsschritt `safe_filename` für safe filename aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn safe_filename(value: &str, fallback: &str) -> String {
    let value = value
        .rsplit(['/', '\\'])
        .next()
        .unwrap_or(value)
        .trim();
    let mut output = String::with_capacity(value.len());
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for character in value.chars().take(160) {
        if character.is_ascii_alphanumeric() || matches!(character, '.' | '-' | '_' | ' ') {
            output.push(character);
        } else {
            output.push('_');
        }
    }
    let output = output.trim_matches(['.', ' ']).trim();
    if output.is_empty() {
        fallback.to_string()
    } else {
        output.to_string()
    }
}

// Was: Führt den Arbeitsschritt `extension_for` für extension for aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn extension_for(filename: &str, media_type: &str, bytes: &[u8]) -> String {
    if bytes.len() >= 12 && &bytes[..4] == b"RIFF" && &bytes[8..12] == b"WAVE" {
        return "wav".to_string();
    }
    if bytes.starts_with(b"ID3") || bytes.first().is_some_and(|value| *value == 0xff) {
        return "mp3".to_string();
    }
    if media_type.contains("tetra") || filename.to_ascii_lowercase().ends_with(".tacelp") {
        return "tacelp".to_string();
    }
    Path::new(filename)
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("bin")
        .to_ascii_lowercase()
}

// Was: Diese Funktion schreibt atomic.
// Warum: Die Ausgabe wird dadurch einheitlich erzeugt und Schreibfehler können behandelt werden.
pub fn write_atomic(path: &Path, bytes: &[u8], fsync: bool) -> Result<(), String> {
    let parent = path.parent().ok_or_else(|| "target has no parent directory".to_string())?;
    fs::create_dir_all(parent).map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    let part = path.with_extension(format!(
        "{}.part",
        path.extension().and_then(|value| value.to_str()).unwrap_or("tmp")
    ));
    let mut file = File::create(&part).map_err(|error| format!("cannot create {}: {error}", part.display()))?;
    file.write_all(bytes).map_err(|error| format!("cannot write {}: {error}", part.display()))?;
    if fsync {
        file.sync_all().map_err(|error| format!("cannot sync {}: {error}", part.display()))?;
    }
    fs::rename(&part, path).map_err(|error| format!("cannot publish {}: {error}", path.display()))?;
    Ok(())
}

// Was: Führt den Arbeitsschritt `copy_atomic` für copy atomic aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn copy_atomic(source: &Path, destination: &Path, fsync: bool) -> Result<u64, String> {
    let parent = destination.parent().ok_or_else(|| "destination has no parent".to_string())?;
    fs::create_dir_all(parent).map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    let part = destination.with_extension(format!(
        "{}.part",
        destination.extension().and_then(|value| value.to_str()).unwrap_or("tmp")
    ));
    let mut input = File::open(source).map_err(|error| format!("cannot open {}: {error}", source.display()))?;
    let mut output = File::create(&part).map_err(|error| format!("cannot create {}: {error}", part.display()))?;
    let bytes = std::io::copy(&mut input, &mut output)
        .map_err(|error| format!("cannot copy {}: {error}", source.display()))?;
    if fsync {
        output.sync_all().map_err(|error| format!("cannot sync {}: {error}", part.display()))?;
    }
    fs::rename(&part, destination)
        .map_err(|error| format!("cannot publish {}: {error}", destination.display()))?;
    Ok(bytes)
}

// Was: Führt den Arbeitsschritt `inspect_original` für inspect original aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn inspect_original(path: &Path, media_type: &str) -> Result<AudioMetadata, String> {
    let mut file = File::open(path).map_err(|error| format!("cannot open {}: {error}", path.display()))?;
    let mut header = [0u8; 12];
    let read = file.read(&mut header).map_err(|error| format!("cannot read {}: {error}", path.display()))?;
    file.seek(SeekFrom::Start(0)).map_err(|error| error.to_string())?;
    if read >= 12 && &header[..4] == b"RIFF" && &header[8..12] == b"WAVE" {
        return inspect_wav(path);
    }
    if media_type.contains("tetra") || path.extension().is_some_and(|value| value == "tacelp") {
        return inspect_tacelp(path);
    }
    if (read >= 3 && &header[..3] == b"ID3")
        || (read >= 2 && header[0] == 0xff && header[1] & 0xe0 == 0xe0)
    {
        return Ok(AudioMetadata {
            format: "mp3".to_string(),
            codec: Some("mpeg_audio".to_string()),
            data_bytes: Some(fs::metadata(path).map_err(|error| error.to_string())?.len()),
            ..AudioMetadata::default()
        });
    }
    Err("unsupported media format; expected RIFF/WAVE, MP3 or packed .tacelp".to_string())
}

// Was: Führt den Arbeitsschritt `inspect_tacelp` für inspect tacelp aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn inspect_tacelp(path: &Path) -> Result<AudioMetadata, String> {
    let size = fs::metadata(path).map_err(|error| format!("cannot stat {}: {error}", path.display()))?.len();
    if size == 0 || size % TETRA_FRAME_BYTES as u64 != 0 {
        return Err(format!(
            "packed TETRA file must contain a non-zero whole number of {TETRA_FRAME_BYTES}-byte frames"
        ));
    }
    let frames = size / TETRA_FRAME_BYTES as u64;
    Ok(AudioMetadata {
        format: "tacelp".to_string(),
        codec: Some("tetra_acelp0".to_string()),
        duration_ms: Some(frames.saturating_mul(60)),
        data_bytes: Some(size),
        tetra_frame_count: Some(frames),
        ..AudioMetadata::default()
    })
}

// Was: Führt den Arbeitsschritt `inspect_wav` für inspect wav aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn inspect_wav(path: &Path) -> Result<AudioMetadata, String> {
    let bytes = fs::read(path).map_err(|error| format!("cannot read WAV {}: {error}", path.display()))?;
    if bytes.len() < 44 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("invalid RIFF/WAVE header".to_string());
    }
    let mut offset = 12usize;
    let mut format_code = None;
    let mut channels = None;
    let mut sample_rate = None;
    let mut bits = None;
    let mut data_bytes = None;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while offset.saturating_add(8) <= bytes.len() {
        let id = &bytes[offset..offset + 4];
        let length = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()) as usize;
        let start = offset + 8;
        let end = start.saturating_add(length);
        if end > bytes.len() {
            return Err("WAV chunk exceeds file length".to_string());
        }
        if id == b"fmt " && length >= 16 {
            format_code = Some(u16::from_le_bytes(bytes[start..start + 2].try_into().unwrap()));
            channels = Some(u16::from_le_bytes(bytes[start + 2..start + 4].try_into().unwrap()));
            sample_rate = Some(u32::from_le_bytes(bytes[start + 4..start + 8].try_into().unwrap()));
            bits = Some(u16::from_le_bytes(bytes[start + 14..start + 16].try_into().unwrap()));
        } else if id == b"data" {
            data_bytes = Some(length as u64);
        }
        offset = end + (length & 1);
    }
    let format_code = format_code.ok_or_else(|| "WAV fmt chunk missing".to_string())?;
    let channels = channels.ok_or_else(|| "WAV channel count missing".to_string())?;
    let sample_rate = sample_rate.ok_or_else(|| "WAV sample rate missing".to_string())?;
    let bits = bits.ok_or_else(|| "WAV bits-per-sample missing".to_string())?;
    let data_bytes = data_bytes.ok_or_else(|| "WAV data chunk missing".to_string())?;
    if channels == 0 || sample_rate == 0 || bits == 0 {
        return Err("WAV contains invalid zero-valued format fields".to_string());
    }
    let bytes_per_second = sample_rate as u64 * channels as u64 * ((bits as u64 + 7) / 8);
    let duration_ms = if bytes_per_second == 0 {
        None
    } else {
        Some(data_bytes.saturating_mul(1000) / bytes_per_second)
    };
    Ok(AudioMetadata {
        format: "wav".to_string(),
        codec: Some(match format_code {
            1 => "pcm".to_string(),
            3 => "ieee_float".to_string(),
            value => format!("wav_format_{value}"),
        }),
        channels: Some(channels),
        sample_rate_hz: Some(sample_rate),
        bits_per_sample: Some(bits),
        duration_ms,
        data_bytes: Some(data_bytes),
        tetra_frame_count: None,
    })
}

// Was: Prüft, ob canonical preview zutrifft.
// Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
pub fn is_canonical_preview(metadata: &AudioMetadata) -> bool {
    metadata.format == "wav"
        && metadata.codec.as_deref() == Some("pcm")
        && metadata.channels == Some(1)
        && metadata.sample_rate_hz == Some(8_000)
        && metadata.bits_per_sample == Some(16)
}

// Was: Diese Funktion verarbeitet asset.
// Warum: Die einzelnen Verarbeitungsschritte bleiben damit gebündelt und leichter testbar.
pub fn process_asset(
    config: &MediaLibraryConfig,
    original_path: &Path,
    asset_directory: &Path,
    media_type: &str,
) -> Result<ProcessResult, String> {
    let mut metadata = inspect_original(original_path, media_type)?;
    fs::create_dir_all(asset_directory)
        .map_err(|error| format!("cannot create asset directory {}: {error}", asset_directory.display()))?;
    let preview_path = asset_directory.join("preview.wav");
    let tetra_path = asset_directory.join("audio.tacelp");
    let mut preview_ready = false;
    let mut broadcast_ready = false;
    let mut preview = None;
    let mut tetra = None;

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    match metadata.format.as_str() {
        "tacelp" => {
            validate_tetra_file(original_path, config.codec.frame_bytes)?;
            if original_path != tetra_path {
                copy_atomic(original_path, &tetra_path, config.storage.fsync_imports)?;
            }
            broadcast_ready = true;
            tetra = Some(tetra_path.clone());
            if !config.codec.decoder_command.is_empty() {
                run_template_command(&config.codec.decoder_command, original_path, &preview_path)?;
                let preview_meta = inspect_wav(&preview_path)?;
                if !is_canonical_preview(&preview_meta) {
                    return Err("configured TETRA decoder did not produce canonical 8 kHz mono PCM16 WAV".to_string());
                }
                preview_ready = true;
                preview = Some(preview_path.clone());
            }
        }
        "wav" => {
            if is_canonical_preview(&metadata) {
                if original_path != preview_path {
                    copy_atomic(original_path, &preview_path, config.storage.fsync_imports)?;
                }
            } else {
                run_template_command(&config.codec.ffmpeg_command, original_path, &preview_path)?;
            }
            let preview_meta = inspect_wav(&preview_path)?;
            if !is_canonical_preview(&preview_meta) {
                return Err("preview processor did not produce canonical 8 kHz mono PCM16 WAV".to_string());
            }
            metadata.duration_ms = preview_meta.duration_ms;
            preview_ready = true;
            preview = Some(preview_path.clone());
            if !config.codec.encoder_command.is_empty() {
                run_template_command(&config.codec.encoder_command, &preview_path, &tetra_path)?;
                let tetra_meta = validate_tetra_file(&tetra_path, config.codec.frame_bytes)?;
                metadata.tetra_frame_count = tetra_meta.tetra_frame_count;
                broadcast_ready = true;
                tetra = Some(tetra_path.clone());
            }
        }
        "mp3" => {
            run_template_command(&config.codec.ffmpeg_command, original_path, &preview_path)?;
            let preview_meta = inspect_wav(&preview_path)?;
            if !is_canonical_preview(&preview_meta) {
                return Err("ffmpeg did not produce canonical 8 kHz mono PCM16 WAV".to_string());
            }
            metadata.channels = preview_meta.channels;
            metadata.sample_rate_hz = preview_meta.sample_rate_hz;
            metadata.bits_per_sample = preview_meta.bits_per_sample;
            metadata.duration_ms = preview_meta.duration_ms;
            preview_ready = true;
            preview = Some(preview_path.clone());
            if !config.codec.encoder_command.is_empty() {
                run_template_command(&config.codec.encoder_command, &preview_path, &tetra_path)?;
                let tetra_meta = validate_tetra_file(&tetra_path, config.codec.frame_bytes)?;
                metadata.tetra_frame_count = tetra_meta.tetra_frame_count;
                broadcast_ready = true;
                tetra = Some(tetra_path.clone());
            }
        }
        _ => return Err(format!("unsupported media format {}", metadata.format)),
    }

    let preview_sha256 = preview.as_ref().map(|path| sha256_file(path)).transpose()?;
    let tetra_sha256 = tetra.as_ref().map(|path| sha256_file(path)).transpose()?;
    Ok(ProcessResult {
        metadata,
        preview_path: preview,
        tetra_path: tetra,
        preview_sha256,
        tetra_sha256,
        preview_ready,
        broadcast_ready,
    })
}

// Was: Diese Funktion prüft TETRA file.
// Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
fn validate_tetra_file(path: &Path, frame_bytes: usize) -> Result<AudioMetadata, String> {
    if frame_bytes != TETRA_FRAME_BYTES {
        return Err("only 35-byte packed TETRA ACELP frames are supported".to_string());
    }
    inspect_tacelp(path)
}

// Was: Diese Funktion führt template command.
// Warum: Der Lebenszyklus des Dienstes bleibt so an einer zentralen Stelle steuerbar.
pub fn run_template_command(template: &[String], input: &Path, output: &Path) -> Result<(), String> {
    if template.is_empty() {
        return Err("required media command is not configured".to_string());
    }
    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent).map_err(|error| format!("cannot create {}: {error}", parent.display()))?;
    }
    let part = command_partial_path(output);
    let _ = fs::remove_file(&part);
    let input_value = input.to_string_lossy().to_string();
    let output_value = part.to_string_lossy().to_string();
    let args = template
        .iter()
        .map(|value| value.replace("{input}", &input_value).replace("{output}", &output_value))
        .collect::<Vec<_>>();
    let (program, arguments) = args.split_first().ok_or_else(|| "media command is empty".to_string())?;
    let status = Command::new(program)
        .args(arguments)
        .status()
        .map_err(|error| format!("cannot execute {program}: {error}"))?;
    if !status.success() {
        let _ = fs::remove_file(&part);
        return Err(format!("media command {program} failed with status {status}"));
    }
    let metadata = fs::metadata(&part)
        .map_err(|error| format!("media command did not create {}: {error}", part.display()))?;
    if !metadata.is_file() || metadata.len() == 0 {
        let _ = fs::remove_file(&part);
        return Err("media command produced no regular output file".to_string());
    }
    fs::rename(&part, output).map_err(|error| format!("cannot publish {}: {error}", output.display()))?;
    Ok(())
}


// Was: Führt den Arbeitsschritt `command_partial_path` für command partial path aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn command_partial_path(output: &Path) -> PathBuf {
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let stem = output
        .file_stem()
        .and_then(|value| value.to_str())
        .unwrap_or("output");
    let extension = output
        .extension()
        .and_then(|value| value.to_str())
        .unwrap_or("bin");
    parent.join(format!("{stem}.part.{extension}"))
}

// Was: Führt den Arbeitsschritt `waveform` für waveform aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn waveform(path: &Path, points: usize) -> Result<Vec<f32>, String> {
    let bytes = fs::read(path).map_err(|error| format!("cannot read preview {}: {error}", path.display()))?;
    let data = wav_data_chunk(&bytes)?;
    if data.len() < 2 {
        return Ok(Vec::new());
    }
    let samples = data
        .chunks_exact(2)
        .map(|pair| i16::from_le_bytes([pair[0], pair[1]]))
        .collect::<Vec<_>>();
    let points = points.clamp(16, 2048).min(samples.len().max(1));
    let chunk = samples.len().div_ceil(points);
    Ok(samples
        .chunks(chunk.max(1))
        .map(|window| {
            window
                .iter()
                .map(|sample| sample.unsigned_abs() as f32 / i16::MAX as f32)
                .fold(0.0f32, f32::max)
        })
        .collect())
}

// Was: Führt den Arbeitsschritt `wav_data_chunk` für wav data chunk aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn wav_data_chunk(bytes: &[u8]) -> Result<&[u8], String> {
    if bytes.len() < 12 || &bytes[..4] != b"RIFF" || &bytes[8..12] != b"WAVE" {
        return Err("not a RIFF/WAVE file".to_string());
    }
    let mut offset = 12usize;
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    while offset.saturating_add(8) <= bytes.len() {
        let id = &bytes[offset..offset + 4];
        let length = u32::from_le_bytes(bytes[offset + 4..offset + 8].try_into().unwrap()) as usize;
        let start = offset + 8;
        let end = start.saturating_add(length);
        if end > bytes.len() {
            return Err("WAV chunk exceeds file length".to_string());
        }
        if id == b"data" {
            return Ok(&bytes[start..end]);
        }
        offset = end + (length & 1);
    }
    Err("WAV data chunk missing".to_string())
}

// Was: Führt den Arbeitsschritt `total_directory_bytes` für total directory bytes aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn total_directory_bytes(path: &Path) -> u64 {
    let mut total = 0u64;
    let Ok(entries) = fs::read_dir(path) else {
        return 0;
    };
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for entry in entries.flatten() {
        let path = entry.path();
        let Ok(metadata) = fs::symlink_metadata(&path) else {
            continue;
        };
        if metadata.file_type().is_symlink() {
            continue;
        }
        if metadata.is_dir() {
            total = total.saturating_add(total_directory_bytes(&path));
        } else if metadata.is_file() {
            total = total.saturating_add(metadata.len());
        }
    }
    total
}

// Was: Führt den Arbeitsschritt `file_is_within` für file is within aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn file_is_within(path: &Path, root: &Path) -> bool {
    let Ok(path) = path.canonicalize() else {
        return false;
    };
    let Ok(root) = root.canonicalize() else {
        return false;
    };
    path.starts_with(root) && path.is_file()
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    use std::time::{SystemTime, UNIX_EPOCH};

    // Was: Prüft automatisch den Fall directory.
    // Warum: Der Test schützt das Verhalten vor späteren Änderungen und macht Fehler reproduzierbar.
    fn test_directory(label: &str) -> PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system clock after epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "netcore-media-library-{label}-{}-{nonce}",
            std::process::id()
        ))
    }

    // Was: Führt den Arbeitsschritt `canonical_wav` für canonical wav aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn canonical_wav(samples: &[i16]) -> Vec<u8> {
        let data_bytes = samples.len() * 2;
        let mut bytes = Vec::with_capacity(44 + data_bytes);
        bytes.extend_from_slice(b"RIFF");
        bytes.extend_from_slice(&(36u32 + data_bytes as u32).to_le_bytes());
        bytes.extend_from_slice(b"WAVEfmt ");
        bytes.extend_from_slice(&16u32.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&1u16.to_le_bytes());
        bytes.extend_from_slice(&8_000u32.to_le_bytes());
        bytes.extend_from_slice(&16_000u32.to_le_bytes());
        bytes.extend_from_slice(&2u16.to_le_bytes());
        bytes.extend_from_slice(&16u16.to_le_bytes());
        bytes.extend_from_slice(b"data");
        bytes.extend_from_slice(&(data_bytes as u32).to_le_bytes());
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for sample in samples {
            bytes.extend_from_slice(&sample.to_le_bytes());
        }
        bytes
    }

    #[test]
    // Was: Führt den Arbeitsschritt `canonical_wav_is_inspected_and_waveformed` für canonical wav is inspected and waveformed aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn canonical_wav_is_inspected_and_waveformed() {
        let directory = test_directory("wav");
        let path = directory.join("sample.wav");
        let samples = [0, 1_000, -2_000, i16::MAX, i16::MIN];
        write_atomic(&path, &canonical_wav(&samples), false).expect("write WAV");
        let metadata = inspect_wav(&path).expect("inspect WAV");
        assert!(is_canonical_preview(&metadata));
        assert_eq!(metadata.channels, Some(1));
        assert_eq!(metadata.sample_rate_hz, Some(8_000));
        let peaks = waveform(&path, 16).expect("waveform");
        assert!(!peaks.is_empty());
        assert!(peaks.iter().copied().fold(0.0f32, f32::max) > 0.99);
        fs::remove_dir_all(directory).ok();
    }

    #[test]
    // Was: Führt den Arbeitsschritt `packed_tetra_requires_whole_frames` für packed TETRA requires whole frames aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn packed_tetra_requires_whole_frames() {
        let directory = test_directory("tetra");
        let valid = directory.join("valid.tacelp");
        let invalid = directory.join("invalid.tacelp");
        write_atomic(&valid, &vec![0x5a; TETRA_FRAME_BYTES * 3], false).expect("write valid");
        write_atomic(&invalid, &vec![0x5a; TETRA_FRAME_BYTES + 1], false).expect("write invalid");
        assert_eq!(inspect_tacelp(&valid).unwrap().tetra_frame_count, Some(3));
        assert!(inspect_tacelp(&invalid).is_err());
        fs::remove_dir_all(directory).ok();
    }

    #[test]
    // Was: Führt den Arbeitsschritt `command_partial_path_keeps_media_extension` für command partial path keeps Audio- und Mediendaten extension aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn command_partial_path_keeps_media_extension() {
        assert_eq!(
            command_partial_path(Path::new("/tmp/preview.wav")),
            PathBuf::from("/tmp/preview.part.wav")
        );
        assert_eq!(
            command_partial_path(Path::new("/tmp/audio.tacelp")),
            PathBuf::from("/tmp/audio.part.tacelp")
        );
    }

    #[test]
    // Was: Führt den Arbeitsschritt `safe_filename_removes_path_and_control_characters` für safe filename removes path and Steuerung characters aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn safe_filename_removes_path_and_control_characters() {
        assert_eq!(safe_filename("../../hello\nworld.wav", "fallback"), "hello_world.wav");
        assert_eq!(safe_filename("...", "fallback"), "fallback");
    }
}
