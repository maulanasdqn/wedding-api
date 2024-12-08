use api::routes;
use server::init;
use tokio;

#[tokio::main]
async fn main() {
	init::run(routes).await;
}
