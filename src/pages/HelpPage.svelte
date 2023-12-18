<script>
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <h1 class="title">Help</h1>
        <div class="row">
            <a href="https://tauri.app" target="_blank">
                <img src="/tauri.svg" class="logo tauri" alt="Tauri Logo" />
            </a>
        </div>
        <hr />
    </svelte:fragment>
    <svelte:fragment slot="body">
        <p>
            A UI to manage .desktop files built with Tauri using Svelte and Rust
        </p>
    </svelte:fragment>
</PrimaryLayout>

<style>
    p {
        margin: 1rem;
    }
</style>
