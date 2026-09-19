use std::sync::Arc;

use axum::{body::Bytes, extract::{Json, Path, State}, http::{header, StatusCode}, response::{IntoResponse, Response}};

use crate::{application::commands::create_access_user::{CreateAccessUser, CreateAccessUserError, CreateAccessUserInput}, http::dto::{AccessUserResponse, CreateAccessUserRequest}, repositories::access_user_write_repository::AccessUserWriteRepository};

#[derive(Clone)]
pub struct AppState {
    pub access_user_repository: Arc<dyn AccessUserWriteRepository>,
}

pub async fn create_access_user(State(state): State<AppState>, Json(payload): Json<CreateAccessUserRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let command = CreateAccessUser::new(state.access_user_repository);
    let user = command.execute(CreateAccessUserInput { name: payload.name, email: payload.email, password: payload.password }).await.map_err(map_error)?;
    Ok((StatusCode::CREATED, [(header::CONTENT_TYPE, "application/json")], Json(AccessUserResponse::from(&user))))
}

fn map_error(error: CreateAccessUserError) -> (StatusCode, String) {
    match error {
        CreateAccessUserError::Validation(message) => (StatusCode::BAD_REQUEST, message.to_owned()),
        CreateAccessUserError::DuplicateEmail => (StatusCode::CONFLICT, "email already registered".to_owned()),
        CreateAccessUserError::Storage => (StatusCode::INTERNAL_SERVER_ERROR, "failed to store access user".to_owned()),
    }
}

pub async fn hello() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], r#"{"message": "Hello, World!"}"#)
}

pub async fn hello_default() -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], r#"{"message": "Hello, stranger!"}"#)
}

pub async fn hello_name(Path(name): Path<String>) -> Response {
    if name.is_empty() || name.contains('/') {
        return (StatusCode::NOT_FOUND, "Not Found\n").into_response();
    }
    let body = format!(r#"{{"message": "Hello, {name}!"}}"#);
    ([(header::CONTENT_TYPE, "application/json")], body).into_response()
}

pub async fn echo(body: Bytes) -> impl IntoResponse {
    ([(header::CONTENT_TYPE, "application/json")], body)
}

pub async fn not_found() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found\n")
}
