// src/stores/radarStore.ts
import { writable } from 'svelte/store';

// Define radar target type
export interface RadarTarget {
  id: number;
  distance: number;
  angle: number;
  confidence: number;
  movement: string;
  velocity: number;
  isActive: boolean;
}

// Define history entry type
export interface TargetHistoryEntry {
  timestamp: string;
  targetId: number;
  distance: number;
  angle: number;
  movement: string;
  velocity: number;
  confidence: number;
}

// Create stores for current target data
export const radarTargets = writable<RadarTarget[]>([]);
export const connectionStatus = writable<string>('Disconnected');
export const isConnected = writable<boolean>(false);
export const connectionError = writable<string>('');

// Store for activity history
export const targetHistory = writable<TargetHistoryEntry[]>([]);

// Function to update the radar targets and track in history
export function updateTargets(targets: RadarTarget[]): void {
  // Update the current targets state
  radarTargets.set(targets);
  
  // Create history entries
  const timestamp = new Date().toISOString();
  
  // Only add active targets to history
  const newEntries = targets
    .filter(target => target.isActive)
    .map(target => ({
      timestamp,
      targetId: target.id,
      distance: target.distance,
      angle: target.angle,
      movement: target.movement,
      velocity: target.velocity,
      confidence: target.confidence
    }));
  
  // Add to history (limiting to last 200 entries to prevent excessive memory usage)
  targetHistory.update(history => {
    // Only record changes if they're significant enough (to avoid cluttering the history)
    if (newEntries.length === 0) return history;
    
    // Check if this update is significantly different from the previous one
    if (history.length > 0) {
      // Group entries from the most recent timestamp
      const latestTimestamp = history[0]?.timestamp;
      const latestEntries = history.filter(entry => entry.timestamp === latestTimestamp);
      
      // If all targets are the same and values are very similar, don't record a new entry
      // This prevents filling the history with nearly identical entries
      if (latestEntries.length === newEntries.length) {
        const allSimilar = newEntries.every(newEntry => {
          const prevEntry = latestEntries.find(e => e.targetId === newEntry.targetId);
          if (!prevEntry) return false;
          
          // Check if values are very similar (less than 5% change)
          const distanceChange = Math.abs(newEntry.distance - prevEntry.distance) / prevEntry.distance;
          const angleChange = Math.abs(newEntry.angle - prevEntry.angle) / (Math.abs(prevEntry.angle) + 0.1);
          const velocityChange = Math.abs(newEntry.velocity - prevEntry.velocity) / (prevEntry.velocity + 0.1);
          
          // If movement type changed or any value changed significantly, it's different enough
          return (
            newEntry.movement === prevEntry.movement &&
            distanceChange < 0.05 &&
            angleChange < 0.05 &&
            velocityChange < 0.05
          );
        });
        
        // If all entries are very similar to the previous ones, skip this update
        if (allSimilar) return history;
      }
    }
    
    return [...newEntries, ...history].slice(0, 200);
  });
}

// Helper to clear history
export function clearHistory(): void {
  targetHistory.set([]);
}

// Helper to get filtered history for a specific target
export function getTargetHistory(targetId: number): TargetHistoryEntry[] {
  let result: TargetHistoryEntry[] = [];
  const unsubscribe = targetHistory.subscribe(history => {
    result = history.filter(entry => entry.targetId === targetId);
  });
  unsubscribe();
  return result;
}