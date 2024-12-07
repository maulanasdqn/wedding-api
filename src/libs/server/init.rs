use std::net::SocketAddr;

use axum::{serve, Router};
use log::{error, info};
use tokio::net::TcpListener;

pub async fn run<F, Fut>(router_fn: F)
where
	F: Fn() -> Fut,
	Fut: std::future::Future<Output = Router>,
{
	let router = router_fn().await;
	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	let listener = TcpListener::bind(&addr).await.unwrap();
	info!("Listening on http://{}", addr);
	match serve(listener, router).await {
		Ok(_) => info!("Server stopped gracefully."),
		Err(err) => error!("Server encountered an error: {}", err),
	}
}
