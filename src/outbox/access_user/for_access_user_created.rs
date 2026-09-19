use crate::domain::access_user::{events::AccessUserCreated, AccessUser};

use crate::outbox::outbox_record::{AccessUserEvent, OutboxRecord, OutboxStatus};

impl OutboxRecord {
    pub fn for_access_user_created(user: &AccessUser, event_id: String) -> Self {
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
            payload: AccessUserEvent::Created(payload),
            status: OutboxStatus::Pending,
        }
    }
}
