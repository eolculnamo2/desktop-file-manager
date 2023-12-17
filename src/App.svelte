<script lang="ts">
    import AddEntryPage from "./pages/AddEntryPage.svelte";
    import IndexPage from "./pages/IndexPage.svelte";
    import ViewAppEntryPage from "./pages/ViewAppEntryPage.svelte";
    import { PageName, currentPage } from "./store/nav_store";
</script>

{#if $currentPage.page === PageName.INDEX}
    <IndexPage />
{:else if $currentPage.page === PageName.VIEW_APP_ENTRY}
    <ViewAppEntryPage entry={$currentPage.entry} />
{:else if $currentPage.page === PageName.ADD_APP_ENTRY}
    <AddEntryPage />
{/if}
