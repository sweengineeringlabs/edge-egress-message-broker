//! Integration tests — SAF public API for the egress message publisher.
//!
//! Covers rules 125 (SAF pub fn must have API-level tests) and 77 (all pub fns tested).

#[cfg(feature = "in-memory")]
mod tests {
    use swe_edge_egress_message_broker::{
        ApplicationConfigBuilder, Message, MessageBrokerSvc, MessagePublisher, Validator,
    };

    struct AlwaysValid;
    impl Validator for AlwaysValid {
        fn validate(&self) -> Result<(), String> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_default_publisher_saf_factory_returns_healthy_publisher() {
        let p = MessageBrokerSvc::default_publisher();
        assert!(p.health_check().await.is_ok());
    }

    #[tokio::test]
    async fn test_publish_to_succeeds_with_no_subscribers() {
        let p = MessageBrokerSvc::default_publisher();
        let msg = Message::new(b"test".to_vec());
        assert!(MessageBrokerSvc::publish_to(&p, "events.test", msg)
            .await
            .is_ok());
    }

    #[tokio::test]
    async fn test_check_health_returns_ok_for_in_memory_publisher() {
        let p = MessageBrokerSvc::default_publisher();
        assert!(MessageBrokerSvc::check_health(&p).await.is_ok());
    }

    #[test]
    fn test_validate_returns_ok_for_always_valid() {
        assert!(MessageBrokerSvc::validate(&AlwaysValid).is_ok());
    }

    #[test]
    fn test_application_config_builder_builds_with_custom_capacity() {
        let cfg = ApplicationConfigBuilder::new().with_capacity(256);
        assert_eq!(cfg.capacity, 256);
    }
}
