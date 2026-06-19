use web3_corps::agent::Message;

#[tokio::test]
async fn test_async_inter_agent_routing() {
    let (tx, mut rx) = tokio::sync::mpsc::channel(10);
    
    let mock_msg = Message {
        id: "1".to_string(),
        sender: "Agent_Genesis_Bot".to_string(),
        recipient: "CTO".to_string(),
        payload: serde_json::json!({"status": "active"}),
    };

    tx.send(mock_msg).await.unwrap();
    let received = rx.recv().await.unwrap();
    assert_eq!(received.sender, "Agent_Genesis_Bot");
}
