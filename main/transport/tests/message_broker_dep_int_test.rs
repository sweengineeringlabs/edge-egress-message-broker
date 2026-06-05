//! Integration test — exercises `swe-edge-runtime-message-broker` dep directly.
//!
//! Satisfies rule 95: dependencies used in src/ must have integration/e2e coverage.

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_runtime_message_broker::{in_memory_broker, Message, MessageBroker};

    #[tokio::test]
    async fn test_message_broker_dep_health_check_returns_ok() {
        let broker = in_memory_broker();
        assert!(broker.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_message_broker_dep_publish_returns_ok_with_no_subscribers() {
        let broker = in_memory_broker();
        assert!(broker
            .publish("dep.test", Message::new(b"ok".to_vec()))
            .await
            .is_ok());
    }
}
