<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import {
        ENCODING,
        AppEntry,
        APP_TYPE,
        APP_VISIBILITY,
    } from "../modules/AppEntry";
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
    } from "carbon-components-svelte";
    import { PageName, goToPage } from "../store/nav_store";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import AlignRight from "../lib/AlignRight.svelte";
    import { emit, listen, type UnlistenFn } from "@tauri-apps/api/event";
    import { onMount } from "svelte";
    import { loadMyApps, loadShared } from "../store/entries_store";

    let form = new AppEntry();
    let errorMsg: Nullish<string> = null;
    let isLoading = false;

    onMount(() => {
        let unlistenOk: Nullish<UnlistenFn>;
        let unlistenErr: Nullish<UnlistenFn>;
        listen("entry_create_ok", async () => {
            isLoading = true;
            if (form.absolutePath == APP_VISIBILITY.LOCAL) {
                await loadMyApps(true);
            }
            if (form.absolutePath == APP_VISIBILITY.SHARED) {
                await loadShared(true);
            }
            isLoading = false;
            goToPage({ page: PageName.INDEX });
        }).then((ul) => {
            unlistenOk = ul;
        });
        listen("entry_create_err", (msg) => {
            errorMsg = msg.payload as string;
        }).then((ul) => (unlistenErr = ul));
        return () => {
            unlistenOk?.();
            unlistenErr?.();
        };
    });
    function getIconSrc(icon: Nullish<string>) {
        if (!icon) {
            return "/tauri.svg";
        } else if (icon.startsWith("/")) {
            return convertFileSrc(icon);
        } else {
            return convertFileSrc(
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
                    <h1 class="title">Create App Entry</h1>
                    <div class="row">
                        <img
                            src={getIconSrc(form.icon)}
                            class="logo tauri"
                            alt={form.icon ? form.name : "Tauri Icon"}
                        />
                    </div>
                </Column>
            </Row>
        </Grid>

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
                emit("entry_create", AppEntry.toResponse(form));
            }}
        >
            <FormGroup>
                <TextInput inline labelText="App Name" bind:value={form.name} />
            </FormGroup>
            <FormGroup>
                <RadioButtonGroup
                    legendText="My App or Shared?"
                    name="appScope"
                    bind:selected={form.absolutePath}
                >
                    <RadioButton
                        labelText="My App"
                        value={APP_VISIBILITY.LOCAL}
                    />
                    <RadioButton
                        labelText="Shared App"
                        value={APP_VISIBILITY.SHARED}
                    />
                </RadioButtonGroup>
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
