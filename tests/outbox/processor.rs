use backend_api::outbox::processor::OutboxProcessor;

#[test]
fn processor_accepts_empty_pending_batch() {
    assert_eq!(OutboxProcessor::pending(&[]).count(), 0);
}
