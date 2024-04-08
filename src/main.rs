mod pufa;

use axum::{
    routing::get,
    Router,
    response::Json,
    http::StatusCode
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_pufa_word));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pufa_word() -> (StatusCode, Json<Value>) {
    match pufa::get_result().await {
        Ok(value) => (StatusCode::OK, Json(json!({
            "success": true,
            "data": value,
            "error_message": null
        }))),
        Err(value) => (StatusCode::SERVICE_UNAVAILABLE, Json(json!({
            "success": false,
            "data": null,
            "error_message": value.to_string()
        }))),
    }
}