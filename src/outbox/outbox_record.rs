use crate::domain::{access_user::AccessUser, events::AccessUserCreated};

#[derive(Debug, Clone)]
pub struct OutboxRecord {
    pub id: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: AccessUserCreated,
    pub status: OutboxStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum OutboxStatus {
    Pending,
    Published,
    Failed,
}

impl OutboxRecord {
    pub fn for_user(user: &AccessUser, event_id: String) -> Self {
        let payload = AccessUserCreated {
            event_id: event_id.clone(),
            aggregate_id: user.id.clone(),
            event_type: "AccessUserCreated",
            email: user.email.to_string(),
            name: user.name.as_str().to_owned(),
            status: "active",
            version: user.version,
        };
        Self {
            id: event_id,
            aggregate_id: user.id.clone(),
            event_type: "AccessUserCreated".to_owned(),
            payload,
            status: OutboxStatus::Pending,
        }
    }
}
