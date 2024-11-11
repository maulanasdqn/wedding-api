use super::models;
use crate::{
    commons::structs::response::SuccessResponse,
    utils::format_response::{format_response, TMeta, TResponse},
};
use axum::response::Json;

pub async fn fetch_authors() -> Json<TResponse<Vec<models::Author>>> {
    let meta = TMeta {
        page: 1,
        per_page: 10,
        total: 50,
    };

    let authors = vec![
        models::Author {
            id: 1,
            name: "John Doe".to_string(),
        },
        models::Author {
            id: 2,
            name: "Jane Doe".to_string(),
        },
    ];

    let response = format_response(authors, meta);

    Json(response)
}

pub async fn fetch_author_detail(id: i32) -> Json<models::Author> {
    let author = models::Author {
        id,
        name: "John Doe".to_string(),
    };

    Json(author)
}

pub async fn create_author(new_author: models::Author) -> Json<SuccessResponse> {
    println!("New author: {:?}", new_author);
    let response = SuccessResponse {
        message: "Author created successfully".to_string(),
    };

    Json(response)
}
