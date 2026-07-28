// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

use soapysdr;
use tetra_config::bluestation::{SharedConfig, StackMode, sec_phy_soapy::CfgSoapySdr};

use tetra_pdus::phy::traits::rxtx_dev::RxTxDevError;

use super::dsp_types::*;
use super::soapy_settings;
use super::soapy_settings::{SdrSettings, SupportedDevice};
use super::soapy_time::{ticks_to_time_ns, time_ns_to_ticks};

// Was: Vergibt für stream type einen fachlich verständlichen Typnamen.
// Warum: Der Alias macht Signaturen lesbarer und hält technische Details aus dem aufrufenden Code heraus.
type StreamType = ComplexSample;
// Was: Legt den festen Wert `SOAPY_FREQ_OFFSET` für soapy freq offset fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const SOAPY_FREQ_OFFSET: f64 = 20000.0;

// Was: Bündelt die zusammengehörigen Werte für rx result in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct RxResult {
    /// Number of samples read
    pub len: usize,
    /// Sample counter for the first sample read
    pub count: SampleCount,
}

// Was: Bündelt die zusammengehörigen Werte für soapy io in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SoapyIo {
    rx_ch: usize,
    tx_ch: usize,
    rx_fs: f64,
    tx_fs: f64,
    /// Timestamp for the first sample read from SDR.
    /// This is subtracted from all following timestamps,
    /// so that sample counter startsB210 from 0 even if timestamp does not.
    initial_time: Option<i64>,
    rx_next_count: SampleCount,
    prev_time_ns: i64,

    /// If false, timestamp of latest RX read is used to estimate
    /// current hardware time. This is used in case get_hardware_time
    /// is unacceptably slow or not supported.
    use_get_hardware_time: bool,

    dev: soapysdr::Device,
    /// Receive stream. None if receiving is disabled.
    rx: Option<soapysdr::RxStream<StreamType>>,
    /// Transmit stream. None if transmitting is disabled.
    tx: Option<soapysdr::TxStream<StreamType>>,
}

/// Soapy/Lime timestamps can occasionally jitter by a single sample.
/// Treat tiny deltas as contiguous to avoid triggering large block realignments downstream.
// Was: Legt den festen Wert `RX_TIMESTAMP_JITTER_TOLERANCE_SAMPLES` für rx timestamp Laufzeitschwankung tolerance samples fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
const RX_TIMESTAMP_JITTER_TOLERANCE_SAMPLES: SampleCount = 1;

/// It is annoying to repeat error handling so do that in a macro.
/// ? could be used but then it could not print which SoapySDR call failed.
// Was: Definiert das Makro `soapycheck`, das wiederkehrenden Rust-Code erzeugt.
// Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
macro_rules! soapycheck {
    ($text:literal, $soapysdr_call:expr) => {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match $soapysdr_call {
            Ok(ret) => ret,
            Err(err) => {
                tracing::error!("SoapySDR: Failed to {}: {}", $text, err);
                return Err(err);
            }
        }
    };
}

