use warp::{Reply, Rejection};
use sqlx::SqlitePool;
use crate::models::{ApiKey, CreateApiKey};
use crate::database;

pub async fn create_api_key(
    create_key: CreateApiKey,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let api_key = database::create_api_key(&pool, create_key).await.map_err(|e| {
        eprintln!("Failed to create API key: {:?}", e);
        warp::reject::custom(ApiError::DatabaseError)
    })?;

    Ok(warp::reply::json(&api_key))
}

pub async fn handle_api_request(
    api_key: String,
    pool: SqlitePool,
) -> Result<impl Reply, Rejection> {
    let key = database::get_api_key(&pool, &api_key).await.map_err(|e| {
        eprintln!("Failed to get API key: {:?}", e);
        warp::reject::custom(ApiError::DatabaseError)
    })?;

    if let Some(key) = key {
        database::increment_usage(&pool, &api_key).await.map_err(|e| {
            eprintln!("Failed to increment usage: {:?}", e);
            warp::reject::custom(ApiError::DatabaseError)
        })?;

        Ok(warp::reply::json(&"API request successful"))
    } else {
        Err(warp::reject::custom(ApiError::InvalidApiKey))
    }
}

#[derive(Debug)]
pub enum ApiError {
    DatabaseError,
    InvalidApiKey,
}

impl warp::reject::Reject for ApiError {}