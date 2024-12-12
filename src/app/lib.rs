mod auth;
mod reservations;

use auth::{auth_middleware::authorization_middleware, auth_router};
use axum::{
	http::{header, HeaderValue, Method},
	middleware::from_fn,
	response::Redirect,
	routing::get,
	Router,
};
use reservations::reservation_router;
use std::env;
use storage::upload_routes;
use tower_http::cors::CorsLayer;
use utoipa::{
	openapi::security::{Http, HttpAuthScheme, SecurityScheme},
	Modify, OpenApi,
};
use utoipa_rapidoc::RapiDoc;
use utoipa_swagger_ui::SwaggerUi;

pub async fn routes() -> Router {
	#[derive(OpenApi)]
	#[openapi(
        paths(
            auth::auth_controller::post_login,
            auth::auth_controller::post_refresh,
            reservations::reservation_controller::create,
            reservations::reservation_controller::get_all
        ),
        components(
            schemas(
                auth::auth_dto::AuthDto,
                auth::auth_dto::AuthResponse,
                auth::auth_dto::MessageResponse,
                reservations::reservation_dto::ReservationDto,
                reservations::reservation_dto::ReservationListResponse,
                reservations::reservation_dto::TMetas
            )
        ),
        info(
            title = "Wedding API",
            description = "Wedding API Documentation",
            version = "0.1.0",
            contact(
                name = "Maulana Sodiqin",
                url = ""
            ),
            license(
                name = "MIT",
                url = "https://opensource.org/licenses/MIT",
            )
        ),
        modifiers(&SecurityAddon),
        tags(
            (name = "Authentication", description = "Authentication Endpoint"),
            (name = "Reservations", description = "Reservations Endpoint")
        )
    )]
	struct ApiDoc;

	struct SecurityAddon;

	impl Modify for SecurityAddon {
		fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
			if let Some(components) = openapi.components.as_mut() {
				components.add_security_scheme(
					"Bearer",
					SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
				);
			}
		}
	}

	let cors_origins = match env::var("RUST_ENV").as_deref() {
		Ok("development") => vec!["http://localhost:5173"],
		Ok("production") => {
			vec!["https://wedding.fenny.me", "https://wedding.msdqn.dev"]
		}
		_ => vec!["https://wedding.fenny.me"],
	};

	let allowed_origins: Vec<HeaderValue> = cors_origins
		.into_iter()
		.filter_map(|origin| origin.parse::<HeaderValue>().ok())
		.collect();

	let cors_middleware = CorsLayer::new()
		.allow_origin(allowed_origins)
		.allow_methods([Method::GET, Method::POST, Method::OPTIONS])
		.allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE])
		.allow_credentials(true);

	Router::new()
		.route("/", get(|| async { Redirect::temporary("/api/docs") }))
		.nest(
			"/api",
			Router::new()
				.nest("", protected_routes().await)
				.nest("/auth", auth_router()),
		)
		.merge(
			SwaggerUi::new("/api/docs")
				.url("/api-docs/openapi.json", ApiDoc::openapi()),
		)
		.merge(RapiDoc::new("/api-docs/openapi.json").path("/api/rapidoc"))
		.layer(cors_middleware)
}

async fn protected_routes() -> Router {
	Router::new()
		.nest("", upload_routes().await)
		.nest("/reservations", reservation_router())
		.layer(from_fn(authorization_middleware))
}
