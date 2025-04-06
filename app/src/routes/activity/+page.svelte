<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import {
    targetHistory,
    clearHistory,
    radarTargets,
  } from "../../stores/radarStore";
  import type {
    TargetHistoryEntry,
    RadarTarget,
  } from "../../stores/radarStore";

  // Variables for storing reactive data
  let historyEntries: TargetHistoryEntry[] = [];
  let currentTargets: RadarTarget[] = [];
  let expandedTargets: Record<number, boolean> = {};

  // Subscribe to store changes
  $: historyEntries = $targetHistory;
  $: currentTargets = $radarTargets;

  // Function to group history entries by target ID
  function groupHistoryByTarget(history: TargetHistoryEntry[]) {
    const grouped: Record<number, TargetHistoryEntry[]> = {};

    history.forEach((entry) => {
      if (!grouped[entry.targetId]) {
        grouped[entry.targetId] = [];
      }

      grouped[entry.targetId].push(entry);
    });

    // Sort each target's entries by timestamp (newest first)
    Object.keys(grouped).forEach((targetId) => {
      grouped[Number(targetId)].sort(
        (a, b) =>
          new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime(),
      );
    });

    return grouped;
  }

  // Group history by target and re-compute whenever historyEntries changes
  $: groupedHistory = groupHistoryByTarget(historyEntries);

  // Auto-refresh to ensure we get updates (in case the store updates don't trigger reactivity)
  let refreshInterval: number | null = null;

  onMount(() => {
    // Set a lightweight refresh interval (every 5 seconds) to ensure the UI stays updated
    refreshInterval = window.setInterval(() => {
      // This will trigger reactivity even if the underlying data structure reference hasn't changed
      historyEntries = [...$targetHistory];
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

  // Function to check if a target is currently active
  function isTargetActive(targetId: number): boolean {
    return currentTargets.some((t) => t.id === targetId && t.isActive);
  }

  // Toggle expanded state for a target
  function toggleExpandedState(targetId: number): void {
    expandedTargets = {
      ...expandedTargets,
      [targetId]: !expandedTargets[targetId],
    };
  }
</script>

<main>
  <h1>Radar Target Activity History</h1>

  <div class="actions">
    <button on:click={handleClearHistory} class="clear-btn"
      >Clear History</button
    >
  </div>

  {#if historyEntries.length === 0}
    <div class="empty-state">
      <p>No activity history recorded yet.</p>
      <p class="hint">
        Activity is recorded automatically when radar targets are detected.
      </p>
    </div>
  {:else}
    <div class="history-stats">
      <div class="stat">
        <span class="stat-value">{Object.keys(groupedHistory).length}</span>
        <span class="stat-label">Unique Targets</span>
      </div>
      <div class="stat">
        <span class="stat-value">{historyEntries.length}</span>
        <span class="stat-label">Total Records</span>
      </div>
      <div class="stat">
        <span class="stat-value"
          >{historyEntries.length > 0
            ? new Date(historyEntries[0].timestamp).toLocaleString()
            : "N/A"}</span
        >
        <span class="stat-label">Latest Record</span>
      </div>
      <div class="stat">
        <span class="stat-value"
          >{historyEntries.length > 0
            ? new Date(
                historyEntries[historyEntries.length - 1].timestamp,
              ).toLocaleString()
            : "N/A"}</span
        >
        <span class="stat-label">First Record</span>
      </div>
    </div>

    <div class="history-container">
      {#each Object.keys(groupedHistory) as targetId}
        {@const id = Number(targetId)}
        <div class="target-section">
          <div class="target-header">
            <h2>Target ID: {targetId}</h2>
            <span
              class="status-badge {isTargetActive(id) ? 'active' : 'inactive'}"
            >
              {isTargetActive(id) ? "Active" : "Inactive"}
            </span>
          </div>

          <div class="target-stats">
            <div class="stat">
              <span class="stat-value">{groupedHistory[id].length}</span>
              <span class="stat-label">Records</span>
            </div>
            <div class="stat">
              <span class="stat-value"
                >{formatTimestamp(groupedHistory[id][0].timestamp)}</span
              >
              <span class="stat-label">Last Update</span>
            </div>
          </div>

          <table>
            <thead>
              <tr>
                <th>Time</th>
                <th>Distance (m)</th>
                <th>Angle (°)</th>
                <th>Movement</th>
                <th>Velocity (m/s)</th>
                <th>Confidence</th>
              </tr>
            </thead>
            <tbody>
              {#each groupedHistory[id].slice(0, 10) as entry, i}
                <tr class={i === 0 ? "newest-entry" : ""}>
                  <td>{formatTimestamp(entry.timestamp)}</td>
                  <td>{entry.distance.toFixed(2)}</td>
                  <td>{entry.angle.toFixed(1)}</td>
                  <td class={entry.movement}>{entry.movement}</td>
                  <td>{entry.velocity.toFixed(2)}</td>
                  <td>{entry.confidence}</td>
                </tr>
              {/each}

              {#if groupedHistory[id].length > 10}
                <tr class="more-row">
                  <td colspan="6">
                    <button
                      class="show-more-btn"
                      on:click={() => toggleExpandedState(id)}
                    >
                      {expandedTargets[id]
                        ? "Hide"
                        : `Show ${Math.min(groupedHistory[id].length - 10, 10)} more entries...`}
                    </button>
                  </td>
                </tr>
              {/if}

              {#if expandedTargets[id] && groupedHistory[id].length > 10}
                {#each groupedHistory[id].slice(10, 20) as entry}
                  <tr>
                    <td>{formatTimestamp(entry.timestamp)}</td>
                    <td>{entry.distance.toFixed(2)}</td>
                    <td>{entry.angle.toFixed(1)}</td>
                    <td class={entry.movement}>{entry.movement}</td>
                    <td>{entry.velocity.toFixed(2)}</td>
                    <td>{entry.confidence}</td>
                  </tr>
                {/each}
              {/if}
            </tbody>
          </table>
        </div>
      {/each}
    </div>
  {/if}
</main>
