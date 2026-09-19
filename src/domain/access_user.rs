use super::{access_user_status::AccessUserStatus, value_objects::{DisplayName, EmailAddress}};

#[derive(Debug, Clone)]
pub struct AccessUser {
    pub id: String,
    pub email: EmailAddress,
    pub name: DisplayName,
    pub password_hash: String,
    pub status: AccessUserStatus,
    pub created_at: String,
    pub updated_at: String,
    pub version: u64,
}

impl AccessUser {
    pub fn new(id: String, email: EmailAddress, name: DisplayName, password_hash: String, now: String) -> Self {
        Self { id, email, name, password_hash, status: AccessUserStatus::Active, created_at: now.clone(), updated_at: now, version: 1 }
    }
}
