use axum::{response::Html, routing::get, Router};

pub async fn docs_router() -> Router {
	Router::new()
		.route(
			"/docs/openapi",
			get(|| async { include_str!("openapi.json") }),
		)
		.route(
			"/docs",
			get(|| async { Html(swagger_ui("/api/docs/openapi")) }),
		)
}

pub fn swagger_ui(spec_url: &'static str) -> String {
	format!(
		r#"<html>
            <head>
                <title>Swagger UI</title>
                <link rel="stylesheet" href="https://unpkg.com/swagger-ui-dist@4.4.0/swagger-ui.css" />
            </head>
            <body>
                <div id="swagger-ui"></div>
                <script src="https://unpkg.com/swagger-ui-dist@4.4.0/swagger-ui-bundle.js" charset="UTF-8"></script>
                <script>
                    window.onload = () => {{
                        window.ui = SwaggerUIBundle({{
                            url: '{}',
                            dom_id: '#swagger-ui',
                            deepLinking: true,
                            presets: [
                                SwaggerUIBundle.presets.apis,
                                SwaggerUIBundle.SwaggerUIStandalonePreset
                            ],
                            plugins: [
                                SwaggerUIBundle.plugins.DownloadUrl
                            ],
                        }})
                    }}
                </script>
            </body>
        </html>"#,
		spec_url
	)
}
