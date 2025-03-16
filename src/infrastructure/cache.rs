use redis::aio::ConnectionManager;
use redis::AsyncCommands;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::error::Result;

#[derive(Clone)]
pub struct Cache {
    client: Arc<Mutex<ConnectionManager>>,
}

impl Cache {
    pub fn new(client: ConnectionManager) -> Self {
        Self {
            client: Arc::new(Mutex::new(client)),
        }
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>> {
        let mut conn = self.client.lock().await;
        let result: Option<String> = conn.get(key).await?;
        Ok(result)
    }

    pub async fn set_with_expiry(&self, key: &str, value: &str, expiry_secs: u64) -> Result<()> {
        let mut conn = self.client.lock().await;
        // Explicitly specify the return type as () for set_ex
        let _: () = conn.set_ex(key, value, expiry_secs as usize as u64).await?;
        Ok(())
    }

    pub fn url_cache_key(short_code: &str) -> String {
        format!("url:{}", short_code)
    }

    pub async fn ping(&self) -> Result<()> {
        let mut conn = self.client.lock().await;
        // Explicitly specify the return type as () for the PING command
        let _: () = redis::cmd("PING").query_async(&mut *conn).await?;
        Ok(())
    }
}
