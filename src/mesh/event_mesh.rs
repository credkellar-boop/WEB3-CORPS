#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub sender: String,
}

impl Message {
    pub fn new(id: &str, sender: &str) -> Self {
        Self {
            id: id.to_string(),
            sender: sender.to_string(),
        }
    }
}
