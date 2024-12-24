<script lang="ts">
  import { invoke } from "@tauri-apps/api/core";
  import { onMount } from "svelte";
  import { writable } from "svelte/store";

  const osInfo = writable("");

  async function fetchOsInfo() {
    try {
      const info: string = await invoke("get_os_info");
      osInfo.set(info);
    } catch (error) {
      console.error("Failed to fetch OS info:", error);
    }
  }

  onMount(() => {
    fetchOsInfo();
  });
</script>

<main class="container mx-auto p-1">
  <h1 class="text-3xl font-bold text-center">Template Application</h1>
  <p class="text-center mt-4">System: {$osInfo}</p>
</main>
