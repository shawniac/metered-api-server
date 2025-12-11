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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_from_env() {
        std::env::set_var("DATABASE_URL", "sqlite://test.db");
        std::env::set_var("SERVER_PORT", "8080");
        let config = Config::from_env();
        assert_eq!(config.database_url, "sqlite://test.db");
        assert_eq!(config.server_port, 8080);
        // Clean up
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("SERVER_PORT");
    }

    #[test]
    fn test_config_defaults() {
        // Ensure vars are not set
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("SERVER_PORT");
        let config = Config::from_env();
        assert_eq!(config.database_url, "sqlite://database.db");
        assert_eq!(config.server_port, 3030);
    }
}
        Config {
            database_url,
            server_port,
        }
    }
}