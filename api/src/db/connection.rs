use deadpool_postgres::{Object, Pool, Runtime};
use log::warn;
use std::env;
use tokio_postgres::Client;
use tokio_postgres::Error;
use tokio_postgres::NoTls;

pub async fn connection() -> Result<Client, Error> {
    let host = env::var("RUST_DB_HOST").unwrap_or_else(|e| {
        eprintln!("{e}");
        String::from("host")
    });
    let user = env::var("RUST_DB_USER").unwrap_or_else(|e| {
        eprintln!("{e}");
        String::from("user")
    });
    let password = env::var("RUST_DB_PASSWORD").unwrap_or_else(|e| {
        eprintln!("{e}");
        String::from("password")
    });
    let db_name = env::var("RUST_DB_NAME").unwrap_or_else(|e| {
        eprintln!("{e}");
        String::from("db_name")
    });
    let config = format!("host={host} user={user} password={password} dbname={db_name}");
    // Connect to the database.
    let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

    // The connection object performs the actual communication with the database,
    // so spawn it off to run on its own.
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });
    println!("connection is succeed");
    Ok(client)
}

#[cfg(test)]
pub mod e_tests {
    use super::*;

    pub async fn connection_hoge() -> Result<Client, Error> {
        let host = env::var("RUST_DB_HOST").unwrap_or_else(|e| {
            eprintln!("{e}");
            String::from("host")
        });
        let user = env::var("RUST_DB_USER").unwrap_or_else(|e| {
            eprintln!("{e}");
            String::from("user")
        });
        let password = env::var("RUST_DB_PASSWORD").unwrap_or_else(|e| {
            eprintln!("{e}");
            String::from("password")
        });
        let db_name = String::from("hoge");
        let config = format!("host={host} user={user} password={password} dbname={db_name}");
        // Connect to the database.
        let (client, connection) = tokio_postgres::connect(&config, NoTls).await?;

        // The connection object performs the actual communication with the database,
        // so spawn it off to run on its own.
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        println!("connection is succeed");
        Ok(client)
    }
}

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
