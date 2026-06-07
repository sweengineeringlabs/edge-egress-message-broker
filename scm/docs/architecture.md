# Architecture — edge-egress-message-broker

## Sequence

> A domain handler obtains a `MessagePublisher` from the factory and publishes a typed message to a topic; the publisher delegates to the underlying broker backend.

```mermaid
sequenceDiagram
    participant Handler
    participant MessagePublisherSvc
    participant BrokerMessagePublisher
    participant MessageBroker

    Handler->>MessagePublisherSvc: create_publisher(config)
    MessagePublisherSvc-->>Handler: Arc<dyn MessagePublisher>

    Handler->>BrokerMessagePublisher: publish(topic, payload)
    BrokerMessagePublisher->>BrokerMessagePublisher: serialize payload → Message{bytes, headers}
    BrokerMessagePublisher->>MessageBroker: publish(topic, Message)
    MessageBroker-->>BrokerMessagePublisher: Result<(), BrokerError>
    BrokerMessagePublisher-->>Handler: Result<(), PublishError>
```

## Data Flow

> A domain payload is wrapped into a `Message`, routed to the broker backend (in-memory / NATS / Kafka), and acknowledged.

```mermaid
flowchart LR
    A["domain payload: T\n+ topic: &str\n+ headers: HeaderMap"] --> B["BrokerMessagePublisher\n::publish(topic, payload)"]
    B --> C["serialize → Message\n───────────\ntopic: String\npayload: Bytes\nheaders: HashMap"]
    C --> D["MessageBroker::publish"]
    D --> E{broker backend}
    E -->|in-memory| F["tokio broadcast channel"]
    E -->|NATS| G["async-nats subject"]
    E -->|Kafka| H["rdkafka producer"]
    F --> I["Result<(), BrokerError>"]
    G --> I
    H --> I
    I -->|Ok| J["publish acknowledged"]
    I -->|Err| K["PublishError\n::Broker(BrokerError)"]
```
