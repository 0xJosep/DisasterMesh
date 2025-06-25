use crate::message::{Message, MessageContent};
use crate::types::{MessageId, UserId};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use sled::Db;
use std::sync::Arc;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct MessageManager {
    db: Arc<Db>,
    seen: Arc<RwLock<sled::Tree>>, // message ids seen
}

impl MessageManager {
    pub async fn new() -> Result<Self> {
        let db = sled::open(".disastermesh_store").context("open sled")?;
        let seen = db.open_tree("seen")?;
        Ok(Self { db: Arc::new(db), seen: Arc::new(RwLock::new(seen)) })
    }

    /// Create a new signed (signature omitted in stub) message
    pub async fn create_message(
        &self,
        sender: UserId,
        recipient: Option<UserId>,
        content: MessageContent,
    ) -> Result<Message> {
        let message = Message::new(sender, recipient, content);
        self.db.insert(message.id.to_bytes(), bincode::serialize(&message)?)?;
        Ok(message)
    }

    /// Placeholder "encryption" – simply serializes with bincode
    pub async fn encrypt_message(&self, content: &MessageContent, _peer_pub: &UserId) -> Result<Vec<u8>> {
        Ok(bincode::serialize(content)?)
    }

    /// Placeholder "decryption" – bincode deserialization
    pub async fn decrypt_message(&self, data: &[u8], _priv_key: &UserId) -> Result<MessageContent> {
        Ok(bincode::deserialize(data)?)
    }

    pub async fn validate_message(&self, msg: &Message) -> Result<()> {
        // TTL check
        let age = SystemTime::now()
            .duration_since(msg.timestamp)
            .unwrap_or(Duration::from_secs(0));
        if age > msg.ttl {
            anyhow::bail!("Message expired")
        }
        // TODO signature validation
        Ok(())
    }

    pub async fn is_new_message(&self, id: &MessageId) -> bool {
        !self.db.contains_key(id.to_bytes()).unwrap_or(false)
    }

    pub async fn mark_message_seen(&self, id: &MessageId) -> Result<()> {
        self.db.insert(id.to_bytes(), &[])?;
        Ok(())
    }
} 