use async_trait::async_trait;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};

use crate::types::PeerId;
use anyhow::Result;

#[derive(Debug, Clone)]
pub enum TransportEvent {
    PeerConnected(PeerId),
    PeerDisconnected(PeerId),
    DataReceived { peer: PeerId, data: Vec<u8> },
    Error(String),
}

#[async_trait]
pub trait Transport: Send + Sync {
    async fn start(&mut self) -> Result<()>;
    async fn send(&self, peer: PeerId, data: Vec<u8>) -> Result<()>;
    async fn broadcast(&self, data: Vec<u8>) -> Result<()>;
    fn get_peers(&self) -> Vec<PeerId>;
    fn subscribe_events(&self) -> broadcast::Receiver<TransportEvent>;
}

/// A basic in-memory mock transport useful for early tests
pub struct MockTransport {
    peers: Arc<RwLock<Vec<PeerId>>>,
    tx: broadcast::Sender<TransportEvent>,
}

impl MockTransport {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(1024);
        Self { peers: Arc::new(RwLock::new(Vec::new())), tx }
    }
}

impl Default for MockTransport {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl Transport for MockTransport {
    async fn start(&mut self) -> Result<()> {
        // No-op for mock
        Ok(())
    }

    async fn send(&self, peer: PeerId, data: Vec<u8>) -> Result<()> {
        let _ = self.tx.send(TransportEvent::DataReceived { peer, data });
        Ok(())
    }

    async fn broadcast(&self, data: Vec<u8>) -> Result<()> {
        let peers = self.peers.read().await.clone();
        for peer in peers {
            let _ = self.tx.send(TransportEvent::DataReceived { peer, data: data.clone() });
        }
        Ok(())
    }

    fn get_peers(&self) -> Vec<PeerId> {
        futures::executor::block_on(async { self.peers.read().await.clone() })
    }

    fn subscribe_events(&self) -> broadcast::Receiver<TransportEvent> {
        self.tx.subscribe()
    }
} 