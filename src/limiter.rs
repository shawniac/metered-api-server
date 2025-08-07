use axum::{
    extract::{Request, State},
    http::{HeaderMap, StatusCode},
    middleware::Next,
    response::Response,
};
use chrono::{DateTime, Duration, Utc};
use dashmap::DashMap;
use std::sync::Arc;
use crate::config::Config;

pub type ApiToken = String;
pub type RouteId = String;

#[derive(Clone)]
pub struct RateLimitCounters {
    pub minute_requests: u64,
    pub hour_requests: u64,
    pub day_requests: u64,
    pub last_request_time: DateTime<Utc>,
}

impl RateLimitCounters {
    pub fn new() -> Self {
        Self {
            minute_requests: 0,
            hour_requests: 0,
            day_requests: 0,
            last_request_time: Utc::now(),
        }
    }
}

pub struct AppState {
    pub limiter: RateLimiter,
    pub config: Config,
}

pub struct RateLimiter {
    pub storage: Arc<DashMap<ApiToken, DashMap<RouteId, RateLimitCounters>>>,
}

impl RateLimiter {
    pub fn new() -> Self {
        Self {
            storage: Arc::new(DashMap::new()),
        }
    }
}

pub async fn rate_limit_middleware(
    State(state): State<Arc<AppState>>,
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {
    let headers = req.headers();
    let token = extract_token(headers)?;
    let route_id = get_route_id(&req)?;

    let mut counters = state
        .limiter
        .storage
        .entry(token.clone())
        .or_insert_with(DashMap::new)
        .entry(route_id)
        .or_insert_with(RateLimitCounters::new);

    let now = Utc::now();
    let time_since_last_request = now.signed_duration_since(counters.last_request_time);

    // Reset counters based on time windows
    if time_since_last_request > Duration::days(1) {
        counters.day_requests = 0;
        counters.hour_requests = 0;
        counters.minute_requests = 0;
    } else if time_since_last_request > Duration::hours(1) {
        counters.hour_requests = 0;
        counters.minute_requests = 0;
    } else if time_since_last_request > Duration::minutes(1) {
        counters.minute_requests = 0;
    }

    // Check rate limits
    if counters.minute_requests >= state.config.requests_per_minute
        || counters.hour_requests >= state.config.requests_per_hour
        || counters.day_requests >= state.config.requests_per_day
    {
        return Err(StatusCode::TOO_MANY_REQUESTS);
    }

    // Increment counters
    counters.minute_requests += 1;
    counters.hour_requests += 1;
    counters.day_requests += 1;
    counters.last_request_time = now;

    Ok(next.run(req).await)
}

fn extract_token(headers: &HeaderMap) -> Result<ApiToken, StatusCode> {
    headers
        .get("Authorization")
        .and_then(|header| header.to_str().ok())
        .and_then(|header| header.strip_prefix("Bearer "))
        .map(|token| token.to_string())
        .ok_or(StatusCode::UNAUTHORIZED)
}

fn get_route_id(req: &Request) -> Result<RouteId, StatusCode> {
    let path = req.uri().path();
    path.strip_prefix('/')
        .map(|s| s.to_string())
        .ok_or(StatusCode::INTERNAL_SERVER_ERROR)
}
