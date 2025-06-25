use serde::{Deserialize, Serialize};
use std::time::{Duration, SystemTime};
use uuid::Uuid;

/// Unique identifier for a message (UUID v4)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct MessageId(Uuid);

impl MessageId {
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    pub fn to_bytes(self) -> [u8; 16] {
        *self.0.as_bytes()
    }

    pub fn from_bytes(bytes: &[u8; 16]) -> Self {
        Self(Uuid::from_bytes(*bytes))
    }

    pub fn is_valid(&self) -> bool {
        // UUID v4 always valid – placeholder for future validation
        true
    }
}

impl Default for MessageId {
    fn default() -> Self {
        Self::new()
    }
}

/// Public-key hash identifying a user
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct UserId(pub [u8; 32]);

impl UserId {
    pub fn random() -> Self {
        Self(rand::random())
    }
}

/// Identifier for a peer device (transport-specific)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PeerId(pub [u8; 32]);

/// Helper alias used in several structs
pub type Timestamp = SystemTime;

/// Standard TTL used when none specified
pub const DEFAULT_TTL: Duration = Duration::from_secs(3600);
