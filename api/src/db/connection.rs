use deadpool_postgres::{Object, Pool, Runtime};
use log::warn;
use std::env;
use tokio_postgres::NoTls;

#[derive(Debug, Clone)]
pub struct DbManager {
    pool: Pool,
}

impl DbManager {
    pub fn new() -> Self {
        let mut cfg = deadpool_postgres::Config::new();
        cfg.host = env::var("RUST_DB_HOST").ok();
        cfg.user = env::var("RUST_DB_USER").ok();
        cfg.password = env::var("RUST_DB_PASSWORD").ok();
        // todo:set db_name by docker-compose.yml
        cfg.dbname = env::var("RUST_DB_NAME").ok();
        let pool = cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
        warn!("Error DB pool Couldn't connect");
        Self { pool }
    }
    pub async fn client(&self) -> Object {
        self.pool.get().await.unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_db_connection() {
        let pool = DbManager::new();
        let client = pool.client().await;
        let row = client.query_one("SELECT 1", &[]).await;
        assert!(row.is_ok());
    }
}
