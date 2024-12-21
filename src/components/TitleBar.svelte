<!-- filepath: /d:/GitHub/NathanHering/Tauri Apps/Ignyos.Desktop/src/components/TitleBar.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function minimize() {
    await invoke("minimize");
  }

  async function toggleMaximize() {
    await invoke("toggle_maximize");
  }

  async function close() {
    await invoke("close");
  }
</script>

<div class="titlebar flex justify-between items-center p-2 bg-gray-800 text-white">
  <div class="title">Ignyos</div>
  <div class="titlebar-buttons flex space-x-2">
    <button class="titlebar-button p-1 bg-gray-700 rounded" on:click={minimize} aria-label="Minimize">_</button>
    <button class="titlebar-button p-1 bg-gray-700 rounded" on:click={toggleMaximize} aria-label="Maximize">[]</button>
    <button class="titlebar-button p-1 bg-gray-700 rounded" on:click={close} aria-label="Close">X</button>
  </div>
</div>

<style>
  .titlebar {
    -webkit-app-region: drag;
  }

  .titlebar-buttons {
    -webkit-app-region: no-drag;
  }
</style>
