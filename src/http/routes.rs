use axum::{routing::{get, post}, Router};

use crate::http::handlers::{self, AppState};

pub fn router() -> Router {
    let state = AppState { access_user_repository: crate::repositories::access_user_write_repository::InMemoryAccessUserRepository::new() };
    Router::new()
        .route("/hello", get(handlers::hello))
        .route("/hello/", get(handlers::hello_default))
        .route("/hello/{*name}", get(handlers::hello_name))
        .route("/echo", post(handlers::echo))
        .route("/api/v1/access-users", post(handlers::create_access_user))
        .fallback(handlers::not_found)
        .with_state(state)
}
