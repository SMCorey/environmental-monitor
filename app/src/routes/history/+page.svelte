<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    environmentalHistory,
    clearHistory,
    currentEnvironmentalData,
  } from "../../stores/dataStore";
  import type {
    EnvironmentalHistoryEntry,
    EnvironmentalData,
  } from "../../stores/dataStore";

  // Variables for storing reactive data
  let historyEntries: EnvironmentalHistoryEntry[] = [];
  let currentData: EnvironmentalData | null = null;

  // Subscribe to store changes
  $: historyEntries = $environmentalHistory;
  $: currentData = $currentEnvironmentalData;

  // Auto-refresh to ensure we get updates
  let refreshInterval: number | null = null;

  onMount(() => {
    // Set a lightweight refresh interval to ensure the UI stays updated
    refreshInterval = window.setInterval(() => {
      // This will trigger reactivity even if the underlying data structure reference hasn't changed
      historyEntries = [...$environmentalHistory];
    }, 5000);
  });

  onDestroy(() => {
    if (refreshInterval !== null) {
      window.clearInterval(refreshInterval);
    }
  });

  // Format timestamp for display
  function formatTimestamp(timestamp: string): string {
    const date = new Date(timestamp);
    return date.toLocaleString();
  }

  // Function to handle clearing history
  function handleClearHistory(): void {
    if (confirm("Are you sure you want to clear all history?")) {
      clearHistory();
    }
  }

  // Determine status class based on thresholds
  function getStatusClass(type: string, value: number): string {
    switch (type) {
      case 'temperature':
        return value < 15 ? 'low' : value > 30 ? 'high' : 'normal';
      case 'humidity':
        return value < 30 ? 'low' : value > 70 ? 'high' : 'normal';
      case 'pressure':
        return value < 980 ? 'low' : value > 1020 ? 'high' : 'normal';
      case 'gas':
        return value < 5 ? 'low' : value > 15 ? 'high' : 'normal';
      default:
        return 'normal';
    }
  }

  // Format value with appropriate unit
  function formatValue(type: string, value: number): string {
    switch (type) {
      case 'temperature':
        return `${value.toFixed(1)} °C`;
      case 'humidity':
        return `${value.toFixed(1)} %`;
      case 'pressure':
        return `${value.toFixed(1)} hPa`;
      case 'gas':
        return `${value.toFixed(2)} kΩ`;
      case 'altitude':
        return `${value.toFixed(1)} m`;
      default:
        return value.toFixed(2);
    }
  }

  // Calculate time elapsed since a specific timestamp
  function timeElapsed(timestamp: string): string {
    const elapsed = Date.now() - new Date(timestamp).getTime();
    
    if (elapsed < 60000) { // Less than a minute
      return `${Math.floor(elapsed / 1000)} seconds ago`;
    } else if (elapsed < 3600000) { // Less than an hour
      return `${Math.floor(elapsed / 60000)} minutes ago`;
    } else if (elapsed < 86400000) { // Less than a day
      return `${Math.floor(elapsed / 3600000)} hours ago`;
    } else {
      return `${Math.floor(elapsed / 86400000)} days ago`;
    }
  }

  // Group history entries by time intervals
  function groupHistoryByTime(history: EnvironmentalHistoryEntry[]) {
    const now = new Date();
    
    // Create time buckets
    const lastHour: EnvironmentalHistoryEntry[] = [];
    const today: EnvironmentalHistoryEntry[] = [];
    const yesterday: EnvironmentalHistoryEntry[] = [];
    const older: EnvironmentalHistoryEntry[] = [];
    
    history.forEach(entry => {
      const entryTime = new Date(entry.timestamp);
      const diffMs = now.getTime() - entryTime.getTime();
      const diffHours = diffMs / (1000 * 60 * 60);
      
      if (diffHours < 1) {
        lastHour.push(entry);
      } else if (
        entryTime.getDate() === now.getDate() &&
        entryTime.getMonth() === now.getMonth() &&
        entryTime.getFullYear() === now.getFullYear()
      ) {
        today.push(entry);
      } else if (
        entryTime.getDate() === now.getDate() - 1 &&
        entryTime.getMonth() === now.getMonth() &&
        entryTime.getFullYear() === now.getFullYear()
      ) {
        yesterday.push(entry);
      } else {
        older.push(entry);
      }
    });
    
    return { lastHour, today, yesterday, older };
  }

  // Process history entries by time groups
  $: timeGroups = groupHistoryByTime(historyEntries);
</script>

