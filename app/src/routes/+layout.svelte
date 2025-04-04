<!-- // Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info -->

<!-- export const prerender = true;  // NOT SURE HOW IT STILL WORKS WITHOUT THIS LINE
export const ssr = false;  // NOT SURE HOW IT STILL WORKS WITHOUT THIS LINE -->


<script lang="ts">
  import '../app.css';
  import { page } from '$app/stores';
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
      <a href="/" class={currentPath === '/' ? 'active' : ''}>
        <span class="nav-text">Home</span>
      </a>
      <a href="/mqtt" class={currentPath === '/mqtt' ? 'active' : ''}>
        <span class="nav-text">MQTT Client</span>
      </a>
      <a href="/visualizer" class={currentPath === '/visualizer' ? 'active' : ''}>
        <span class="nav-text">Visual Data</span>
      </a>
      <a href="/activity" class={currentPath === '/activity' ? 'active' : ''}>
        <span class="nav-text">Activity Log</span>
      </a>
    </div>
  </div>
</nav>

<main>
  <slot />
</main>