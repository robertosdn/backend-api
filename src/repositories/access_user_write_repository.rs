use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use crate::{domain::access_user::AccessUser, outbox::outbox_record::OutboxRecord};

#[derive(Debug)]
pub enum WriteRepositoryError {
    DuplicateEmail,
    Storage,
}

#[async_trait]
pub trait AccessUserWriteRepository: Send + Sync {
    async fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError>;
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

    pub fn user_count(&self) -> usize {
        self.state.lock().map(|state| state.users.len()).unwrap_or_default()
    }

    pub fn outbox_count(&self) -> usize {
        self.state.lock().map(|state| state.outbox.len()).unwrap_or_default()
    }
}

#[async_trait]
impl AccessUserWriteRepository for InMemoryAccessUserRepository {
    async fn save_user_and_event(&self, user: AccessUser, event: OutboxRecord) -> Result<(), WriteRepositoryError> {
        let mut state = self.state.lock().map_err(|_| WriteRepositoryError::Storage)?;
        if state.users.iter().any(|existing| existing.email == user.email) {
            return Err(WriteRepositoryError::DuplicateEmail);
        }
        state.users.push(user);
        state.outbox.push(event);
        Ok(())
    }
}