<main>
  <h1>Environmental Data History</h1>

  <div class="actions">
    <button on:click={handleClearHistory} class="clear-btn">Clear History</button>
  </div>

  {#if historyEntries.length === 0}
    <div class="empty-state">
      <p>No environmental data history recorded yet.</p>
      <p class="hint">
        History is recorded automatically when sensor readings are received.
      </p>
    </div>
  {:else}
    <div class="history-stats">
      <div class="stat">
        <span class="stat-value">{historyEntries.length}</span>
        <span class="stat-label">Total Readings</span>
      </div>
      <div class="stat">
        <span class="stat-value">
          {historyEntries.length > 0
            ? formatTimestamp(historyEntries[0].timestamp)
            : "N/A"}
        </span>
        <span class="stat-label">Latest Reading</span>
      </div>
      <div class="stat">
        <span class="stat-value">
          {historyEntries.length > 0
            ? formatTimestamp(historyEntries[historyEntries.length - 1].timestamp)
            : "N/A"}
        </span>
        <span class="stat-label">First Reading</span>
      </div>
    </div>

    <!-- Current readings (if available) -->
    {#if currentData}
      <div class="current-readings">
        <h2>Current Readings</h2>
        <div class="reading-grid">
          <div class="reading-card">
            <div class="reading-icon">🌡️</div>
            <div class="reading-name">Temperature</div>
            <div class="reading-value {getStatusClass('temperature', currentData.temperature)}">
              {formatValue('temperature', currentData.temperature)}
            </div>
          </div>
          <div class="reading-card">
            <div class="reading-icon">💧</div>
            <div class="reading-name">Humidity</div>
            <div class="reading-value {getStatusClass('humidity', currentData.humidity)}">
              {formatValue('humidity', currentData.humidity)}
            </div>
          </div>
          <div class="reading-card">
            <div class="reading-icon">🔄</div>
            <div class="reading-name">Pressure</div>
            <div class="reading-value {getStatusClass('pressure', currentData.pressure)}">
              {formatValue('pressure', currentData.pressure)}
            </div>
          </div>
          <div class="reading-card">
            <div class="reading-icon">💨</div>
            <div class="reading-name">Gas</div>
            <div class="reading-value {getStatusClass('gas', currentData.gas)}">
              {formatValue('gas', currentData.gas)}
            </div>
          </div>
          <div class="reading-card">
            <div class="reading-icon">🏔️</div>
            <div class="reading-name">Altitude</div>
            <div class="reading-value">
              {formatValue('altitude', currentData.altitude)}
            </div>
          </div>
        </div>
      </div>
    {/if}

    <!-- Recent History (Last Hour) -->
    {#if timeGroups.lastHour.length > 0}
      <div class="history-section">
        <h2>Recent History (Last Hour)</h2>
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Temperature</th>
              <th>Humidity</th>
              <th>Pressure</th>
              <th>Gas</th>
              <th>Altitude</th>
            </tr>
          </thead>
          <tbody>
            {#each timeGroups.lastHour as entry}
              <tr>
                <td>{formatTimestamp(entry.timestamp)} ({timeElapsed(entry.timestamp)})</td>
                <td class={getStatusClass('temperature', entry.temperature)}>
                  {formatValue('temperature', entry.temperature)}
                </td>
                <td class={getStatusClass('humidity', entry.humidity)}>
                  {formatValue('humidity', entry.humidity)}
                </td>
                <td class={getStatusClass('pressure', entry.pressure)}>
                  {formatValue('pressure', entry.pressure)}
                </td>
                <td class={getStatusClass('gas', entry.gas)}>
                  {formatValue('gas', entry.gas)}
                </td>
                <td>{formatValue('altitude', entry.altitude)}</td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    {/if}

    <!-- Today's History -->
    {#if timeGroups.today.length > 0}
      <div class="history-section">
        <h2>Today's History</h2>
        <table>
          <thead>
            <tr>
              <th>Time</th>
              <th>Temperature</th>
              <th>Humidity</th>
              <th>Pressure</th>
              <th>Gas</th>
              <th>Altitude</th>
            </tr>
          </thead>
          <tbody>
            {#each timeGroups.today.slice(0, 5) as entry}
              <tr>
                <td>{formatTimestamp(entry.timestamp)}</td>
                <td class={getStatusClass('temperature', entry.temperature)}>
                  {formatValue('temperature', entry.temperature)}
                </td>
                <td class={getStatusClass('humidity', entry.humidity)}>
                  {formatValue('humidity', entry.humidity)}
                </td>
                <td class={getStatusClass('pressure', entry.pressure)}>
                  {formatValue('pressure', entry.pressure)}
                </td>
                <td class={getStatusClass('gas', entry.gas)}>
                  {formatValue('gas', entry.gas)}
                </td>
                <td>{formatValue('altitude', entry.altitude)}</td>
              </tr>
            {/each}
            
            {#if timeGroups.today.length > 5}
              <tr>
                <td colspan="6" class="show-more">
                  <button class="show-more-btn">
                    Show {timeGroups.today.length - 5} more entries...
                  </button>
                </td>
              </tr>
            {/if}
          </tbody>
        </table>
      </div>
    {/if}

    <!-- Earlier History -->
    {#if timeGroups.yesterday.length > 0 || timeGroups.older.length > 0}
      <div class="history-section">
        <h2>Earlier History</h2>
        <div class="history-summary">
          {#if timeGroups.yesterday.length > 0}
            <div class="summary-item">
              <span class="summary-value">{timeGroups.yesterday.length}</span>
              <span class="summary-label">readings from yesterday</span>
            </div>
          {/if}
          {#if timeGroups.older.length > 0}
            <div class="summary-item">
              <span class="summary-value">{timeGroups.older.length}</span>
              <span class="summary-label">older readings</span>
            </div>
          {/if}
          <button class="show-all-btn">View All History</button>
        </div>
      </div>
    {/if}
  {/if}
</main>

<style>
  main {
    padding: 1rem;
  }

  h1 {
    margin-bottom: 1.5rem;
  }

  h2 {
    margin: 1.5rem 0 0.75rem;
  }

  .actions {
    margin-bottom: 1.5rem;
  }

  .clear-btn {
    background-color: var(--error-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
  }

  .clear-btn:hover {
    opacity: 0.9;
  }

  .empty-state {
    background-color: var(--bg-secondary);
    padding: 2rem;
    border-radius: 8px;
    text-align: center;
    margin: 2rem 0;
  }

  .hint {
    color: var(--text-muted);
    margin-top: 0.5rem;
  }

  .history-stats {
    display: flex;
    flex-wrap: wrap;
    gap: 1rem;
    margin-bottom: 2rem;
  }

  .stat {
    background-color: var(--bg-secondary);
    padding: 1rem;
    border-radius: 8px;
    flex: 1;
    min-width: 180px;
    box-shadow: var(--card-shadow);
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-value {
    font-size: 1.25rem;
    font-weight: bold;
    margin-bottom: 0.25rem;
  }

  .stat-label {
    color: var(--text-muted);
    font-size: 0.9rem;
  }

  .current-readings {
    margin: 1.5rem 0;
  }

  .reading-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(180px, 1fr));
    gap: 1rem;
  }

  .reading-card {
    background-color: var(--bg-secondary);
    padding: 1rem;
    border-radius: 8px;
    box-shadow: var(--card-shadow);
    text-align: center;
  }

  .reading-icon {
    font-size: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .reading-name {
    font-size: 0.9rem;
    color: var(--text-muted);
    margin-bottom: 0.5rem;
  }

  .reading-value {
    font-size: 1.25rem;
    font-weight: bold;
  }

  .reading-value.high {
    color: var(--error-color);
  }

  .reading-value.low {
    color: var(--warning-color);
  }

  .reading-value.normal {
    color: var(--success-color);
  }

  .history-section {
    margin: 2rem 0;
  }

  table {
    width: 100%;
    border-collapse: collapse;
    margin: 1rem 0;
    background-color: var(--bg-secondary);
    border-radius: 8px;
    overflow: hidden;
    box-shadow: var(--card-shadow);
  }

  th {
    background-color: var(--bg-tertiary);
    padding: 0.75rem;
    text-align: left;
    font-weight: bold;
    color: var(--text-primary);
  }

  td {
    padding: 0.75rem;
    border-top: 1px solid var(--border-color);
  }

  tr:nth-child(even) {
    background-color: var(--bg-tertiary);
  }

  td.high {
    color: var(--error-color);
  }

  td.low {
    color: var(--warning-color);
  }

  td.normal {
    color: var(--success-color);
  }

  .show-more {
    text-align: center;
    padding: 0.5rem;
  }

  .show-more-btn, .show-all-btn {
    background-color: var(--accent-color);
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    cursor: pointer;
    font-size: 0.9rem;
  }

  .show-more-btn:hover, .show-all-btn:hover {
    background-color: var(--accent-hover);
  }

  .history-summary {
    display: flex;
    flex-wrap: wrap;
    align-items: center;
    gap: 1.5rem;
    background-color: var(--bg-secondary);
    padding: 1.5rem;
    border-radius: 8px;
    box-shadow: var(--card-shadow);
  }

  .summary-item {
    display: flex;
    align-items: baseline;
    gap: 0.5rem;
  }

  .summary-value {
    font-weight: bold;
    font-size: 1.2rem;
  }

  .summary-label {
    color: var(--text-muted);
  }
</style>