use serde::{Deserialize, Serialize};

use crate::domain::access_user::AccessUser;

#[derive(Debug, Deserialize)]
pub struct CreateAccessUserRequest {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct AccessUserResponse {
    pub id: String,
    pub email: String,
    pub name: String,
    pub status: String,
    pub created_at: String,
    pub updated_at: String,
    pub version: u64,
}

impl From<&AccessUser> for AccessUserResponse {
    fn from(user: &AccessUser) -> Self {
        Self { id: user.id.clone(), email: user.email.to_string(), name: user.name.as_str().to_owned(), status: match user.status { crate::domain::access_user_status::AccessUserStatus::Active => "active".to_owned(), crate::domain::access_user_status::AccessUserStatus::Disabled => "disabled".to_owned() }, created_at: user.created_at.clone(), updated_at: user.updated_at.clone(), version: user.version }
    }
}
