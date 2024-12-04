pub mod schemas;
use log::{error, info};
use sea_orm::{Database, DatabaseConnection};
use std::env;

pub async fn get_db() -> DatabaseConnection {
	let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

	match Database::connect(database_url).await {
		Ok(db_connection) => {
			info!("Successfully connected to the database.");
			db_connection
		}
		Err(err) => {
			error!("Failed to connect to the database: {}", err);
			panic!("Database connection failed: {}", err);
		}
	}
}
