use backend_api::domain::access_user::value_objects::{
    validate_password_policy, DisplayName, EmailAddress,
};

#[test]
fn normalizes_email_and_validates_name_and_password() {
    assert_eq!(EmailAddress::parse(" Alice@Example.COM ").unwrap().as_str(), "alice@example.com");
    assert_eq!(DisplayName::parse(" Alice Silva ").unwrap().as_str(), "Alice Silva");
    assert!(validate_password_policy("S3cret!Pass123").is_ok());
}
