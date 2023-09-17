use tokio_postgres::Client;

pub struct AppState{
    client : Client
}