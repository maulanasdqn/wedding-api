use super::routes::create_routes;
use axum::Router;

pub async fn run() {
    let init: Router = create_routes();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, init).await.unwrap();
}
