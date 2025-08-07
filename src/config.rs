use serde::Deserialize;
use std::env;
use dotenv::dotenv;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub requests_per_minute: u64,
    pub requests_per_hour: u64,
    pub requests_per_day: u64,
    pub server_addr: String,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv().ok();

        let requests_per_minute = env::var("REQUESTS_PER_MINUTE")
            .unwrap_or_else(|_| "5".to_string())
            .parse()
            .expect("Failed to parse REQUESTS_PER_MINUTE");

        let requests_per_hour = env::var("REQUESTS_PER_HOUR")
            .unwrap_or_else(|_| "100".to_string())
            .parse()
            .expect("Failed to parse REQUESTS_PER_HOUR");

        let requests_per_day = env::var("REQUESTS_PER_DAY")
            .unwrap_or_else(|_| "1000".to_string())
            .parse()
            .expect("Failed to parse REQUESTS_PER_DAY");

        let server_addr = env::var("SERVER_ADDR")
            .unwrap_or_else(|_| "127.0.0.1:3000".to_string());

        Self {
            requests_per_minute,
            requests_per_hour,
            requests_per_day,
            server_addr,
        }
    }
}
