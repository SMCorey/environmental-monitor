## MQTT Implementation - Architecture Overview

This document provides a high-level overview of our MQTT implementation for team members who may not be familiar with Rust or MQTT.

### System Architecture

```
+-----------------------+        +---------------------+        +---------------------+
|                       |        |                     |        |                     |
|  IoT Device           |        |  Tauri Desktop App  |        |  User Interface     |
|                       |        |                     |        |                     |
|  [MQTT Publisher]     |<------>|  [MQTT Client]      |<------>|  [SvelteKit+TS]     |
|                       |        |  (Rust Backend)     |        |  (Frontend)         |
+-----------------------+        +---------------------+        +---------------------+
                                      ^                                  ^
                                      |                                  |
                                      v                                  v
                                 +------------------------------------+
                                 |                                    |
                                 |  MQTT Broker                       |
                                 |  (e.g., HiveMQ, Mosquitto)         |
                                 |                                    |
                                 +------------------------------------+
```

### Key Components

1. **MQTT Client (Rust/Tauri Backend)**
   - Manages MQTT connections, subscriptions, and message handling
   - Provides secure state management with thread-safe access
   - Exposes a simple API to the frontend via Tauri commands
   - Broadcasts events to frontend when messages arrive

2. **SvelteKit Frontend**
   - Provides user interface for MQTT operations
   - Listens for MQTT events from the backend
   - Calls backend functions to connect, subscribe, publish, etc.
   - Displays messages and connection status

3. **IoT Device**
   - Publishes sensor data or other information to MQTT topics
   - Subscribes to control topics for receiving commands

4. **MQTT Broker**
   - Central message router that receives published messages
   - Forwards messages to subscribed clients
   - Can be public (like HiveMQ) or privately hosted

### Data Flow

1. **Connecting to a Broker**:
   - Frontend calls `connect_mqtt` command with broker details
   - Rust backend establishes connection and starts event loop
   - Connection status events sent to frontend

2. **Subscribing to Topics**:
   - Frontend calls `subscribe_mqtt` command with topic and QoS
   - Backend subscribes to the topic on the broker
   - Subscription confirmations sent to frontend

3. **Receiving Messages**:
   - IoT device publishes data to broker
   - Broker forwards to our subscribed client
   - Backend processes message and emits event to frontend
   - Frontend displays the message

4. **Publishing Messages**:
   - Frontend calls `publish_mqtt` with topic and payload
   - Backend publishes to the broker
   - Any client (including our IoT device) subscribed to that topic receives the message

### Key Concepts for Non-Rust Developers

- **State Management**: Rust uses a unique approach to memory safety. Our MQTT state is wrapped in thread-safe containers (Mutex) to prevent data races.

- **Async/Await**: The code uses asynchronous programming for non-blocking I/O, allowing the UI to remain responsive during network operations.

- **Event Loop**: MQTT communication happens in a background task that continuously processes incoming messages and connection events.

- **Error Handling**: Rust's Result type provides explicit error handling, which we propagate back to the frontend as needed.

### MQTT Quality of Service (QoS) Levels

- **QoS 0** (At Most Once): Fire and forget, no guarantee of delivery
- **QoS 1** (At Least Once): Guaranteed delivery, but might be duplicated
- **QoS 2** (Exactly Once): Guaranteed delivery exactly once (highest overhead)

### Communication with Frontend

The backend communicates with the frontend through:

1. **Commands**: JavaScript->Rust function calls (invoke)
2. **Events**: Rust->JavaScript notifications (emit)

Event types include:
- `mqtt_event`: Connection status changes
- `mqtt_message`: Incoming MQTT messages
- `mqtt_subscription`: Subscription confirmations