import { readonly, writable } from "svelte/store";
import { AppEntry, type AppEntryResponse } from "../modules/AppEntry";
import { invoke } from "@tauri-apps/api";

const _sharedApps = writable<AppEntry[]>([]);
const _myApps = writable<AppEntry[]>([]);

// only need to force load list on save
// a separate listener will listen for updates from notify.rs in the future
export async function loadList() {
    const newShared = await invoke<AppEntryResponse[]>("get_shared_apps");
    _sharedApps.set(newShared.map(AppEntry.create));

    const newMine = await invoke<AppEntryResponse[]>("get_user_apps");
    _myApps.set(newMine.map(AppEntry.create));
}

export const sharedApps = readonly(_sharedApps);
export const myApps = readonly(_myApps);
