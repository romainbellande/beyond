use mongodb::{Client, Database};

pub struct DatabaseManager {
    url: String,
    name: String,
}

impl DatabaseManager {
    pub fn new(url: String, name: String) -> Self {
        Self { url, name }
    }

    pub async fn start(&self) -> Database {
        log::info!("connecting to database {} ...", &self.name);
        let client = Client::with_uri_str(&self.url).await.unwrap();
        log::info!("connection to database {} successfull!", &self.name);
        client.database(&self.name)
    }
}
