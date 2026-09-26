// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use std::os::raw::c_int;
use std::ptr::NonNull;

// Was: Legt den festen Wert `ECHOLINK_GSM_FRAME_BYTES` für echolink gsm Funkrahmen bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub(crate) const ECHOLINK_GSM_FRAME_BYTES: usize = 33;
// Was: Legt den festen Wert `ECHOLINK_GSM_FRAMES_PER_PACKET` für echolink gsm frames per Datenpaket fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub(crate) const ECHOLINK_GSM_FRAMES_PER_PACKET: usize = 4;
// Was: Legt den festen Wert `ECHOLINK_GSM_PACKET_BYTES` für echolink gsm Datenpaket bytes fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
pub(crate) const ECHOLINK_GSM_PACKET_BYTES: usize = ECHOLINK_GSM_FRAME_BYTES * ECHOLINK_GSM_FRAMES_PER_PACKET;

// Was: Legt den festen Wert `PCM_SAMPLES_PER_GSM_FRAME` für pcm samples per gsm Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const PCM_SAMPLES_PER_GSM_FRAME: usize = 160;
// Was: Legt den festen Wert `PCM_SAMPLES_PER_ECHOLINK_PACKET` für pcm samples per echolink Datenpaket fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const PCM_SAMPLES_PER_ECHOLINK_PACKET: usize = PCM_SAMPLES_PER_GSM_FRAME * ECHOLINK_GSM_FRAMES_PER_PACKET;

// Was: Legt den festen Wert `TETRA_PCM_SAMPLES_PER_FRAME` für TETRA pcm samples per Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_PCM_SAMPLES_PER_FRAME: usize = 240;
// Was: Legt den festen Wert `TETRA_PCM_SAMPLES_PER_BLOCK` für TETRA pcm samples per block fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_PCM_SAMPLES_PER_BLOCK: usize = TETRA_PCM_SAMPLES_PER_FRAME * 2;
// Was: Legt den festen Wert `TETRA_CODED_BITS_PER_FRAME` für TETRA coded bits per Funkrahmen fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const TETRA_CODED_BITS_PER_FRAME: usize = 137;
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

#[repr(C)]
// Was: Bündelt die zusammengehörigen Werte für raw gsm in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct RawGsm {
    _private: [u8; 0],
}

#[link(name = "gsm")]
unsafe extern "C" {
    // Was: Führt den Arbeitsschritt `gsm_create` für gsm create aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn gsm_create() -> *mut RawGsm;
    // Was: Führt den Arbeitsschritt `gsm_destroy` für gsm destroy aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn gsm_destroy(g: *mut RawGsm);
    // Was: Führt den Arbeitsschritt `gsm_encode` für gsm Kodierung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn gsm_encode(g: *mut RawGsm, s: *const i16, c: *mut u8);
    // Was: Führt den Arbeitsschritt `gsm_decode` für gsm Dekodierung aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn gsm_decode(g: *mut RawGsm, c: *const u8, s: *mut i16) -> c_int;
}

// Was: Bündelt die zusammengehörigen Werte für TETRA codec handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct TetraCodecHandle {
    ptr: NonNull<RawTetraCodec>,
}

// Was: Implementiert das zugehörige Verhalten für `Send for TetraCodecHandle {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
unsafe impl Send for TetraCodecHandle {}

// Was: Implementiert das zugehörige Verhalten für `TetraCodecHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TetraCodecHandle {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from_raw(ptr: *mut RawTetraCodec) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }
}

// Was: Implementiert das zugehörige Verhalten für `Drop for TetraCodecHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Drop for TetraCodecHandle {
    // Was: Führt den Arbeitsschritt `drop` für drop aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn drop(&mut self) {
        // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
        // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
        unsafe {
            tetra_codec_destroy(self.ptr.as_ptr());
        }
    }
}

// Was: Bündelt die zusammengehörigen Werte für gsm handle in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct GsmHandle {
    ptr: NonNull<RawGsm>,
}

// Was: Implementiert das zugehörige Verhalten für `Send for GsmHandle {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
unsafe impl Send for GsmHandle {}

