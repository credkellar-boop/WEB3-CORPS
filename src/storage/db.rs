use std::collections::HashMap;
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct StateLedgerDatabase {
    pub storage_kv: Arc<RwLock<HashMap<String, String>>>,
}

impl StateLedgerDatabase {
    pub fn open() -> Self {
        Self { storage_kv: Arc::new(RwLock::new(HashMap::new())) }
    }

    pub fn write_state(&self, key: &str, value: &str) -> anyhow::Result<()> {
        let mut db = self.storage_kv.write().map_err(|_| anyhow::anyhow!("DB Lock Poisoned"))?;
        db.insert(key.to_string(), value.to_string());
        Ok(())
    }

    pub fn read_state(&self, key: &str) -> anyhow::Result<Option<String>> {
        let db = self.storage_kv.read().map_err(|_| anyhow::anyhow!("DB Lock Poisoned"))?;
        Ok(db.get(key).cloned())
    }
}
