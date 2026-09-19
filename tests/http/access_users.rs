use axum::{body::Body, http::{Request, StatusCode}};
use backend_api::app;
use http_body_util::BodyExt;
use tower::ServiceExt;

#[path = "../fixtures/mod.rs"]
mod fixtures;

#[tokio::test]
async fn creates_access_user_with_valid_data_and_hides_password_hash() {
    let payload = format!(
        r#"{{"name":"Alice Silva","email":"{}","password":"{}"}}"#,
        fixtures::VALID_EMAIL,
        fixtures::VALID_PASSWORD,
    );
    let request = Request::builder()
        .method("POST")
        .uri("/api/v1/access-users")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .unwrap();
    let response = app().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let body = String::from_utf8(body.to_vec()).unwrap();

    assert_eq!(status, StatusCode::CREATED, "body: {body}");
    assert!(body.contains("\"id\""));
    assert!(body.contains("\"email\":\"user@example.com\""));
    assert!(body.contains("\"name\":\"Alice Silva\""));
    assert!(!body.contains("password_hash"));
    assert!(!body.contains(fixtures::VALID_PASSWORD));
}

#[tokio::test]
async fn rejects_duplicate_email_and_invalid_payloads() {
    let router = app();
    let payload = format!(
        r#"{{"name":"Alice Silva","email":"{}","password":"{}"}}"#,
        fixtures::VALID_EMAIL,
        fixtures::VALID_PASSWORD,
    );

    let first = Request::builder()
        .method("POST")
        .uri("/api/v1/access-users")
        .header("content-type", "application/json")
        .body(Body::from(payload.clone()))
        .unwrap();
    assert_eq!(router.clone().oneshot(first).await.unwrap().status(), StatusCode::CREATED);

    let duplicate = Request::builder()
        .method("POST")
        .uri("/api/v1/access-users")
        .header("content-type", "application/json")
        .body(Body::from(payload))
        .unwrap();
    assert_eq!(router.clone().oneshot(duplicate).await.unwrap().status(), StatusCode::CONFLICT);

    let invalid = Request::builder()
        .method("POST")
        .uri("/api/v1/access-users")
        .header("content-type", "application/json")
        .body(Body::from(r#"{"name":"","email":"bad","password":"weak"}"#))
        .unwrap();
    assert_eq!(router.oneshot(invalid).await.unwrap().status(), StatusCode::BAD_REQUEST);
}
