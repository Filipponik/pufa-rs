use crate::pufa::cache::Cacheable;
use crate::pufa::rwlock_cache::RwLockCache;
use crate::use_case::{get_actual_word_query, get_cached_word_query};
use axum::extract::State;
use axum::http::StatusCode;
use axum::routing::get;
use axum::{Json, Router};
use chrono::Utc;
use serde::Serialize;
use std::sync::Arc;
use thiserror::Error;
use tracing::info;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Server error: {0}")]
    Server(#[from] std::io::Error),
}

#[derive(Clone)]
struct AppState<T: Cacheable + Clone> {
    cache_driver: T,
}

pub async fn start() -> Result<(), Error> {
    let state = Arc::new(AppState {
        cache_driver: RwLockCache,
    });

    let app = Router::new()
        .route("/", get(get_cached_pufa_word))
        .route("/actual", get(get_actual_pufa_word))
        .with_state(state);

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

async fn get_cached_pufa_word(
    State(state): State<Arc<AppState<RwLockCache>>>,
) -> (StatusCode, Json<ResponseBody>) {
    let query = get_cached_word_query::Query::new(60);
    let handler = get_cached_word_query::Handler::new(query);
    let pufa_word = handler.handle(state.cache_driver.clone()).await;
    let response = match pufa_word {
        Err(error) => Response::new(
            StatusCode::SERVICE_UNAVAILABLE,
            ResponseBody::new_error(&error.to_string()),
        ),
        Ok(cache_state) => Response::new(
            StatusCode::OK,
            ResponseBody::new_success(
                &cache_state.last_word,
                &cache_state.get_formatted_updated_at(),
            ),
        ),
    };

    response.to_axum_response()
}

async fn get_actual_pufa_word() -> (StatusCode, Json<ResponseBody>) {
    let query = get_actual_word_query::Query;
    let handler = get_actual_word_query::Handler::new(query);
    let pufa_word = handler.handle().await;
    let response = match pufa_word {
        Err(error) => Response::new(
            StatusCode::SERVICE_UNAVAILABLE,
            ResponseBody::new_error(&error.to_string()),
        ),
        Ok(cache_state) => Response::new(
            StatusCode::OK,
            ResponseBody::new_success(&cache_state, &Utc::now().to_rfc3339()),
        ),
    };

    response.to_axum_response()
}
