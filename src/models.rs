use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: i64,
    pub key: String,
    pub email: String,
    pub usage_count: i64,
}

#[derive(Debug, Deserialize)]
pub struct CreateApiKey {
    pub email: String,
}