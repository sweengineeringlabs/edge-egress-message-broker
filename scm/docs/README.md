# swe-edge-egress-message-broker

## WHAT

Outbound message publishing for swe-edge — publishes domain events and commands to named topics
via a provider-agnostic `MessagePublisher` trait backed by the message-broker port contract.

Key capabilities:

- **`MessagePublisher`** — core trait: `publish(topic, msg) → Result<(), PublisherError>`; with health checks
- **`BrokerMessagePublisher`** — bridge trait connecting `MessagePublisher` to concrete broker back-ends (NATS, in-memory, default)
- **`MessagePublisherSvc`** — SAF factory: `from_config()` → `Arc<dyn MessagePublisher>`; callers never name the concrete type
- **`MessagePublisherHandle`** — dependency-injection handle for wiring publishers into handlers and services
- **`ApplicationConfigBuilder`** — fluent config assembly; validation via `SubstitutionPolicy`

## WHY

| Problem | Solution |
|---------|----------|
| Handlers directly call NATS/Kafka SDKs, coupling domain code to message infrastructure | `MessagePublisher` trait decouples domain events from transport; swap backends via config |
| Multiple handlers publishing to the same broker create redundant connections | Shared `Arc<dyn MessagePublisher>` via `MessagePublisherHandle`; one connection, many publishers |
| Publisher config (topic names, broker URL) buried in handler code | Config loaded from `[message_publisher]` TOML section via `from_config()`; centralised and rotatable |
| Diamond dep conflicts when publisher types change | One crate, one tag — all consumers pin the same version; kgraph detects conflicts pre-commit |
