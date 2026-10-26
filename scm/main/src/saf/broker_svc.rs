//! SAF factory methods on [`MessagePublisherSvc`].

use crate::api::traits::message_publisher::MessagePublisher;
use crate::api::traits::validator::Validator;
use crate::api::types::message_broker_svc::MessagePublisherSvc;
use crate::api::types::message_publisher_handle::MessagePublisherHandle;
use crate::api::types::publisher_result::PublisherResult;
use configbuilder::{ConfigBuilder, ConfigLoaderFactory};
use swe_edge_message_broker::{Message, MessageBroker};

impl MessagePublisherSvc {
    /// Return a [`ConfigBuilderImpl`](configbuilder::ConfigBuilderImpl) pre-seeded
    /// with this crate's package name and version.
    pub fn create_config_builder() -> configbuilder::ConfigBuilderImpl {
        ConfigLoaderFactory::create_config_builder()
            .with_name(env!("CARGO_PKG_NAME"))
            .with_version(env!("CARGO_PKG_VERSION"))
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

    /// Wrap an already-constructed [`MessagePublisher`] in a handle for injection.
    ///
    /// Use this when the assembler has already constructed and configured the publisher.
    pub fn publisher(p: impl MessagePublisher + 'static) -> MessagePublisherHandle {
        MessagePublisherHandle::new(p)
    }

    /// Wrap an already-constructed [`MessageBroker`] as a publisher handle.
    ///
    /// The `BrokerPublisherAdapter` adapts the broker's publish/health_check to
    /// the [`MessagePublisher`] contract. The backend is fully owned by the caller —
    /// this crate never constructs runtime brokers itself.
    pub fn from_broker(b: impl MessageBroker + 'static) -> MessagePublisherHandle {
        MessagePublisherHandle::new(crate::core::BrokerPublisherAdapter::new(b))
    }
}
