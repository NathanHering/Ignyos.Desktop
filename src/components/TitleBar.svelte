<!-- filepath: /d:/GitHub/NathanHering/Tauri Apps/Ignyos.Desktop/src/components/TitleBar.svelte -->
<script lang="ts">
  import { Window } from "@tauri-apps/api/window";

  function minimize() {}

  function maximize() {
    //Window.toggleMaximize();
  }

  function close() {
    //Window.close();
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
