use axum::Router;

pub fn app() -> Router {
    crate::http::routes::router()
}
