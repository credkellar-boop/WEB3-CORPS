mkdir -p src/mesh
cat << 'EOF' > src/mesh/event_mesh.rs
#[derive(Debug, Clone)]
pub struct Message {
    pub id: String,
    pub sender: String,
}

// Add any missing implementation blocks below if cargo check throws further errors
impl Message {
    pub fn new(id: &str, sender: &str) -> Self {
        Self {
            id: id.to_string(),
            sender: sender.to_string(),
        }
    }
}
EOF
