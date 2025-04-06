<script lang="ts">
    import { onMount } from "svelte";

    let darkMode = false;

    onMount(() => {
        const savedTheme = localStorage.getItem("theme");

        const prefersDarkScheme = window.matchMedia(
            "(prefers-color-scheme: dark)",
        );

        if (savedTheme) {
            darkMode = savedTheme === "dark";
        } else {
            darkMode = prefersDarkScheme.matches;
        }

        applyTheme();

        prefersDarkScheme.addEventListener("change", (e) => {
            if (!localStorage.getItem("theme")) {
                darkMode = e.matches;
                applyTheme();
            }
        });
    });

    function toggleTheme() {
        darkMode = !darkMode;

        localStorage.setItem("theme", darkMode ? "dark" : "light");

        applyTheme();
    }

    function applyTheme() {
        if (darkMode) {
            document.documentElement.setAttribute("data-theme", "dark");
        } else {
            document.documentElement.removeAttribute("data-theme");
        }
    }

    function handleLightClick() {
        if (darkMode) {
            toggleTheme();
        }
    }

    function handleDarkClick() {
        if (!darkMode) {
            toggleTheme();
        }
    }
</script>

<div class="theme-switch">
    <button
        class="theme-icon light-icon"
        class:active={!darkMode}
        on:click={handleLightClick}
        aria-label="switch to light mode"
    >
        ☀️
    </button>

    <label class="switch">
        <input
            type="checkbox"
            bind:checked={darkMode}
            on:change={toggleTheme}
        />
        <span class="slider"></span>
    </label>

    <button
        class="theme-icon dark-icon"
        class:active={darkMode}
        on:click={handleDarkClick}
        aria-label="switch to dark mode"
    >
        🌙
    </button>
</div>

<style>
    .theme-switch {
        display: flex;
        align-items: center;
        margin-left: 1.5rem;
    }

    .theme-icon {
        background: none;
        border: none;
        cursor: pointer;
        font-size: 1.2rem;
        padding: 4px;
        opacity: 0.5;
        transition:
            opacity 0.3s,
            transform 0.3s;
    }

    .theme-icon.active {
        opacity: 1;
        transform: scale(1.2);
    }

    .switch {
        position: relative;
        display: inline-block;
        width: 50px;
        height: 24px;
        margin: 0 8px;
    }

    .switch input {
        opacity: 0;
        width: 0;
        height: 0;
    }

    .slider {
        position: absolute;
        cursor: pointer;
        top: 0;
        left: 0;
        right: 0;
        bottom: 0;
        background-color: #ccc;
        transition: 0.4s;
        border-radius: 34px;
    }

    .slider:before {
        position: absolute;
        content: "";
        height: 16px;
        width: 16px;
        left: 4px;
        bottom: 4px;
        background-color: white;
        transition: 0.4s;
        border-radius: 50%;
    }

    input:checked + .slider {
        background-color: var(--accent-color, #3a7bd5);
    }

    input:focus + .slider {
        box-shadow: 0 0 1px var(--accent-color, #3a7bd5);
    }

    input:checked + .slider:before {
        transform: translateX(26px);
    }
</style>
