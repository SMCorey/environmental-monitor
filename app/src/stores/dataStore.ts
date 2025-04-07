// src/stores/dataStore.ts
import { writable } from 'svelte/store';

// Define the environmental sensor data type
export interface EnvironmentalData {
  temperature: number;
  pressure: number;
  humidity: number;
  gas: number;
  altitude: number;
  timestamp?: string;
}

// Define history entry type for environmental data
export interface EnvironmentalHistoryEntry {
  timestamp: string;
  temperature: number;
  pressure: number;
  humidity: number;
  gas: number;
  altitude: number;
}

// Create stores for current environmental data
export const currentEnvironmentalData = writable<EnvironmentalData | null>(null);
export const connectionStatus = writable<string>('Disconnected');
export const isConnected = writable<boolean>(false);
export const connectionError = writable<string>('');

// Store for environmental data history
export const environmentalHistory = writable<EnvironmentalHistoryEntry[]>([]);

// Function to update the environmental data and track in history
export function updateEnvironmentalData(data: EnvironmentalData): void {
  // Add timestamp if not provided
  const dataWithTimestamp = {
    ...data,
    timestamp: data.timestamp || new Date().toISOString()
  };
  
  // Update the current data state
  currentEnvironmentalData.set(dataWithTimestamp);
  
  // Add to history (limiting to last 200 entries to prevent excessive memory usage)
  environmentalHistory.update(history => {
    const historyEntry: EnvironmentalHistoryEntry = {
      timestamp: dataWithTimestamp.timestamp!,
      temperature: data.temperature,
      pressure: data.pressure,
      humidity: data.humidity,
      gas: data.gas,
      altitude: data.altitude
    };
    
    // Only record changes if they're significant enough (to avoid cluttering the history)
    if (history.length > 0) {
      const latestEntry = history[0];
      
      // Check if this update is significantly different from the previous one
      const tempChange = Math.abs(data.temperature - latestEntry.temperature) / Math.abs(latestEntry.temperature || 1);
      const pressureChange = Math.abs(data.pressure - latestEntry.pressure) / Math.abs(latestEntry.pressure || 1);
      const humidityChange = Math.abs(data.humidity - latestEntry.humidity) / Math.abs(latestEntry.humidity || 1);
      const gasChange = Math.abs(data.gas - latestEntry.gas) / Math.abs(latestEntry.gas || 1);
      const altitudeChange = Math.abs(data.altitude - latestEntry.altitude) / Math.abs(latestEntry.altitude || 1);
      
      // If all values are very similar, skip this update (less than 1% change)
      const isSignificantChange = 
        tempChange > 0.01 || 
        pressureChange > 0.01 || 
        humidityChange > 0.01 || 
        gasChange > 0.01 || 
        altitudeChange > 0.01;
        
      if (!isSignificantChange) {
        return history;
      }
    }
    
    return [historyEntry, ...history].slice(0, 200);
  });
}

// Helper to clear history
export function clearHistory(): void {
  environmentalHistory.set([]);
}

// Helper to get history for a specific time range
export function getHistoryByTimeRange(startTime: Date, endTime: Date): EnvironmentalHistoryEntry[] {
  let result: EnvironmentalHistoryEntry[] = [];
  const unsubscribe = environmentalHistory.subscribe(history => {
    result = history.filter(entry => {
      const entryTime = new Date(entry.timestamp);
      return entryTime >= startTime && entryTime <= endTime;
    });
  });
  unsubscribe();
  return result;
}

// Compatibility layer for existing components that might still use the old radar interfaces
// This allows gradual migration without breaking existing code

// Re-export connection status stores with the same names
export { connectionStatus as radarConnectionStatus };
export { isConnected as radarIsConnected };
export { connectionError as radarConnectionError };