// Was: Implementiert das zugehörige Verhalten für `SoapyIo`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SoapyIo {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new(cfg: &SharedConfig) -> Result<Self, soapysdr::Error> {
        let binding = cfg.config();
        let soapy_cfg = binding
            .phy_io
            .soapysdr
            .as_ref()
            .expect("SoapySdr config must be set for SoapySdr PhyIo");

        let mode = cfg.config().stack_mode;

        let (dev, sdr_settings) = open_device(&soapy_cfg, mode)?;

        let rx_ch = sdr_settings.rx_ch;
        let tx_ch = sdr_settings.tx_ch;

        // Get PPM-corrected carrier frequencies and optional explicit SDR center frequencies.
        // The center-frequency overrides are intentionally tied to the physical SDR
        // directions (`rx_center_freq` / `tx_center_freq`), not to TETRA DL/UL naming.
        let (dl_corrected, _) = soapy_cfg.dl_freq_corrected();
        let (ul_corrected, _) = soapy_cfg.ul_freq_corrected();
        let rx_center_corrected = soapy_cfg.rx_center_freq_corrected().map(|(freq, _)| freq);
        let tx_center_corrected = soapy_cfg.tx_center_freq_corrected().map(|(freq, _)| freq);

        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        let (rx_freq, tx_freq) = match mode {
            StackMode::Bs => (
                Some(rx_center_corrected.unwrap_or(ul_corrected - SOAPY_FREQ_OFFSET)),
                Some(tx_center_corrected.unwrap_or(dl_corrected)),
            ),
            StackMode::Ms => (
                Some(rx_center_corrected.unwrap_or(dl_corrected - SOAPY_FREQ_OFFSET)),
                Some(tx_center_corrected.unwrap_or(ul_corrected)),
            ),
            StackMode::Mon => {
                unimplemented!("Monitor mode not implemented yet");
            }
        };

        tracing::info!(
            "SDR centers: RX {:.6} MHz / TX {:.6} MHz{}",
            rx_freq.unwrap_or(0.0) / 1e6,
            tx_freq.unwrap_or(0.0) / 1e6,
            if soapy_cfg.rx_center_freq.is_some() || soapy_cfg.tx_center_freq.is_some() {
                " (explicit center override)"
            } else {
                ""
            }
        );

        let rx_enabled = rx_freq.is_some();
        let tx_enabled = tx_freq.is_some();

        let mut rx_fs: f64 = 0.0;
        if rx_enabled {
            soapycheck!(
                "set RX sample rate",
                dev.set_sample_rate(soapysdr::Direction::Rx, rx_ch, sdr_settings.fs)
            );
            // Read the actual sample rate obtained and store it
            // to avoid having to read it again every time it is needed.
            rx_fs = soapycheck!("get RX sample rate", dev.sample_rate(soapysdr::Direction::Rx, rx_ch));
        }
        let mut tx_fs: f64 = 0.0;
        if tx_enabled {
            soapycheck!(
                "set TX sample rate",
                dev.set_sample_rate(soapysdr::Direction::Tx, tx_ch, sdr_settings.fs)
            );
            tx_fs = soapycheck!("get TX sample rate", dev.sample_rate(soapysdr::Direction::Tx, tx_ch));
        }

        if rx_enabled {
            // If rx_enabled is true, we already know rx_freq is not None,
            // so unwrap is fine here.
            soapycheck!(
                "set RX center frequency",
                dev.set_frequency(soapysdr::Direction::Rx, rx_ch, rx_freq.unwrap(), soapysdr::Args::new())
            );

            if let Some(ref ant) = sdr_settings.rx_ant {
                soapycheck!("set RX antenna", dev.set_antenna(soapysdr::Direction::Rx, rx_ch, ant.as_str()));
            }

            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for (name, gain) in &sdr_settings.rx_gain {
                soapycheck!(
                    "set RX gain",
                    dev.set_gain_element(soapysdr::Direction::Rx, rx_ch, name.as_str(), *gain)
                );
            }
        }

        if tx_enabled {
            soapycheck!(
                "set TX center frequency",
                dev.set_frequency(soapysdr::Direction::Tx, tx_ch, tx_freq.unwrap(), soapysdr::Args::new())
            );

            if let Some(ref ant) = sdr_settings.tx_ant {
                soapycheck!("set TX antenna", dev.set_antenna(soapysdr::Direction::Tx, tx_ch, ant.as_str()));
            }

            // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
            // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
            for (name, gain) in &sdr_settings.tx_gain {
                soapycheck!(
                    "set TX gain",
                    dev.set_gain_element(soapysdr::Direction::Tx, tx_ch, name.as_str(), *gain)
                );
            }
        }

        let mut rx_args = soapysdr::Args::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (key, value) in sdr_settings.rx_args {
            rx_args.set(key, value);
        }

        let mut tx_args = soapysdr::Args::new();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (key, value) in sdr_settings.tx_args {
            tx_args.set(key, value);
        }

        let mut rx = if rx_enabled {
            Some(soapycheck!("setup RX stream", dev.rx_stream_args(&[rx_ch], rx_args)))
        } else {
            None
        };
        let mut tx = if tx_enabled {
            Some(soapycheck!("setup TX stream", dev.tx_stream_args(&[tx_ch], tx_args)))
        } else {
            None
        };
        if let Some(rx) = &mut rx {
            soapycheck!("activate RX stream", rx.activate(None));
        }
        if let Some(tx) = &mut tx {
            soapycheck!("activate TX stream", tx.activate(None));
        }
        Ok(Self {
            rx_ch,
            tx_ch,
            rx_fs,
            tx_fs,
            initial_time: None,
            rx_next_count: 0,
            prev_time_ns: -1,
            use_get_hardware_time: sdr_settings.use_get_hardware_time,
            dev,
            rx,
            tx,
        })
    }

    // Was: Diese Funktion empfängt den vorgesehenen Arbeitsschritt.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    pub fn receive(&mut self, buffer: &mut [StreamType]) -> Result<RxResult, RxTxDevError> {
        if let Some(rx) = &mut self.rx {
            // RX is enabled
            // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
            // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
            match rx.read(&mut [buffer], 1000000) {
                Ok(len) => {
                    // Get timestamp, set initial time if not yet set
                    let time = rx.time_ns();
                    // rust-soapysdr does not let us if a timestamp was available
                    // so we have to guess by checking whether it has changed from its previous value.
                    let timestamp_available = time != self.prev_time_ns;
                    self.prev_time_ns = time;

                    if self.initial_time.is_none() && timestamp_available {
                        self.initial_time = Some(time - ticks_to_time_ns(self.rx_next_count, self.rx_fs));
                        tracing::trace!("Set initial_time to {} ns", self.initial_time.unwrap());
                    };

                    // Re-compute total count from timestamp (gracefully handles lost samples).
                    let mut count = if timestamp_available {
                        time_ns_to_ticks(time - self.initial_time.unwrap(), self.rx_fs)
                    } else {
                        // If timestamp was not available,
                        // assume the read continues right after the previous read.
                        // Some drivers, particularly SoapyRemote,
                        // may provide a timestamp only in some of the reads.
                        self.rx_next_count
                    };

                    // Smooth tiny timestamp jitter (e.g. +/-1 sample) to keep counters monotonic
                    // This is known to happen for LimeSDR Mini v2 after some time
                    let delta_from_expected = count - self.rx_next_count;
                    if delta_from_expected.abs() <= RX_TIMESTAMP_JITTER_TOLERANCE_SAMPLES {
                        if delta_from_expected != 0 {
                            // Re-anchor phase so persistent +/-1 sample offset is corrected
                            let initial_time = self.initial_time.unwrap() + ticks_to_time_ns(delta_from_expected, self.rx_fs); // unwrap never fails
                            self.initial_time = Some(initial_time);
                            tracing::debug!(
                                "RX timestamp jitter {} sample(s); re-anchoring initial_time by {} ns",
                                delta_from_expected,
                                ticks_to_time_ns(delta_from_expected, self.rx_fs)
                            );
                        }
                        count = self.rx_next_count;
                    }

                    // Store expected sample count for the next sample to be read.
                    // This is used in case timestamp is missing.
                    self.rx_next_count = count + len as SampleCount;

                    Ok(RxResult { len, count })
                }
                Err(_) => Err(RxTxDevError::RxReadError),
            }
        } else {
            // RX is disabled
            Err(RxTxDevError::RxReadError)
        }
    }

    // Was: Diese Funktion überträgt den vorgesehenen Arbeitsschritt.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn transmit(&mut self, buffer: &[StreamType], count: Option<SampleCount>) -> Result<(), RxTxDevError> {
        if let Some(tx) = &mut self.tx {
            if let Some(initial_time) = self.initial_time {
                tx.write_all(
                    &[buffer],
                    count.map(|count| initial_time + ticks_to_time_ns(count, self.tx_fs)),
                    false,
                    1000000,
                )
                .map_err(|_| RxTxDevError::RxReadError)
            } else {
                // initial_time is not available, so TX is not possible yet
                Err(RxTxDevError::RxReadError)
            }
        } else {
            // TX is disabled
            Err(RxTxDevError::RxReadError)
        }
    }

    // Was: Führt den Arbeitsschritt `current_time` für current time aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn current_time(&self) -> Result<i64, RxTxDevError> {
        self.dev.get_hardware_time(None).map_err(|_| RxTxDevError::RxReadError)
    }

    /// Current hardware time as RX sample count
    // Was: Führt den Arbeitsschritt `rx_current_count` für rx current count aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_current_count(&self) -> Result<SampleCount, RxTxDevError> {
        if !self.rx_enabled() {
            return Ok(0);
        }
        if self.use_get_hardware_time {
            Ok(time_ns_to_ticks(self.current_time()? - self.initial_time.unwrap_or(0), self.rx_fs))
        } else {
            Ok(self.rx_next_count - 1)
        }
    }

    /// Current hardware time as TX sample count
    // Was: Führt den Arbeitsschritt `tx_current_count` für tx current count aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tx_current_count(&self) -> Result<SampleCount, RxTxDevError> {
        if !self.tx_enabled() {
            return Ok(0);
        }
        if self.use_get_hardware_time {
            Ok(time_ns_to_ticks(self.current_time()? - self.initial_time.unwrap_or(0), self.tx_fs))
        } else {
            // Assumes equal RX and TX sample rates
            // and does not work if RX is disabled.
            // This is not a problem right now but could be fixed if needed.
            Ok(self.rx_next_count - 1)
        }
    }

    // Was: Führt den Arbeitsschritt `tx_possible` für tx possible aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tx_possible(&self) -> bool {
        // initial_time is obtained from the first RX read (that includes a timestamp),
        // so prevent TX before it is available.
        self.tx_enabled() && self.initial_time.is_some()
    }

    // Was: Führt den Arbeitsschritt `rx_sample_rate` für rx sample rate aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_sample_rate(&self) -> f64 {
        self.rx_fs
    }

    // Was: Führt den Arbeitsschritt `tx_sample_rate` für tx sample rate aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tx_sample_rate(&self) -> f64 {
        self.tx_fs
    }

    // Was: Führt den Arbeitsschritt `rx_center_frequency` für rx center frequency aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_center_frequency(&self) -> Result<f64, soapysdr::Error> {
        self.dev.frequency(soapysdr::Direction::Rx, self.rx_ch)
    }

    // Was: Führt den Arbeitsschritt `tx_center_frequency` für tx center frequency aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tx_center_frequency(&self) -> Result<f64, soapysdr::Error> {
        self.dev.frequency(soapysdr::Direction::Tx, self.tx_ch)
    }

    // Was: Führt den Arbeitsschritt `rx_enabled` für rx enabled aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn rx_enabled(&self) -> bool {
        self.rx.is_some()
    }

    // Was: Führt den Arbeitsschritt `tx_enabled` für tx enabled aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn tx_enabled(&self) -> bool {
        self.tx.is_some()
    }

    /// Read SDR temperature in °C if the device exposes a temp-like sensor.
    /// LimeSDR returns "temp" via list_sensors; USRP usually "fp_temp" or similar;
    /// SXceiver / µCell don't currently expose any sensor and this returns None.
    /// We probe sensor names rather than hard-coding per-driver, so any future radio
    /// that follows the Soapy convention works without code changes.
    // Was: Diese Funktion liest temperature c.
    // Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
    pub fn read_temperature_c(&self) -> Option<f32> {
        let sensors = self.dev.list_sensors().ok()?;
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for name in sensors {
            let s = name.to_string();
            let lower = s.to_lowercase();
            if lower.contains("temp") {
                if let Ok(val) = self.dev.read_sensor(&s) {
                    if let Ok(parsed) = val.to_string().parse::<f32>() {
                        return Some(parsed);
                    }
                }
            }
        }
        None
    }

    /// Read back the currently-active TX gain per stage, in dB.
    /// Returns the same gain-element names the radio uses (e.g. "PAD","IAMP" on LimeSDR).
    // Was: Diese Funktion liest tx gains.
    // Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
    pub fn read_tx_gains(&self) -> Vec<(String, f32)> {
        if !self.tx_enabled() {
            return Vec::new();
        }
        self.dev
            .list_gains(soapysdr::Direction::Tx, self.tx_ch)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|name| {
                let s = name.to_string();
                self.dev
                    .gain_element(soapysdr::Direction::Tx, self.tx_ch, s.clone())
                    .ok()
                    .map(|g| (s, g as f32))
            })
            .collect()
    }

    /// Read back the currently-active RX gain per stage, in dB.
    // Was: Diese Funktion liest rx gains.
    // Warum: Der Datenzugriff wird dadurch einheitlich behandelt und Fehler können zentral gemeldet werden.
    pub fn read_rx_gains(&self) -> Vec<(String, f32)> {
        if !self.rx_enabled() {
            return Vec::new();
        }
        self.dev
            .list_gains(soapysdr::Direction::Rx, self.rx_ch)
            .unwrap_or_default()
            .into_iter()
            .filter_map(|name| {
                let s = name.to_string();
                self.dev
                    .gain_element(soapysdr::Direction::Rx, self.rx_ch, s.clone())
                    .ok()
                    .map(|g| (s, g as f32))
            })
            .collect()
    }
}

