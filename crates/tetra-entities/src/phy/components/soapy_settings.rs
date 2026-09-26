// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Device-specific SoapySDR settings

use std::sync::OnceLock;
use tetra_config::bluestation::{StackMode, sec_phy_soapy::*};

/// Global record of the SDR device that was auto-detected at startup. Set once by
/// `SdrSettings::get_settings()` and read by the dashboard to show a hardware badge.
/// Empty if no SoapySDR backend is in use (e.g. file backend, monitor-only stack).
// Was: Legt den festen Wert `DETECTED_SDR_NAME` für detected sdr name fest.
// Warum: Der benannte Wert vermeidet schwer verständliche Zahlen oder Texte direkt in der Programmlogik und hält Änderungen zentral.
static DETECTED_SDR_NAME: OnceLock<String> = OnceLock::new();

/// Public accessor for the dashboard / telemetry / logging.
/// Returns `None` if no SDR has been detected yet.
// Was: Führt den Arbeitsschritt `detected_sdr_name` für detected sdr name aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn detected_sdr_name() -> Option<String> {
    DETECTED_SDR_NAME.get().cloned()
}

/// Enum of all supported devices
// Was: Listet die möglichen Varianten für supported device auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum SupportedDevice {
    LimeSdr(LimeSdrModel),
    SXceiver,
    MuCell,
    PlutoSdr,
    Usrp(UsrpModel),
}

#[derive(Debug, PartialEq)]
// Was: Listet die möglichen Varianten für lime sdr model auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum LimeSdrModel {
    LimeSdrUsb,
    LimeSdrMiniV2,
    LimeNetMicro,
    /// Other LimeSDR models with FX3 driver
    OtherFx3,
    /// Other LimeSDR models with FT601 driver
    OtherFt601,
}

#[derive(Debug, PartialEq)]
// Was: Listet die möglichen Varianten für usrp model auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum UsrpModel {
    B200,
    B210,
    Other,
}

// Was: Implementiert das zugehörige Verhalten für `SupportedDevice`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SupportedDevice {
    /// Detect an SDR device based on driver key and hardware key.
    /// Return None if the device is not supported.
    // Was: Führt den Arbeitsschritt `detect` für detect aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn detect(driver_key: &str, hardware_key: &str) -> Option<Self> {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match (driver_key, hardware_key) {
            ("FX3", "LimeSDR-USB") => Some(Self::LimeSdr(LimeSdrModel::LimeSdrUsb)),
            ("FX3", _) => Some(Self::LimeSdr(LimeSdrModel::OtherFx3)),

            ("FT601", "LimeSDR-Mini_v2") => Some(Self::LimeSdr(LimeSdrModel::LimeSdrMiniV2)),
            ("FT601", "LimeNET-Micro") => Some(Self::LimeSdr(LimeSdrModel::LimeNetMicro)),
            ("FT601", _) => Some(Self::LimeSdr(LimeSdrModel::OtherFt601)),

            ("sx", _) => Some(Self::SXceiver),
            ("mucell", _) => Some(Self::MuCell),

            ("PlutoSDR", _) => Some(Self::PlutoSdr),

            // USRP B210 seems to report as ("b200", "B210"),
            // but the driver key is also known to be "uhd" in some cases.
            // The reason is unknown but might be due to
            // gateware, firmware or driver version differences.
            // Try to detect USRP correctly in all cases.
            ("b200", "B200") | ("uhd", "B200") => Some(Self::Usrp(UsrpModel::B200)),
            ("b200", "B210") | ("uhd", "B210") => Some(Self::Usrp(UsrpModel::B210)),
            ("b200", _) | ("uhd", _) => Some(Self::Usrp(UsrpModel::Other)),
            // TODO: add other USRP models if needed
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
// Was: Bündelt die zusammengehörigen Werte für sdr settings in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct SdrSettings {
    /// Settings template, holding which SDR is used
    pub name: String,
    /// If false, timestamp of latest RX read is used to estimate
    /// current hardware time. This is used in case get_hardware_time
    /// is unacceptably slow or not supported.
    pub use_get_hardware_time: bool,
    /// Receive and transmit sample rate.
    pub fs: f64,
    /// Receive channel number
    pub rx_ch: usize,
    /// Transmit channel number
    pub tx_ch: usize,
    /// Receive antenna
    pub rx_ant: Option<String>,
    /// Transmit antenna
    pub tx_ant: Option<String>,
    /// Receive gains
    pub rx_gain: Vec<(String, f64)>,
    /// Transmit gains
    pub tx_gain: Vec<(String, f64)>,

    /// Receive stream arguments
    pub rx_args: Vec<(String, String)>,
    /// Transmit stream arguments
    pub tx_args: Vec<(String, String)>,

    /// Additional device arguments
    pub dev_args: Vec<(String, String)>,
}

// Was: Listet die möglichen Varianten für error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum Error {
    InvalidConfiguration,
}

