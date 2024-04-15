mod pufa;

use axum::{http::StatusCode, response::Json, routing::get, Router};
use chrono::{DateTime, Utc};
use lazy_static::lazy_static;
use serde_json::{json, Value};
use tokio::sync::RwLock;

#[derive(Debug)]
struct AppState {
    just_started: bool,
    last_word: String,
    updated_at: DateTime<Utc>,
}

impl AppState {
    fn get_formatted_updated_at(&self) -> String {
        self.updated_at.to_rfc3339().to_string()
    }

    fn set_current_updated_at(&mut self) {
        self.updated_at = chrono::offset::Utc::now();
    }

    fn set_last_word(&mut self, new_word: &str) {
        self.just_started = false;
        self.last_word = new_word.to_string();
    }
}

lazy_static! {
    static ref STATE: RwLock<AppState> = RwLock::new(AppState {
        just_started: true,
        last_word: "".to_string(),
        updated_at: chrono::offset::Utc::now(),
    });
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(get_pufa_word));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn get_pufa_word() -> (StatusCode, Json<Value>) {
    let read_lock = STATE.read().await;
    let seconds_diff: i64 =
        chrono::offset::Utc::now().timestamp() - read_lock.updated_at.timestamp();

    if !&read_lock.just_started && seconds_diff < 60 {
        return (
            StatusCode::OK,
            Json(json!({
                "success": true,
                "data": &read_lock.last_word,
                "error_message": null,
                "updated_at": &read_lock.get_formatted_updated_at(),
            })),
        );
    }
    drop(read_lock);

    let pufa_word = pufa::get_result().await;
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
        Ok(word) => {
            let mut write_lock = STATE.write().await;
            write_lock.set_current_updated_at();
            write_lock.set_last_word(&word);
            write_lock.set_last_word(&word);

            (
                StatusCode::OK,
                Json(json!({
                    "success": true,
                    "data": word,
                    "error_message": null,
                    "updated_at": write_lock.get_formatted_updated_at(),
                })),
            )
        }
    }
}
