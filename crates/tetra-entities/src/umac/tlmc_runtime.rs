// NETCORE-KOMMENTAR – Was: Enthält einen Teil der Logik für laufende TETRA-Protokollinstanzen und Zustandsautomaten.
// NETCORE-KOMMENTAR – Warum: Die Trennung in eine eigene Datei macht Zuständigkeit, Wartung und Fehlersuche übersichtlicher.

//! Runtime state machine for the merged TLC/TMC service access point.
//!
//! TLMC is a local management interface between MLE and layer 2.  It does not
//! carry over the air and it is not part of the future TBS/backend protocol.
//! The runtime deliberately separates request/state handling from the actual RF
//! adapter: UMAC owns this state machine, while LMAC/PHY observations complete
//! scan, monitor, cell-read and selection operations.

use std::collections::HashMap;
use std::fmt;

use tetra_core::EndpointId;
use tetra_saps::common::{
    CellCandidate, CellIdentity, CellServiceLevel, ChannelChangeDecision, ChannelChangeHandle,
    ChannelClassAssessmentRequest, ChannelClassLabel, ChannelClassMeasurement, Layer2Report,
    LowerLayerResourceAvailability, LowerLayerResourceReason, MeasurementReport, MeasurementValue,
    QualityIndication, RfChannelNumber, ScanRequestId, SelectionResult, TlmcScanState,
    TlmcSelectionState,
};
use tetra_saps::tlmc::{
    TlmcAssessmentInd, TlmcAssessmentListReq, TlmcCellReadConf, TlmcCellReadReq,
    TlmcConfigureConf, TlmcConfigureInd, TlmcConfigureReq, TlmcMeasurementInd,
    TlmcMonitorChannel, TlmcMonitorInd, TlmcMonitorListReq, TlmcScanConf, TlmcScanReportInd,
    TlmcScanReq, TlmcSelectConf, TlmcSelectInd, TlmcSelectReq, TlmcSelectResp,
};

