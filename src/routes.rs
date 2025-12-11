use warp::Filter;
use std::sync::Arc;
use tokio::sync::Mutex;
use std::collections::HashMap;

use crate::errors::AppError;

type MeterStore = Arc<Mutex<HashMap<String, Vec<u64>>>>;

pub fn routes(store: MeterStore) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    let health = warp::path("health")
        .and(warp::get())
        .map(|| warp::reply::json(&serde_json::json!({"status": "ok"})));

    let api_data = warp::path!("api" / "data")
        .and(warp::post())
        .and(warp::header::optional("x-api-key"))
        .and(with_meter(store.clone()))
        .map(|api_key: Option<String>, _| {
            warp::reply::json(&serde_json::json!({
                "message": "Data processed",
                "api_key": api_key
            }))
        });

    let metrics = warp::path("metrics")
        .and(warp::get())
        .and(with_meter(store.clone()))
        .and_then(move |_| {
            let store = store.clone();
            async move {
                let data = store.lock().await;
                let counts: HashMap<String, usize> = data.iter().map(|(k, v)| (k.clone(), v.len())).collect();
                Ok::<_, warp::Rejection>(warp::reply::json(&counts))
            }
        });

    health.or(api_data).or(metrics).recover(handle_rejection)
}

fn with_meter(store: MeterStore) -> impl Filter<Extract = (), Error = warp::Rejection> + Clone {
    warp::header::optional("x-api-key")
        .map(move |api_key: Option<String>| {
            let store = store.clone();
            async move {
                let key = api_key.unwrap_or_else(|| "anonymous".to_string());
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                let mut data = store.lock().await;
                let timestamps = data.entry(key).or_insert(Vec::new());
                // Remove timestamps older than 60 seconds
#[cfg(test)]
mod tests {
    use super::*;
    use warp::test::request;
    use std::collections::HashMap;
    use std::sync::Arc;
    use tokio::sync::Mutex;

    #[tokio::test]
    async fn test_health() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
        let routes = routes(store);
        let resp = request().method("GET").path("/health").reply(&routes).await;
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["status"], "ok");
    }

    #[tokio::test]
    async fn test_api_data() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
    #[tokio::test]
    async fn test_api_data_anonymous() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
        let routes = routes(store);
        let resp = request()
            .method("POST")
            .path("/api/data")
            .reply(&routes)
            .await;
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Data processed");
        assert_eq!(body["api_key"], serde_json::Value::Null);
    }

    #[tokio::test]
    async fn test_metrics() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
        {
            let mut data = store.lock().await;
            data.insert("test-key".to_string(), vec![1, 2, 3]);
        }
        let routes = routes(store);
        let resp = request().method("GET").path("/metrics").reply(&routes).await;
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["test-key"], 3);
    }

    #[tokio::test]
    async fn test_rate_limiting() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
        let routes = routes(store);
        // Make 10 requests
        for _ in 0..10 {
            let resp = request()
                .method("POST")
                .path("/api/data")
                .header("x-api-key", "rate-test")
                .reply(&routes)
    #[tokio::test]
    async fn test_not_found() {
        let store: MeterStore = Arc::new(Mutex::new(HashMap::new()));
        let routes = routes(store);
        let resp = request().method("GET").path("/nonexistent").reply(&routes).await;
        assert_eq!(resp.status(), 404);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert!(body["error"].is_string());
    }
                .await;
            assert_eq!(resp.status(), 200);
        }
        // 11th should be 429
        let resp = request()
            .method("POST")
            .path("/api/data")
            .header("x-api-key", "rate-test")
            .reply(&routes)
            .await;
        assert_eq!(resp.status(), 429);
    }
        let routes = routes(store);
        let resp = request()
            .method("POST")
            .path("/api/data")
            .header("x-api-key", "test-key")
            .reply(&routes)
            .await;
        assert_eq!(resp.status(), 200);
        let body: serde_json::Value = serde_json::from_slice(resp.body()).unwrap();
        assert_eq!(body["message"], "Data processed");
        assert_eq!(body["api_key"], "test-key");
    }
}
                timestamps.retain(|&t| now.saturating_sub(t) < 60);
                // Check rate limit: 10 requests per minute
                if timestamps.len() >= 10 {
                    return Err(warp::reject::custom(AppError::RateLimitExceeded));
                }
                timestamps.push(now);
                Ok(())
            }
        })
        .and_then(|result| async { result })
}

async fn handle_rejection(err: warp::Rejection) -> Result<impl warp::Reply, std::convert::Infallible> {
    if let Some(app_err) = err.find::<AppError>() {
        Ok(app_err.into_response())
    } else {
        Ok(warp::reply::with_status(
            warp::reply::json(&serde_json::json!({"error": "Internal server error"})),
            warp::http::StatusCode::INTERNAL_SERVER_ERROR,
        ))
    }
}