use mongodb::{Client, Database};
use std::env;

pub fn setup_logs() {
    std::env::set_var("RUST_LOG", "actix_web=warn");
    fast_log::init(fast_log::config::Config::new().console()).unwrap();
}

pub async fn setup_database() -> Database {
    let database_url = env::var("DATABASE_URL").expect("database url not set");
    let database_name = env::var("DATABASE_NAME").expect("database name not set");
    log::info!("connecting to database {} ...", &database_name);
    let client = Client::with_uri_str(database_url).await.unwrap();
    log::info!("connection to database {} successfull!", &database_name);
    client.database(&database_name)
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub db: Database,
}

pub async fn get_app_state() -> AppState {
    let db = setup_database().await.clone();

    AppState { db }
}
