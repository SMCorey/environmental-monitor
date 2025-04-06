use rumqttc::{AsyncClient, Event, MqttOptions, Packet, QoS};
use serde::Serialize;
use std::time::Duration;
// Tauri imports for event handling and state management
use tauri::{AppHandle, Emitter, State};
use tokio::sync::Mutex;
use tokio::task::JoinHandle;
use rand::Rng;

// --- State Management ---

// Client state holds the MQTT connection and task handle
// Default trait allows us to create an empty state instance
#[derive(Default)]
pub struct MqttClientState {
    client: Option<AsyncClient>,        // MQTT client when connected, None when not connected
    eventloop_task_handle: Option<JoinHandle<()>>, // Background task handle for event processing
}

// Main state container, wrapped in Mutex for thread safety
// This is what gets registered with Tauri's state management
pub struct MqttState(pub Mutex<MqttClientState>);


// --- Event Payloads ---

// These structs define the data structure that will be sent to the frontend
// as JSON when events occur (connection changes, new messages, etc.)

// Connection status events (connected, error, disconnected)
#[derive(Serialize, Clone)]
struct ConnectionEventPayload<'a> {
    message: &'a str,  // Human-readable message
    status: &'a str,   // Status code: "connected", "disconnected", "error"
}

// Incoming MQTT message events
#[derive(Serialize, Clone)]
struct MessageEventPayload {
    topic: String,     // The topic the message was received on
    payload: String,   // The message content (as UTF-8 string)
}

// Subscription confirmation events
#[derive(Serialize, Clone)]
struct SubscriptionEventPayload<'a> {
    topic: &'a str,    // Topic that was successfully subscribed to
    qos: u8,           // Negotiated QoS level (0,1,2)
}

// --- Tauri Commands ---
// These functions are exposed to the frontend and can be called from JavaScript

