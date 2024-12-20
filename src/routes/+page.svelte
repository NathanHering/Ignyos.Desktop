<script lang="ts">
  import "../app.css";
  import { invoke } from "@tauri-apps/api/core";

  let name = $state("");
  let greetMsg = $state("");

  async function greet(event: Event) {
    event.preventDefault();
    // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
    greetMsg = await invoke("greet", { name });
  }
</script>

<main class="container mx-auto p-4">
  <h1 class="text-3xl font-bold text-center">Welcome to Ignyos</h1>
</main>
