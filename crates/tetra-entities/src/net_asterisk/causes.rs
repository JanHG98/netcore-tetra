//! Translate only at the SIP boundary. CMCE/Brew SAP causes are TETRA values,
//! not Q.850 numbers (in particular, Q.850 16 is NOT TETRA identity unknown).
use tetra_pdus::cmce::enums::disconnect_cause::DisconnectCause;

pub(super) fn from_q850(cause: u8) -> DisconnectCause {
    use DisconnectCause::*;
    match cause {
        1 | 22 | 28 => UnknownExternalSubscriberIdentity,
        2 | 3 | 18 | 19 | 20 | 27 => CalledPartyNotReachable,
        8 => PreEmptiveUseOfResource,
        16 | 26 => UserRequestedDisconnection,
        17 => CalledPartyBusy,
        21 => CallRejectedByTheCalledParty,
        34 | 38 | 41 | 42 | 44 | 47 => CongestionInInfrastructure,
        50 | 57 => NotAllowedTrafficCase,
        58 | 65 | 69 | 79 => RequestedServiceNotAvailable,
        81 => InvalidCallIdentifier,
        88 => IncompatibleTrafficCase,
        102 => ExpiryOfTimer,
        _ => CauseNotDefinedOrUnknown,
    }
}

pub(super) fn from_sip_status(code: u16) -> DisconnectCause {
    use DisconnectCause::*;
    match code {
        401 | 403 | 407 | 603 => CallRejectedByTheCalledParty,
        404 | 410 | 484 | 485 | 604 => UnknownExternalSubscriberIdentity,
        408 | 504 => ExpiryOfTimer,
        480 | 483 => CalledPartyNotReachable,
        486 | 600 => CalledPartyBusy,
        487 => UserRequestedDisconnection,
        488 | 606 => IncompatibleTrafficCase,
        500 | 502 | 503 | 513 => CongestionInInfrastructure,
        501 | 505 => RequestedServiceNotAvailable,
        _ => CauseNotDefinedOrUnknown,
    }
}

/// Ignore SIP Reason values and malformed/out-of-range causes; those use the
/// response status (or normal BYE/CANCEL clearing) as their fallback.
pub(super) fn from_reason(value: &str) -> Option<DisconnectCause> {
    for reason in value.split(',') {
        let mut parts = reason.trim().split(';');
        if !parts.next()?.trim().eq_ignore_ascii_case("Q.850") {
            continue;
        }
        for part in parts {
            let Some((name, value)) = part.split_once('=') else { continue };
            if name.trim().eq_ignore_ascii_case("cause") {
                if let Ok(cause) = value.trim().parse::<u8>() {
                    if cause <= 127 { return Some(from_q850(cause)); }
                }
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn telephony_causes_do_not_leak_into_tetra_number_space() {
        use DisconnectCause::*;
        for (q850, expected) in [(16, UserRequestedDisconnection), (17, CalledPartyBusy),
            (21, CallRejectedByTheCalledParty), (34, CongestionInInfrastructure),
            (1, UnknownExternalSubscriberIdentity), (102, ExpiryOfTimer),
            (127, CauseNotDefinedOrUnknown)] {
            assert_eq!(from_q850(q850), expected);
        }
        assert_eq!(from_sip_status(486), CalledPartyBusy);
        assert_eq!(from_sip_status(404), UnknownExternalSubscriberIdentity);
        assert_eq!(from_sip_status(503), CongestionInInfrastructure);
        assert_eq!(from_sip_status(488), IncompatibleTrafficCase);
    }

    #[test]
    fn reason_header_checks_protocol_case_and_numeric_range() {
        assert_eq!(from_reason("SIP;cause=200, q.850;Cause=16;text=\"Normal clearing\""),
            Some(DisconnectCause::UserRequestedDisconnection));
        for bad in ["SIP;cause=16", "Q.850;cause=garbage", "Q.850;cause=256", "Q.850;cause=128", "Q.850"] {
            assert_eq!(from_reason(bad), None);
        }
    }
}
