<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import {
    currentEnvironmentalData,
    connectionStatus,
    isConnected,
    connectionError,
    updateEnvironmentalData,
    updateFromJsonString,
    environmentalHistory
  } from '../../stores/dataStore';

  // Subscribe to stores
  $: data = $currentEnvironmentalData;
  $: error = $connectionError;
  $: connectionState = $connectionStatus;
  $: connected = $isConnected;
  $: history = $environmentalHistory;

  // MQTT event listeners
  let unlistenMqttMessage: Function | null = null;
  let unlistenMqttEvent: Function | null = null;
  
  // Sample data for testing
  const sampleData = {
    temperature: 25.36630058,
    pressure: 992.87,
    humidity: 37.7179985,
    gas: 11.712,
    altitude: 171.0763855
  };

  onMount(async () => {
    try {
      await invoke('connect_mqtt', {
        brokerAddress: 'broker.emqx.io',
        port: 1883,
        clientIdPrefix: 'env-monitor'
      });
    } catch (connectError) {
      console.error('Connection error:', connectError);
      connectionStatus.set(`Error: ${connectError}`);
    }

    unlistenMqttMessage = await listen('mqtt_message', (event) => {
      try {
        const { payload } = event.payload as { topic: string; payload: string };
        // Pass the JSON string directly to our parser function
        updateFromJsonString(payload);
      } catch (e) {
        connectionError.set('Failed to parse incoming data.');
        console.error(e);
      }
    });

    unlistenMqttEvent = await listen('mqtt_event', async (event) => {
      const { message, status } = event.payload as { message: string; status: string };
      connectionStatus.set(message);
      isConnected.set(status === 'connected');

      if (status === 'connected') {
        try {
          // Subscribe to the environmental sensor topic
          await invoke('subscribe_mqtt', { topic: 'tauri/sensor', qosLevel: 0 });
          
          // Load sample data initially
          updateEnvironmentalData(sampleData);
        } catch (subError) {
          console.error('Auto-subscribe error:', subError);
        }
      }
    });
  });

  onDestroy(() => {
    if (unlistenMqttMessage) unlistenMqttMessage();
    if (unlistenMqttEvent) unlistenMqttEvent();
    if (connected) handleDisconnect();
  });

  async function handleDisconnect() {
    try {
      await invoke('disconnect_mqtt');
    } catch (error) {
      console.error('Disconnection error:', error);
    }
  }
  
  // Helper function to get status class based on value thresholds
  function getStatusClass(value: number, lowThreshold: number, highThreshold: number): string {
    if (value < lowThreshold) return 'low';
    if (value > highThreshold) return 'high';
    return 'normal';
  }
  
  // Function to generate mock environmental data with small variations from current
  async function publishMockUpdate() {
    if (!connected || !data) return;
    
    try {
      // Create variations of the current data
      const mockData = {
        temperature: data.temperature + (Math.random() * 0.5 - 0.25),
        pressure: data.pressure + (Math.random() * 1 - 0.5),
        humidity: data.humidity + (Math.random() * 2 - 1),
        gas: data.gas + (Math.random() * 0.8 - 0.4),
        altitude: data.altitude + (Math.random() * 0.5 - 0.25)
      };
      
      // Publish to MQTT
      await invoke('publish_mqtt', {
        // SWAP SUBSCRIBE TO THIS TO LOAD FAKE DATA
        topic: 'tauri/sensor/dev',
        payload: JSON.stringify(mockData),
        qosLevel: 0,
        retain: false
      });
      
      console.log('Published mock data:', mockData);
    } catch (error) {
      console.error('Error publishing mock data:', error);
    }
  }
</script>

