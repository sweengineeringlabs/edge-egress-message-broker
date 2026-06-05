//! SAF factory methods on [`MessageBrokerSvc`].

use crate::api::port::message_publisher::MessagePublisher;
use crate::api::port::publisher_result::PublisherResult;
use crate::api::traits::validator::Validator;
use crate::api::types::message::message_broker_svc::MessageBrokerSvc;
use swe_edge_message_broker::Message;

#[cfg(any(feature = "in-memory", feature = "nats"))]
use crate::api::types::message::message_publisher_handle::MessagePublisherHandle;

impl MessageBrokerSvc {
    /// Return a [`ConfigBuilderImpl`](swe_edge_configbuilder::ConfigBuilderImpl) pre-seeded
    /// with this crate's package name and version.
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
    /// Returns a [`MessagePublisherHandle`] that implements [`MessagePublisher`] and
    /// can be cloned to share the same underlying channel.
    ///
    /// Requires the `in-memory` feature.
    #[cfg(feature = "in-memory")]
    pub fn default_publisher() -> MessagePublisherHandle {
        MessagePublisherHandle::new(crate::core::DefaultMessagePublisher::new())
    }

    /// Connect to a NATS server and return a publisher handle.
    ///
    /// # Errors
    /// Returns [`PublisherError::Connection`] when the server is unreachable.
    ///
    /// Requires the `nats` feature.
    #[cfg(feature = "nats")]
    pub async fn nats_publisher(
        url: &str,
    ) -> Result<MessagePublisherHandle, crate::api::error::PublisherError> {
        use swe_edge_runtime_message_broker::MessageBrokerFactory;
        let broker = MessageBrokerFactory::nats(url)
            .await
            .map_err(crate::api::error::PublisherError::from)?;
        Ok(MessagePublisherHandle::new(
            crate::core::NatsMessagePublisher::new(broker),
        ))
    }
}