#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Listet die möglichen Varianten für tlmc Laufzeit error auf.
// Warum: Die feste Variantenliste verhindert ungültige Zwischenwerte und zwingt den Code zu einer bewussten Fallbehandlung.
pub enum TlmcRuntimeError {
    InvalidConfiguration(&'static str),
    OperationBusy(&'static str),
    UnknownRequest(&'static str),
    RequestMismatch(&'static str),
    ChannelNotMonitored(RfChannelNumber),
    ChannelClassNotRequested(ChannelClassLabel),
    NoPendingSelection,
}

// Was: Implementiert das zugehörige Verhalten für `fmt::Display for TlmcRuntimeError`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl fmt::Display for TlmcRuntimeError {
    // Was: Führt den Arbeitsschritt `fmt` für fmt aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Was: Unterscheidet die möglichen Varianten und führt für jeden Fall den passenden Ablauf aus.
        // Warum: Protokoll- und Zustandswerte müssen vollständig behandelt werden, damit kein Fall stillschweigend falsch weiterläuft.
        match self {
            Self::InvalidConfiguration(reason) => write!(f, "invalid TLMC configuration: {reason}"),
            Self::OperationBusy(operation) => write!(f, "TLMC operation already active: {operation}"),
            Self::UnknownRequest(operation) => write!(f, "unknown TLMC request: {operation}"),
            Self::RequestMismatch(operation) => write!(f, "TLMC request correlation mismatch: {operation}"),
            Self::ChannelNotMonitored(channel) => write!(f, "channel {} is not monitored", channel.0),
            Self::ChannelClassNotRequested(label) => write!(f, "channel class {} was not requested", label.0),
            Self::NoPendingSelection => write!(f, "no TLMC selection is waiting for a response"),
        }
    }
}

// Was: Implementiert das zugehörige Verhalten für `std::error::Error for TlmcRuntimeError {}`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl std::error::Error for TlmcRuntimeError {}

/// Read-only operational view for diagnostics and the future TBS WebUI.
#[derive(Debug, Clone, PartialEq, Eq)]
// Was: Bündelt die zusammengehörigen Werte für tlmc Laufzeit snapshot in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmcRuntimeSnapshot {
    pub scan_state: TlmcScanState,
    pub selection_state: TlmcSelectionState,
    pub configured_endpoint: Option<EndpointId>,
    pub monitored_channels: Vec<RfChannelNumber>,
    pub assessed_channel_classes: Vec<ChannelClassLabel>,
    pub pending_cell_read: Option<(ScanRequestId, RfChannelNumber)>,
    pub known_resource_count: usize,
    pub unavailable_resource_count: usize,
    pub last_measurement: Option<MeasurementReport>,
}

#[derive(Debug, Clone, Default)]
// Was: Bündelt die zusammengehörigen Werte für tlmc Laufzeit in einem Datentyp.
// Warum: Ein eigener Datentyp verhindert lose Einzelwerte und macht gültige Zustände leichter erkennbar.
pub struct TlmcRuntime {
    configuration: TlmcConfigureReq,
    endpoint_resources: HashMap<EndpointId, LowerLayerResourceAvailability>,
    monitored_channels: HashMap<RfChannelNumber, TlmcMonitorChannel>,
    assessment_classes: HashMap<ChannelClassLabel, ChannelClassAssessmentRequest>,
    pending_scan: Option<TlmcScanReq>,
    pending_cell_read: Option<TlmcCellReadReq>,
    pending_select: Option<TlmcSelectReq>,
    pending_select_indication: Option<TlmcSelectInd>,
    scan_state: TlmcScanState,
    selection_state: TlmcSelectionState,
    current_cell: Option<CellIdentity>,
    last_measurement: Option<MeasurementReport>,
    last_monitor: HashMap<RfChannelNumber, TlmcMonitorInd>,
}

// Was: Implementiert das zugehörige Verhalten für `TlmcRuntime`.
// Warum: Die Operationen bleiben dadurch direkt bei dem Datentyp, dessen Zustand sie lesen oder verändern.
impl TlmcRuntime {
    // Was: Erzeugt eine neue Instanz mit den vorgesehenen Anfangswerten.
    // Warum: Das Objekt wird dadurch vollständig und mit sicheren Anfangswerten angelegt.
    pub fn new() -> Self {
        Self::default()
    }

    // Was: Diese Funktion erzeugt den vorgesehenen Arbeitsschritt.
    // Warum: Die Oberfläche und andere Dienste erhalten dadurch eine in sich stimmige Momentaufnahme.
    pub fn snapshot(&self) -> TlmcRuntimeSnapshot {
        let mut monitored_channels: Vec<_> = self.monitored_channels.keys().copied().collect();
        monitored_channels.sort_by_key(|channel| channel.0);

        let mut assessed_channel_classes: Vec<_> = self.assessment_classes.keys().copied().collect();
        assessed_channel_classes.sort_by_key(|label| label.0);

        TlmcRuntimeSnapshot {
            scan_state: self.scan_state.clone(),
            selection_state: self.selection_state.clone(),
            configured_endpoint: self.configuration.endpoint_id,
            monitored_channels,
            assessed_channel_classes,
            pending_cell_read: self
                .pending_cell_read
                .as_ref()
                .map(|request| (request.request_id, request.channel_number)),
            known_resource_count: self.endpoint_resources.len(),
            unavailable_resource_count: self
                .endpoint_resources
                .values()
                .filter(|availability| **availability == LowerLayerResourceAvailability::Unavailable)
                .count(),
            last_measurement: self.last_measurement.clone(),
        }
    }

    // Was: Führt den Arbeitsschritt `configuration` für configuration aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn configuration(&self) -> &TlmcConfigureReq {
        &self.configuration
    }

