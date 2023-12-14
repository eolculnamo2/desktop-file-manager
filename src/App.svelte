<script lang="ts">
import { listen } from '@tauri-apps/api/event';
 import { invoke } from "@tauri-apps/api/tauri"
import Greet from './lib/Greet.svelte'


// Still some things to figure out
// but at least we're loading the list!!!
// Next is improving this then we'll make an add app button :)

let appList = [];

// TODO fix any
listen("list-upated", (result: any) => {
   console.log('payload', result.payload);
   appList = result.payload.Ok;
})


  async function loadList(){
    // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
    await invoke("app_list_changed");
  }
</script>

<main class="container">
  <h1>Welcome to Tauri!</h1>

  <div class="row">
    <a href="https://vitejs.dev" target="_blank">
      <img src="/vite.svg" class="logo vite" alt="Vite Logo" />
    </a>
    <a href="https://tauri.app" target="_blank">
      <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
    </a>
    <a href="https://svelte.dev" target="_blank">
      <img src="/svelte.svg" class="logo svelte" alt="Svelte Logo" />
    </a>
  </div>

  <div>Derp</div>
  <button on:click={loadList}>LOAD ME</button>
  {#each appList as item}
      <div>{item.name}</div>
  {/each}
  <div class="row">
    <Greet />
  </div>


</main>

<style>
  .logo.vite:hover {
    filter: drop-shadow(0 0 2em #747bff);
  }

  .logo.svelte:hover {
    filter: drop-shadow(0 0 2em #ff3e00);
  }
</style>
