use std::sync::{Arc, Mutex};

use crate::{domain::{access_user::AccessUser, events::AccessUserCreated}, outbox::outbox_record::OutboxRecord};

#[derive(Debug)]
pub enum WriteRepositoryError {
    DuplicateEmail,
    Storage,
}

pub trait AccessUserWriteRepository: Send + Sync {
    fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError>;
}

#[derive(Default)]
struct RepositoryState {
    users: Vec<AccessUser>,
    outbox: Vec<OutboxRecord>,
}

#[derive(Default)]
pub struct InMemoryAccessUserRepository {
    state: Mutex<RepositoryState>,
}

impl InMemoryAccessUserRepository {
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    pub fn event_for(user: &AccessUser, event_id: String) -> OutboxRecord {
        let payload = AccessUserCreated { event_id: event_id.clone(), aggregate_id: user.id.clone(), event_type: "AccessUserCreated", email: user.email.to_string(), name: user.name.as_str().to_owned(), status: "active", version: user.version };
        OutboxRecord { id: event_id, aggregate_id: user.id.clone(), event_type: "AccessUserCreated".to_owned(), payload, status: crate::outbox::outbox_record::OutboxStatus::Pending }
    }

    pub fn user_count(&self) -> usize {
        self.state.lock().map(|state| state.users.len()).unwrap_or_default()
    }

    pub fn outbox_count(&self) -> usize {
        self.state.lock().map(|state| state.outbox.len()).unwrap_or_default()
    }
}

impl AccessUserWriteRepository for InMemoryAccessUserRepository {
    fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError> {
        let mut state = self.state.lock().map_err(|_| WriteRepositoryError::Storage)?;
        if state.users.iter().any(|existing| existing.email == user.email) {
            return Err(WriteRepositoryError::DuplicateEmail);
        }
        state.users.push(user);
        state.outbox.push(event);
        Ok(())
    }
}
