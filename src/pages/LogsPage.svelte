<script lang="ts">
    import { Checkbox, Grid, Row } from "carbon-components-svelte";
    import {
        allLogs,
        addLogsFromDisk,
        logsFromDisk,
        type Log,
    } from "../store/log_store";
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import { invoke } from "@tauri-apps/api";

    let logsToDisplay = ["info", "warn", "error"];
    function formatLog(l: Log) {
        const localTimestamp = new Date(
            l.timestamp.secs_since_epoch * 1000,
        ).toLocaleTimeString();
        return ` ${localTimestamp} -- ${l.message}`;
    }

    async function getLogsFromDisk() {
        const diskLogs = await invoke<Log[]>("get_logs_from_disk");
        console.log("diskLogs", diskLogs);
        addLogsFromDisk(diskLogs);
    }

    // start loading from disk before mount
    getLogsFromDisk();
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <h1 class="title">Logs</h1>
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
            </a>
        </div>
        <h3>Log Level</h3>
        <div class="checkboxes">
            <Grid>
                <Row>
                    <Checkbox
                        bind:group={logsToDisplay}
                        labelText="Trace"
                        value="trace"
                    />
                    <Checkbox
                        bind:group={logsToDisplay}
                        labelText="Info"
                        value="info"
                    />
                    <Checkbox
                        bind:group={logsToDisplay}
                        labelText="Warn"
                        value="warn"
                    />
                    <Checkbox
                        bind:group={logsToDisplay}
                        labelText="Error"
                        value="error"
                    />
                </Row>
            </Grid>
        </div>
        <br />
        <hr />
    </svelte:fragment>
    <svelte:fragment slot="body">
        {#if $allLogs.length > 0}
            <Grid>
                {#each $allLogs as log}
                    {#if logsToDisplay.includes(log.level.toLowerCase())}
                        <Row>
                            <div class="message-row">
                                <span class={log.level.toLowerCase()}
                                    >{log.level.toUpperCase()}</span
                                >
                                {formatLog(log)}
                            </div>
                        </Row>
                    {/if}
                {/each}
            </Grid>
        {:else}
            <p>No logs</p>
        {/if}
    </svelte:fragment>
</PrimaryLayout>

<style>
    .message-row {
        line-height: 2;
        padding: 1rem;
        margin: 0.5rem 0;
        font-size: 1rem;
    }
    .title {
        margin-bottom: 2rem;
    }
    .checkboxes {
        max-width: 400px;
    }
    h3 {
        margin: 1rem;
    }
    .trace {
        color: #999;
    }
    .info {
        color: #c4c4c4;
    }
    .warn {
        color: #b88a00;
    }
    .error {
        color: #b80000;
    }
</style>
