<script lang="ts">
    import PrimaryLayout from "../layouts/PrimaryLayout.svelte";
    import { ENCODING, type AppEntry } from "../modules/AppEntry";
    import {
        Form,
        Button,
        TextInput,
        FormGroup,
        RadioButtonGroup,
        RadioButton,
        Select,
        SelectItem,
    } from "carbon-components-svelte";
    import { PageName, goToPage } from "../store/nav_store";
    import { convertFileSrc } from "@tauri-apps/api/tauri";

    // LEFT OFF HERE. NEED TO FIGURE OUT WHERE ICONS
    // ARE LOCATED FOR SHARED APPS
    export let entry: AppEntry;
    let form = structuredClone(entry);
</script>

<PrimaryLayout>
    <svelte:fragment slot="header">
        <Button kind="ghost" on:click={() => goToPage({ page: PageName.INDEX })}
            >Back</Button
        >
        <h1 class="title">Edit {entry.name}</h1>
        <div class="row">
            <img
                src={entry.icon ? convertFileSrc(entry.icon) : "/tauri.svg"}
                class="logo tauri"
                alt={entry.icon ? entry.name : "Tauri Icon"}
            />
        </div>
    </svelte:fragment>
    <svelte:fragment slot="body">
        <Form
            on:submit={(e) => {
                e.preventDefault();
            }}
        >
            <TextInput inline labelText="App Name" bind:value={form.name} />
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
                            //@ts-ignore
                            const value = e.target.value;
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
                <Select id="select-1" labelText="Select menu">
                    <SelectItem
                        disabled
                        hidden
                        value="placeholder-item"
                        text="Choose an option"
                    />
                    <SelectItem value="option-1" text="Option 1" />
                    <SelectItem value="option-2" text="Option 2" />
                    <SelectItem value="option-3" text="Option 3" />
                </Select>
            </FormGroup>
            <Button type="submit">Submit</Button>
        </Form>
    </svelte:fragment>
</PrimaryLayout>
