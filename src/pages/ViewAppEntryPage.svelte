<script lang="ts">
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
    } from "carbon-components-svelte";
    import { PageName, goToPage } from "../store/nav_store";
    import { convertFileSrc } from "@tauri-apps/api/tauri";
    import AlignRight from "../lib/AlignRight.svelte";
    import { emit } from "@tauri-apps/api/event";

    // LEFT OFF HERE. NEED TO FIGURE OUT WHERE ICONS
    // ARE LOCATED FOR SHARED APPS
    export let entry: AppEntry;
    let form = structuredClone(entry);

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
            </Row>
        </Grid>
    </svelte:fragment>
    <svelte:fragment slot="body">
        <Form
            on:submit={(e) => {
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
                <Button type="submit">Submit</Button>
            </AlignRight>
        </Form>
    </svelte:fragment>
</PrimaryLayout>
