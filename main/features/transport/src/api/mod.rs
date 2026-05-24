//! API layer — egress message publisher port contracts.
pub(crate) mod default_message_publisher;
pub(crate) mod message_publisher_config;
pub(crate) mod nats_message_publisher;
pub(crate) mod port;
pub(crate) mod traits;
pub(crate) mod validator;

pub use message_publisher_config::MessagePublisherConfig;
pub use port::{MessagePublisher, PublisherError, PublisherResult};
pub use traits::Validator;
