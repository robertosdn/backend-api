use axum::{
    body::Bytes,
    extract::{Json, Path},
    http::{header, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, post},
    Router,
};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use serde::{Deserialize, Serialize};
use std::{
    sync::{Mutex, OnceLock},
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

#[derive(Debug, Deserialize)]
struct CreateAccessUserRequest {
    name: String,
    email: String,
    password: String,
}

#[derive(Debug, Serialize, Clone)]
struct AccessUserResponse {
    id: String,
    email: String,
    name: String,
    status: String,
    created_at: String,
    updated_at: String,
    version: u64,
}

#[derive(Debug, Clone)]
struct StoredAccessUser {
    id: String,
    email: String,
    name: String,
    password_hash: String,
    status: String,
    created_at: String,
    updated_at: String,
    version: u64,
}

fn now_iso8601() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
        .to_string()
}

fn is_valid_name(name: &str) -> bool {
    let trimmed = name.trim();
    !trimmed.is_empty() && trimmed.len() <= 255
}

fn normalize_email(email: &str) -> String {
    email.trim().to_ascii_lowercase()
}

fn is_valid_email(email: &str) -> bool {
    let value = normalize_email(email);
    let at = value.find('@');
    let dot = value.rfind('.');
    matches!((at, dot), (Some(at_idx), Some(dot_idx)) if at_idx > 0 && dot_idx > at_idx + 1 && dot_idx < value.len() - 1)
}

fn is_valid_password(password: &str) -> bool {
    let has_upper = password.chars().any(|ch| ch.is_ascii_uppercase());
    let has_lower = password.chars().any(|ch| ch.is_ascii_lowercase());
    let has_digit = password.chars().any(|ch| ch.is_ascii_digit());
    let has_symbol = password.chars().any(|ch| !ch.is_ascii_alphanumeric());
    password.len() >= 8 && has_upper && has_lower && has_digit && has_symbol
}

fn access_users_store() -> &'static Mutex<Vec<StoredAccessUser>> {
    static STORE: OnceLock<Mutex<Vec<StoredAccessUser>>> = OnceLock::new();
    STORE.get_or_init(|| Mutex::new(Vec::new()))
}

fn hash_password(password: &str) -> Result<String, String> {
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    argon2
        .hash_password(password.as_bytes(), &salt)
        .map(|hash| hash.to_string())
        .map_err(|_| "failed to hash password".to_owned())
}

fn verify_password(password: &str, password_hash: &str) -> Result<bool, String> {
    let parsed_hash = PasswordHash::new(password_hash).map_err(|_| "invalid password hash".to_owned())?;
    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok())
}

async fn create_access_user(Json(payload): Json<CreateAccessUserRequest>) -> Result<impl IntoResponse, (StatusCode, String)> {
    let name = payload.name.trim().to_string();
    if !is_valid_name(&name) {
        return Err((StatusCode::BAD_REQUEST, "invalid name".to_owned()));
    }

    let email = normalize_email(&payload.email);
    if !is_valid_email(&email) {
        return Err((StatusCode::BAD_REQUEST, "invalid email".to_owned()));
    }

    if !is_valid_password(&payload.password) {
        return Err((StatusCode::BAD_REQUEST, "password does not meet policy".to_owned()));
    }

    let mut users = access_users_store().lock().unwrap();
    if users.iter().any(|user| user.email == email) {
        return Err((StatusCode::CONFLICT, "email already registered".to_owned()));
    }

    let now = now_iso8601();
    let password_hash = match hash_password(&payload.password) {
        Ok(hash) => hash,
        Err(_) => return Err((StatusCode::INTERNAL_SERVER_ERROR, "failed to hash password".to_owned())),
    };
    let user = StoredAccessUser {
        id: Uuid::new_v4().to_string(),
        email: email.clone(),
        name: name.clone(),
        password_hash,
        status: "active".to_owned(),
        created_at: now.clone(),
        updated_at: now.clone(),
        version: 1,
    };

    users.push(user.clone());

    Ok((
        StatusCode::CREATED,
        [(header::CONTENT_TYPE, "application/json")],
        serde_json::to_string(&AccessUserResponse {
            id: user.id,
            email: user.email,
            name: user.name,
            status: user.status,
            created_at: user.created_at,
            updated_at: user.updated_at,
            version: user.version,
        }).expect("response serialization should succeed"),
    ))
}

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
        .route("/api/v1/access-users", post(create_access_user))
        .fallback(not_found)
}

#[cfg(test)]
mod tests;
