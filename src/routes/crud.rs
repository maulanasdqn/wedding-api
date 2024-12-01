use crate::utils::format_response::{format_response, TResponse};
use crate::{models::Greeting, utils::format_response::TMeta};
use rocket::http::{ContentType, Status};
use rocket::response::status::Custom;
use rocket::response::Responder;
use rocket::serde::json::Json;
use rocket::Response;
use rocket::State;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use sqlx::{query, query_as, PgPool, Row};
use std::io::Cursor;
use validator::Validate;

type TRes = TResponse<Vec<Greeting>>;

#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i64>,
    pub per_page: Option<i64>,
}

#[derive(Deserialize, Serialize)]
pub struct ErrorResponse {
    pub message: String,
}

impl<'r> Responder<'r, 'static> for ErrorResponse {
    fn respond_to(self, _: &'r rocket::Request<'_>) -> rocket::response::Result<'static> {
        let json = to_string(&self).map_err(|_| Status::InternalServerError)?;
        Response::build()
            .header(ContentType::JSON)
            .sized_body(json.len(), Cursor::new(json))
            .ok()
    }
}

#[post("/api/greetings/create", data = "<greeting>")]
pub async fn create_greeting(
    conn: &State<PgPool>,
    greeting: Json<Greeting>,
) -> Result<Json<TRes>, Custom<ErrorResponse>> {
    greeting.validate().map_err(|e| {
        Custom(
            Status::BadRequest,
            ErrorResponse {
                message: format!("{}", e),
            },
        )
    })?;

    query(
        r#"
        INSERT INTO app_greetings (fullname, attendence, greeting, greeting_audio)
        VALUES ($1, $2, $3, $4)
        "#,
    )
    .bind(&greeting.fullname)
    .bind(&greeting.attendence)
    .bind(&greeting.greeting)
    .bind(&greeting.greeting_audio)
    .execute(&**conn)
    .await
    .map_err(|e| {
        Custom(
            Status::InternalServerError,
            ErrorResponse {
                message: format!("Database error: {}", e),
            },
        )
    })?;

    get_greetings(conn, None, None, None).await
}

#[get("/api/greetings?<page>&<per_page>&<search>")]
pub async fn get_greetings(
    conn: &State<PgPool>,
    page: Option<i64>,
    per_page: Option<i64>,
    search: Option<String>,
) -> Result<Json<TRes>, Custom<ErrorResponse>> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(10);
    let offset = (page - 1) * per_page;

    // Construct the base query for counting rows
    let count_query = if let Some(search) = &search {
        format!(
            "SELECT COUNT(*) FROM app_greetings WHERE fullname ILIKE '%{}%' OR attendence ILIKE '%{}%' OR greeting ILIKE '%{}%'",
            search, search, search
        )
    } else {
        "SELECT COUNT(*) FROM app_greetings".to_string()
    };

    let total: i64 = query(&count_query)
        .fetch_one(&**conn)
        .await
        .map(|row| row.get(0))
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                ErrorResponse {
                    message: format!("Database error: {}", e),
                },
            )
        })?;

    // Prepare metadata
    let meta = TMeta {
        page,
        per_page,
        total,
    };

    // Fetch paginated data with optional search
    let greetings = get_greetings_from_db(conn, per_page, offset, search).await?;
    let response = format_response(greetings, meta);

    Ok(Json(response))
}

async fn get_greetings_from_db(
    conn: &State<PgPool>,
    limit: i64,
    offset: i64,
    search: Option<String>,
) -> Result<Vec<Greeting>, Custom<ErrorResponse>> {
    // Adjust the query based on whether search is provided
    let base_query = if let Some(search) = &search {
        format!(
            r#"
            SELECT id, fullname, attendence, greeting, greeting_audio
            FROM app_greetings
            WHERE fullname ILIKE '%{}%' OR attendence ILIKE '%{}%' OR greeting ILIKE '%{}%'
            LIMIT $1 OFFSET $2
            "#,
            search, search, search
        )
    } else {
        r#"
        SELECT id, fullname, attendence, greeting, greeting_audio
        FROM app_greetings
        LIMIT $1 OFFSET $2
        "#
        .to_string()
    };

    query_as::<_, Greeting>(&base_query)
        .bind(limit)
        .bind(offset)
        .fetch_all(&**conn)
        .await
        .map_err(|e| {
            Custom(
                Status::InternalServerError,
                ErrorResponse {
                    message: format!("Database error: {}", e),
                },
            )
        })
}

#[put("/api/greetings/<id>/update", data = "<greeting>")]
pub async fn update_greeting(
    conn: &State<PgPool>,
    id: i32,
    greeting: Json<Greeting>,
) -> Result<Json<TRes>, Custom<ErrorResponse>> {
    greeting.validate().map_err(|e| {
        Custom(
            Status::BadRequest,
            ErrorResponse {
                message: format!("{}", e),
            },
        )
    })?;

    query(
        r#"
        UPDATE app_greetings
        SET fullname = $1, attendence = $2, greeting = $3, greeting_audio = $4
        WHERE id = $5
        "#,
    )
    .bind(&greeting.fullname)
    .bind(&greeting.attendence)
    .bind(&greeting.greeting)
    .bind(&greeting.greeting_audio)
    .bind(id)
    .execute(&**conn)
    .await
    .map_err(|e| {
        Custom(
            Status::InternalServerError,
            ErrorResponse {
                message: format!("Database error: {}", e),
            },
        )
    })?;
    get_greetings(conn, None, None, None).await
}

#[delete("/api/greetings/<id>/delete")]
pub async fn delete_greeting(conn: &State<PgPool>, id: i32) -> Result<Status, Custom<String>> {
    query(
        r#"
        DELETE FROM app_greetings
        WHERE id = $1
        "#,
    )
    .bind(id)
    .execute(&**conn)
    .await
    .map_err(|e| {
        Custom(
            Status::InternalServerError,
            format!("Database error: {}", e),
        )
    })?;
    Ok(Status::NoContent)
}
