use disaster_mesh::{MessageContent, MessageManager, UserId};

#[tokio::test]
async fn test_message_creation() {
    let manager = MessageManager::new().await.unwrap();
    let sender = UserId::random();
    let content = MessageContent::Text("Hello".into());

    let message = manager
        .create_message(sender, None, content.clone())
        .await
        .unwrap();

    assert_eq!(message.sender, sender);
    assert_eq!(message.content, content);
}
