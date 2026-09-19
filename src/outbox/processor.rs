use super::outbox_record::OutboxRecord;

pub struct OutboxProcessor;

impl OutboxProcessor {
    pub fn pending(records: &[OutboxRecord]) -> impl Iterator<Item = &OutboxRecord> {
        records.iter()
    }
}
