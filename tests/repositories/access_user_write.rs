use backend_api::repositories::access_user_write_repository::InMemoryAccessUserRepository;

#[test]
fn repository_starts_empty() {
    let repository = InMemoryAccessUserRepository::new();
    assert!(repository.users.lock().unwrap().is_empty());
    assert!(repository.outbox.lock().unwrap().is_empty());
}
