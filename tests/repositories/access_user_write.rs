use backend_api::repositories::access_user_write_repository::InMemoryAccessUserRepository;

#[test]
fn repository_starts_empty() {
    let repository = InMemoryAccessUserRepository::new();
    assert_eq!(repository.user_count(), 0);
    assert_eq!(repository.outbox_count(), 0);
}
