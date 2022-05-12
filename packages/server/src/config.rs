use std::env;

pub struct Config {
    pub database_url: String,
    pub database_name: String,
    pub jwt_secret: String,
    pub jwt_exp: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL not set"),
            database_name: env::var("DATABASE_NAME").expect("DATABASE_NAME not set"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET not set"),
            jwt_exp: env::var("JWT_EXP")
                .expect("JWT_EXP not set")
                .parse::<usize>()
                .expect("JWT_EXP is not a valid UTC timestamp"),
        }
    }
}
