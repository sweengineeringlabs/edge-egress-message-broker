//! Integration tests — MessagePublisher port via SAF factories.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_egress_message_broker::{default_publisher, Message, MessagePublisher};

    #[tokio::test]
    async fn test_default_publisher_health_check_returns_ok() {
        let p = default_publisher();
        assert!(p.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_default_publisher_publish_returns_ok_with_no_subscribers() {
        let p = default_publisher();
        let msg = Message::new(b"ping".to_vec());
        assert!(p.publish("test.events", msg).await.is_ok());
    }

    #[tokio::test]
    async fn test_default_publisher_clone_produces_independent_handle() {
        let p1 = default_publisher();
        let p2 = p1.clone();
        let msg = Message::new(b"x".to_vec());
        assert!(p1.publish("a", msg.clone()).await.is_ok());
        assert!(p2.publish("b", msg).await.is_ok());
    }
}
