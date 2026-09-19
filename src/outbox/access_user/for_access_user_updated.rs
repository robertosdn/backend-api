use crate::domain::access_user::{events::AccessUserUpdated, AccessUser};

use crate::outbox::outbox_record::{AccessUserEvent, OutboxRecord, OutboxStatus};

impl OutboxRecord {
    pub fn for_access_user_updated(user: &AccessUser, event_id: String) -> Self {
        let payload = AccessUserUpdated {
            event_id: event_id.clone(),
            aggregate_id: user.id.clone(),
            event_type: "AccessUserUpdated",
            email: user.email.to_string(),
            name: user.name.as_str().to_owned(),
            status: "active",
            version: user.version,
        };
        Self {
            id: event_id,
            aggregate_id: user.id.clone(),
            event_type: "AccessUserUpdated".to_owned(),
            payload: AccessUserEvent::Updated(payload),
            status: OutboxStatus::Pending,
        }
    }
}
