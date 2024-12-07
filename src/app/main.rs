use api::routes;
use server::init::run;
use tokio;

#[tokio::main]
async fn main() {
	run(routes).await;
}
