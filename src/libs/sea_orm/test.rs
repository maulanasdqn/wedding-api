#[cfg(test)]
mod tests {
    use dotenvy::dotenv;
    use sqlx::query_as;
    use std::env;

    use crate::libs::sea_orm::connect::connect_db;

    #[tokio::test]
    async fn test_db_connection() {
        dotenv().ok();
        let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let pool = connect_db()
            .await
            .expect("Failed to connect to the database");

        let row: (i64,) = query_as("SELECT 1::BIGINT")
            .fetch_one(&pool)
            .await
            .expect("Failed to execute test query");

        assert_eq!(row.0, 1, "Test query returned an unexpected result");
    }
}
