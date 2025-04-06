<script lang="ts">
    import { onMount, onDestroy } from 'svelte';
    import { invoke } from '@tauri-apps/api/core';
    import { listen } from '@tauri-apps/api/event';
  
    // Connection state
    let brokerAddress = $state("broker.emqx.io");
    let port = $state(1883);
    let clientId = $state("");
    let isConnected = $state(false);
    let connectionStatus = $state("Disconnected");
    
    // Subscription state
    let topic = $state("test/topic");
    let qosLevel = $state(0);
    let subscriptions = $state<string[]>([]);
    
    // Publishing state
    let publishTopic = $state("test5/tauri/radar");
    let publishMessage = $state("Hello MQTT");
    let publishQosLevel = $state(0);
    let publishRetain = $state(false);
    
    // Messages
    let messages = $state<{topic: string, payload: string, timestamp: string}[]>([]);
    
    // Unlisten functions
    let unlistenMqttEvent: Function | null = null;
    let unlistenMqttMessage: Function | null = null;
    let unlistenMqttSubscription: Function | null = null;
    
    onMount(async () => {
      // Listen for MQTT events (connection status changes)
      unlistenMqttEvent = await listen('mqtt_event', (event) => {
        const { message, status } = event.payload as { message: string, status: string };
        connectionStatus = message;
        isConnected = status === 'connected';
      });
      
      // Listen for MQTT messages
      unlistenMqttMessage = await listen('mqtt_message', (event) => {
        const { topic, payload } = event.payload as { topic: string, payload: string };
        const timestamp = new Date().toLocaleTimeString();
        messages = [{ topic, payload, timestamp }, ...messages];
      });
      
      // Listen for subscription confirmations
      unlistenMqttSubscription = await listen('mqtt_subscription', (event) => {
        const { topic } = event.payload as { topic: string, qos: number };
        if (!subscriptions.includes(topic)) {
          subscriptions = [...subscriptions, topic];
        }
      });
    });
    
    onDestroy(() => {
      // Clean up event listeners
      if (unlistenMqttEvent) unlistenMqttEvent();
      if (unlistenMqttMessage) unlistenMqttMessage();
      if (unlistenMqttSubscription) unlistenMqttSubscription();
      
      // Disconnect if connected
      if (isConnected) {
        handleDisconnect();
      }
    });
    
    async function handleConnect() {
      try {
        await invoke('connect_mqtt', { 
          brokerAddress, 
          port, 
          clientIdPrefix: clientId || undefined 
        });
      } catch (error) {
        console.error('Connection error:', error);
        connectionStatus = `Error: ${error}`;
      }
    }
    
    async function handleDisconnect() {
      try {
        await invoke('disconnect_mqtt');
        subscriptions = [];
      } catch (error) {
        console.error('Disconnection error:', error);
      }
    }
    
    async function handleSubscribe() {
      if (!isConnected) {
        alert('Please connect to a broker first');
        return;
      }
      
      try {
        await invoke('subscribe_mqtt', { topic, qosLevel });
      } catch (error) {
        console.error('Subscription error:', error);
      }
    }
    
    async function handlePublish() {
      if (!isConnected) {
        alert('Please connect to a broker first');
        return;
      }
      
      try {
        await invoke('publish_mqtt', { 
          topic: publishTopic, 
          payload: publishMessage,
          qosLevel: publishQosLevel,
          retain: publishRetain
        });
        console.log('Message published');
      } catch (error) {
        console.error('Publish error:', error);
      }
    }
    
    function clearMessages() {
      messages = [];
    }
  </script>
  
  <main>
    <h1>MQTT Client</h1>
    
    <div class="card">
      <h2>Connection</h2>
      <div class="form-group">
        <label for="broker">Broker:</label>
        <input id="broker" type="text" bind:value={brokerAddress} disabled={isConnected} />
      </div>
      <div class="form-group">
        <label for="port">Port:</label>
        <input id="port" type="number" bind:value={port} disabled={isConnected} />
      </div>
      <div class="form-group">
        <label for="client-id">Client ID Prefix (optional):</label>
        <input id="client-id" type="text" bind:value={clientId} disabled={isConnected} />
      </div>
      <div class="status">Status: {connectionStatus}</div>
      <div class="button-group">
        {#if !isConnected}
          <button on:click={handleConnect}>Connect</button>
        {:else}
          <button on:click={handleDisconnect}>Disconnect</button>
        {/if}
      </div>
    </div>
    
    <div class="card">
      <h2>Subscribe</h2>
      <div class="form-group">
        <label for="topic">Topic:</label>
        <input id="topic" type="text" bind:value={topic} disabled={!isConnected} />
      </div>
      <div class="form-group">
        <label for="qos">QoS:</label>
        <select id="qos" bind:value={qosLevel} disabled={!isConnected}>
          <option value={0}>0 - At most once</option>
          <option value={1}>1 - At least once</option>
          <option value={2}>2 - Exactly once</option>
        </select>
      </div>
      <button on:click={handleSubscribe} disabled={!isConnected}>Subscribe</button>
      
      {#if subscriptions.length > 0}
        <div class="subscriptions">
          <h3>Active Subscriptions</h3>
          <ul>
            {#each subscriptions as sub}
              <li>{sub}</li>
            {/each}
          </ul>
        </div>
      {/if}
    </div>
    
    <div class="card">
      <h2>Publish</h2>
      <div class="form-group">
        <label for="pub-topic">Topic:</label>
        <input id="pub-topic" type="text" bind:value={publishTopic} disabled={!isConnected} />
      </div>
      <div class="form-group">
        <label for="pub-message">Message:</label>
        <textarea id="pub-message" bind:value={publishMessage} disabled={!isConnected}></textarea>
      </div>
      <div class="form-group">
        <label for="pub-qos">QoS:</label>
        <select id="pub-qos" bind:value={publishQosLevel} disabled={!isConnected}>
          <option value={0}>0 - At most once</option>
          <option value={1}>1 - At least once</option>
          <option value={2}>2 - Exactly once</option>
        </select>
      </div>
      <div class="form-group checkbox">
        <input id="pub-retain" type="checkbox" bind:checked={publishRetain} disabled={!isConnected} />
        <label for="pub-retain">Retain message</label>
      </div>
      <button on:click={handlePublish} disabled={!isConnected}>Publish</button>
    </div>
    
    <div class="card">
      <div class="message-header">
        <h2>Messages</h2>
        <button class="small" on:click={clearMessages}>Clear</button>
      </div>
      
      {#if messages.length === 0}
        <div class="no-messages">No messages received yet</div>
      {:else}
        <div class="message-list">
          {#each messages as message}
            <div class="message-item">
              <div class="message-topic">{message.topic}</div>
              <div class="message-time">{message.timestamp}</div>
              <div class="message-payload">{message.payload}</div>
            </div>
          {/each}
        </div>
      {/if}
    </div>
  </main>