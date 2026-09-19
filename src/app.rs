use axum::Router;

pub fn app() -> Router {
    crate::http::routes::router(crate::repositories::access_user_write_repository::InMemoryAccessUserRepository::new())
}

pub fn app_with_repository(repository: std::sync::Arc<dyn crate::repositories::access_user_write_repository::AccessUserWriteRepository>) -> Router {
    crate::http::routes::router(repository)
}
