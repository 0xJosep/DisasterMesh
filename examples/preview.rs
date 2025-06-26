use disaster_mesh::{MessageContent, MessageManager, RoutingEngine, PeerId, UserId};
use std::time::Duration;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    println!("DisasterMesh quick preview\n-------------------------");

    // 1) Create a new message
    let manager = MessageManager::new().await?;
    let sender = UserId::random();
    let content = MessageContent::Text("Hello, DisasterMesh!".into());
    let message = manager.create_message(sender, None, content.clone()).await?;
    println!("Created message: {:?}", message);

    // 2) Basic routing engine interaction
    let engine = RoutingEngine::new(Duration::from_secs(120));
    let dest_user = UserId::random();
    let next_hop = PeerId([1; 32]);
    engine.update_route(dest_user, next_hop, 2, 0.75).await;

    if let Some(nh) = engine.next_hop(&dest_user).await {
        println!("Routing entry: destination {:?} via {:?}", dest_user, nh);
    }

    println!("Preview complete – the library compiles, messages can be created, \nand routes can be stored/query\n");
    Ok(())
} 