use std::sync::Arc;

use backend_api::{application::commands::create_access_user::{CreateAccessUser, CreateAccessUserInput}, repositories::access_user_write_repository::InMemoryAccessUserRepository};

#[test]
fn creates_user_through_command_and_records_outbox_event() {
    let repository = InMemoryAccessUserRepository::new();
    let command = CreateAccessUser::new(Arc::clone(&repository) as Arc<_>);
    let user = command.execute(CreateAccessUserInput { name: "Alice".to_owned(), email: "alice@example.com".to_owned(), password: "S3cret!Pass123".to_owned() }).unwrap();
    assert_eq!(user.email.as_str(), "alice@example.com");
    assert!(user.password_hash.starts_with("$argon2id$"));
    assert_eq!(repository.user_count(), 1);
    assert_eq!(repository.outbox_count(), 1);
}

#[test]
fn rejects_duplicate_email_without_creating_another_event() {
    let repository = InMemoryAccessUserRepository::new();
    let command = CreateAccessUser::new(Arc::clone(&repository) as Arc<_>);
    let input = || CreateAccessUserInput { name: "Alice".to_owned(), email: "Alice@Example.com".to_owned(), password: "S3cret!Pass123".to_owned() };

    command.execute(input()).unwrap();
    assert!(matches!(command.execute(input()), Err(backend_api::application::commands::create_access_user::CreateAccessUserError::DuplicateEmail)));
    assert_eq!(repository.user_count(), 1);
    assert_eq!(repository.outbox_count(), 1);
}

#[test]
fn rejects_invalid_input_before_persisting_user_or_event() {
    let repository = InMemoryAccessUserRepository::new();
    let command = CreateAccessUser::new(Arc::clone(&repository) as Arc<_>);

    let result = command.execute(CreateAccessUserInput { name: " ".to_owned(), email: "invalid".to_owned(), password: "weak".to_owned() });

    assert!(matches!(result, Err(backend_api::application::commands::create_access_user::CreateAccessUserError::Validation(_))));
    assert_eq!(repository.user_count(), 0);
    assert_eq!(repository.outbox_count(), 0);
}
