// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::ptr::NonNull;

// Was: Legt den festen Wert `TETRA_PCM_SAMPLE_RATE` für TETRA pcm sample rate fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_PCM_SAMPLE_RATE: u32 = 8_000;
// Was: Legt den festen Wert `TETRA_PCM_SAMPLES_PER_FRAME` für TETRA pcm samples per Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_PCM_SAMPLES_PER_FRAME: usize = 240;
// Was: Legt den festen Wert `TETRA_PCM_SAMPLES_PER_BLOCK` für TETRA pcm samples per block fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_PCM_SAMPLES_PER_BLOCK: usize = TETRA_PCM_SAMPLES_PER_FRAME * 2;
// Was: Legt den festen Wert `TETRA_CODED_BITS_PER_FRAME` für TETRA coded bits per Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub const TETRA_CODED_BITS_PER_FRAME: usize = 137;
// Was: Legt den festen Wert `TETRA_CODED_BYTES_PER_FRAME` für TETRA coded bytes per Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_CODED_BYTES_PER_FRAME: usize = (TETRA_CODED_BITS_PER_FRAME + 7) / 8;
// Was: Legt den festen Wert `TETRA_TMD_BITS_PER_BLOCK` für TETRA tmd bits per block fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_TMD_BITS_PER_BLOCK: usize = TETRA_CODED_BITS_PER_FRAME * 2;
// Was: Legt den festen Wert `TETRA_TMD_PACKED_BYTES` für TETRA tmd packed bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_TMD_PACKED_BYTES: usize = (TETRA_TMD_BITS_PER_BLOCK + 7) / 8;

#[repr(C)]
// Was: Bündelt die zusammengehörigen Werte für raw TETRA codec in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct RawTetraCodec {
    _private: [u8; 0],
}

#[link(name = "tetra-codec")]
unsafe extern "C" {
    // Was: Führt den Arbeitsschritt `tetra_encoder_create` für TETRA encoder create aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tetra_encoder_create() -> *mut RawTetraCodec;
    // Was: Führt den Arbeitsschritt `tetra_decoder_create` für TETRA decoder create aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tetra_decoder_create() -> *mut RawTetraCodec;
    // Was: Führt den Arbeitsschritt `tetra_codec_destroy` für TETRA codec destroy aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tetra_codec_destroy(st: *mut RawTetraCodec);
    // Was: Führt den Arbeitsschritt `tetra_encode` für TETRA Kodierung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tetra_encode(st: *mut RawTetraCodec, pcm: *const i16, coded: *mut u8);
    // Was: Führt den Arbeitsschritt `tetra_decode` für TETRA Dekodierung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn tetra_decode(st: *mut RawTetraCodec, coded: *const u8, pcm: *mut i16, bfi: i32);
}

// Was: Bündelt die zusammengehörigen Werte für codec handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct CodecHandle {
    ptr: NonNull<RawTetraCodec>,
}

// One codec state belongs to one media stream and is only used through &mut self.
// Was: Implementiert das zugehörige Verhalten für `Send for CodecHandle {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
unsafe impl Send for CodecHandle {}

// Was: Implementiert das zugehörige Verhalten für `CodecHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl CodecHandle {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from_raw(ptr: *mut RawTetraCodec) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }
}

// Was: Implementiert das zugehörige Verhalten für `Drop for CodecHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Drop for CodecHandle {
    // Was: Führt den Arbeitsschritt `drop` für drop aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn drop(&mut self) {
        // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
        // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
        unsafe { tetra_codec_destroy(self.ptr.as_ptr()) }
    }
}

/// Stateful decoder for one TETRA speech stream.
// Was: Bündelt die zusammengehörigen Werte für TETRA speech decoder in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TetraSpeechDecoder {
    decoder: CodecHandle,
}

