use crate::types::UserId;
use serde::{Deserialize, Serialize};

/// Control packets for the routing protocol (AODV-inspired)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoutingControl {
    /// Route Request – broadcast when a node needs a route to `destination`.
    Rreq {
        origin: UserId,
        destination: UserId,
        /// Unique per-origin request ID to match RREP replies.
        request_id: u32,
        hop_count: u8,
    },

    /// Route Reply – unicast back to the originator of a matching RREQ.
    Rrep {
        origin: UserId,
        destination: UserId,
        hop_count: u8,
    },

    /// Route Error – notifies that given destinations are unreachable.
    Rerr {
        unreachable: Vec<UserId>,
    },
} 