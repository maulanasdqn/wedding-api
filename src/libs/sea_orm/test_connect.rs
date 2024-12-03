#[cfg(test)]
mod tests {
    use sea_orm::{ConnectionTrait, DatabaseConnection, Statement};
    use std::env;

    use crate::libs::sea_orm::connect::connect_db;

    #[tokio::test]
    async fn test_db_connection() {
        let _database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set for tests");

        let pool: DatabaseConnection = connect_db()
            .await
            .expect("Failed to connect to the database");

        let statement = Statement::from_string(
            pool.get_database_backend(),
            "SELECT 1::BIGINT AS test_result".to_owned(),
        );

        let result = pool
            .query_one(statement)
            .await
            .expect("Failed to execute test query");

        if let Some(row) = result {
            let test_result: i64 = row.try_get("", "test_result").unwrap();
            assert_eq!(test_result, 1, "Test query returned an unexpected result");
        } else {
            panic!("Test query returned no result");
        }
    }
}
