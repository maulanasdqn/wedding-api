use axum::{serve, Router};
use std::{future::Future, net::SocketAddr};
use tokio::net::TcpListener;

pub async fn run<F, Fut>(router_fn: F)
where
	F: Fn() -> Fut,
	Fut: Future<Output = Router>,
{
	let router = router_fn().await;
	let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
	let listener = TcpListener::bind(&addr).await.unwrap();
	print!("Listening on http://{}", addr);
	match serve(listener, router).await {
		Ok(_) => print!("Server stopped gracefully."),
		Err(err) => print!("Server encountered an error: {}", err),
	}
}
