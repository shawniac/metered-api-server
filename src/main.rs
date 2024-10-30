mod handlers;
mod models;
mod database;

use warp::Filter;
use sqlx::SqlitePool;

#[tokio::main]
async fn main() {
    let pool = database::initialize_database().await.expect("Failed to initialize database");

    let api_route = warp::path("api")
        .and(warp::post())
        .and(warp::body::json())
        .and(with_database(pool.clone()))
        .and_then(handlers::create_api_key);

    let protected_route = warp::path("protected")
        .and(warp::get())
        .and(warp::header("x-api-key"))
        .and(with_database(pool.clone()))
        .and_then(handlers::handle_api_request);

    let routes = api_route.or(protected_route);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

fn with_database(pool: SqlitePool) -> impl Filter<Extract = (SqlitePool,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
