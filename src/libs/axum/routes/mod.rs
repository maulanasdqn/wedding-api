use crate::{
    app::authors::controllers::author_controller, commons::constants::endpoints::endpoints,
};
use axum::Router;

pub fn create_routes() -> Router {
    Router::new().nest(
        endpoints::ROOT,
        Router::new().nest(endpoints::AUTHORS, author_controller()),
    )
}