// Messy logic related to opening a device follows...

/// Struct to temporarily hold stuff related to opening and detecting a device
// Was: Bündelt die zusammengehörigen Werte für opened device in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
struct OpenedDevice {
    dev_args: soapysdr::Args,
    dev: soapysdr::Device,
    driver_key: String,
    hardware_key: String,
    detected_device: SupportedDevice,
    soapyremote_used: bool,
}

// Was: Diese Funktion öffnet given device.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn open_given_device(dev_args: soapysdr::Args) -> Result<OpenedDevice, soapysdr::Error> {
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let soapyremote_used = match dev_args.get("driver") {
        Some("remote") => true,
        _ => false,
    };
    tracing::info!("Trying to open a device with arguments: {}", dev_args);

    let dev_args_copy: soapysdr::Args = dev_args.iter().collect();
    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let dev = match soapysdr::Device::new(dev_args_copy) {
        Ok(dev) => dev,
        Err(err) => {
            tracing::info!("Skipping a SoapySDR device because opening failed: {}", err);
            return Err(err);
        }
    };
    let driver_key = dev.driver_key().unwrap_or_default();
    let hardware_key = dev.hardware_key().unwrap_or_default();

    // Check whether the device is supported
    if let Some(detected_device) = SupportedDevice::detect(&driver_key, &hardware_key) {
        tracing::info!(
            "Found supported device with driver_key '{}' hardware_key '{}'",
            driver_key,
            hardware_key
        );
        Ok(OpenedDevice {
            dev_args,
            dev,
            driver_key,
            hardware_key,
            detected_device,
            soapyremote_used,
        })
    } else {
        tracing::info!(
            "Skipping unsupported device with driver_key '{}' hardware_key '{}'",
            driver_key,
            hardware_key
        );
        Err(soapysdr::Error {
            code: soapysdr::ErrorCode::NotSupported,
            message: "Unsupported device".to_string(),
        })
    }
}

