use axum::{body::Body, http::{Method, Request, StatusCode}};
use backend_api::app;
use http_body_util::BodyExt;
use tower::ServiceExt;

async fn request(method: Method, uri: &str, body: Body) -> (StatusCode, String) {
    let request = Request::builder().method(method).uri(uri).body(body).unwrap();
    let response = app().oneshot(request).await.unwrap();
    let status = response.status();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    (status, String::from_utf8(body.to_vec()).unwrap())
}

#[tokio::test]
async fn serves_expected_routes() {
    let (status, body) = request(Method::GET, "/hello", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, World!"}"#);

    let (status, body) = request(Method::GET, "/hello/Roberto", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, Roberto!"}"#);

    let (status, body) = request(Method::GET, "/hello/", Body::empty()).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, r#"{"message": "Hello, stranger!"}"#);
}

#[tokio::test]
async fn echoes_body_and_rejects_unknown_routes() {
    let payload = r#"{"test":123,"nested":{"a":1}}"#;
    let (status, body) = request(Method::POST, "/echo", Body::from(payload)).await;
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, payload);

    let (status, body) = request(Method::GET, "/nonexistent", Body::empty()).await;
    assert_eq!(status, StatusCode::NOT_FOUND);
    assert_eq!(body, "Not Found\n");
}
