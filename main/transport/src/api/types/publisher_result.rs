//! Result type alias for outbound message publishing operations.

use crate::api::error::publisher_error::PublisherError;

/// Result type for [`MessagePublisher`](crate::api::traits::MessagePublisher) operations.
pub type PublisherResult<T> = Result<T, PublisherError>;
