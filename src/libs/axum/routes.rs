use axum::Router;

use crate::{
    app::reservations::controllers::reservation_controller, commons::constants::endpoints,
};

pub fn create_routes() -> Router {
    Router::new().nest(
        endpoints::ROOT,
        Router::new().nest(endpoints::RESERVATIONS, reservation_controller()),
    )
}
