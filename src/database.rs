use dotenv::dotenv;
use std::env;
use tempfile::TempDir;

use crate::models::{ApiKey, CreateApiKey};
use sqlx::{sqlite::SqlitePool, Result};

pub async fn initialize_database() -> Result<SqlitePool> {
    dotenv().ok();

    let temp_dir = TempDir::new().expect("Failed to create temp dir");
    let temp_path = temp_dir.path().to_str().unwrap();

    let db_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .replace("{TEMP_DIR}", temp_path);

    println!("Using database: {}", db_url);

    let pool = SqlitePool::connect(&db_url).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS api_keys (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            key TEXT NOT NULL UNIQUE,
            email TEXT NOT NULL,
            usage_count INTEGER NOT NULL DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

pub async fn create_api_key(pool: &SqlitePool, create_key: CreateApiKey) -> Result<ApiKey> {
    let key = uuid::Uuid::new_v4().to_string();

    let api_key = sqlx::query_as!(
        ApiKey,
        "INSERT INTO api_keys (key, email) VALUES (?, ?) RETURNING *",
        key,
        create_key.email
    )
    .fetch_one(pool)
    .await?;

    Ok(api_key)
}

pub async fn increment_usage(pool: &SqlitePool, key: &str) -> Result<()> {
    sqlx::query!(
        "UPDATE api_keys SET usage_count = usage_count + 1 WHERE key = ?",
        key
    )
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_api_key(pool: &SqlitePool, key: &str) -> Result<Option<ApiKey>> {
    let api_key = sqlx::query_as!(ApiKey, "SELECT * FROM api_keys WHERE key = ?", key)
        .fetch_optional(pool)
        .await?;

    Ok(api_key)
}
