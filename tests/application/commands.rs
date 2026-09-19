use std::sync::Arc;

use backend_api::{application::commands::create_access_user::{CreateAccessUser, CreateAccessUserInput}, repositories::access_user_write_repository::InMemoryAccessUserRepository};

#[test]
fn creates_user_through_command_and_records_outbox_event() {
    let repository = InMemoryAccessUserRepository::new();
    let command = CreateAccessUser::new(Arc::clone(&repository) as Arc<_>);
    let user = command.execute(CreateAccessUserInput { name: "Alice".to_owned(), email: "alice@example.com".to_owned(), password: "S3cret!Pass123".to_owned() }).unwrap();
    assert_eq!(user.email.as_str(), "alice@example.com");
    assert_eq!(repository.outbox.lock().unwrap().len(), 1);
}
