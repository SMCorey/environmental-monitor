// src/lib.rs
mod mqtt;
use tokio::sync::Mutex;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Create the MQTT state
    let mqtt_state = mqtt::MqttState(Mutex::new(Default::default()));

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // Manage the MQTT state
        .manage(mqtt_state)
        // Register ALL commands
        .invoke_handler(tauri::generate_handler![
            greet,
            mqtt::connect_mqtt,
            mqtt::subscribe_mqtt,
            mqtt::publish_mqtt,
            mqtt::disconnect_mqtt
        ])
        // No setup hook - register events directly in run
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}