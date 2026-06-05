//! Integration tests — MessagePublisherHandle type.

#[cfg(feature = "in-memory")]
mod in_memory {
    use swe_edge_egress_message_broker::{
        MessageBrokerSvc, MessagePublisher, MessagePublisherHandle,
    };

    /// @covers: MessagePublisherHandle — wraps publisher via MessageBrokerSvc
    #[tokio::test]
    async fn test_message_publisher_handle_wraps_default_publisher() {
        let h: MessagePublisherHandle = MessageBrokerSvc::default_publisher();
        assert!(h.health_check().await.is_ok());
    }
}
