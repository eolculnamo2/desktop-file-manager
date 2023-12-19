<script lang="ts">
    import { onMount } from "svelte";
    import AddEntryPage from "./pages/AddEntryPage.svelte";
    import IndexPage from "./pages/IndexPage.svelte";
    import ViewAppEntryPage from "./pages/ViewAppEntryPage.svelte";
    import { PageName, currentPage } from "./store/nav_store";
    import { listen } from "@tauri-apps/api/event";
    import { handleNewLog, type Log } from "./store/log_store";
    import LogsPage from "./pages/LogsPage.svelte";
    import AboutPage from "./pages/AboutPage.svelte";
    import HelpPage from "./pages/HelpPage.svelte";
    import type { AllApps } from "./modules/AppEntry";
    import { filesChanged } from "./store/entries_store";

    onMount(() => {
        listen("log", (message) => {
            const l = message.payload as Log;
            handleNewLog(l);
        });
        listen("files_altered", (message) => {
            const l = message.payload as AllApps;
            filesChanged(l);
        });
    });
</script>

{#if $currentPage.page === PageName.INDEX}
    <IndexPage />
{:else if $currentPage.page === PageName.VIEW_APP_ENTRY}
    <ViewAppEntryPage entry={$currentPage.entry} />
{:else if $currentPage.page === PageName.ADD_APP_ENTRY}
    <AddEntryPage />
{:else if $currentPage.page === PageName.LOGS}
    <LogsPage />
{:else if $currentPage.page === PageName.ABOUT}
    <AboutPage />
{:else if $currentPage.page === PageName.HELP}
    <HelpPage />
{/if}
