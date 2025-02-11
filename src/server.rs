use crate::pufa::rwlock_cache::RwLockCache;
use crate::use_case::get_word_query::{Handler, Query};
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use serde::Serialize;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Server error: {0}")]
    Server(#[from] std::io::Error),
}

pub async fn start() -> Result<(), Error> {
    let app = Router::new().route("/", get(get_pufa_word));
    let port: u16 = 3000;
    let addr = format!("0.0.0.0:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    info!("Server started at http://{addr}, connect from localhost at http://localhost:{port}");
    axum::serve(listener, app).await?;

    Ok(())
}

struct Response {
    status: StatusCode,
    body: ResponseBody,
}

impl Response {
    const fn new(status: StatusCode, body: ResponseBody) -> Self {
        Self { status, body }
    }

    fn to_axum_response(&self) -> (StatusCode, Json<ResponseBody>) {
        (self.status, Json(self.body.clone()))
    }
}

#[derive(Clone, Debug, Serialize)]
struct ResponseBody {
    success: bool,
    data: Option<String>,
    error_message: Option<String>,
    updated_at: Option<String>,
}

impl ResponseBody {
    fn new_success(word: &str, updated_at: &str) -> Self {
        Self {
            success: true,
            data: Some(word.to_string()),
            error_message: None,
            updated_at: Some(updated_at.to_string()),
        }
    }

    fn new_error(error_message: &str) -> Self {
        Self {
            success: false,
            data: None,
            error_message: Some(error_message.to_string()),
            updated_at: None,
        }
    }
}

async fn get_pufa_word() -> (StatusCode, Json<ResponseBody>) {
    let pufa_word = Handler::new(Query::new(60)).handle(RwLockCache).await;
    let response = match pufa_word {
        Err(error) => Response::new(
            StatusCode::SERVICE_UNAVAILABLE,
            ResponseBody::new_error(&error.to_string()),
        ),
        Ok(cache_state) => Response::new(
            StatusCode::SERVICE_UNAVAILABLE,
            ResponseBody::new_success(
                &cache_state.last_word,
                &cache_state.get_formatted_updated_at(),
            ),
        ),
    };

    response.to_axum_response()
}
