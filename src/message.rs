use crate::types::{MessageId, Timestamp, UserId};
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Content variants for messages
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type", content = "data")]
pub enum MessageContent {
    Text(String),
    File { name: String, data: Vec<u8> },
    Routing(crate::routing_control::RoutingControl),
}

/// Priority levels – lower value is higher priority
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum MessagePriority {
    Emergency = 0,
    Urgent = 1,
    Normal = 2,
    Background = 3,
}

/// Main envelope for all messages shared across the mesh
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: MessageId,
    pub sender: UserId,
    pub recipient: Option<UserId>,
    pub content: MessageContent,
    pub timestamp: Timestamp,
    pub ttl: Duration,
    pub hop_count: u8,
    pub signature: Vec<u8>,
}

impl Message {
    pub fn new(sender: UserId, recipient: Option<UserId>, content: MessageContent) -> Self {
        Self {
            id: MessageId::new(),
            sender,
            recipient,
            content,
            timestamp: std::time::SystemTime::now(),
            ttl: Duration::from_secs(3600),
            hop_count: 0,
            signature: Vec::new(),
        }
    }
}
