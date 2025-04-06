<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import { listen } from '@tauri-apps/api/event';
  import {
    radarTargets,
    connectionStatus,
    isConnected,
    connectionError,
    updateTargets
  } from '../../stores/radarStore';

  $: targets = $radarTargets;
  $: error = $connectionError;
  $: connectionState = $connectionStatus;
  $: connected = $isConnected;

  let unlistenMqttMessage: Function | null = null;
  let unlistenMqttEvent: Function | null = null;
  let publishInterval: number | null = null;

  // Sweep animation state
  let sweepAngle = -90;
  let sweepTimer: number;

  onMount(() => {
    // Radar sweep rotates once every 10s
    const scanDuration = 5000;
    const fps = 60;
    const totalSteps = (scanDuration / 1000) * fps;
    const step = 360 / totalSteps;

    sweepTimer = window.setInterval(() => {
      sweepAngle = (sweepAngle + step) % 360;
    }, 1000 / fps);
  });

  onDestroy(() => {
    if (unlistenMqttMessage) unlistenMqttMessage();
    if (unlistenMqttEvent) unlistenMqttEvent();
    if (publishInterval !== null) window.clearInterval(publishInterval);
    if (sweepTimer) window.clearInterval(sweepTimer);
    if (connected) handleDisconnect();
  });

  function generateMockData() {
    const movementStates = ['approaching', 'stationary', 'departing', 'crossing'];
    const mockData = [
      {
        id: 1,
        distance: 2.35 + (Math.random() * 0.8 - 0.4),
        angle: -15.2 + (Math.random() * 10 - 5),
        confidence: Math.floor(150 + Math.random() * 100),
        movement: movementStates[Math.floor(Math.random() * 3)],
        velocity: Math.random() * 0.8,
        isActive: Math.random() > 0.05
      },
      {
        id: 2,
        distance: 3.8 + (Math.random() * 1.2 - 0.6),
        angle: 5.6 + (Math.random() * 12 - 6),
        confidence: Math.floor(120 + Math.random() * 120),
        movement: movementStates[Math.floor(Math.random() * 4)],
        velocity: Math.random() * 0.6,
        isActive: Math.random() > 0.08
      }
    ];

    if (Math.random() > 0.75) {
      mockData.push({
        id: 3,
        distance: 5.2 + (Math.random() * 1.5 - 0.75),
        angle: 30 + (Math.random() * 30 - 15),
        confidence: Math.floor(100 + Math.random() * 150),
        movement: movementStates[Math.floor(Math.random() * 4)],
        velocity: Math.random() * 1.0,
        isActive: Math.random() > 0.1
      });
    }

    if (Math.random() > 0.85) {
      mockData.push({
        id: 4,
        distance: 4.0 + (Math.random() * 2.0 - 1.0),
        angle: -40 + (Math.random() * 20 - 10),
        confidence: Math.floor(80 + Math.random() * 120),
        movement: movementStates[Math.floor(Math.random() * 4)],
        velocity: Math.random() * 1.2,
        isActive: Math.random() > 0.15
      });
    }

    if (Math.random() > 0.95) {
      mockData.push({
        id: 5,
        distance: 6.5 + (Math.random() * 1.0 - 0.5),
        angle: 60 + (Math.random() * 25 - 12.5),
        confidence: Math.floor(70 + Math.random() * 80),
        movement: movementStates[Math.floor(Math.random() * 4)],
        velocity: Math.random() * 0.4,
        isActive: Math.random() > 0.2
      });
    }

    mockData.forEach(target => {
      if (!target.isActive) {
        target.confidence = Math.floor(target.confidence * 0.6);
        target.velocity = 0;
        target.movement = 'stationary';
      } else if (target.movement === 'stationary') {
        target.velocity = Math.min(target.velocity, 0.1);
      } else if (target.movement === 'departing') {
        target.distance += 0.2;
      } else if (target.movement === 'approaching') {
        target.distance -= 0.1;
      }
    });

    return mockData;
  }

  async function publishMockData() {
    if (!connected) return;

    try {
      const mockTargets = generateMockData();
      await invoke('publish_mqtt', {
        topic: 'test5/tauri/radar',
        payload: JSON.stringify({ targets: mockTargets }),
        qosLevel: 0,
        retain: false
      });

      console.log('Published mock data:', mockTargets);
    } catch (error) {
      console.error('Error publishing mock data:', error);
    }
  }

  onMount(async () => {
    try {
      await invoke('connect_mqtt', {
        brokerAddress: 'broker.emqx.io',
        port: 1883,
        clientIdPrefix: undefined
      });
    } catch (connectError) {
      console.error('Connection error:', connectError);
      connectionStatus.set(`Error: ${connectError}`);
    }

    unlistenMqttMessage = await listen('mqtt_message', (event) => {
      try {
        const { payload } = event.payload as { topic: string; payload: string };
        const parsed = JSON.parse(payload);
        if (Array.isArray(parsed.targets)) {
          updateTargets(parsed.targets);
        }
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
          await invoke('subscribe_mqtt', { topic: 'test5/tauri/radar', qosLevel: 0 });
          await publishMockData();
          publishInterval = window.setInterval(publishMockData, 5000);
        } catch (subError) {
          console.error('Auto-subscribe or publish error:', subError);
        }
      }
    });
  });

  async function handleDisconnect() {
    try {
      await invoke('disconnect_mqtt');
    } catch (error) {
      console.error('Disconnection error:', error);
    }
  }
