mod app;
mod commons;
mod libs;
mod utils;
use libs::axum::init::run;
use tokio;

#[tokio::main]
async fn main() {
    run().await;
}
