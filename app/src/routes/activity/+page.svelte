<script lang="ts">
  import { onMount, onDestroy } from 'svelte';
  import { targetHistory, clearHistory, radarTargets } from '../../stores/radarStore';
  import type { TargetHistoryEntry } from '../../stores/radarStore';
  
  // Subscribe to history store
  $: historyEntries = $targetHistory;
  $: currentTargets = $radarTargets;
  
  // Function to group history entries by target ID
  function groupHistoryByTarget(history: TargetHistoryEntry[]) {
    const grouped: Record<number, TargetHistoryEntry[]> = {};
    
    history.forEach(entry => {
      if (!grouped[entry.targetId]) {
        grouped[entry.targetId] = [];
      }
      
      grouped[entry.targetId].push(entry);
    });
    
    // Sort each target's entries by timestamp (newest first)
    Object.keys(grouped).forEach(targetId => {
      grouped[Number(targetId)].sort((a, b) => 
        new Date(b.timestamp).getTime() - new Date(a.timestamp).getTime()
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
    return currentTargets.some(t => t.id === targetId && t.isActive);
  }
</script>

<main>
  <h1>Radar Target Activity History</h1>
  
  <div class="actions">
    <button on:click={handleClearHistory} class="clear-btn">Clear History</button>
  </div>
  
  {#if historyEntries.length === 0}
    <div class="empty-state">
      <p>No activity history recorded yet.</p>
      <p class="hint">Activity is recorded automatically when radar targets are detected.</p>
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
        <span class="stat-value">{historyEntries.length > 0 ? new Date(historyEntries[0].timestamp).toLocaleString() : 'N/A'}</span>
        <span class="stat-label">Latest Record</span>
      </div>
      <div class="stat">
        <span class="stat-value">{historyEntries.length > 0 ? new Date(historyEntries[historyEntries.length - 1].timestamp).toLocaleString() : 'N/A'}</span>
        <span class="stat-label">First Record</span>
      </div>
    </div>
    
    <div class="history-container">
      {#each Object.keys(groupedHistory) as targetId}
        {@const id = Number(targetId)}
        <div class="target-section">
          <div class="target-header">
            <h2>Target ID: {targetId}</h2>
            <span class="status-badge {isTargetActive(id) ? 'active' : 'inactive'}">
              {isTargetActive(id) ? 'Active' : 'Inactive'}
            </span>
          </div>
          
          <div class="target-stats">
            <div class="stat">
              <span class="stat-value">{groupedHistory[id].length}</span>
              <span class="stat-label">Records</span>
            </div>
            <div class="stat">
              <span class="stat-value">{formatTimestamp(groupedHistory[id][0].timestamp)}</span>
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
              {#each groupedHistory[id].slice(0, 20) as entry, i}
                {#if i < 10}
                  <tr class={i === 0 ? 'newest-entry' : ''}>
                    <td>{formatTimestamp(entry.timestamp)}</td>
                    <td>{entry.distance.toFixed(2)}</td>
                    <td>{entry.angle.toFixed(1)}</td>
                    <td class={entry.movement}>{entry.movement}</td>
                    <td>{entry.velocity.toFixed(2)}</td>
                    <td>{entry.confidence}</td>
                  </tr>
                {:else if i === 10}
                  <tr class="more-row">
                    <td colspan="6">
                      <button class="show-more-btn">
                        Show {Math.min(groupedHistory[id].length - 10, 10)} more entries...
                      </button>
                    </td>
                  </tr>
                {:else}
                  <tr>
                    <td>{formatTimestamp(entry.timestamp)}</td>
                    <td>{entry.distance.toFixed(2)}</td>
                    <td>{entry.angle.toFixed(1)}</td>
                    <td class={entry.movement}>{entry.movement}</td>
                    <td>{entry.velocity.toFixed(2)}</td>
                    <td>{entry.confidence}</td>
                  </tr>
                {/if}
              {/each}
            </tbody>
          </table>
        </div>
      {/each}
    </div>
  {/if}
</main>

<style>
  main {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem;
  }
  
  h1 {
    margin-bottom: 1.5rem;
  }
  
  .actions {
    margin-bottom: 1.5rem;
    display: flex;
    justify-content: flex-end;
  }
  
  .empty-state {
    padding: 2rem;
    background-color: #f8f9fa;
    border-radius: 8px;
    text-align: center;
  }
  
  .hint {
    color: #6c757d;
    font-size: 0.9rem;
  }
  
  .history-stats {
    display: flex;
    gap: 1rem;
    margin-bottom: 1.5rem;
  }
  
  .history-container {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }
  
  .target-section {
    border: 1px solid #ddd;
    border-radius: 8px;
    padding: 1rem;
    background-color: #f9f9f9;
  }
  
  .target-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 0.5rem;
  }
  
  .target-stats {
    display: flex;
    gap: 1rem;
    margin-bottom: 1rem;
  }
  
  .stat {
    display: flex;
    flex-direction: column;
    background-color: #f0f0f0;
    padding: 0.75rem;
    border-radius: 6px;
    flex: 1;
  }
  
  .stat-value {
    font-weight: bold;
    font-size: 1.1rem;
  }
  
  .stat-label {
    font-size: 0.8rem;
    color: #666;
    margin-top: 0.25rem;
  }
  
  .status-badge {
    padding: 0.25rem 0.75rem;
    border-radius: 999px;
    font-size: 0.8rem;
    font-weight: 500;
  }
  
  .status-badge.active {
    background-color: #d1fae5;
    color: #047857;
  }
  
  .status-badge.inactive {
    background-color: #fee2e2;
    color: #b91c1c;
  }
  
  h2 {
    margin: 0;
    font-size: 1.25rem;
  }
  
  table {
    width: 100%;
    border-collapse: collapse;
    margin-top: 0.5rem;
  }
  
  th, td {
    border: 1px solid #ddd;
    padding: 0.5rem;
    text-align: center;
    font-size: 0.9rem;
  }
  
  th {
    background-color: #f4f4f4;
    font-weight: 500;
  }
  
  td.approaching {
    color: #047857;
    font-weight: 500;
  }
  
  td.stationary {
    color: #b91c1c;
  }
  
  td.departing {
    color: #c2410c;
    font-weight: 500;
  }
  
  td.crossing {
    color: #1d4ed8;
    font-weight: 500;
  }
  
  tr.newest-entry td {
    background-color: #f0fdf4;
    animation: highlight-new 3s ease-out;
  }
  
  .more-row td {
    padding: 0.5rem;
    text-align: center;
  }
  
  .show-more-btn {
    background: none;
    border: none;
    color: #4f46e5;
    cursor: pointer;
    font-size: 0.9rem;
    padding: 0.25rem 0.5rem;
  }
  
  .show-more-btn:hover {
    text-decoration: underline;
  }
  
  .clear-btn {
    background-color: #ef4444;
    color: white;
    border: none;
    padding: 0.5rem 1rem;
    border-radius: 4px;
    font-size: 0.9rem;
    cursor: pointer;
  }
  
  .clear-btn:hover {
    background-color: #dc2626;
  }
  
  @keyframes highlight-new {
    0% { background-color: #dcfce7; }
    100% { background-color: #f0fdf4; }
  }
  
  @media (prefers-color-scheme: dark) {
    .target-section {
      background-color: #1f2937;
      border-color: #374151;
    }
    
    .empty-state {
      background-color: #1f2937;
    }
    
    .hint {
      color: #9ca3af;
    }
    
    .stat {
      background-color: #111827;
    }
    
    .stat-label {
      color: #9ca3af;
    }
    
    h2 {
      color: #f9fafb;
    }
    
    th {
      background-color: #111827;
      color: #f9fafb;
    }
    
    td {
      color: #f9fafb;
      border-color: #374151;
    }
    
    .status-badge.active {
      background-color: #064e3b;
      color: #a7f3d0;
    }
    
    .status-badge.inactive {
      background-color: #7f1d1d;
      color: #fecaca;
    }
    
    td.approaching {
      color: #10b981;
    }
    
    td.stationary {
      color: #ef4444;
    }
    
    td.departing {
      color: #f97316;
    }
    
    td.crossing {
      color: #60a5fa;
    }
    
    tr.newest-entry td {
      background-color: #064e3b;
      animation: highlight-new-dark 3s ease-out;
    }
    
    @keyframes highlight-new-dark {
      0% { background-color: #065f46; }
      100% { background-color: #064e3b; }
    }
    
    .show-more-btn {
      color: #818cf8;
    }
  }
</style>