//! Result type alias for outbound message publishing operations.

use crate::api::port::publisher::publisher_error::PublisherError;

/// Result type for [`MessagePublisher`](super::message_publisher::MessagePublisher) operations.
pub type PublisherResult<T> = Result<T, PublisherError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publisher_result_ok_variant_holds_value() {
        let r: PublisherResult<u32> = Ok(42);
        assert_eq!(r.unwrap(), 42);
    }
}
