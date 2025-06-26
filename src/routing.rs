use crate::types::{PeerId, UserId};
use std::collections::HashMap;
use std::time::{Duration, SystemTime};
use tokio::sync::RwLock;
use std::sync::Arc;

/// Routing information for a single destination
#[derive(Debug, Clone)]
pub struct RouteInfo {
    pub destination: UserId,
    pub next_hop: PeerId,
    pub hop_count: u8,
    pub last_updated: SystemTime,
    pub link_quality: f32,
}

impl RouteInfo {
    fn is_expired(&self, max_age: Duration) -> bool {
        self.last_updated
            .elapsed()
            .map(|e| e > max_age)
            .unwrap_or(true)
    }
}

/// A minimal routing engine maintaining a table of the best-known routes.
#[derive(Clone)]
pub struct RoutingEngine {
    routes: Arc<RwLock<HashMap<UserId, RouteInfo>>>,
    max_age: Duration,
}

impl RoutingEngine {
    /// Create a new routing engine.
    pub fn new(max_age: Duration) -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            max_age,
        }
    }

    /// Update or insert a route. If an existing route has a worse hop count or
    /// link quality, it will be replaced.
    pub async fn update_route(
        &self,
        destination: UserId,
        next_hop: PeerId,
        hop_count: u8,
        link_quality: f32,
    ) {
        let mut routes = self.routes.write().await;
        let should_replace = routes
            .get(&destination)
            .map(|existing| {
                hop_count < existing.hop_count
                    || (hop_count == existing.hop_count && link_quality > existing.link_quality)
            })
            .unwrap_or(true);

        if should_replace {
            routes.insert(
                destination,
                RouteInfo {
                    destination,
                    next_hop,
                    hop_count,
                    last_updated: SystemTime::now(),
                    link_quality,
                },
            );
        }
    }

    /// Retrieve the next hop for a destination, if a valid route exists.
    pub async fn next_hop(&self, destination: &UserId) -> Option<PeerId> {
        let routes = self.routes.read().await;
        routes.get(destination).map(|r| r.next_hop)
    }

    /// Remove expired routes.
    pub async fn cleanup(&self) {
        let mut routes = self.routes.write().await;
        routes.retain(|_, route| !route.is_expired(self.max_age));
    }

    /// For testing and diagnostics: return a snapshot of current table.
    pub async fn dump(&self) -> Vec<RouteInfo> {
        let routes = self.routes.read().await;
        routes.values().cloned().collect()
    }
} 