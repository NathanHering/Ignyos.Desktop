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

<div class="titlebar">
  <div class="title">Ignyos</div>
  <div class="titlebar-buttons">
    <button class="titlebar-button" on:click={minimize} aria-label="Minimize"
      >_</button
    >
    <button
      class="titlebar-button"
      on:click={toggleMaximize}
      aria-label="Maximize">[]</button
    >
    <button class="titlebar-button" on:click={close} aria-label="Close"
      >X</button
    >
  </div>
</div>

<style>
  .titlebar {
    -webkit-app-region: drag;
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem;
    background-color: #333;
    color: white;
  }

  .titlebar-buttons {
    -webkit-app-region: no-drag;
    display: flex;
    gap: 0.5rem;
  }

  .titlebar-button {
    cursor: pointer;
    padding: 0.5rem;
    background: none;
    border: none;
    color: white;
    font-size: 1rem;
  }

  .titlebar-button:hover {
    background-color: rgba(255, 255, 255, 0.1);
  }
</style>
