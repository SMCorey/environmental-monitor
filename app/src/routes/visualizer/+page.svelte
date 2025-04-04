<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';

  // Reactive state to hold incoming radar targets
  let targets = $state<{
    id: number;
    distance: number;
    angle: number;
    confidence: number;
    movement: string;
    velocity: number;
    isActive: boolean;
  }[]>([]);

  // Reactive state for connection errors
  let error = $state("");

  // Connection status
  let isConnected = $state(false);
  let connectionStatus = $state("Disconnected");

  // Functions to remove MQTT listeners when unmounting
  let unlistenMqttMessage: Function | null = null;
  let unlistenMqttEvent: Function | null = null;

  // Temporary mock data for publishing after connection
  const mockTargets = [
    {
      id: 1,
      distance: 2.35,
      angle: -15.2,
      confidence: 198,
      movement: "approaching",
      velocity: 0.2,
      isActive: true
    },
    {
      id: 2,
      distance: 3.8,
      angle: 5.6,
      confidence: 172,
      movement: "stationary",
      velocity: 0,
      isActive: true
    }
  ];

  // Set up MQTT listeners and auto-connect on mount
  onMount(async () => {
    try {
      await invoke('connect_mqtt', {
        brokerAddress: "broker.emqx.io",
        port: 1883,
        clientIdPrefix: undefined
      });
    } catch (connectError) {
      console.error('Connection error:', connectError);
      connectionStatus = `Error: ${connectError}`;
    }

    unlistenMqttMessage = await listen('mqtt_message', (event) => {
      try {
        const { payload } = event.payload as { topic: string; payload: string };
        const parsed = JSON.parse(payload);
        if (Array.isArray(parsed.targets)) {
          targets = parsed.targets;
        }
      } catch (e) {
        error = "Failed to parse incoming data.";
        console.error(e);
      }
    });

    unlistenMqttEvent = await listen('mqtt_event', async (event) => {
      const { message, status } = event.payload as { message: string, status: string };
      connectionStatus = message;
      isConnected = status === 'connected';
      if (isConnected) {
        try {
          await invoke('subscribe_mqtt', { topic: "test/topic", qosLevel: 0 });

          // Publish mock data after subscribing
          await invoke('publish_mqtt', {
            topic: "test/topic",
            payload: JSON.stringify({ targets: mockTargets }),
            qosLevel: 0,
            retain: false
          });

        } catch (subError) {
          console.error('Auto-subscribe or publish error:', subError);
        }
      }
    });
  });

  // Disconnect and cleanup when component is destroyed
  onDestroy(() => {
    if (unlistenMqttMessage) unlistenMqttMessage();
    if (unlistenMqttEvent) unlistenMqttEvent();
    if (isConnected) handleDisconnect();
  });

  // Disconnect from MQTT broker
  async function handleDisconnect() {
    try {
      await invoke('disconnect_mqtt');
    } catch (error) {
      console.error('Disconnection error:', error);
    }
  }
</script>

<main>
  <h1>Radar Target Visualization</h1>

  <p class="status">Status: {connectionStatus}</p>

  {#if targets.length === 0}
    <p>No target data received yet.</p>
  {:else}
    <!-- Radar-style visualization with gridlines and center point -->
    <svg width="400" height="400" viewBox="-200 -200 400 400" style="background:#eef; border-radius: 50%;">
      <circle cx="0" cy="0" r="190" stroke="#ccc" stroke-width="1" fill="none" />
      <line x1="-190" y1="0" x2="190" y2="0" stroke="#ccc" stroke-width="0.5" />
      <line x1="0" y1="-190" x2="0" y2="190" stroke="#ccc" stroke-width="0.5" />
      <line x1="-135" y1="-135" x2="135" y2="135" stroke="#eee" stroke-width="0.5" />
      <line x1="-135" y1="135" x2="135" y2="-135" stroke="#eee" stroke-width="0.5" />
      <circle cx="0" cy="0" r="4" fill="black" />

      {#each targets as t}
        <circle
          r="6"
          fill={t.movement === 'approaching' ? 'green' : 'red'}
          cx={t.distance * 30 * Math.cos((t.angle * Math.PI) / 180)}
          cy={t.distance * 30 * Math.sin((t.angle * Math.PI) / 180)}
        >
          <title>ID: {t.id}
Distance: {t.distance}m
Angle: {t.angle}°</title>
        </circle>
      {/each}
    </svg>

    <!-- Legend -->
    <div style="margin: 1rem 0; font-size: 0.9rem;">
      <span style="display: inline-block; width: 12px; height: 12px; background: green; border-radius: 50%; margin-right: 6px;"></span>Approaching
      <span style="display: inline-block; width: 12px; height: 12px; background: red; border-radius: 50%; margin: 0 6px 0 12px;"></span>Stationary
    </div>

    <table>
      <thead>
        <tr>
          <th>ID</th>
          <th>Distance (m)</th>
          <th>Angle (°)</th>
          <th>Confidence</th>
          <th>Movement</th>
          <th>Velocity (m/s)</th>
        </tr>
      </thead>
      <tbody>
        {#each targets as t}
          <tr>
            <td>{t.id}</td>
            <td>{t.distance}</td>
            <td>{t.angle}</td>
            <td>{t.confidence}</td>
            <td>{t.movement}</td>
            <td>{t.velocity}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}

  {#if error}
    <p style="color: red;">{error}</p>
  {/if}
</main>

<style>
  main {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem;
  }

  .status {
    margin-bottom: 1rem;
    font-weight: 500;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 1rem;
  }

  th, td {
    border: 1px solid #ddd;
    padding: 0.5rem;
    text-align: center;
  }

  th {
    background-color: #f4f4f4;
  }

  @media (prefers-color-scheme: dark) {
    th {
      background-color: #1f2937;
      color: #f9fafb;
    }
    td {
      background-color: #111827;
      color: #f9fafb;
    }
  }
</style>
