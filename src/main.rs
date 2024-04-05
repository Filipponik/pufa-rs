mod pufa;

use axum::{
    routing::get,
    Router,
    response::Json,
};
use serde_json::{Value, json};

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(get_pufa_word));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pufa_word() -> Json<Value> {
    Json(match pufa::get_result().await {
        Ok(value) => json!({
            "success": true,
            "data": value,
            "error_message": null
        }),
        Err(value) => json!({
            "success": false,
            "data": null,
            "error_message": value.to_string()
        }),
    })
}