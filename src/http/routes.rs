use axum::{routing::{get, post}, Router};
use std::sync::Arc;

use crate::http::handlers::{self, AppState};
use crate::repositories::access_user_write_repository::AccessUserWriteRepository;

pub fn router(repository: Arc<dyn AccessUserWriteRepository>) -> Router {
    let state = AppState { access_user_repository: repository };
    Router::new()
        .route("/hello", get(handlers::hello))
        .route("/hello/", get(handlers::hello_default))
        .route("/hello/{*name}", get(handlers::hello_name))
        .route("/echo", post(handlers::echo))
        .route("/api/v1/access-users", post(handlers::create_access_user))
        .fallback(handlers::not_found)
        .with_state(state)
}
