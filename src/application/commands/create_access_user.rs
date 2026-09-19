use std::{sync::Arc, time::{SystemTime, UNIX_EPOCH}};

use uuid::Uuid;

use crate::{
    auth::password_hash,
    domain::{access_user::AccessUser, value_objects::{validate_password_policy, DisplayName, EmailAddress}},
    repositories::access_user_write_repository::{AccessUserWriteRepository, InMemoryAccessUserRepository, WriteRepositoryError},
};

pub struct CreateAccessUser {
    repository: Arc<dyn AccessUserWriteRepository>,
}

pub struct CreateAccessUserInput {
    pub name: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug)]
pub enum CreateAccessUserError {
    Validation(&'static str),
    DuplicateEmail,
    Storage,
}

impl CreateAccessUser {
    pub fn new(repository: Arc<dyn AccessUserWriteRepository>) -> Self {
        Self { repository }
    }

    pub fn execute(&self, input: CreateAccessUserInput) -> Result<AccessUser, CreateAccessUserError> {
        let name = DisplayName::parse(&input.name).map_err(CreateAccessUserError::Validation)?;
        let email = EmailAddress::parse(&input.email).map_err(CreateAccessUserError::Validation)?;
        validate_password_policy(&input.password).map_err(CreateAccessUserError::Validation)?;
        let password_hash = password_hash::hash(&input.password).map_err(|_| CreateAccessUserError::Storage)?;
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs().to_string();
        let user = AccessUser::new(Uuid::new_v4().to_string(), email, name, password_hash, now);
        let event = InMemoryAccessUserRepository::event_for(&user, Uuid::new_v4().to_string());
        self.repository.save_user_and_event(user.clone(), event).map_err(|error| match error {
            WriteRepositoryError::DuplicateEmail => CreateAccessUserError::DuplicateEmail,
            WriteRepositoryError::Storage => CreateAccessUserError::Storage,
        })?;
        Ok(user)
    }
}