// Was: Implementiert das zugehörige Verhalten für `GsmHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl GsmHandle {
    // Was: Wandelt Eingangsdaten in raw um.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn from_raw(ptr: *mut RawGsm) -> Option<Self> {
        NonNull::new(ptr).map(|ptr| Self { ptr })
    }
}

// Was: Implementiert das zugehörige Verhalten für `Drop for GsmHandle`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl Drop for GsmHandle {
    // Was: Führt den Arbeitsschritt `drop` für drop aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn drop(&mut self) {
        // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
        // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
        unsafe {
            gsm_destroy(self.ptr.as_ptr());
        }
    }
}

// Was: Bündelt die zusammengehörigen Werte für echolink audio transcoder in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub(crate) struct EcholinkAudioTranscoder {
    tetra_encoder: TetraCodecHandle,
    tetra_decoder: TetraCodecHandle,
    gsm: GsmHandle,
    tetra_to_gsm_pcm: Vec<i16>,
    gsm_to_tetra_pcm: Vec<i16>,
}

// Was: Implementiert das zugehörige Verhalten für `EcholinkAudioTranscoder`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl EcholinkAudioTranscoder {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub(crate) fn new() -> Option<Self> {
        let tetra_encoder = TetraCodecHandle::from_raw(unsafe { tetra_encoder_create() })?;
        let tetra_decoder = TetraCodecHandle::from_raw(unsafe { tetra_decoder_create() })?;
        let gsm = GsmHandle::from_raw(unsafe { gsm_create() })?;
        Some(Self {
            tetra_encoder,
            tetra_decoder,
            gsm,
            tetra_to_gsm_pcm: Vec::with_capacity(PCM_SAMPLES_PER_ECHOLINK_PACKET * 2),
            gsm_to_tetra_pcm: Vec::with_capacity(TETRA_PCM_SAMPLES_PER_BLOCK * 2),
        })
    }

    // Was: Diese Funktion dekodiert tmd to gsm packets.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub(crate) fn decode_tmd_to_gsm_packets(&mut self, acelp: &[u8]) -> Option<Vec<Vec<u8>>> {
        let coded = split_tmd_block_to_codec_frames(acelp)?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for frame in &coded {
            let mut pcm = [0i16; TETRA_PCM_SAMPLES_PER_FRAME];
            // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
            // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
            unsafe {
                tetra_decode(self.tetra_decoder.ptr.as_ptr(), frame.as_ptr(), pcm.as_mut_ptr(), 0);
            }
            self.tetra_to_gsm_pcm.extend_from_slice(&pcm);
        }

        let mut packets = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.tetra_to_gsm_pcm.len() >= PCM_SAMPLES_PER_ECHOLINK_PACKET {
            let mut payload = vec![0u8; ECHOLINK_GSM_PACKET_BYTES];
            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for frame_idx in 0..ECHOLINK_GSM_FRAMES_PER_PACKET {
                let pcm_offset = frame_idx * PCM_SAMPLES_PER_GSM_FRAME;
                let gsm_offset = frame_idx * ECHOLINK_GSM_FRAME_BYTES;
                // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
                // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
                unsafe {
                    gsm_encode(
                        self.gsm.ptr.as_ptr(),
                        self.tetra_to_gsm_pcm[pcm_offset..].as_ptr(),
                        payload[gsm_offset..].as_mut_ptr(),
                    );
                }
            }
            self.tetra_to_gsm_pcm.drain(..PCM_SAMPLES_PER_ECHOLINK_PACKET);
            packets.push(payload);
        }
        Some(packets)
    }

    // Was: Diese Funktion dekodiert gsm payload to tmd.
    // Warum: Empfangene Protokolldaten müssen vor der weiteren Nutzung eindeutig verstanden und geprüft werden.
    pub(crate) fn decode_gsm_payload_to_tmd(&mut self, payload: &[u8]) -> Vec<Vec<u8>> {
        let complete_frames = payload.len() / ECHOLINK_GSM_FRAME_BYTES;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for frame_idx in 0..complete_frames {
            let offset = frame_idx * ECHOLINK_GSM_FRAME_BYTES;
            let mut pcm = [0i16; PCM_SAMPLES_PER_GSM_FRAME];
            let rc = unsafe { gsm_decode(self.gsm.ptr.as_ptr(), payload[offset..].as_ptr(), pcm.as_mut_ptr()) };
            if rc == 0 {
                self.gsm_to_tetra_pcm.extend_from_slice(&pcm);
            }
        }

        let mut out = Vec::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        while self.gsm_to_tetra_pcm.len() >= TETRA_PCM_SAMPLES_PER_BLOCK {
            let mut pcm_a = [0i16; TETRA_PCM_SAMPLES_PER_FRAME];
            let mut pcm_b = [0i16; TETRA_PCM_SAMPLES_PER_FRAME];
            pcm_a.copy_from_slice(&self.gsm_to_tetra_pcm[..TETRA_PCM_SAMPLES_PER_FRAME]);
            pcm_b.copy_from_slice(&self.gsm_to_tetra_pcm[TETRA_PCM_SAMPLES_PER_FRAME..TETRA_PCM_SAMPLES_PER_BLOCK]);
            self.gsm_to_tetra_pcm.drain(..TETRA_PCM_SAMPLES_PER_BLOCK);

            let mut coded_a = [0u8; TETRA_CODED_BYTES_PER_FRAME];
            let mut coded_b = [0u8; TETRA_CODED_BYTES_PER_FRAME];
            // Was: Führt einen Abschnitt aus, dessen Speichersicherheit Rust nicht selbst vollständig prüfen kann.
            // Warum: Der Zugriff ist technisch notwendig, wird aber bewusst auf diesen kleinen und überprüfbaren Bereich begrenzt.
            unsafe {
                tetra_encode(self.tetra_encoder.ptr.as_ptr(), pcm_a.as_ptr(), coded_a.as_mut_ptr());
                tetra_encode(self.tetra_encoder.ptr.as_ptr(), pcm_b.as_ptr(), coded_b.as_mut_ptr());
            }
            out.push(join_codec_frames_to_tmd_block(&coded_a, &coded_b));
        }
        out
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
fn join_codec_frames_to_tmd_block(frame_a: &[u8; TETRA_CODED_BYTES_PER_FRAME], frame_b: &[u8; TETRA_CODED_BYTES_PER_FRAME]) -> Vec<u8> {
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
    // Was: Führt den Arbeitsschritt `codec_frames_pack_to_tmd_block_and_split_back` für codec frames pack to tmd block and und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn codec_frames_pack_to_tmd_block_and_split_back() {
        let mut frame_a = [0u8; TETRA_CODED_BYTES_PER_FRAME];
        let mut frame_b = [0u8; TETRA_CODED_BYTES_PER_FRAME];
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for bit_idx in 0..TETRA_CODED_BITS_PER_FRAME {
            set_packed_bit(&mut frame_a, bit_idx, (bit_idx % 2) as u8);
            set_packed_bit(&mut frame_b, bit_idx, ((bit_idx + 1) % 3 == 0) as u8);
        }

        let packed = join_codec_frames_to_tmd_block(&frame_a, &frame_b);
        assert_eq!(packed.len(), TETRA_TMD_PACKED_BYTES);
        let split = split_tmd_block_to_codec_frames(&packed).expect("packed block must split");

        assert_eq!(split[0], frame_a);
        assert_eq!(split[1], frame_b);
    }

    #[test]
    // Was: Führt den Arbeitsschritt `split_accepts_tmd_block_with_leading_marker_byte` für split accepts tmd block with leading marker und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn split_accepts_tmd_block_with_leading_marker_byte() {
        let mut packed = vec![0xaa];
        packed.extend(join_codec_frames_to_tmd_block(
            &[0xffu8; TETRA_CODED_BYTES_PER_FRAME],
            &[0u8; TETRA_CODED_BYTES_PER_FRAME],
        ));

        let split = split_tmd_block_to_codec_frames(&packed).expect("marked block must split");
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for bit_idx in 0..TETRA_CODED_BITS_PER_FRAME {
            assert_eq!(get_packed_bit(&split[0], bit_idx), 1);
        }
        assert!(split[1].iter().all(|bit| *bit == 0));
    }
}
