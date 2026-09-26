use crate::net_media::{MediaDownlinkSource, MediaUplinkSink};

use super::umac_bs::UmacBs;

/// Compatibility shim for the modern Basisstation bootstrap while the live RF
/// state machine runs the exact v1.7.0 dual-carrier UMAC implementation.
///
/// v1.7.0 predates the central Media-Switch bridge. Keep the public setter so
/// the current binary can be built unchanged, but deliberately do not inject
/// central-media work into the TDMA/UMAC hot path. Once registration stability
/// is verified, the media bridge can be reintroduced behind an RF-neutral
/// worker boundary.
impl UmacBs {
    pub fn set_media_bridge(&mut self, _uplink_sink: MediaUplinkSink, _downlink_source: MediaDownlinkSource) {
        tracing::info!("UMAC v1.7 dual-RF compatibility mode: central media bridge detached from RF hot path");
    }
}
