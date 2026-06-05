//! Integration tests — InMemoryMessagePublisher interface contract.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_egress_message_broker::{Message, MessageBrokerSvc, MessagePublisher};

    /// @covers: InMemoryMessagePublisher — satisfied by DefaultMessagePublisher via SAF
    #[tokio::test]
    async fn test_in_memory_publisher_satisfies_message_publisher_contract() {
        let p = MessageBrokerSvc::default_publisher();
        let msg = Message::new(b"in-memory-test".to_vec());
        assert!(p.publish("test.topic", msg).await.is_ok());
        assert!(p.health_check().await.is_ok());
    }
}
