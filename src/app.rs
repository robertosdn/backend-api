use axum::{
    body::Bytes,
    extract::Path,
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};

async fn hello() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], r#"{"message": "Hello, World!"}"#)
}

async fn hello_default() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], r#"{"message": "Hello, stranger!"}"#)
}

async fn hello_name(Path(name): Path<String>) -> Response {
    if name.is_empty() || name.contains('/') {
        return (StatusCode::NOT_FOUND, "Not Found\n").into_response();
    }

    let body = format!(r#"{{"message": "Hello, {name}!"}}"#);
    ([(header::CONTENT_TYPE, "application/json")], body).into_response()
}

async fn echo(body: Bytes) -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], body)
}

async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found\n")
}

pub fn app() -> Router {
    Router::new()
        .route("/hello", get(hello))
        .route("/hello/", get(hello_default))
        .route("/hello/{*name}", get(hello_name))
        .route("/echo", post(echo))
        .fallback(not_found)
}

#[cfg(test)]
mod tests;
