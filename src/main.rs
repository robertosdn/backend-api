use backend_api::{app_with_repository, infrastructure::mysql::MySqlAccessUserRepository};
use sqlx::mysql::MySqlPoolOptions;
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
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "mysql://app:app@mysql:3306/app".to_owned());
    let pool = MySqlPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("failed to connect to MySQL");
    let repository = MySqlAccessUserRepository::new(pool);
    axum::serve(listener, app_with_repository(std::sync::Arc::new(repository)))
        .await
        .expect("HTTP server failed");
}
