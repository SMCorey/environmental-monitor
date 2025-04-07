<script lang="ts">
  // Simplified homepage with no dynamic elements
</script>

<main>
  <div class="hero">
    <div class="hero-text">
      <h1>Environment Reader</h1>
      <p class="tagline">Real-time sensor data tracking & analysis</p>
      
      <div class="buttons">
        <a href="/visualizer" class="button primary">View Sensors</a>
        <a href="/history" class="button secondary">Data History</a>
      </div>
    </div>
    
    <div class="sensor-container">
      <div class="sensor-display">
        <div class="grid-line horizontal"></div>
        <div class="grid-line vertical"></div>
        <div class="sensor-ring outer"></div>
        <div class="sensor-ring middle"></div>
        <div class="sensor-ring inner"></div>
      </div>
    </div>
  </div>

  <div class="features">
    <div class="feature-grid">
      <div class="feature-card">
        <div class="feature-icon">🌡️</div>
        <h3>Multiple Sensors</h3>
        <p>Monitor temperature, humidity, pressure, gas and altitude</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">📊</div>
        <h3>Visual Analytics</h3>
        <p>Intuitive visualization of environmental data trends</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">📝</div>
        <h3>Data Logging</h3>
        <p>Track and analyze environmental changes over time</p>
      </div>
    </div>
  </div>
</main>

<style>
  /* Existing styles will apply, but we might need to rename some classes */
  .sensor-container {
    composes: radar-container;
  }
  
  .sensor-display {
    composes: radar;
  }
  
  .sensor-ring {
    composes: radar-ring;
  }
  
  .sensor-point {
    position: absolute;
    width: 12px;
    height: 12px;
    border-radius: 50%;
    transform: translate(-50%, -50%);
  }
  
  .temp-point {
    background-color: #e74c3c;
    top: 30%;
    left: 35%;
  }
  
  .humidity-point {
    background-color: #3498db;
    top: 45%;
    left: 65%;
  }
  
  .pressure-point {
    background-color: #f39c12;
    top: 65%;
    left: 40%;
  }
  
  .gas-point {
    background-color: #2ecc71;
    top: 55%;
    left: 75%;
  }
</style>