//! Api interface counterpart for `core/nats_message_publisher`.

pub use crate::api::port::MessagePublisher;
pub use crate::api::traits::Validator;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_publisher_interface_is_accessible() {
        fn _assert(_: &dyn MessagePublisher) {}
    }

    #[test]
    fn test_validator_interface_is_accessible() {
        fn _assert(_: &dyn Validator) {}
    }
}
