<script lang="ts">
    import IndexPage from "./pages/IndexPage.svelte";
    import ViewAppEntryPage from "./pages/ViewAppEntryPage.svelte";
    import { PageName, currentPage } from "./store/nav_store";

    // Example of listen
    // listen("list-upated", (result: any) => {
    //     console.log("payload", result.payload);
    //     appList = result.payload.Ok;
    // });
</script>

{#if $currentPage.page === PageName.INDEX}
    <IndexPage />
{:else if $currentPage.page === PageName.VIEW_APP_ENTRY}
    <ViewAppEntryPage entry={$currentPage.entry} />
{/if}
