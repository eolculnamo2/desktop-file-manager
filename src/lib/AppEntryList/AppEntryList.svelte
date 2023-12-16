<script lang="ts">
    import { AppEntry } from "../../modules/AppEntry";
    export let entries: AppEntry[];
    export let onEntryClick: Nullish<(entry: AppEntry, index: number) => void> =
        null;
    import {
        StructuredList,
        StructuredListHead,
        StructuredListRow,
        StructuredListCell,
        StructuredListBody,
        StructuredListInput,
    } from "carbon-components-svelte";
</script>

<StructuredList selection selected="row-1-value">
    <StructuredListHead>
        <StructuredListRow head>
            <StructuredListCell head>Name</StructuredListCell>
            <StructuredListCell head>Type</StructuredListCell>
            <StructuredListCell head>Icon</StructuredListCell>
            <StructuredListCell head>{""}</StructuredListCell>
        </StructuredListRow>
    </StructuredListHead>
    <StructuredListBody>
        {#each entries as entry, i}
            <StructuredListRow
                label
                for="row-{i}"
                on:click={() => onEntryClick?.(entry, i)}
            >
                <StructuredListCell>{entry.name}</StructuredListCell>
                <StructuredListCell>{entry.appType}</StructuredListCell>
                <StructuredListCell>
                    {entry.icon ?? "No Icon"}
                </StructuredListCell>
                <StructuredListInput
                    id="row-{i}"
                    value="row-{i}-value"
                    title="row-{i}-title"
                    name="row-{i}-name"
                />
                <StructuredListCell></StructuredListCell>
            </StructuredListRow>
        {/each}
    </StructuredListBody>
</StructuredList>
