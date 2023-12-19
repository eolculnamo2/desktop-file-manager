import { readonly, writable } from "svelte/store"

export type Log = {
    timestamp: {
        secs_since_epoch: number,
    },
    message: string,
    level: string,
}

const _logs = writable<Log[]>([]);
export const logs = readonly(_logs);

export function handleNewLog(l: Log) {
    _logs.update(prev => [l, ...prev]);
}
