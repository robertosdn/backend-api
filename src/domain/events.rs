use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
pub struct AccessUserCreated {
    pub event_id: String,
    pub aggregate_id: String,
    pub event_type: &'static str,
    pub email: String,
    pub name: String,
    pub status: &'static str,
    pub version: u64,
}