    // Was: Führt den Arbeitsschritt `current_cell` für current cell aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn current_cell(&self) -> Option<&CellIdentity> {
        self.current_cell.as_ref()
    }

    // Was: Führt den Arbeitsschritt `scan_state` für scan Zustand aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn scan_state(&self) -> &TlmcScanState {
        &self.scan_state
    }

    // Was: Führt den Arbeitsschritt `selection_state` für selection Zustand aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn selection_state(&self) -> &TlmcSelectionState {
        &self.selection_state
    }

    // Was: Führt den Arbeitsschritt `pending_scan` für pending scan aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn pending_scan(&self) -> Option<&TlmcScanReq> {
        self.pending_scan.as_ref()
    }

    // Was: Führt den Arbeitsschritt `pending_cell_read` für pending cell read aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn pending_cell_read(&self) -> Option<&TlmcCellReadReq> {
        self.pending_cell_read.as_ref()
    }

    // Was: Führt den Arbeitsschritt `pending_select` für pending select aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn pending_select(&self) -> Option<&TlmcSelectReq> {
        self.pending_select.as_ref()
    }

    // Was: Diese Funktion wendet configure.
    // Warum: Die Änderung wird dadurch nur über einen definierten und prüfbaren Weg wirksam.
    pub fn apply_configure(&mut self, request: TlmcConfigureReq) -> Result<TlmcConfigureConf, TlmcRuntimeError> {
        Self::validate_configure(&request)?;
        Self::merge_configure(&mut self.configuration, request);
        Ok(Self::configure_confirmation(&self.configuration))
    }

    // Was: Diese Funktion prüft configure.
    // Warum: Unzulässige Werte werden dadurch erkannt, bevor sie im Betrieb Schaden anrichten.
    fn validate_configure(request: &TlmcConfigureReq) -> Result<(), TlmcRuntimeError> {
        if let Some(distribution) = request.distribution_on_18th_frame {
            if !(1..=4).contains(&distribution.timeslot) {
                return Err(TlmcRuntimeError::InvalidConfiguration(
                    "frame-18 monitoring timeslot must be in 1..=4",
                ));
            }
        }
        if let Some(startpoint) = request.energy_economy_startpoint {
            startpoint
                .validate()
                .map_err(TlmcRuntimeError::InvalidConfiguration)?;
        }
        if let Some(startpoint) = request.dual_watch_startpoint {
            startpoint
                .validate()
                .map_err(TlmcRuntimeError::InvalidConfiguration)?;
        }
        if let Some(schedule) = request.schedule_repetition_information {
            schedule
                .validate()
                .map_err(TlmcRuntimeError::InvalidConfiguration)?;
        }
        Ok(())
    }

    // Was: Diese Funktion führt configure.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn merge_configure(current: &mut TlmcConfigureReq, update: TlmcConfigureReq) {
        // Was: Definiert das Makro `merge_option`, das wiederkehrenden Rust-Code erzeugt.
        // Warum: Gleichartige Strukturen werden dadurch nur einmal beschrieben und können nicht unbemerkt auseinanderlaufen.
        macro_rules! merge_option {
            ($field:ident) => {
                if update.$field.is_some() {
                    current.$field = update.$field;
                }
            };
        }

        merge_option!(threshold_values);
        merge_option!(distribution_on_18th_frame);
        merge_option!(scch_information);
        merge_option!(energy_economy_group);
        merge_option!(energy_economy_startpoint);
        merge_option!(dual_watch_energy_economy_group);
        merge_option!(dual_watch_startpoint);
        merge_option!(mle_activity_indicator);
        merge_option!(channel_change_accepted);
        merge_option!(channel_change_handle);
        merge_option!(operating_mode);
        merge_option!(call_release);
        merge_option!(valid_addresses);
        merge_option!(ms_default_data_priority);
        merge_option!(layer_2_data_priority_lifetime);
        merge_option!(layer_2_data_priority_signalling_delay);
        merge_option!(data_priority_random_access_delay_factor);
        merge_option!(schedule_repetition_information);
        merge_option!(data_class_activity_information);
        merge_option!(endpoint_id);
        merge_option!(periodic_reporting_timer);
        merge_option!(graceful_service_degradation_mode_control);
        merge_option!(llc_timer_status);
        merge_option!(link_performance_information);
    }

    // Was: Führt den Arbeitsschritt `configure_confirmation` für configure confirmation aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn configure_confirmation(configuration: &TlmcConfigureReq) -> TlmcConfigureConf {
        TlmcConfigureConf {
            threshold_values: configuration.threshold_values.clone(),
            distribution_on_18th_frame: configuration.distribution_on_18th_frame,
            scch_information: configuration.scch_information,
            energy_economy_group: configuration.energy_economy_group,
            energy_economy_startpoint: configuration.energy_economy_startpoint,
            dual_watch_energy_economy_group: configuration.dual_watch_energy_economy_group,
            dual_watch_startpoint: configuration.dual_watch_startpoint,
            operating_mode: configuration.operating_mode.clone(),
            call_release: configuration.call_release,
            valid_addresses: configuration.valid_addresses,
            ms_default_data_priority: configuration.ms_default_data_priority,
            layer_2_data_priority_lifetime: configuration.layer_2_data_priority_lifetime,
            layer_2_data_priority_signalling_delay: configuration.layer_2_data_priority_signalling_delay,
            data_priority_random_access_delay_factor: configuration.data_priority_random_access_delay_factor,
            schedule_repetition_information: configuration.schedule_repetition_information,
            data_class_activity_information: configuration.data_class_activity_information,
            endpoint_id: configuration.endpoint_id,
        }
    }

    /// Record an edge-triggered lower-layer resource transition.
    // Was: Führt den Arbeitsschritt `resource_transition` für resource transition aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn resource_transition(
        &mut self,
        endpoint_id: EndpointId,
        availability: LowerLayerResourceAvailability,
        reason: LowerLayerResourceReason,
    ) -> Option<TlmcConfigureInd> {
        if self.endpoint_resources.get(&endpoint_id).copied() == Some(availability) {
            return None;
        }
        self.endpoint_resources.insert(endpoint_id, availability);
        Some(TlmcConfigureInd {
            endpoint_id,
            lower_layer_resource_availability: availability,
            reason,
        })
    }

    // Was: Führt den Arbeitsschritt `record_measurement` für Datensatz measurement aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn record_measurement(&mut self, measurement: MeasurementReport) -> TlmcMeasurementInd {
        self.last_measurement = Some(measurement.clone());
        TlmcMeasurementInd { measurement }
    }

    // Was: Diese Funktion setzt monitor list.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_monitor_list(&mut self, request: TlmcMonitorListReq) {
        self.monitored_channels.clear();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for channel in request.channels {
            self.monitored_channels.insert(channel.channel_number, channel);
        }
    }

    // Was: Prüft, ob monitored zutrifft.
    // Warum: Aufrufer erhalten dadurch eine eindeutige Ja-Nein-Entscheidung ohne eigene Detailprüfung.
    pub fn is_monitored(&self, channel_number: RfChannelNumber) -> bool {
        self.monitored_channels.contains_key(&channel_number)
    }

    // Was: Führt den Arbeitsschritt `record_monitor` für Datensatz monitor aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn record_monitor(
        &mut self,
        channel_number: RfChannelNumber,
        path_loss_c2: MeasurementValue,
        quality: Option<QualityIndication>,
        channel_classes: Vec<ChannelClassMeasurement>,
    ) -> Result<TlmcMonitorInd, TlmcRuntimeError> {
        if !self.is_monitored(channel_number) {
            return Err(TlmcRuntimeError::ChannelNotMonitored(channel_number));
        }
        let indication = TlmcMonitorInd {
            channel_number,
            path_loss_c2,
            quality,
            channel_classes,
        };
        self.last_monitor.insert(channel_number, indication.clone());
        Ok(indication)
    }

    // Was: Diese Funktion setzt assessment list.
    // Warum: Änderungen am Zustand laufen dadurch über einen klaren und kontrollierbaren Weg.
    pub fn set_assessment_list(&mut self, request: TlmcAssessmentListReq) {
        self.assessment_classes.clear();
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for class in request.classes {
            self.assessment_classes.insert(class.label, class);
        }
    }

    // Was: Führt den Arbeitsschritt `record_assessment` für Datensatz assessment aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn record_assessment(
        &self,
        assessments: Vec<ChannelClassMeasurement>,
    ) -> Result<TlmcAssessmentInd, TlmcRuntimeError> {
        // Was: Durchläuft mehrere Einträge oder wiederholt den folgenden Arbeitsschritt solange die Bedingung gilt.
        // Warum: Gleichartige Daten werden dadurch vollständig und nach denselben Regeln verarbeitet.
        for assessment in &assessments {
            if !self.assessment_classes.contains_key(&assessment.label) {
                return Err(TlmcRuntimeError::ChannelClassNotRequested(assessment.label));
            }
        }
        Ok(TlmcAssessmentInd { assessments })
    }

    // Was: Führt den Arbeitsschritt `begin_scan` für begin scan aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn begin_scan(&mut self, request: TlmcScanReq) -> Result<(), TlmcRuntimeError> {
        if self.pending_scan.is_some() {
            return Err(TlmcRuntimeError::OperationBusy("scan"));
        }
        self.scan_state = TlmcScanState::Requested {
            request_id: request.request_id,
            channel: request.channel_number,
        };
        self.pending_scan = Some(request.clone());
        self.scan_state = TlmcScanState::Running {
            request_id: request.request_id,
            channel: request.channel_number,
        };
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `complete_scan` für complete scan aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn complete_scan(
        &mut self,
        request_id: ScanRequestId,
        channel_number: RfChannelNumber,
        threshold_level: MeasurementValue,
        report: Layer2Report,
        channel_classes: Vec<ChannelClassMeasurement>,
        identity: Option<CellIdentity>,
        service_level: CellServiceLevel,
    ) -> Result<TlmcScanConf, TlmcRuntimeError> {
        let request = self
            .pending_scan
            .take()
            .ok_or(TlmcRuntimeError::UnknownRequest("scan"))?;
        if request.request_id != request_id || request.channel_number != channel_number {
            self.pending_scan = Some(request);
            return Err(TlmcRuntimeError::RequestMismatch("scan"));
        }

        if report == Layer2Report::Success {
            let candidate = CellCandidate {
                identity,
                channel_number,
                service_level,
                measurements: MeasurementReport {
                    endpoint_id: self.configuration.endpoint_id,
                    channel_number: Some(channel_number),
                    path_loss_c1: Some(threshold_level),
                    path_loss_c2: None,
                    path_loss_c3: None,
                    path_loss_c4: None,
                    path_loss_c5: None,
                    quality: None,
                },
            };
            self.scan_state = TlmcScanState::Completed {
                request_id,
                candidate,
            };
        } else {
            self.scan_state = TlmcScanState::Failed { request_id, report };
        }

        Ok(TlmcScanConf {
            request_id,
            channel_number,
            measurement_method: request.measurement_method,
            threshold_level,
            report,
            channel_classes,
        })
    }

    // Was: Führt den Arbeitsschritt `record_scan_report` für Datensatz scan report aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn record_scan_report(
        &mut self,
        request_id: Option<ScanRequestId>,
        channel_number: RfChannelNumber,
        path_loss_c1: MeasurementValue,
        report: Option<Layer2Report>,
        channel_classes: Vec<ChannelClassMeasurement>,
    ) -> TlmcScanReportInd {
        TlmcScanReportInd {
            request_id,
            channel_number,
            path_loss_c1,
            report,
            channel_classes,
        }
    }

    // Was: Führt den Arbeitsschritt `begin_cell_read` für begin cell read aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn begin_cell_read(&mut self, request: TlmcCellReadReq) -> Result<(), TlmcRuntimeError> {
        if self.pending_cell_read.is_some() {
            return Err(TlmcRuntimeError::OperationBusy("cell read"));
        }
        self.pending_cell_read = Some(request);
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `complete_cell_read` für complete cell read aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn complete_cell_read(
        &mut self,
        request_id: ScanRequestId,
        channel_number: RfChannelNumber,
        report: Layer2Report,
    ) -> Result<TlmcCellReadConf, TlmcRuntimeError> {
        let request = self
            .pending_cell_read
            .take()
            .ok_or(TlmcRuntimeError::UnknownRequest("cell read"))?;
        if request.request_id != request_id || request.channel_number != channel_number {
            self.pending_cell_read = Some(request);
            return Err(TlmcRuntimeError::RequestMismatch("cell read"));
        }
        Ok(TlmcCellReadConf {
            request_id,
            channel_number,
            report,
        })
    }

    // Was: Führt den Arbeitsschritt `begin_select` für begin select aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn begin_select(&mut self, request: TlmcSelectReq) -> Result<(), TlmcRuntimeError> {
        if self.pending_select.is_some() || self.pending_select_indication.is_some() {
            return Err(TlmcRuntimeError::OperationBusy("selection"));
        }
        let candidate = Self::candidate_from_select_request(&request);
        self.selection_state = TlmcSelectionState::Requested {
            candidate: candidate.clone(),
            cause: request.cause,
        };
        self.pending_select = Some(request);
        self.selection_state = TlmcSelectionState::AwaitingResponse { candidate };
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `complete_select` für complete select aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn complete_select(
        &mut self,
        channel_number: RfChannelNumber,
        threshold_level: Option<MeasurementValue>,
        main_carrier_number: Option<RfChannelNumber>,
        report: Option<Layer2Report>,
        result: SelectionResult,
        identity: Option<CellIdentity>,
    ) -> Result<TlmcSelectConf, TlmcRuntimeError> {
        let request = self
            .pending_select
            .take()
            .ok_or(TlmcRuntimeError::UnknownRequest("selection"))?;
        if request.channel_number != channel_number {
            self.pending_select = Some(request);
            return Err(TlmcRuntimeError::RequestMismatch("selection"));
        }

        let effective_result = if result == SelectionResult::Success {
            let selected_cell = identity.or_else(|| {
                self.configuration.valid_addresses.map(|address| CellIdentity {
                    mcc: address.mcc,
                    mnc: address.mnc,
                    location_area: None,
                    colour_code: None,
                    main_carrier: main_carrier_number.unwrap_or(channel_number).0,
                    cell_type: Default::default(),
                })
            });
            if let Some(cell) = selected_cell {
                self.current_cell = Some(cell.clone());
                self.selection_state = TlmcSelectionState::Completed { cell };
                SelectionResult::Success
            } else {
                let failure = SelectionResult::Other(0);
                self.selection_state = TlmcSelectionState::Failed { result: failure };
                failure
            }
        } else {
            self.selection_state = TlmcSelectionState::Failed { result };
            result
        };

        Ok(TlmcSelectConf {
            channel_number,
            threshold_level,
            main_carrier_number,
            report,
            result: effective_result,
        })
    }

    // Was: Diese Funktion empfängt select indication.
    // Warum: Eingehende Daten werden so geordnet geprüft, bevor sie weiterverteilt werden.
    pub fn receive_select_indication(&mut self, indication: TlmcSelectInd) -> Result<(), TlmcRuntimeError> {
        if self.pending_select.is_some() || self.pending_select_indication.is_some() {
            return Err(TlmcRuntimeError::OperationBusy("selection indication"));
        }
        let candidate = CellCandidate {
            identity: None,
            channel_number: indication.channel_number,
            service_level: CellServiceLevel::NormalService,
            measurements: MeasurementReport {
                endpoint_id: self.configuration.endpoint_id,
                channel_number: Some(indication.channel_number),
                path_loss_c1: indication.threshold_level,
                path_loss_c2: None,
                path_loss_c3: None,
                path_loss_c4: None,
                path_loss_c5: None,
                quality: None,
            },
        };
        self.selection_state = TlmcSelectionState::AwaitingResponse { candidate };
        self.pending_select_indication = Some(indication);
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `respond_select` für respond select aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn respond_select(&mut self, response: TlmcSelectResp) -> Result<(), TlmcRuntimeError> {
        let indication = self
            .pending_select_indication
            .take()
            .ok_or(TlmcRuntimeError::NoPendingSelection)?;
        if indication.channel_number != response.channel_number
            || indication.channel_change_handle != response.channel_change_handle
        {
            self.pending_select_indication = Some(indication);
            return Err(TlmcRuntimeError::RequestMismatch("selection response"));
        }

        let candidate = CellCandidate {
            identity: None,
            channel_number: response.channel_number,
            service_level: CellServiceLevel::NormalService,
            measurements: MeasurementReport {
                endpoint_id: self.configuration.endpoint_id,
                channel_number: Some(response.channel_number),
                path_loss_c1: response.threshold_level,
                path_loss_c2: None,
                path_loss_c3: None,
                path_loss_c4: None,
                path_loss_c5: None,
                quality: None,
            },
        };
        self.selection_state = match response.decision {
            ChannelChangeDecision::Accept => TlmcSelectionState::AwaitingResponse { candidate },
            ChannelChangeDecision::Reject | ChannelChangeDecision::Ignore => TlmcSelectionState::Failed {
                result: SelectionResult::Reject,
            },
        };
        Ok(())
    }

    // Was: Führt den Arbeitsschritt `channel_change_handle` für Kanal change handle aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    pub fn channel_change_handle(&self) -> Option<ChannelChangeHandle> {
        self.pending_select_indication
            .as_ref()
            .and_then(|indication| indication.channel_change_handle)
    }

    // Was: Führt den Arbeitsschritt `candidate_from_select_request` für candidate from select request aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn candidate_from_select_request(request: &TlmcSelectReq) -> CellCandidate {
        CellCandidate {
            identity: None,
            channel_number: request.channel_number,
            service_level: CellServiceLevel::NormalService,
            measurements: MeasurementReport {
                endpoint_id: None,
                channel_number: Some(request.channel_number),
                path_loss_c1: request.threshold_level,
                path_loss_c2: None,
                path_loss_c3: None,
                path_loss_c4: None,
                path_loss_c5: None,
                quality: None,
            },
        }
    }
}

