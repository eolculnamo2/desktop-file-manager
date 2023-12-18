<script lang="ts">
    import AppEntryList from "../lib/AppEntryList.svelte";
    import { onMount } from "svelte";
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import { PageName, goToPage } from "../store/nav_store";
    import { loadList, myApps, sharedApps } from "../store/entries_store";
    import AlignRight from "../lib/AlignRight.svelte";
    import { Button } from "carbon-components-svelte";

    onMount(() => {
        loadList();
    });
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <AlignRight>
            <Button
                kind="tertiary"
                on:click={() => goToPage({ page: PageName.ADD_APP_ENTRY })}
                >Add Entry</Button
            >
        </AlignRight>
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
            entries={$myApps}
            onEntryClick={(entry) =>
                goToPage({ page: PageName.VIEW_APP_ENTRY, entry })}
        />
        <h3>Shared Apps</h3>
        <AppEntryList
            entries={$sharedApps}
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
