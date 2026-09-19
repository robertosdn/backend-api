use std::sync::{Arc, Mutex};

use crate::{domain::{access_user::AccessUser, value_objects::EmailAddress, events::AccessUserCreated}, outbox::outbox_record::OutboxRecord};

#[derive(Debug)]
pub enum WriteRepositoryError {
    DuplicateEmail,
    Storage,
}

pub trait AccessUserWriteRepository: Send + Sync {
    fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError>;
}

#[derive(Default)]
pub struct InMemoryAccessUserRepository {
    pub users: Mutex<Vec<AccessUser>>,
    pub outbox: Mutex<Vec<OutboxRecord>>,
}

impl InMemoryAccessUserRepository {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn event_for(user: &AccessUser, event_id: String) -> OutboxRecord {
        let payload = AccessUserCreated { event_id: event_id.clone(), aggregate_id: user.id.clone(), event_type: "AccessUserCreated", email: user.email.to_string(), name: user.name.as_str().to_owned(), status: "active", version: user.version };
        OutboxRecord { id: event_id, aggregate_id: user.id.clone(), event_type: "AccessUserCreated".to_owned(), payload, status: crate::outbox::outbox_record::OutboxStatus::Pending }
    }
}

impl AccessUserWriteRepository for InMemoryAccessUserRepository {
    fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError> {
        let mut users = self.users.lock().map_err(|_| WriteRepositoryError::Storage)?;
        if users.iter().any(|existing| existing.email == user.email) {
            return Err(WriteRepositoryError::DuplicateEmail);
        }
        let mut outbox = self.outbox.lock().map_err(|_| WriteRepositoryError::Storage)?;
        users.push(user);
        outbox.push(event);
        Ok(())
    }
}

pub fn email_exists(_email: &EmailAddress) -> bool {
    false
}