// Connect to an MQTT broker
#[tauri::command]
pub async fn connect_mqtt(
    app_handle: AppHandle,              // Used to emit events back to the frontend
    state: State<'_, MqttState>,        // Access to the shared MQTT state
    broker_address: String,             // MQTT broker hostname/IP
    port: u16,                          // MQTT broker port (usually 1883 for unencrypted)
    client_id_prefix: Option<String>,   // Optional custom client ID prefix
) -> Result<(), String> {
    println!(
        "Attempting to connect to MQTT broker: {}:{}",
        broker_address, port
    );

    // Lock the state for thread-safe access
    let mut client_state = state.0.lock().await;

    // Prevent multiple simultaneous connections
    if client_state.client.is_some() {
        println!("Already connected or connection attempt in progress.");
        return Err("Already connected.".to_string());
    }

    // Generate a random client ID suffix for uniqueness
    let random_suffix: String = rand::rng()
        .sample_iter(&rand::distr::Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    // Create full client ID, using provided prefix or default
    let client_id = format!(
        "{}-{}",
        client_id_prefix.unwrap_or_else(|| "tauri-mqtt".to_string()),
        random_suffix
    );

    // Configure MQTT connection options
    let mut mqttoptions = MqttOptions::new(client_id, broker_address.clone(), port);
    mqttoptions
        .set_keep_alive(Duration::from_secs(10))  // Keep-alive interval
        .set_clean_session(true);                 // Start with clean session (no persistent state)

    // Create the MQTT client and event loop
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);  // 10 = incoming message buffer size

    // Clone references for use in the background task
    let app_handle_clone = app_handle.clone();
    let broker_addr_clone = broker_address.clone();

    // Spawn a background task to handle MQTT events
    let task_handle = tokio::spawn(async move {
        println!("MQTT Event Loop started.");
        loop {
            match eventloop.poll().await {
                Ok(notification) => {
                    println!("Received MQTT Notification: {:?}", notification);

                    match notification {
                        // Handle successful connection
                        Event::Incoming(Packet::ConnAck(_)) => {
                             println!("MQTT Connected successfully!");
                             // Notify frontend of connection success
                             let _ = app_handle_clone.emit("mqtt_event", ConnectionEventPayload {
                                 message: &format!("Connected to {}", broker_addr_clone),
                                 status: "connected",
                             });
                        }
                        
                        // Handle incoming MQTT messages
                        Event::Incoming(Packet::Publish(publish)) => {
                            // Convert binary payload to string if possible
                            match String::from_utf8(publish.payload.to_vec()) {
                                Ok(payload_str) => {
                                    // Forward message to frontend
                                    let _ = app_handle_clone.emit("mqtt_message", MessageEventPayload {
                                        topic: publish.topic.clone(),
                                        payload: payload_str,
                                    });
                                }
                                Err(_) => {
                                     // Handle binary data that isn't valid UTF-8
                                     println!("Received non-UTF8 payload on topic: {}", publish.topic);
                                     let _ = app_handle_clone.emit("mqtt_message", MessageEventPayload {
                                        topic: publish.topic.clone(),
                                        payload: format!("<Binary data: {} bytes>", publish.payload.len()),
                                    });
                                }
                            }
                        }
                        
                        // Handle subscription acknowledgment
                        Event::Incoming(Packet::SubAck(suback)) => {
                            println!(
                                "Subscription Acknowledged: Pkid: {}, Return Codes: {:?}",
                                suback.pkid, suback.return_codes
                            );
                            // Note: Actual subscription notification happens in the subscribe_mqtt function
                        }
                        
                        // Handle disconnection
                        Event::Incoming(Packet::Disconnect) | Event::Outgoing(rumqttc::Outgoing::Disconnect) => {
                            println!("MQTT Disconnected (incoming or outgoing packet).");
                            // Notify frontend of disconnection
                            let _ = app_handle_clone.emit("mqtt_event", ConnectionEventPayload {
                                 message: "Disconnected from broker.",
                                 status: "disconnected",
                             });
                             break; // Exit the event loop
                        }
                        
                        // Ignore other packets for simplicity
                        Event::Incoming(_) => {}
                        Event::Outgoing(_) => {}
                    }
                }
                Err(e) => {
                    // Handle connection errors
                    eprintln!("MQTT Connection Error: {:?}", e);
                    // Notify frontend of the error
                    let _ = app_handle_clone.emit("mqtt_event", ConnectionEventPayload {
                        message: &format!("Connection error: {}", e),
                        status: "error",
                    });
                    break; // Exit the event loop on error
                }
            }
        }
        println!("MQTT Event Loop ended.");
    });

    // Store the client and task handle in state for later use
    client_state.client = Some(client);
    client_state.eventloop_task_handle = Some(task_handle);

    println!("MQTT Client created and event loop spawned.");
    Ok(())
}

// Subscribe to an MQTT topic
#[tauri::command]
pub async fn subscribe_mqtt(
    app_handle: AppHandle,      // For sending events to frontend
    state: State<'_, MqttState>,// Access to MQTT state
    topic: String,              // Topic to subscribe to
    qos_level: u8,              // Quality of Service (0, 1, or 2)
) -> Result<(), String> {
    println!("Attempting to subscribe to topic: {}", topic);
    let client_state = state.0.lock().await;

    // Convert QoS level from number to enum
    let qos = match qos_level {
        0 => QoS::AtMostOnce,      // Fire and forget (may not be delivered)
        1 => QoS::AtLeastOnce,     // Guaranteed delivery but might be duplicated
        2 => QoS::ExactlyOnce,     // Guaranteed delivery exactly once (highest overhead)
        _ => return Err("Invalid QoS level. Use 0, 1, or 2.".to_string()),
    };

    // Check if we're connected before trying to subscribe
    if let Some(client) = &client_state.client {
        match client.subscribe(topic.clone(), qos).await {
            Ok(_) => {
                println!("Successfully requested subscription to topic: {}", topic);
                // Notify frontend of successful subscription
                let _ = app_handle.emit("mqtt_subscription", SubscriptionEventPayload {
                    topic: &topic,
                    qos: qos_level,
                 });
                Ok(())
            },
            Err(e) => {
                eprintln!("Failed to subscribe to topic '{}': {:?}", topic, e);
                Err(format!("Failed to subscribe: {}", e))
            }
        }
    } else {
        Err("Not connected to MQTT broker.".to_string())
    }
}