<main>
  <h1>Environmental Readings</h1>

  <p class="status">Connection: {connectionState}</p>
  
  {#if error}
    <p class="error">{error}</p>
  {/if}
  
  <div class="actions">
    <button on:click={publishMockUpdate} disabled={!connected || !data}>
      Simulate Update
    </button>
  </div>

  {#if !data}
    <p class="no-data">No sensor data received yet.</p>
  {:else}
    <div class="dashboard">
      <div class="sensor-card">
        <div class="sensor-icon">🌡️</div>
        <h3>Temperature</h3>
        <div class="sensor-value {getStatusClass(data.temperature, 15, 30)}">
          {data.temperature.toFixed(1)} °C
        </div>
        <div class="sensor-details">
          Updated: {data.timestamp ? new Date(data.timestamp).toLocaleTimeString() : 'N/A'}
        </div>
      </div>
      
      <div class="sensor-card">
        <div class="sensor-icon">💧</div>
        <h3>Humidity</h3>
        <div class="sensor-value {getStatusClass(data.humidity, 30, 70)}">
          {data.humidity.toFixed(1)} %
        </div>
        <div class="sensor-details">
          Updated: {data.timestamp ? new Date(data.timestamp).toLocaleTimeString() : 'N/A'}
        </div>
      </div>
      
      <div class="sensor-card">
        <div class="sensor-icon">🔄</div>
        <h3>Pressure</h3>
        <div class="sensor-value {getStatusClass(data.pressure, 980, 1020)}">
          {data.pressure.toFixed(1)} hPa
        </div>
        <div class="sensor-details">
          Updated: {data.timestamp ? new Date(data.timestamp).toLocaleTimeString() : 'N/A'}
        </div>
      </div>
      
      <div class="sensor-card">
        <div class="sensor-icon">💨</div>
        <h3>Gas</h3>
        <div class="sensor-value {getStatusClass(data.gas, 5, 15)}">
          {data.gas.toFixed(2)} kΩ
        </div>
        <div class="sensor-details">
          Updated: {data.timestamp ? new Date(data.timestamp).toLocaleTimeString() : 'N/A'}
        </div>
      </div>
      
      <div class="sensor-card">
        <div class="sensor-icon">🏔️</div>
        <h3>Altitude</h3>
        <div class="sensor-value">
          {data.altitude.toFixed(1)} m
        </div>
        <div class="sensor-details">
          Updated: {data.timestamp ? new Date(data.timestamp).toLocaleTimeString() : 'N/A'}
        </div>
      </div>
    </div>
    
    <h2>Historical Data</h2>
    {#if history.length === 0}
      <p>No historical data available yet.</p>
    {:else}
      <div class="history-container">
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Temperature (°C)</th>
              <th>Humidity (%)</th>
              <th>Pressure (hPa)</th>
              <th>Gas (kΩ)</th>
              <th>Altitude (m)</th>
            </tr>
          </thead>
          <tbody>
            {#each history.slice(0, 10) as entry, i}
              <tr class={i === 0 ? 'newest-entry' : ''}>
                <td>{new Date(entry.timestamp).toLocaleTimeString()}</td>
                <td class={getStatusClass(entry.temperature, 15, 30)}>
                  {entry.temperature.toFixed(1)}
                </td>
                <td class={getStatusClass(entry.humidity, 30, 70)}>
                  {entry.humidity.toFixed(1)}
                </td>
                <td class={getStatusClass(entry.pressure, 980, 1020)}>
                  {entry.pressure.toFixed(1)}
                </td>
                <td class={getStatusClass(entry.gas, 5, 15)}>
                  {entry.gas.toFixed(2)}
                </td>
                <td>{entry.altitude.toFixed(1)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}
  {/if}
</main>

<style>
  .dashboard {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
    gap: 20px;
    margin: 20px 0;
  }
  
  .sensor-card {
    background-color: var(--bg-secondary);
    border-radius: 8px;
    padding: 16px;
    box-shadow: var(--card-shadow);
    text-align: center;
  }
  
  .sensor-icon {
    font-size: 32px;
    margin-bottom: 8px;
  }
  
  .sensor-value {
    font-size: 24px;
    font-weight: bold;
    margin: 10px 0;
  }
  
  .sensor-value.high {
    color: var(--error-color);
  }
  
  .sensor-value.low {
    color: var(--warning-color);
  }
  
  .sensor-value.normal {
    color: var(--success-color);
  }
  
  .sensor-details {
    font-size: 12px;
    color: var(--text-muted);
  }
  
  .history-container {
    margin-top: 20px;
    overflow-x: auto;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
  }
  
  th, td {
    padding: 8px 12px;
    text-align: right;
    border-bottom: 1px solid var(--border-color);
  }
  
  th {
    background-color: var(--bg-tertiary);
    text-align: center;
    font-weight: bold;
  }
  
  tr.newest-entry {
    background-color: var(--bg-tertiary);
    font-weight: bold;
  }
  
  .actions {
    margin: 20px 0;
  }
  
  button {
    background-color: var(--accent-color);
    color: white;
    border: none;
    padding: 8px 16px;
    border-radius: 4px;
    cursor: pointer;
  }
  
  button:hover {
    background-color: var(--accent-hover);
  }
  
  button:disabled {
    background-color: var(--text-muted);
    cursor: not-allowed;
  }
  
  .error {
    color: var(--error-color);
    font-weight: bold;
  }
  
  .no-data {
    padding: 40px;
    text-align: center;
    background-color: var(--bg-secondary);
    border-radius: 8px;
    margin: 20px 0;
  }
</style>