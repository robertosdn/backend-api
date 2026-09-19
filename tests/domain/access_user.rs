use backend_api::domain::access_user::{value_objects::{DisplayName, EmailAddress}, AccessUser};

#[test]
fn creates_active_access_user_with_initial_version() {
    let user = AccessUser::new(
        "user-1".to_owned(),
        EmailAddress::parse("user@example.com").unwrap(),
        DisplayName::parse("User").unwrap(),
        "argon2id-hash".to_owned(),
        "now".to_owned(),
    );
    assert_eq!(user.id, "user-1");
    assert_eq!(user.version, 1);
}