#[cfg(test)]
// Was: Bindet das Untermodul tests in diesen Bereich ein.
// Warum: Die Funktionalität bleibt dadurch thematisch getrennt und trotzdem über das übergeordnete Modul erreichbar.
mod tests {
    use super::*;
    use tetra_saps::common::{
        ChannelBandwidth, ChannelInformation, ChannelRole, ChannelTopology, Frame18Distribution,
        ModulationMode, ScanningMeasurementMethod, SelectionCause,
    };

    // Was: Führt den Arbeitsschritt `channel_info` für Kanal info aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn channel_info() -> ChannelInformation {
        ChannelInformation {
            modulation: ModulationMode::PhaseModulation,
            bandwidth: ChannelBandwidth::Khz25,
            topology: ChannelTopology::Conforming,
        }
    }

    #[test]
    // Was: Führt den Arbeitsschritt `configure_merges_partial_updates_and_reports_resource_edges` für configure merges partial updates and reports resource und weitere Angaben aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn configure_merges_partial_updates_and_reports_resource_edges() {
        let mut runtime = TlmcRuntime::new();
        let confirmation = runtime
            .apply_configure(TlmcConfigureReq {
                endpoint_id: Some(7),
                distribution_on_18th_frame: Some(Frame18Distribution { timeslot: 2 }),
                ..Default::default()
            })
            .unwrap();
        assert_eq!(confirmation.endpoint_id, Some(7));

        assert!(runtime
            .resource_transition(
                7,
                LowerLayerResourceAvailability::Unavailable,
                LowerLayerResourceReason::LossOfRadioResources,
            )
            .is_some());
        assert!(runtime
            .resource_transition(
                7,
                LowerLayerResourceAvailability::Unavailable,
                LowerLayerResourceReason::LossOfRadioResources,
            )
            .is_none());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `scan_and_selection_have_correlated_lifecycles` für scan and selection have correlated lifecycles aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn scan_and_selection_have_correlated_lifecycles() {
        let mut runtime = TlmcRuntime::new();
        runtime
            .begin_scan(TlmcScanReq {
                request_id: ScanRequestId(9),
                channel_number: RfChannelNumber(720),
                measurement_method: ScanningMeasurementMethod::NonInterrupting,
                characteristics: None,
                threshold_level: None,
                channel_classes: Vec::new(),
            })
            .unwrap();
        let scan = runtime
            .complete_scan(
                ScanRequestId(9),
                RfChannelNumber(720),
                MeasurementValue::dbm(-91),
                Layer2Report::Success,
                Vec::new(),
                None,
                CellServiceLevel::NormalService,
            )
            .unwrap();
        assert_eq!(scan.report, Layer2Report::Success);

        runtime
            .begin_select(TlmcSelectReq {
                channel_number: RfChannelNumber(720),
                channel_information: Some(channel_info()),
                threshold_level: Some(MeasurementValue::dbm(-91)),
                main_carrier_number: Some(RfChannelNumber(720)),
                main_carrier_information: Some(channel_info()),
                cause: SelectionCause::InitialCellSelection,
            })
            .unwrap();
        let selected = runtime
            .complete_select(
                RfChannelNumber(720),
                Some(MeasurementValue::dbm(-91)),
                Some(RfChannelNumber(720)),
                Some(Layer2Report::Success),
                SelectionResult::Success,
                Some(CellIdentity {
                    mcc: 262,
                    mnc: 1,
                    location_area: Some(1),
                    colour_code: Some(1),
                    main_carrier: 720,
                    cell_type: Default::default(),
                }),
            )
            .unwrap();
        assert_eq!(selected.result, SelectionResult::Success);
        assert!(runtime.current_cell().is_some());
    }

    #[test]
    // Was: Führt den Arbeitsschritt `monitor_requires_an_explicit_monitor_list` für monitor requires an explicit monitor list aus.
    // Warum: Der abgegrenzte Arbeitsschritt kann dadurch wiederverwendet, getestet und leichter verstanden werden.
    fn monitor_requires_an_explicit_monitor_list() {
        let mut runtime = TlmcRuntime::new();
        assert!(matches!(
            runtime.record_monitor(
                RfChannelNumber(720),
                MeasurementValue::db(-5),
                None,
                Vec::new(),
            ),
            Err(TlmcRuntimeError::ChannelNotMonitored(_))
        ));

        runtime.set_monitor_list(TlmcMonitorListReq {
            channels: vec![TlmcMonitorChannel {
                channel_number: RfChannelNumber(720),
                characteristics: tetra_saps::common::RfChannelCharacteristics {
                    modulation: ModulationMode::PhaseModulation,
                    bandwidth: ChannelBandwidth::Khz25,
                    max_ms_tx_power_dbm: None,
                    min_rx_access_level_dbm: None,
                    discontinuous: None,
                    role: ChannelRole::NeighbourMainCarrier,
                    topology: ChannelTopology::Conforming,
                },
                channel_classes: Vec::new(),
            }],
        });
        assert!(runtime
            .record_monitor(
                RfChannelNumber(720),
                MeasurementValue::db(-5),
                Some(QualityIndication { raw: 12 }),
                Vec::new(),
            )
            .is_ok());
    }
}
