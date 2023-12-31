<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import { ENCODING, AppEntry, APP_TYPE } from "../modules/AppEntry";
    import {
        Form,
        Button,
        TextInput,
        FormGroup,
        RadioButtonGroup,
        RadioButton,
        Select,
        SelectItem,
        Toggle,
        Grid,
        Row,
        Column,
        ToastNotification,
        ProgressBar,
        ComposedModal,
        ModalHeader,
        ModalBody,
        ModalFooter,
    } from "carbon-components-svelte";
    import { PageName, goToPage } from "../store/nav_store";
    import { convertFileSrc, invoke } from "@tauri-apps/api/tauri";
    import AlignRight from "../lib/AlignRight.svelte";
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";

    // LEFT OFF HERE. NEED TO FIGURE OUT WHERE ICONS
    // ARE LOCATED FOR SHARED APPS
    export let entry: AppEntry;
    let form = structuredClone(entry);
    let errorMsg: Nullish<string> = null;
    let isLoading = false;
    let showConfirmDelete = false;

    onMount(() => {
        let unlistenOk: Nullish<UnlistenFn>;
        let unlistenErr: Nullish<UnlistenFn>;
        listen("entry_update_ok", async () => {
            goToPage({ page: PageName.INDEX });
        }).then((ul) => {
            unlistenOk = ul;
        });
        listen("entry_update_err", (msg) => {
            errorMsg = msg.payload as string;
        }).then((ul) => (unlistenErr = ul));
        return () => {
            unlistenOk?.();
            unlistenErr?.();
        };
    });
    async function openFile() {
        await open({
            directory: false,
            defaultPath: entry.absolutePath,
        });
    }
    async function deleteFile() {
        // todo, handle error case
        const result = await invoke("delete_entry_command", {
            appEntry: AppEntry.toResponse(entry),
        });
        goToPage({ page: PageName.INDEX });
    }
    // TODO make location for this configurable
    // Want to also benchmark rayon for reading desktop files
    function getIconSrc(icon: Nullish<string>) {
        if (!icon) {
            return "/tauri.svg";
        } else if (icon.startsWith("/")) {
            // assume absolute path
            return convertFileSrc(icon);
        } else {
            return convertFileSrc(
                // this really isn't acceptable as it is.
                // Need this to be able to more intelligently infer location
                // https://askubuntu.com/questions/153575/where-does-gnome-nautilus-store-directory-icons
                `/usr/share/icons/hicolor/symbolic/apps/${icon}-symbolic.svg`,
            );
        }
    }
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <Grid>
            <Row>
                <div>
                    <Button
                        kind="ghost"
                        on:click={() => goToPage({ page: PageName.INDEX })}
                        >Back</Button
                    >
                </div>
                <Column>
                    <h1 class="title">Edit {entry.name}</h1>
                    <div class="row">
                        <img
                            src={getIconSrc(entry.icon)}
                            class="logo tauri"
                            alt={entry.icon ? entry.name : "Tauri Icon"}
                        />
                    </div>
                </Column>
                <div class="button-wrap">
                    <Button kind="tertiary" on:click={openFile}>Open Raw</Button
                    >
                    <Button
                        kind="danger-tertiary"
                        on:click={() => (showConfirmDelete = true)}
                        >Delete Entry</Button
                    >
                </div>
            </Row>
        </Grid>
        <ComposedModal
            open={showConfirmDelete}
            on:submit={deleteFile}
            on:close={() => (showConfirmDelete = false)}
        >
            <ModalHeader label="Changes" title="Confirm changes" />
            <ModalBody hasForm>
                Are you sure you want to delete this Desktop File? This does
                *not* delete the associated application.
            </ModalBody>
            <ModalFooter primaryButtonText="Proceed" />
        </ComposedModal>
        {#if isLoading}
            <ProgressBar helperText="Loading..." />
        {/if}

        {#if errorMsg}
            <ToastNotification
                fullWidth
                lowContrast
                title="Error"
                subtitle={errorMsg}
            />
            <br />
        {/if}
    </svelte:fragment>
    <svelte:fragment slot="body">
        <Form
            on:submit={(e) => {
                errorMsg = null;
                e.preventDefault();
                emit("entry_update", AppEntry.toResponse(form));
            }}
        >
            <FormGroup>
                <TextInput inline labelText="App Name" bind:value={form.name} />
            </FormGroup>
            <FormGroup>
                <TextInput
                    inline
                    labelText="Comments"
                    bind:value={form.comment}
                />
            </FormGroup>
            <FormGroup>
                <TextInput inline labelText="Icon" bind:value={form.icon} />
            </FormGroup>
            <FormGroup>
                <TextInput inline labelText="Exec" bind:value={form.exec} />
            </FormGroup>
            <FormGroup legendText="Terminal">
                <Toggle bind:toggled={form.terminal}>
                    <span slot="labelA">False</span>
                    <span slot="labelB">True</span>
                </Toggle>
            </FormGroup>
            <FormGroup legendText="Encoding">
                <RadioButtonGroup name="Encoding" selected={form.encoding.kind}>
                    <RadioButton
                        on:change={() =>
                            (form = { ...form, encoding: ENCODING.UTF8 })}
                        id="radio-1"
                        value="UTF8"
                        labelText="UTF8"
                    />
                    <RadioButton
                        on:change={() =>
                            (form = {
                                ...form,
                                encoding: ENCODING.OTHER(""),
                            })}
                        id="radio-2"
                        value="Other"
                        labelText="Other"
                    />
                </RadioButtonGroup>
                {#if form.encoding.kind === "Other"}
                    <TextInput
                        inline
                        labelText="Other Value"
                        on:input={(e) => {
                            const value = e.detail;
                            if (typeof value !== "string") {
                                throw Error("Invalid Option input");
                            }
                            form = {
                                ...form,
                                encoding: {
                                    kind: "Other",
                                    value,
                                },
                            };
                        }}
                    />
                {/if}
            </FormGroup>
            <FormGroup>
                <Select
                    id="select-1"
                    labelText="Application Type"
                    selected={form.appType}
                >
                    <SelectItem
                        value={APP_TYPE.Application}
                        text={APP_TYPE.Application}
                    />
                    <SelectItem value={APP_TYPE.Link} text={APP_TYPE.Link} />
                    <SelectItem
                        value={APP_TYPE.Directory}
                        text={APP_TYPE.Directory}
                    />
                </Select>
            </FormGroup>
            <AlignRight>
                <Button type="submit" disabled={isLoading}>Submit</Button>
            </AlignRight>
        </Form>
    </svelte:fragment>
</PrimaryLayout>

<style>
    .button-wrap {
        display: flex;
        justify-content: space-between;
        gap: 1rem;
        align-items: center;
    }
</style>
