use super::app;
use axum::{body::Body, http::{Method, Request, StatusCode}, Router};
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn request(router: Router, method: Method, uri: &str, body: Body) -> (StatusCode, String) {
    let request = Request::builder()
        .method(method)
        .uri(uri)
        .body(body)
        .unwrap();
    let response = router.oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[tokio::test]
async fn serves_expected_routes() {
    let (status, body) = request(app(), Method::GET, "/hello", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, World!"}"#);

    let (status, body) = request(app(), Method::GET, "/hello/Roberto", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, Roberto!"}"#);

    let (status, body) = request(app(), Method::GET, "/hello/", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, stranger!"}"#);

    let (status, _) = request(app(), Method::GET, "/hello/with/slashes", Body::empty()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn echoes_body_and_rejects_unknown_routes() {
    let payload = r#"{"test":123,"nested":{"a":1}}"#;
    let (status, body) = request(app(), Method::POST, "/echo", Body::from(payload)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, payload);

    let (status, body) = request(app(), Method::GET, "/nonexistent", Body::empty()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "Not Found\n");
}

#[tokio::test]
async fn creates_access_user_with_valid_data_and_hides_password_hash() {
    let payload = r#"{"name":"Alice Silva","email":"alice@example.com","password":"S3cret!Pass123"}"#;
    let request = Request::builder()
        .method(Method::POST)
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
    assert!(body.contains("\"email\":\"alice@example.com\""));
    assert!(body.contains("\"name\":\"Alice Silva\""));
    assert!(!body.contains("password_hash"));
    assert!(!body.contains("S3cret!Pass123"));
}
