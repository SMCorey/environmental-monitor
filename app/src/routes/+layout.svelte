<script lang="ts">
  import "../app.css";
  import { page } from "$app/stores";
  import ThemeSwitcher from "../components/ThemeSwitcher.svelte";
  $: currentPath = $page.url.pathname;
</script>

<nav>
  <div class="nav-container">
    <div class="brand">
      <span class="radar-icon">
        <span class="radar-circle"></span>
        <span class="radar-line"></span>
      </span>
      <span class="brand-text">Presence Detection</span>
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
        <span class="nav-text">Visual Data</span>
      </a>
      <a href="/activity" class={currentPath === "/activity" ? "active" : ""}>
        <span class="nav-text">Activity Log</span>
      </a>
      <ThemeSwitcher />
    </div>
  </div>
</nav>

<main>
  <slot />
</main>
