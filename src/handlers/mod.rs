pub mod home;
pub mod user;
use std::sync::Arc;

use axum::{response::IntoResponse, routing::get, Json, Router};
use tower_http::services::ServeDir;

use crate::AppState;


pub fn create_router(state: Arc<AppState>) -> Router {
    let router = Router::new()
        .route("/", get(home::index))
        .route("/healthcheck", get(health_check_handler))
        .with_state(state);

    router.nest_service("/static", ServeDir::new("static").append_index_html_on_directories(true))
}


async fn health_check_handler() -> impl IntoResponse {
    const MESSAGE: &str = "API Services";

    let json_response = serde_json::json!({
        "status": "ok",
        "message": MESSAGE
    });

    Json(json_response)
}