</script>

<main>
  <h1>Radar Visualization</h1>

  <p class="status">Status: {connectionState}</p>

  {#if targets.length === 0}
    <p>No target data received yet.</p>
  {:else}
    <svg width="400" height="400" viewBox="-200 -200 400 400" style="background:#111; border-radius: 50%;">
      <circle cx="0" cy="0" r="190" stroke="#444" stroke-width="1" fill="none" />
      <line x1="-190" y1="0" x2="190" y2="0" stroke="#333" stroke-width="0.5" />
      <line x1="0" y1="-190" x2="0" y2="190" stroke="#333" stroke-width="0.5" />
      <line x1="-135" y1="-135" x2="135" y2="135" stroke="#333" stroke-width="0.5" />
      <line x1="-135" y1="135" x2="135" y2="-135" stroke="#333" stroke-width="0.5" />
      <circle cx="0" cy="0" r="4" fill="white" />

      <!-- Radar sweeping line -->
      <line
        x1="0"
        y1="0"
        x2={190 * Math.cos((sweepAngle * Math.PI) / 180)}
        y2={190 * Math.sin((sweepAngle * Math.PI) / 180)}
        stroke="lime"
        stroke-width="2"
        stroke-opacity="0.8"
      />

      {#each targets as t}
        <circle
          r={t.isActive ? 6 : 4}
          fill={t.movement === 'approaching' ? 'green' :
                t.movement === 'departing' ? 'orange' :
                t.movement === 'crossing' ? 'blue' : 'red'}
          cx={t.distance * 30 * Math.cos((t.angle * Math.PI) / 180)}
          cy={t.distance * 30 * Math.sin((t.angle * Math.PI) / 180)}
          opacity={t.isActive ? 1 : 0.5}
        >
          <title>ID: {t.id}
Distance: {t.distance.toFixed(2)}m
Angle: {t.angle.toFixed(1)}°
Movement: {t.movement}
Confidence: {t.confidence}</title>
        </circle>
      {/each}
    </svg>

    <div style="margin: 1rem 0; font-size: 0.9rem;">
      <span style="display: inline-block; width: 12px; height: 12px; background: green; border-radius: 50%; margin-right: 6px;"></span>Approaching
      <span style="display: inline-block; width: 12px; height: 12px; background: red; border-radius: 50%; margin: 0 6px 0 12px;"></span>Stationary
      <span style="display: inline-block; width: 12px; height: 12px; background: orange; border-radius: 50%; margin: 0 6px 0 12px;"></span>Departing
      <span style="display: inline-block; width: 12px; height: 12px; background: blue; border-radius: 50%; margin: 0 6px 0 12px;"></span>Crossing
      <span style="display: inline-block; width: 12px; height: 12px; background: red; opacity: 0.5; border-radius: 50%; margin: 0 6px 0 12px;"></span>Inactive
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
            <td>{t.distance.toFixed(2)}</td>
            <td>{t.angle.toFixed(1)}</td>
            <td>{t.confidence}</td>
            <td>{t.movement}</td>
            <td>{t.velocity.toFixed(2)}</td>
          </tr>
        {/each}
      </tbody>
    </table>
  {/if}

  {#if error}
    <p style="color: red;">{error}</p>
  {/if}
</main>
