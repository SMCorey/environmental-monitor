<!-- // Tauri doesn't have a Node.js server to do proper SSR
// so we will use adapter-static to prerender the app (SSG)
// See: https://v2.tauri.app/start/frontend/sveltekit/ for more info -->


<!-- export const prerender = true;  // NOT SURE HOW IT STILL WORKS WITHOUT THIS LINE
export const ssr = false;  // NOT SURE HOW IT STILL WORKS WITHOUT THIS LINE -->    


<script>
  // This layout adds navigation between the home page and MQTT page
</script>

<nav>
  <div class="nav-container">
    <a href="/">Home</a>
    <a href="/mqtt">MQTT Client</a>
    <a href="/visualizer">Visual Data</a>
    <a href="/activity">Activity Log</a>
  </div>
</nav>

<slot />

<style>
  nav {
    background-color: #4f46e5;
    color: white;
    padding: 1rem;
  }
  
  .nav-container {
    max-width: 1200px;
    margin: 0 auto;
    display: flex;
    gap: 2rem;
  }
  
  a {
    color: white;
    text-decoration: none;
    font-weight: 500;
    transition: opacity 0.2s;
  }
  
  a:hover {
    opacity: 0.8;
  }
  
  @media (prefers-color-scheme: dark) {
    nav {
      background-color: #1f2937;
    }
  }
</style>
