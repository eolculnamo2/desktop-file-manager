<script lang="ts">
    import AppEntryList from "../lib/AppEntryList/AppEntryList.svelte";
    import { invoke } from "@tauri-apps/api/tauri";
    import { onMount } from "svelte";
    import { AppEntry, type AppEntryResponse } from "../modules/AppEntry";
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import { PageName, goToPage } from "../store/nav_store";

    let sharedApps: AppEntry[] = [];
    let myApps: AppEntry[] = [];

    async function loadList() {
        const newShared = await invoke<AppEntryResponse[]>("get_shared_apps");
        sharedApps = newShared.map(AppEntry.create);

        const newMine = await invoke<AppEntryResponse[]>("get_user_apps");
        myApps = newMine.map(AppEntry.create);
    }

    onMount(() => {
        loadList();
    });
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <h1 class="title">Gnome App Manager</h1>
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
            </a>
        </div>
    </svelte:fragment>

    <svelte:fragment slot="body">
        <h3>My Apps</h3>
        <AppEntryList
            entries={myApps}
            onEntryClick={(entry) =>
                goToPage({ page: PageName.VIEW_APP_ENTRY, entry })}
        />
        <h3>Shared Apps</h3>
        <AppEntryList
            entries={sharedApps}
            onEntryClick={(entry) =>
                goToPage({ page: PageName.VIEW_APP_ENTRY, entry })}
        />
    </svelte:fragment>
</PrimaryLayout>

<style>
    .title {
        margin-bottom: 2rem;
    }
</style>
