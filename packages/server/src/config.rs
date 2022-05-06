use std::env;

pub struct Config {
    pub database_url: String,
    pub database_name: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME not set"),
        }
    }
}
