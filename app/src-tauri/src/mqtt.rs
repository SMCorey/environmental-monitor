use rumqttc::{MqttOptions, AsyncClient, QoS};
use tokio::{task, time};
use std::time::Duration;


pub async fn mqtt_test() {
    let random_id = rand::rng();

    // sets up connection to the MQTT Broker
    let mut mqttoptions = MqttOptions::new(format!("rumqtt-async-{:?}", random_id), "broker.emqx.io", 1883);
    mqttoptions.set_keep_alive(Duration::from_secs(5));
    mqttoptions.set_clean_session(true);

    // sets up client and subscribes to a topic
    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);
    client.subscribe("rust/mqttest2", QoS::AtLeastOnce).await.unwrap();

    // asynchronously publishes messages to the topic
    task::spawn(async move {
        for i in 0..5 {
            // publish to specific topic, QoS level, retaining, and the actual payload (sensor data will go here)
            match client.publish("rust/mqttest2", QoS::AtLeastOnce, false, "hello from rust!").await {
                Ok(_) => println!("Published message {}", i),
                Err(e) => println!("Error publishing message {}: {:?}", i, e),
            };
            time::sleep(Duration::from_millis(100)).await;
        }
    });

    // Repeatedly executes the loop body as long as the result from poll is OK (Err stops the loop)
    while let Ok(notification) = eventloop.poll().await {
        println!("Received notification: {:?}", notification);

        // Extract the payload if it's a Publish notification
        if let rumqttc::Event::Incoming(rumqttc::Packet::Publish(publish)) = notification {
            println!("Topic: {}", publish.topic);
            println!("QoS: {:?}", publish.qos);

            // Try to interpret the payload in different ways
            println!("Payload (raw bytes): {:?}", publish.payload);

            // As UTF-8 string (if it's text)
            match std::str::from_utf8(&publish.payload) {
                Ok(text) => println!("Payload (as text): {}", text),
                Err(_) => println!("Payload is not valid UTF-8 text")
            }

            // Print individual bytes if it's a small binary payload
            if publish.payload.len() <= 20 {
                println!("Payload (as byte values): {:?}", publish.payload);
            }

            println!("-----------------------");

        }
    }
}