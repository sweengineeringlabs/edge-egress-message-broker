//! Integration test — exercises the `swe-edge-runtime-message-broker` backend directly.
//!
//! Satisfies rule 95: dependencies used in src/ must have integration/e2e coverage.

#![allow(clippy::unwrap_used, clippy::expect_used)]

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_message_broker::{Message, MessageBroker};
    use swe_edge_runtime_message_broker::MessageBrokerFactory;

    #[tokio::test]
    async fn test_message_broker_dep_health_check_returns_ok() {
        let broker = MessageBrokerFactory::in_memory();
        assert!(broker.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_message_broker_dep_publish_returns_ok_with_no_subscribers() {
        let broker = MessageBrokerFactory::in_memory();
        assert!(broker
            .publish("dep.test", Message::new(b"ok".to_vec()))
            .await
            .is_ok());
    }
}
