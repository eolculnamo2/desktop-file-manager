import { get, readonly, writable } from "svelte/store";
import { AppEntry, type AppEntryResponse } from "../modules/AppEntry";
import { invoke } from "@tauri-apps/api";

const CACHE_TIME = 60_000;
const _sharedApps = writable<AppEntry[]>([]);
const _myApps = writable<AppEntry[]>([]);
const _lastLoadShared = writable<number>(0);
const _lastLoadMyApps = writable<number>(0);

// only need to force load list on save
// a separate listener will listen for updates from notify.rs in the future

export async function loadList() {
    loadMyApps();
    loadShared();
}

export async function loadMyApps(forceRefresh = false) {
    if (Date.now() - get(_lastLoadMyApps) > CACHE_TIME || forceRefresh) {
        const newMine = await invoke<AppEntryResponse[]>("get_user_apps");
        _myApps.set(newMine.map(AppEntry.create));
        _lastLoadMyApps.set(Date.now());
    }
}

export async function loadShared(forceRefresh = false) {
    if (Date.now() - get(_lastLoadShared) > CACHE_TIME || forceRefresh) {
        const newShared = await invoke<AppEntryResponse[]>("get_shared_apps");
        _sharedApps.set(newShared.map(AppEntry.create));
        _lastLoadShared.set(Date.now());
    }
}


export const sharedApps = readonly(_sharedApps);
export const myApps = readonly(_myApps);
