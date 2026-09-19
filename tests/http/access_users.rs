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
