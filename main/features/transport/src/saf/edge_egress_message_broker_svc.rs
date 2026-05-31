//! SAF factory functions for the egress message publisher.

use crate::api::port::publisher::publisher_error::PublisherError;
use crate::api::port::publisher::publisher_result::PublisherResult;
use crate::api::port::MessagePublisher;
use crate::api::traits::Validator;
use swe_edge_runtime_message_broker::Message;

/// Return a [`ConfigBuilder`] pre-seeded with this crate's package name and version.
pub fn create_config_builder() -> swe_edge_configbuilder::ConfigBuilderImpl {
    let mut b = swe_edge_configbuilder::ConfigBuilderImpl::new();
    b = b.with_name(env!("CARGO_PKG_NAME"));
    b = b.with_version(env!("CARGO_PKG_VERSION"));
    b
}

/// Validate any type that implements [`Validator`].
///
/// # Errors
/// Returns the validation error string on failure.
pub fn validate<V: Validator>(v: &V) -> Result<(), String> {
    v.validate()
}

/// Publish `msg` to `topic` using any [`MessagePublisher`].
///
/// Convenience wrapper that drives the publisher future synchronously-friendly
/// in async contexts.
pub fn publish_to<'a>(
    publisher: &'a dyn MessagePublisher,
    topic: &'a str,
    msg: Message,
) -> futures::future::BoxFuture<'a, PublisherResult<()>> {
    publisher.publish(topic, msg)
}

/// Run a health check on any [`MessagePublisher`].
pub fn check_health(
    publisher: &dyn MessagePublisher,
) -> futures::future::BoxFuture<'_, PublisherResult<()>> {
    publisher.health_check()
}

/// Construct an in-memory publisher backed by a tokio broadcast channel.
///
/// Requires the `in-memory` feature.
#[cfg(feature = "in-memory")]
pub fn default_publisher() -> impl MessagePublisher + Clone {
    crate::core::DefaultMessagePublisher::new()
}

/// Connect to a NATS server and return a publisher handle.
///
/// # Errors
/// Returns [`PublisherError::Connection`] when the server is unreachable.
///
/// Requires the `nats` feature.
#[cfg(feature = "nats")]
pub async fn nats_publisher(url: &str) -> Result<impl MessagePublisher + Clone, PublisherError> {
    use swe_edge_runtime_message_broker::nats_broker;
    let broker = nats_broker(url).await.map_err(PublisherError::from)?;
    Ok(crate::core::NatsMessagePublisher::new(broker))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::DefaultValidator;
    use futures::future::BoxFuture;

    struct NeverPublisher;
    impl MessagePublisher for NeverPublisher {
        fn publish<'a>(&'a self, _: &'a str, _: Message) -> BoxFuture<'a, PublisherResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
        fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>> {
            Box::pin(futures::future::ready(Ok(())))
        }
    }

    /// @covers: validate
    #[test]
    fn test_validate_returns_ok_for_default_validator() {
        assert!(validate(&DefaultValidator).is_ok());
    }

    /// @covers: publish_to
    #[test]
    fn test_publish_to_accepts_any_message_publisher() {
        let msg = Message::new(b"x".to_vec());
        let _fut = publish_to(&NeverPublisher, "t", msg);
    }

    /// @covers: check_health
    #[test]
    fn test_check_health_accepts_any_message_publisher() {
        let _fut = check_health(&NeverPublisher);
    }

    #[tokio::test]
    async fn test_publish_to_returns_ok_when_publisher_succeeds() {
        let msg = Message::new(b"x".to_vec());
        assert!(publish_to(&NeverPublisher, "t", msg).await.is_ok());
    }

    #[tokio::test]
    async fn test_check_health_returns_ok_when_publisher_is_healthy() {
        assert!(check_health(&NeverPublisher).await.is_ok());
    }

    /// @covers: default_publisher
    #[cfg(feature = "in-memory")]
    #[tokio::test]
    async fn test_default_publisher_health_check_returns_ok() {
        let p = default_publisher();
        assert!(check_health(&p).await.is_ok());
    }

    /// @covers: publish_to
    #[cfg(feature = "in-memory")]
    #[tokio::test]
    async fn test_publish_to_returns_ok_with_no_subscribers() {
        let p = default_publisher();
        let msg = Message::new(b"test".to_vec());
        assert!(publish_to(&p, "topic", msg).await.is_ok());
    }

    /// @covers: check_health
    #[cfg(feature = "in-memory")]
    #[tokio::test]
    async fn test_check_health_returns_ok_for_in_memory_publisher() {
        let p = default_publisher();
        assert!(check_health(&p).await.is_ok());
    }

    /// @covers: default_publisher
    #[test]
    fn test_default_publisher_is_feature_gated_behind_in_memory() {
        let _enabled = cfg!(feature = "in-memory");
    }

    /// @covers: nats_publisher
    #[test]
    fn test_nats_publisher_is_feature_gated() {
        let _enabled = cfg!(feature = "nats");
    }
}