/// Enumerate devices and find the first supported device
// Was: Diese Funktion sucht supported device.
// Warum: Die Suchlogik bleibt damit wiederverwendbar und muss nicht an mehreren Stellen kopiert werden.
fn find_supported_device(filter_args: soapysdr::Args) -> Result<OpenedDevice, soapysdr::Error> {
    // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
    // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
    for dev_args in soapycheck!("Enumerate SoapySDR devices", soapysdr::enumerate(filter_args)) {
        //tracing::info!("Trying to open a device with arguments: {}", args_formatted);
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match open_given_device(dev_args) {
            Ok(opened_device) => return Ok(opened_device),
            Err(_) => {}
        }
    }
    return Err(soapysdr::Error {
        code: soapysdr::ErrorCode::NotSupported,
        message: "No supported devices found".to_string(),
    });
}

/// Open a given device if argument string is given,
/// automatically find the first supported device if not.
// Was: Diese Funktion öffnet device.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
fn open_device(soapy_cfg: &CfgSoapySdr, mode: StackMode) -> Result<(soapysdr::Device, SdrSettings), soapysdr::Error> {
    let mut opened_device = if let Some(arg_string) = &soapy_cfg.device {
        open_given_device(arg_string.as_str().into())
    } else {
        find_supported_device(soapysdr::Args::new())
    }?;

    // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
    // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
    let mut sdr_settings = match SdrSettings::get_settings(&soapy_cfg, opened_device.detected_device, mode) {
        Ok(sdr_settings) => sdr_settings,
        Err(soapy_settings::Error::InvalidConfiguration) => {
            return Err(soapysdr::Error {
                code: soapysdr::ErrorCode::Other,
                message: "Invalid SDR device configuration".to_string(),
            });
        }
    };

    if opened_device.soapyremote_used {
        // Getting hardware time may be too slow over SoapyRemote
        tracing::info!("SoapyRemote detected, forcing use_get_hardware_time=false");
        sdr_settings.use_get_hardware_time = false;
    }

    tracing::info!("Using settings: {:?}", sdr_settings);

    // If additional driver arguments are needed, reopen the device with them
    if sdr_settings.dev_args.len() > 0 {
        // Append additional arguments from settings
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (key, value) in &sdr_settings.dev_args {
            opened_device.dev_args.set(key.as_str(), value.as_str());
        }

        tracing::info!("Reopening device with additional arguments: {}", opened_device.dev_args);

        // Make sure device gets closed first. Not sure if needed.
        std::mem::drop(opened_device.dev);
        opened_device.dev = soapycheck!(
            "open SoapySDR device with additional arguments",
            soapysdr::Device::new(opened_device.dev_args)
        );
        // Make sure it is still the same device.
        // Unlikely to change, but who knows if a device got connected just in between,
        // or if the device broke from first opening attempt and something else got opened
        // because device arguments were not precise enough to guarantee a specific device.
        let new_driver_key = opened_device.dev.driver_key().unwrap_or_default();
        let new_hardware_key = opened_device.dev.hardware_key().unwrap_or_default();
        if new_driver_key != opened_device.driver_key || new_hardware_key != opened_device.hardware_key {
            tracing::info!(
                "Expected the same driver_key='{}' hardware_key='{}' after reopen, got driver_key='{}' hardware_key='{}'",
                opened_device.driver_key,
                opened_device.hardware_key,
                new_driver_key,
                new_hardware_key
            );
            return Err(soapysdr::Error {
                code: soapysdr::ErrorCode::Other,
                message: "Reopened a different device".to_string(),
            });
        }
    }

    Ok((opened_device.dev, sdr_settings))
}
