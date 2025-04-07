<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import ThemeSwitcher from "../components/ThemeSwitcher.svelte";
  $: currentPath = $page.url.pathname;
</script>

<nav>
  <div class="nav-container">
    <div class="brand">
      <span class="env-icon">
        <span class="env-circle"></span>
        <span class="env-graph"></span>
      </span>
      <span class="brand-text">Environmental Reader</span>
    </div>

    <div class="nav-links">
      <a href="/" class={currentPath === "/" ? "active" : ""}>
        <span class="nav-text">Home</span>
      </a>
      <a href="/mqtt" class={currentPath === "/mqtt" ? "active" : ""}>
        <span class="nav-text">MQTT Client</span>
      </a>
      <a
        href="/visualizer"
        class={currentPath === "/visualizer" ? "active" : ""}
      >
        <span class="nav-text">Sensor Data</span>
      </a>
      <a href="/history" class={currentPath === "/history" ? "active" : ""}>
        <span class="nav-text">Data History</span>
      </a>
      <ThemeSwitcher />
    </div>
  </div>
</nav>

<main>
  <slot />
</main>

<style>
  nav {
    background-color: var(--nav-bg);
    border-bottom: 1px solid var(--border-color);
    padding: 0.75rem 1rem;
    position: sticky;
    top: 0;
    z-index: 100;
  }

  .nav-container {
    display: flex;
    justify-content: space-between;
    align-items: center;
    max-width: 1200px;
    margin: 0 auto;
  }

  .brand {
    display: flex;
    align-items: center;
    gap: 0.75rem;
  }

  .brand-text {
    font-weight: bold;
    font-size: 1.25rem;
    color: var(--text-primary);
  }

  .env-icon {
    position: relative;
    width: 32px;
    height: 32px;
    display: block;
  }

  .env-circle {
    position: absolute;
    width: 28px;
    height: 28px;
    border: 2px solid var(--accent-color);
    border-radius: 50%;
    top: 0;
    left: 0;
  }

  .env-graph {
    position: absolute;
    height: 16px;
    width: 2px;
    background-color: var(--accent-color);
    bottom: 2px;
    left: 15px;
    transform-origin: bottom center;
  }

  .env-graph::before, .env-graph::after {
    content: '';
    position: absolute;
    width: 16px;
    height: 1px;
    background-color: var(--accent-color);
    bottom: 0;
    left: -7px;
  }
  
  .env-graph::after {
    bottom: 5px;
    width: 12px;
  }

  .nav-links {
    display: flex;
    gap: 1.5rem;
    align-items: center;
  }

  .nav-links a {
    text-decoration: none;
    color: var(--nav-text);
    font-weight: 500;
    transition: color 0.2s ease;
  }

  .nav-links a:hover {
    color: var(--accent-color);
  }

  .nav-links a.active {
    color: var(--nav-active);
  }

  .nav-text {
    position: relative;
  }

  .nav-links a.active .nav-text::after {
    content: '';
    position: absolute;
    bottom: -4px;
    left: 0;
    width: 100%;
    height: 2px;
    background-color: var(--nav-active);
    border-radius: 1px;
  }

  main {
    max-width: 1200px;
    margin: 0 auto;
    padding: 1.5rem;
  }

  @media (max-width: 768px) {
    .nav-links {
      gap: 1rem;
    }
    
    .brand-text {
      font-size: 1rem;
    }
  }

  @media (max-width: 576px) {
    .nav-container {
      flex-direction: column;
      gap: 0.75rem;
    }
    
    .nav-links {
      width: 100%;
      justify-content: space-between;
    }
  }
</style>