// Was: Implementiert das zugehörige Verhalten für `SdrSettings`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl SdrSettings {
    /// Get settings based on SDR type and SoapySDR configuration
    // Was: Diese Funktion liest settings.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    pub fn get_settings(cfg: &CfgSoapySdr, device: SupportedDevice, mode: StackMode) -> Result<Self, Error> {
        let mut settings = Self::get_defaults(cfg, device, mode);

        // Override settings if specified in configuration
        if let Some(fs) = cfg.fs {
            settings.fs = fs;
        }
        if let Some(ch) = cfg.rx_ch {
            settings.rx_ch = ch;
        }
        if let Some(ch) = cfg.tx_ch {
            settings.tx_ch = ch;
        }
        if let Some(ant) = &cfg.rx_ant {
            settings.rx_ant = Some(ant.clone());
        }
        if let Some(ant) = &cfg.tx_ant {
            settings.tx_ant = Some(ant.clone());
        }

        let mut cfg_gains = cfg.rx_gains.clone();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (name, value) in settings.rx_gain.iter_mut() {
            if let Some(gain) = cfg_gains.remove(&(*name.to_lowercase())) {
                *value = gain;
            }
        }
        if !cfg_gains.is_empty() {
            tracing::error!("Unsupported RX gains for {}: {:?}", settings.name, cfg_gains);
            return Err(Error::InvalidConfiguration);
        }

        let mut cfg_gains = cfg.tx_gains.clone();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for (name, value) in settings.tx_gain.iter_mut() {
            if let Some(gain) = cfg_gains.remove(&(*name.to_lowercase())) {
                *value = gain;
            }
        }
        if !cfg_gains.is_empty() {
            tracing::error!("Unsupported TX gains for {}: {:?}", settings.name, cfg_gains);
            return Err(Error::InvalidConfiguration);
        }

        // TODO: check for extra gain fields in cfg

        // Record the resolved device name so the dashboard can display a hardware badge.
        // OnceLock::set is a no-op if already set; safe to call repeatedly.
        let _ = DETECTED_SDR_NAME.set(settings.name.clone());

        Ok(settings)
    }

    /// Get default settings based on SDR type
    // Was: Diese Funktion liest defaults.
    // Warum: Der Zugriff auf den Wert bleibt dadurch gekapselt und kann später zentral angepasst werden.
    fn get_defaults(cfg: &CfgSoapySdr, device: SupportedDevice, mode: StackMode) -> Self {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match device {
            SupportedDevice::LimeSdr(model) => Self::settings_limesdr(mode, model),

            SupportedDevice::SXceiver => Self::settings_sxceiver(mode, cfg.fs),

            SupportedDevice::MuCell => Self::settings_mucell(mode, cfg.fs),

            SupportedDevice::PlutoSdr => Self::settings_pluto(mode),

            SupportedDevice::Usrp(model) => Self::settings_usrp(mode, model),
        }
    }

    /// Reasonable defaults for many SDR devices.
    /// These should not be directly used for any device
    /// but are useful as a template for the most common settings.
    /// This reduces changed needed in code in case
    /// more fields are added to SdrSettings to handle some special cases.
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn default(mode: StackMode) -> Self {
        Self {
            name: String::new(), // should be always overridden

            // With FCFB bin spacing of 500 Hz and overlap factor or 1/4,
            // FFT size becomes fs/500 and must be a multiple of 4.
            // If possible, use a power-of-two value in kHz
            // because power-of-two FFT sizes are most computationally efficient.
            fs: match mode {
                // 512 kHz is enough for BS use,
                // and some devices struggle with very low sample rates
                // lower than that, making it a good default choice.
                StackMode::Bs | StackMode::Ms => 512e3,
                // Simultaneous UL/DL monitoring at 10 MHz duplex spacing
                // needs something well above 10 MHz.
                StackMode::Mon => 16384e3,
            },

            use_get_hardware_time: true,
            rx_ant: None,
            tx_ant: None,
            rx_gain: vec![],
            tx_gain: vec![],
            rx_ch: 0,
            tx_ch: 0,
            rx_args: vec![],
            tx_args: vec![],
            dev_args: vec![],
        }
    }

    // Was: Führt den Arbeitsschritt `settings_limesdr` für settings limesdr aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn settings_limesdr(mode: StackMode, model: LimeSdrModel) -> Self {
        Self {
            name: match model {
                LimeSdrModel::LimeSdrUsb => "LimeSDR USB",
                LimeSdrModel::LimeSdrMiniV2 => "LimeSDR Mini 2.0",
                LimeSdrModel::LimeNetMicro => "LimeNET Micro",
                LimeSdrModel::OtherFx3 => "Unknown LimeSDR model with FX3",
                LimeSdrModel::OtherFt601 => "Unknown LimeSDR model with FT601",
            }
            .to_string(),

            rx_ant: Some(
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match model {
                    LimeSdrModel::LimeSdrUsb => "LNAL",
                    _ => "LNAW",
                }
                .to_string(),
            ),

            tx_ant: Some(
                // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
                // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
                match model {
                    LimeSdrModel::LimeSdrUsb => "BAND1",
                    _ => "BAND2",
                }
                .to_string(),
            ),

            rx_gain: vec![("LNA".to_string(), 18.0), ("TIA".to_string(), 6.0), ("PGA".to_string(), 10.0)],
            tx_gain: vec![("PAD".to_string(), 22.0), ("IAMP".to_string(), 6.0)],

            // Minimum latency for BS/MS, maximum throughput for monitor
            rx_args: vec![("latency".to_string(), if mode == StackMode::Mon { "1" } else { "0" }.to_string())],
            tx_args: vec![("latency".to_string(), if mode == StackMode::Mon { "1" } else { "0" }.to_string())],

            ..Self::default(mode)
        }
    }

    // Was: Führt den Arbeitsschritt `settings_sxceiver` für settings sxceiver aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn settings_sxceiver(mode: StackMode, fs_override: Option<f64>) -> Self {
        // TODO: pass detected clock rate or list of supported sample rates
        // to get_settings and choose sample rate accordingly.
        // Ok, it is not strictly needed now that sample rate can be overridden.
        // That added another minor issue, though:
        // sample rate affects the optimal period size
        // and override is applied after it is computed.
        // OK, duplicate handle sample rate override here
        // as an ugly little extra special case...
        let fs = fs_override.unwrap_or(600e3);
        Self {
            name: "SXceiver".to_string(),
            fs,

            rx_ant: Some("RX".to_string()),
            tx_ant: Some("TX".to_string()),

            rx_gain: vec![("LNA".to_string(), 42.0), ("PGA".to_string(), 16.0)],
            tx_gain: vec![("DAC".to_string(), 9.0), ("MIXER".to_string(), 30.0)],

            rx_args: vec![("period".to_string(), block_size(fs).to_string())],
            tx_args: vec![("period".to_string(), block_size(fs).to_string())],

            ..Self::default(mode)
        }
    }

    // Was: Führt den Arbeitsschritt `settings_mucell` für settings mucell aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn settings_mucell(mode: StackMode, fs_override: Option<f64>) -> Self {
        // Similar to SXCeiver for now.
        // Might be adapted later for PA gain and other settings.
        let fs = fs_override.unwrap_or(600e3);
        Self {
            name: "µCell".to_string(),
            fs,

            rx_ant: Some("RX".to_string()),
            tx_ant: Some("TX".to_string()),

            rx_gain: vec![("LNA".to_string(), 42.0), ("PGA".to_string(), 16.0)],
            tx_gain: vec![("DAC".to_string(), 9.0), ("MIXER".to_string(), 30.0)],

            rx_args: vec![("period".to_string(), block_size(fs).to_string())],
            tx_args: vec![("period".to_string(), block_size(fs).to_string())],

            ..Self::default(mode)
        }
    }

    // Was: Führt den Arbeitsschritt `settings_usrp` für settings usrp aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn settings_usrp(mode: StackMode, model: UsrpModel) -> Self {
        Self {
            name: match model {
                UsrpModel::B200 => "USRP B200",
                UsrpModel::B210 => "USRP B210",
                UsrpModel::Other => "Unknown USRP model",
            }
            .to_string(),

            rx_ant: Some("TX/RX".to_string()),
            tx_ant: Some("TX/RX".to_string()),

            rx_gain: vec![("PGA".to_string(), 50.0)],
            tx_gain: vec![("PGA".to_string(), 45.0)],

            ..Self::default(mode)
        }
    }

    // Was: Führt den Arbeitsschritt `settings_pluto` für settings pluto aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn settings_pluto(mode: StackMode) -> Self {
        Self {
            name: "Pluto".to_string(),
            // get_hardware_time is apparently not implemented for pluto.
            use_get_hardware_time: false,

            // TODO: check if sample rate could be increased to 1024e3.
            // That would allow a power-of-two FFT size for lower CPU use.
            fs: 1e6,

            rx_ant: Some("A_BALANCED".to_string()),
            tx_ant: Some("A".to_string()),

            rx_gain: vec![("PGA".to_string(), 20.0)],
            tx_gain: vec![("PGA".to_string(), 89.0)],

            dev_args: vec![
                ("direct".to_string(), "1".to_string()),
                ("timestamp_every".to_string(), "1500".to_string()),
                ("loopback".to_string(), "0".to_string()),
            ],

            ..Self::default(mode)
        }
    }
}

/// Get processing block size in samples for a given sample rate.
/// This can be used to optimize performance for some SDRs.
// Was: Führt den Arbeitsschritt `block_size` für block size aus.
// Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
pub fn block_size(fs: f64) -> usize {
    // With current FCFB parameters processing blocks are 1.5 ms long.
    // It is a bit bug prone to have it here in case
    // FCFB parameters are changed, but it makes things simpler for now.
    (fs * 1.5e-3).round() as usize
}