// Was: Implementiert das zugehörige Verhalten für `TetraSpeechDecoder`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraSpeechDecoder {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Option<Self> {
        Some(Self {
            decoder: CodecHandle::from_raw(unsafe { tetra_decoder_create() })?,
        })
    }

    /// Decode one 60 ms TMD speech block into 480 signed 16-bit PCM samples at 8 kHz.
    /// Both packed 35-byte blocks and the 274-byte one-bit-per-byte uplink representation
    /// produced by LMAC are accepted.
    // Was: Diese Funktion dekodiert tmd to pcm.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub fn decode_tmd_to_pcm(&mut self, acelp: &[u8]) -> Option<Vec<i16>> {
        let coded = split_tmd_block_to_codec_frames(acelp)?;
        let mut out = Vec::with_capacity(TETRA_PCM_SAMPLES_PER_BLOCK);
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for frame in &coded {
            let mut pcm = [0i16; TETRA_PCM_SAMPLES_PER_FRAME];
            // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
            // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
            unsafe {
                tetra_decode(self.decoder.ptr.as_ptr(), frame.as_ptr(), pcm.as_mut_ptr(), 0);
            }
            out.extend_from_slice(&pcm);
        }
        Some(out)
    }
}

/// Stateful encoder for one TETRA speech stream.
// Was: Bündelt die zusammengehörigen Werte für TETRA speech encoder in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TetraSpeechEncoder {
    encoder: CodecHandle,
    pcm_buffer: Vec<i16>,
}

// Was: Implementiert das zugehörige Verhalten für `TetraSpeechEncoder`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraSpeechEncoder {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Option<Self> {
        Some(Self {
            encoder: CodecHandle::from_raw(unsafe { tetra_encoder_create() })?,
            pcm_buffer: Vec::with_capacity(TETRA_PCM_SAMPLES_PER_BLOCK * 2),
        })
    }

    /// Queue 8-kHz mono PCM and return every complete 60-ms TMD block now available.
    // Was: Diese Funktion legt pcm.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn push_pcm(&mut self, samples: &[i16]) -> Vec<Vec<u8>> {
        self.pcm_buffer.extend_from_slice(samples);
        let mut out = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.pcm_buffer.len() >= TETRA_PCM_SAMPLES_PER_BLOCK {
            let block: Vec<i16> = self.pcm_buffer.drain(..TETRA_PCM_SAMPLES_PER_BLOCK).collect();
            if let Some(encoded) = self.encode_complete_block(&block) {
                out.push(encoded);
            }
        }
        out
    }

    /// Encode exactly 480 8-kHz mono PCM samples into one packed 274-bit TMD block.
    // Was: Diese Funktion kodiert complete block.
    // Warum: Alle Gegenstellen erhalten dadurch dasselbe erwartete Protokollformat.
    pub fn encode_complete_block(&mut self, pcm: &[i16]) -> Option<Vec<u8>> {
        if pcm.len() != TETRA_PCM_SAMPLES_PER_BLOCK {
            return None;
        }
        let mut coded_a = [0u8; TETRA_CODED_BYTES_PER_FRAME];
        let mut coded_b = [0u8; TETRA_CODED_BYTES_PER_FRAME];
        // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
        // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
        unsafe {
            tetra_encode(self.encoder.ptr.as_ptr(), pcm[..TETRA_PCM_SAMPLES_PER_FRAME].as_ptr(), coded_a.as_mut_ptr());
            tetra_encode(self.encoder.ptr.as_ptr(), pcm[TETRA_PCM_SAMPLES_PER_FRAME..].as_ptr(), coded_b.as_mut_ptr());
        }
        Some(join_codec_frames_to_tmd_block(&coded_a, &coded_b))
    }

    // Was: Führt den Arbeitsschritt `buffered_samples` für buffered samples aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn buffered_samples(&self) -> usize {
        self.pcm_buffer.len()
    }

    // Was: Diese Funktion leert den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn clear(&mut self) {
        self.pcm_buffer.clear();
    }
}

/// Convenience codec containing an encoder and decoder for bidirectional bridges.
// Was: Bündelt die zusammengehörigen Werte für TETRA speech codec in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TetraSpeechCodec {
    pub encoder: TetraSpeechEncoder,
    pub decoder: TetraSpeechDecoder,
}

