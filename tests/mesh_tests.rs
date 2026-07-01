#[tokio::test]
async fn test_async_inter_agent_routing() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);

    let mock_msg = Message {
        id: "1".to_string(),
        sender: "Agent_Genesis_Bot".to_string(),
        // ... the rest of your struct and test remains the same
