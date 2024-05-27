mod pufa;

use axum::{http::StatusCode, response::Json, routing::get, Router};
use serde_json::{json, Value};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_pufa_word));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pufa_word() -> (StatusCode, Json<Value>) {
    let pufa_word = pufa::get_cached_result().await;
    match pufa_word {
        Err(error) => (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({
                "success": false,
                "data": null,
                "error_message": error.to_string(),
                "updated_at": null,
            })),
        ),
        Ok(cache_state) => (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": cache_state.last_word,
                "error_message": null,
                "updated_at": cache_state.get_formatted_updated_at(),
            })),
        ),
    }
}
