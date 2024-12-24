<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import feather from "feather-icons";
  import { onMount } from "svelte";

  async function minimize() {
    await invoke("minimize");
  }

  async function toggleMaximize() {
    await invoke("toggle_maximize");
  }

  async function close() {
    await invoke("close");
  }

  onMount(() => {
    feather.replace({ "stroke-width": 0.75, height: 20, width: 20 });
  });
</script>

<div
  class="titlebar flex justify-between items-center pl-2 bg-gray-800 text-white"
>
  <div>Template Application</div>
  <div class="titlebar-buttons flex space-x-0.5">
    <button
      class="titlebar-button p-2 hover:bg-gray-700"
      on:click={minimize}
      aria-label="Minimize"><span data-feather="minus"></span></button
    >
    <button
      class="titlebar-button p-2 hover:bg-gray-700"
      on:click={toggleMaximize}
      aria-label="Maximize"><span data-feather="maximize"></span></button
    >
    <button
      class="titlebar-button p-2 hover:bg-red-900"
      on:click={close}
      aria-label="Close"><span data-feather="x"></span></button
    >
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
