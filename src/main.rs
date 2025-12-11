mod config;
mod errors;
mod routes;

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

type MeterStore = Arc<Mutex<HashMap<String, Vec<u64>>>>;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let config = config::Config::from_env();
    tracing::info!("Starting server on port {}", config.server_port);

    let meter_store: MeterStore = Arc::new(Mutex::new(HashMap::new()));

    let routes = routes::routes(meter_store);

    warp::serve(routes)
        .run(([127, 0, 0, 1], config.server_port))
        .await;

    Ok(())
}
