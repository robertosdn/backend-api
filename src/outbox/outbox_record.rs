use serde::Serialize;

use crate::domain::access_user::events::{AccessUserCreated, AccessUserUpdated};

#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum AccessUserEvent {
    Created(AccessUserCreated),
    Updated(AccessUserUpdated),
}

#[derive(Debug, Clone)]
pub struct OutboxRecord {
    pub id: String,
    pub aggregate_id: String,
    pub event_type: String,
    pub payload: AccessUserEvent,
    pub status: OutboxStatus,
}

#[derive(Debug, Clone, Copy)]
pub enum OutboxStatus {
    Pending,
    Published,
    Failed,
}
