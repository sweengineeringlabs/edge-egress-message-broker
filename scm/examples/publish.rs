//! Example — publish a message using an injected mock broker.
//!
//! Run with:
//! ```bash
//! cargo run --example publish
//! ```

// Examples favour terse `.expect()` over production-grade error handling.
#![allow(clippy::expect_used, clippy::unwrap_used)]

use std::sync::Arc;
use swe_edge_egress_message_publisher::{Message, MessagePublisherSvc};
use swe_edge_message_broker::{
    BrokerError, BrokerFuture, HealthCheckRequest, MessageBroker, MessageStream, PublishRequest,
    SubscribeRequest, SubscribeResponse, ValidationError, ValidationRequest, Validator,
    ValidatorRequest, ValidatorResponse,
};

struct MockBroker;
impl MessageBroker for MockBroker {
    fn publish<'a>(
        &'a self,
        _request: PublishRequest,
    ) -> BrokerFuture<'a, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }
    fn subscribe<'a>(
        &'a self,
        _request: SubscribeRequest,
    ) -> BrokerFuture<'a, Result<SubscribeResponse, BrokerError>> {
        BrokerFuture::new(async {
            let stream: MessageStream = Box::pin(futures::stream::empty());
            Ok(SubscribeResponse { stream })
        })
    }
    fn health_check(
        &self,
        _request: HealthCheckRequest,
    ) -> BrokerFuture<'_, Result<(), BrokerError>> {
        BrokerFuture::new(async { Ok(()) })
    }
    fn validator(&self, _request: ValidatorRequest) -> Result<ValidatorResponse, BrokerError> {
        Ok(ValidatorResponse {
            validator: Arc::new(TestValidator),
        })
    }
}

struct TestValidator;
impl Validator for TestValidator {
    fn validate(&self, _request: ValidationRequest) -> Result<(), ValidationError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() {
    // The assembler injects the broker. Here we use a mock for illustration.
    let publisher = MessagePublisherSvc::from_broker(MockBroker);
    let msg = Message::new(b"hello, world".to_vec());
    MessagePublisherSvc::publish_to(&publisher, "example.topic", msg)
        .await
        .expect("publish failed");
    println!("Message published.");
}