// Was: Implementiert das zugehörige Verhalten für `TetraSpeechCodec`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraSpeechCodec {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Option<Self> {
        Some(Self {
            encoder: TetraSpeechEncoder::new()?,
            decoder: TetraSpeechDecoder::new()?,
        })
    }
}

// Was: Führt den Arbeitsschritt `split_tmd_block_to_codec_frames` für split tmd block to codec frames aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn split_tmd_block_to_codec_frames(data: &[u8]) -> Option<[[u8; TETRA_CODED_BYTES_PER_FRAME]; 2]> {
    let packed = if data.len() == TETRA_TMD_PACKED_BYTES + 1 {
        Some(&data[1..])
    } else if data.len() == TETRA_TMD_PACKED_BYTES {
        Some(data)
    } else {
        None
    };

    let mut frames = [[0u8; TETRA_CODED_BYTES_PER_FRAME]; 2];
    if let Some(packed) = packed {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for bit_idx in 0..TETRA_TMD_BITS_PER_BLOCK {
            let bit = get_packed_bit(packed, bit_idx);
            set_packed_bit(
                &mut frames[bit_idx / TETRA_CODED_BITS_PER_FRAME],
                bit_idx % TETRA_CODED_BITS_PER_FRAME,
                bit,
            );
        }
        return Some(frames);
    }

    if data.len() < TETRA_TMD_BITS_PER_BLOCK {
        return None;
    }
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for bit_idx in 0..TETRA_TMD_BITS_PER_BLOCK {
        set_packed_bit(
            &mut frames[bit_idx / TETRA_CODED_BITS_PER_FRAME],
            bit_idx % TETRA_CODED_BITS_PER_FRAME,
            data[bit_idx] & 1,
        );
    }
    Some(frames)
}

// Was: Diese Funktion verknüpft codec frames to tmd block.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn join_codec_frames_to_tmd_block(
    frame_a: &[u8; TETRA_CODED_BYTES_PER_FRAME],
    frame_b: &[u8; TETRA_CODED_BYTES_PER_FRAME],
) -> Vec<u8> {
    let mut out = vec![0u8; TETRA_TMD_PACKED_BYTES];
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for bit_idx in 0..TETRA_TMD_BITS_PER_BLOCK {
        let frame = if bit_idx < TETRA_CODED_BITS_PER_FRAME { frame_a } else { frame_b };
        let frame_bit = bit_idx % TETRA_CODED_BITS_PER_FRAME;
        set_packed_bit(&mut out, bit_idx, get_packed_bit(frame, frame_bit));
    }
    out
}

// Was: Diese Funktion liest packed bit.
// Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
fn get_packed_bit(data: &[u8], bit_idx: usize) -> u8 {
    (data[bit_idx / 8] >> (7 - (bit_idx % 8))) & 1
}

// Was: Diese Funktion setzt packed bit.
// Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
fn set_packed_bit(data: &mut [u8], bit_idx: usize, bit: u8) {
    if bit & 1 != 0 {
        data[bit_idx / 8] |= 1 << (7 - (bit_idx % 8));
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;

    #[test]
    // Was: Führt den Arbeitsschritt `packed_tmd_round_trip_keeps_274_bits` für packed tmd round trip keeps 274 bits aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn packed_tmd_round_trip_keeps_274_bits() {
        let mut bits = [0u8; TETRA_TMD_BITS_PER_BLOCK];
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (idx, bit) in bits.iter_mut().enumerate() {
            *bit = (idx % 3 == 0) as u8;
        }
        let frames = split_tmd_block_to_codec_frames(&bits).unwrap();
        let packed = join_codec_frames_to_tmd_block(&frames[0], &frames[1]);
        assert_eq!(packed.len(), TETRA_TMD_PACKED_BYTES);
        assert_eq!(frames, split_tmd_block_to_codec_frames(&packed).unwrap());
    }
}
