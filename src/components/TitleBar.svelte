<!-- filepath: /d:/GitHub/NathanHering/Tauri Apps/Ignyos.Desktop/src/components/TitleBar.svelte -->
<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";

  async function minimize() {
    await invoke("minimize");
  }

  async function maximize() {
    await invoke("toggle_maximize");
  }

  async function close() {
    await invoke("close");
  }
</script>

<div class="titlebar">
  <div class="title">My App</div>
  <div class="titlebar-buttons">
    <div class="titlebar-button" on:click={minimize}>_</div>
    <div class="titlebar-button" on:click={maximize}>[]</div>
    <div class="titlebar-button" on:click={close}>X</div>
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
  }
</style>
