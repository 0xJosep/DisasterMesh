use disaster_mesh::{routing::RoutingEngine, PeerId, UserId};
use std::time::Duration;

#[tokio::test]
async fn test_routing_update_and_query() {
    let engine = RoutingEngine::new(Duration::from_secs(60));
    let dest = UserId::random();
    let next_hop = PeerId([1; 32]);

    // Insert route with hop_count 3
    engine
        .update_route(dest, next_hop, 3, 0.8)
        .await;

    // Fetch next hop
    let retrieved = engine.next_hop(&dest).await;
    assert_eq!(retrieved, Some(next_hop));

    // Update with better hop_count
    let better_hop = PeerId([2; 32]);
    engine
        .update_route(dest, better_hop, 2, 0.8)
        .await;
    let retrieved_better = engine.next_hop(&dest).await;
    assert_eq!(retrieved_better, Some(better_hop));
} 