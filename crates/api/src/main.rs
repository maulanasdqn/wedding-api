mod app;
use app::create_routes;
use server::init::run;
use tokio;

#[tokio::main]
async fn main() {
	run(create_routes).await;
}