// Convert environmental data to a format that can be consumed by existing radar visualization
export function convertToRadarFormat(data: EnvironmentalData | null) {
  if (!data) return [];
  
  // Create radar-like targets from the environmental data
  // Each sensor reading becomes a "target" at different angles
  return [
    {
      id: 1,
      distance: normalizeValue(data.temperature, 0, 50, 1, 8),  // Temperature as distance
      angle: 0,  // At 0 degrees
      confidence: 100,
      movement: getMovementForValue(data.temperature, 15, 30),
      velocity: 0,
      isActive: true
    },
    {
      id: 2,
      distance: normalizeValue(data.humidity, 0, 100, 1, 8),  // Humidity as distance
      angle: 72,  // At 72 degrees
      confidence: 100,
      movement: getMovementForValue(data.humidity, 30, 70),
      velocity: 0,
      isActive: true
    },
    {
      id: 3,
      distance: normalizeValue(data.pressure, 950, 1050, 1, 8),  // Pressure as distance
      angle: 144,  // At 144 degrees
      confidence: 100,
      movement: getMovementForValue(data.pressure, 980, 1020),
      velocity: 0,
      isActive: true
    },
    {
      id: 4,
      distance: normalizeValue(data.gas, 0, 20, 1, 8),  // Gas as distance
      angle: 216,  // At 216 degrees
      confidence: 100,
      movement: getMovementForValue(data.gas, 5, 15),
      velocity: 0,
      isActive: true
    },
    {
      id: 5,
      distance: normalizeValue(data.altitude, 0, 500, 1, 8),  // Altitude as distance
      angle: 288,  // At 288 degrees
      confidence: 100,
      movement: 'stationary',
      velocity: 0,
      isActive: true
    }
  ];
}

// Helper function to normalize a value to a range
function normalizeValue(value: number, inMin: number, inMax: number, outMin: number, outMax: number): number {
  // Clamp value to input range
  const clampedValue = Math.max(inMin, Math.min(inMax, value));
  // Map to output range
  return outMin + (clampedValue - inMin) * (outMax - outMin) / (inMax - inMin);
}

// Helper to determine movement type based on value and thresholds
function getMovementForValue(value: number, lowThreshold: number, highThreshold: number): string {
  if (value < lowThreshold) return 'departing';
  if (value > highThreshold) return 'approaching';
  return 'stationary';
}

// Convert radar format to environmental data (not exact, just an approximation for compatibility)
export function convertFromRadarFormat(radarTargets: any[]): EnvironmentalData | null {
  if (!radarTargets.length) return null;
  
  // Create a dummy environmental data object
  return {
    temperature: 25,
    pressure: 1000,
    humidity: 50,
    gas: 10,
    altitude: 100,
    timestamp: new Date().toISOString()
  };
}

// Bridge to allow existing radar code to work with the new environmental data
export const radarTargets = {
  subscribe: (callback: any) => {
    return currentEnvironmentalData.subscribe(data => {
      callback(convertToRadarFormat(data));
    });
  }
};

// Simplified history for radar visualization compatibility
export const targetHistory = {
  subscribe: (callback: any) => {
    return environmentalHistory.subscribe(history => {
      // Convert each history entry to a format that looks like radar history
      const radarHistory = history.flatMap(entry => {
        const radarTargets = convertToRadarFormat({
          temperature: entry.temperature,
          pressure: entry.pressure,
          humidity: entry.humidity,
          gas: entry.gas,
          altitude: entry.altitude
        });
        
        return radarTargets.map(target => ({
          timestamp: entry.timestamp,
          targetId: target.id,
          distance: target.distance,
          angle: target.angle,
          movement: target.movement,
          velocity: target.velocity,
          confidence: target.confidence
        }));
      });
      
      callback(radarHistory);
    });
  }
};

// Compatibility wrapper for updateTargets - accepts radar format but stores as environmental data
export function updateTargets(radarTargets: any[]): void {
  const environmentalData = convertFromRadarFormat(radarTargets);
  if (environmentalData) {
    updateEnvironmentalData(environmentalData);
  }
}

// Function to parse and update sensor data directly from JSON string
export function updateFromJsonString(jsonString: string): void {
  try {
    const data = JSON.parse(jsonString) as EnvironmentalData;
    updateEnvironmentalData(data);
  } catch (e) {
    console.error('Failed to parse environmental data JSON:', e);
  }
}