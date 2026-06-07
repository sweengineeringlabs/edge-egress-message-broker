# swe-edge-egress-message-broker

> **TLDR:** Outbound message publishing for swe-edge — provider-agnostic `MessagePublisher` trait bridging domain events to NATS, Kafka, or in-memory backends. See [Overview](docs/README.md) for details.

Outbound message publishing egress port for `swe-edge`.

## Quick Start

```rust
use swe_edge_egress_message_broker::{MessagePublisherSvc, PublisherConfig};

let publisher = MessagePublisherSvc::from_config(&config).await?;
publisher.publish("domain.events", message).await?;
```

## Documentation

| Document | Description |
|----------|-------------|
| [Overview](docs/README.md) | WHAT + WHY — capabilities and design rationale |
