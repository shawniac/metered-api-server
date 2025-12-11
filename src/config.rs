use std::env;

#[derive(Debug)]
pub struct Config {
    pub database_url: String,
    pub server_port: u16,
}

impl Config {
    pub fn from_env() -> Self {
        dotenv::dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .unwrap_or_else(|_| "sqlite://database.db".to_string());
        let server_port = env::var("SERVER_PORT")
            .unwrap_or_else(|_| "3030".to_string())
            .parse()
            .unwrap_or(3030);

        Config {
            database_url,
            server_port,
        }
    }
}