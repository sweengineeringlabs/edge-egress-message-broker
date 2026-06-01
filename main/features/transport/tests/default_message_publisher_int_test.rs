//! Integration tests — default (in-memory) message publisher.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_egress_message_broker::{Message, MessageBrokerSvc, MessagePublisher};

    /// @covers: DefaultMessagePublisher — publishes via SAF
    #[tokio::test]
    async fn test_default_message_publisher_publishes_successfully() {
        let p = MessageBrokerSvc::default_publisher();
        let msg = Message::new(b"hello".to_vec());
        assert!(p.publish("default.test", msg).await.is_ok());
    }

    /// @covers: DefaultMessagePublisher — health check via SAF
    #[tokio::test]
    async fn test_default_message_publisher_health_check_returns_ok() {
        let p = MessageBrokerSvc::default_publisher();
        assert!(p.health_check().await.is_ok());
    }
}
