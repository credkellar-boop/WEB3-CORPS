use async_trait::async_trait;
use anyhow::Result;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Message {
    pub id: String,
    pub sender: String,
    pub recipient: String,
    pub payload: serde_json::Value,
}

@async_trait
pub trait AutonomousAgent: Send + Sync {
    fn name(&self) -> &str;
    fn role_permissions(&self) -> Vec<String>;
    async fn process_message(&self, msg: Message) -> Result<Option<Message>>;
}
