use tokio_postgres::Client;

pub struct ClientState{
    pub client : Client
}