// Publish a message to an MQTT topic
#[tauri::command]
pub async fn publish_mqtt(
    state: State<'_, MqttState>,// Access to MQTT state
    topic: String,              // Topic to publish to
    payload: String,            // Message content
    qos_level: u8,              // Quality of Service (0, 1, or 2)
    retain: bool,               // Whether broker should store the message for new subscribers
) -> Result<(), String> {
    println!("Attempting to publish to topic: {}", topic);
    let client_state = state.0.lock().await;

    // Convert QoS level from number to enum
    let qos = match qos_level {
        0 => QoS::AtMostOnce,
        1 => QoS::AtLeastOnce,
        2 => QoS::ExactlyOnce,
        _ => return Err("Invalid QoS level. Use 0, 1, or 2.".to_string()),
    };

    // Check if we're connected before trying to publish
    if let Some(client) = &client_state.client {
        // Convert string payload to bytes for MQTT
        let payload_bytes = payload.into_bytes();

        match client.publish(topic.clone(), qos, retain, payload_bytes).await {
             Ok(_) => {
                println!("Successfully published to topic: {}", topic);
                Ok(())
            },
            Err(e) => {
                eprintln!("Failed to publish to topic '{}': {:?}", topic, e);
                Err(format!("Failed to publish: {}", e))
            }
        }
    } else {
        Err("Not connected to MQTT broker.".to_string())
    }
}

// Disconnect from the MQTT broker
#[tauri::command]
pub async fn disconnect_mqtt(
    app_handle: AppHandle,      // For sending events to frontend
    state: State<'_, MqttState>,// Access to MQTT state
) -> Result<(), String> {
    println!("Attempting to disconnect from MQTT broker.");
    let mut client_state = state.0.lock().await;

    // Step 1: Stop the event loop task first (important for clean shutdown)
    if let Some(handle) = client_state.eventloop_task_handle.take() {
        println!("Aborting MQTT event loop task.");
        handle.abort(); // Stop the background task 
        match handle.await {
            Ok(_) => println!("Event loop task successfully aborted."),
            Err(e) if e.is_cancelled() => println!("Event loop task abortion confirmed (cancelled)."),
            Err(e) => eprintln!("Error waiting for event loop task to finish after abort: {:?}", e),
        }
    } else {
        println!("No active event loop task found to abort.");
    }

    // Step 2: Properly disconnect the client
    if let Some(client) = client_state.client.take() {
        match client.disconnect().await {
            Ok(_) => {
                println!("Successfully disconnected MQTT client.");
                // Notify frontend of successful disconnection
                let _ = app_handle.emit("mqtt_event", ConnectionEventPayload {
                    message: "Successfully disconnected.",
                    status: "disconnected",
                });
                Ok(())
            },
            Err(e) => {
                eprintln!("Error during MQTT disconnect: {:?}", e);
                // Still emit disconnected status even if error (we're cleaning up state)
                let _ = app_handle.emit("mqtt_event", ConnectionEventPayload {
                    message: &format!("Error during disconnect: {}", e),
                    status: "disconnected",
                });
                Err(format!("Error disconnecting: {}", e))
            }
        }
    } else {
        println!("No active MQTT client found to disconnect.");
        // Ensure disconnect event is emitted even if client was already gone
        let _ = app_handle.emit("mqtt_event", ConnectionEventPayload {
            message: "Already disconnected or never connected.",
            status: "disconnected",
        });
        Ok(()) // Not an error if already disconnected
    }
}