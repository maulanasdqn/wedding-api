use sqlx::{query, PgPool};
use std::env;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Connect to the database
    let pool = PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to the database");

    // Seed data
    let seed_data = vec![
        ("John Doe", "Present", "Welcome, John!", "john_audio.mp3"),
        (
            "Jane Smith",
            "Absent",
            "We missed you, Jane!",
            "jane_audio.mp3",
        ),
        (
            "Alice Brown",
            "Present",
            "Good to see you, Alice!",
            "alice_audio.mp3",
        ),
        ("Bob White", "Present", "Hello, Bob!", "bob_audio.mp3"),
    ];

    for (fullname, attendence, greeting, greeting_audio) in seed_data {
        query(
            r#"
            INSERT INTO app_greetings (fullname, attendence, greeting, greeting_audio)
            VALUES ($1, $2, $3, $4)
            "#,
        )
        .bind(fullname)
        .bind(attendence)
        .bind(greeting)
        .bind(greeting_audio)
        .execute(&pool)
        .await
        .expect("Failed to insert seed data");
    }

    println!("Database seeding completed!");
}
