use dotenv::{dotenv, Error};

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub host: String,
    pub port: i32,
    pub secret_key: String,
    pub database_url: String,
    pub redis_url: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            host: "".to_owned(),
            port: 0,
            secret_key: "".to_owned(),
            database_url: "".to_owned(),
            redis_url: "".to_owned(),
        }
    }

    pub fn from_env() -> Result<Config, Error> {
        dotenv().ok();

        let mut c = Config::new();

        c.host = std::env::var("HOST").expect("HOST must be set");
        c.port = std::env::var("PORT")
            .expect("PORT must be set")
            .parse::<i32>()
            .expect("PORT must be a number");
        c.secret_key = std::env::var("SECRET_KEY").expect("SECRET_KEY must be set");
        c.database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        c.redis_url = std::env::var("REDIS_URL").expect("REDIS_URL must be set");

        Ok(c)
    }
}
