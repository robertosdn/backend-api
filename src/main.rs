use rest_api_rust::app;
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let address = std::env::var("LISTEN_ADDR")
        .unwrap_or_else(|_| "0.0.0.0:8080".to_owned())
        .parse::<SocketAddr>()
        .expect("LISTEN_ADDR must be a valid socket address");

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind HTTP listener");
    println!("REST server listening on http://{address}");
    axum::serve(listener, app()).await.expect("HTTP server failed");
}
