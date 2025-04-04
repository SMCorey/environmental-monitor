<script lang="ts">
  // Simplified homepage with no dynamic elements
</script>

<main>
  <div class="hero">
    <div class="hero-text">
      <h1>Presence Detection</h1>
      <p class="tagline">Real-time radar-based target tracking</p>
      
      <div class="buttons">
        <a href="/visualizer" class="button primary">View Radar</a>
        <a href="/activity" class="button secondary">Activity Log</a>
      </div>
    </div>
    
    <div class="radar-container">
      <div class="radar">
        <div class="grid-line horizontal"></div>
        <div class="grid-line vertical"></div>
        <div class="radar-ring outer"></div>
        <div class="radar-ring middle"></div>
        <div class="radar-ring inner"></div>
        
        <!-- Demo targets for visualization only -->
        <div class="target demo-target1"></div>
        <div class="target demo-target2"></div>
      </div>
    </div>
  </div>

  <div class="features">
    <div class="feature-grid">
      <div class="feature-card">
        <div class="feature-icon">📡</div>
        <h3>Real-time Tracking</h3>
        <p>Monitor targets with precise distance and angle measurements</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">📊</div>
        <h3>Visual Analytics</h3>
        <p>Intuitive visualization of radar data and target movement</p>
      </div>
      
      <div class="feature-card">
        <div class="feature-icon">📝</div>
        <h3>Activity Logging</h3>
        <p>Track and analyze target movement history over time</p>
      </div>
    </div>
  </div>
</main>