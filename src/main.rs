#[macro_use]
extern crate rocket;

pub mod models;
pub mod routes;
pub mod utils;

use rocket_cors::{AllowedOrigins, CorsOptions};
use routes::crud as crud_routes;
use routes::upload_audio as upload_audio_routes;
use sqlx::postgres::PgPoolOptions;
use sqlx::Executor;
use std::env;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to Postgres");

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS app_greetings (
            id SERIAL PRIMARY KEY,
            fullname TEXT NOT NULL,
            attendence TEXT NOT NULL,
            greeting TEXT NOT NULL,
            greeting_audio TEXT
        )
        "#,
    )
    .await
    .expect("Failed to create app_greetings table");

    pool.execute(
        r#"
        CREATE TABLE IF NOT EXISTS app_greetings_audio_logs (
            id SERIAL PRIMARY KEY,
            audio_url TEXT NOT NULL,
            uploaded_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
        "#,
    )
    .await
    .expect("Failed to create app_greetings_audio_logs table");

    let cors = CorsOptions::default()
        .allowed_origins(AllowedOrigins::all())
        .to_cors()
        .expect("Error while building CORS");

    rocket::build()
        .manage(pool)
        .mount(
            "/",
            routes![
                crud_routes::create_greeting,
                crud_routes::get_greetings,
                crud_routes::update_greeting,
                crud_routes::delete_greeting,
                upload_audio_routes::upload_audio
            ],
        )
        .attach(cors)
}
