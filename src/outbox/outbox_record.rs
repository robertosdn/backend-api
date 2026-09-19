use crate::domain::events::AccessUserCreated;

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
