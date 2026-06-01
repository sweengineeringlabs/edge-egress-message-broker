//! Integration tests — MessagePublisherHandle.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_egress_message_broker::{
        Message, MessageBrokerSvc, MessagePublisher, MessagePublisherHandle,
    };

    /// @covers: MessagePublisherHandle — Clone
    #[tokio::test]
    async fn test_publisher_handle_clone_shares_underlying_channel() {
        let h1: MessagePublisherHandle = MessageBrokerSvc::default_publisher();
        let h2 = h1.clone();
        let msg = Message::new(b"x".to_vec());
        assert!(h1.publish("a", msg.clone()).await.is_ok());
        assert!(h2.publish("b", msg).await.is_ok());
    }

    /// @covers: MessagePublisherHandle — MessagePublisher::health_check
    #[tokio::test]
    async fn test_publisher_handle_health_check_returns_ok() {
        let h: MessagePublisherHandle = MessageBrokerSvc::default_publisher();
        assert!(h.health_check().await.is_ok());
    }
}
