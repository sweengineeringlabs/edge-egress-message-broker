//! `MessagePublisherHandle` — opaque handle to a live publisher instance.

use std::sync::Arc;

use futures::future::BoxFuture;

use crate::api::traits::message_publisher::MessagePublisher;
use crate::api::types::publisher_result::PublisherResult;

/// An opaque, cloneable handle to a [`MessagePublisher`] instance.
///
/// Returned by SAF factory functions such as
/// [`MessagePublisherSvc::publisher`](crate::api::types::message::message_broker_svc::MessagePublisherSvc).
/// Callers use this handle as a `dyn`-compatible publisher without needing to
/// name the underlying concrete type.
#[derive(Clone)]
pub struct MessagePublisherHandle {
    inner: Arc<dyn MessagePublisher>,
}

impl MessagePublisherHandle {
    /// Wrap any `MessagePublisher` implementation in a handle.
    pub(crate) fn new(publisher: impl MessagePublisher + 'static) -> Self {
        Self {
            inner: Arc::new(publisher),
        }
    }
}

impl MessagePublisher for MessagePublisherHandle {
    fn publish<'a>(
        &'a self,
        topic: &'a str,
        msg: swe_edge_message_broker::Message,
    ) -> BoxFuture<'a, PublisherResult<()>> {
        self.inner.publish(topic, msg)
    }

    fn health_check(&self) -> BoxFuture<'_, PublisherResult<()>> {
        self.inner.health_check()
    }
}
