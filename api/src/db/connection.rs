use tokio_postgres::NoTls;
use tokio_postgres::Client;
use tokio_postgres::Error;
use std::env;


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
    let (client, connection) =
        tokio_postgres::connect(&config, NoTls).await?;

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