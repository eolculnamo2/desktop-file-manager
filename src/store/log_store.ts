import { derived, readonly, writable, get } from "svelte/store"
import { writeOnce } from "./store_utils";

export type Log = {
    timestamp: {
        secs_since_epoch: number,
    },
    message: string,
    level: string,
}

const _logs = writable<Log[]>([]);
const _logsFromDisk = writeOnce<Log[]>('writeOnce', []);
export const logs = readonly(_logs);
export const logsFromDisk = readonly(_logsFromDisk);
export const allLogs = derived([logs, logsFromDisk], ([$logs, $logsFromDisk]) => $logs.concat($logsFromDisk));

export function addLogsFromDisk(diskLogs: Log[]) {
    const inMemoryLogs = get(logs);
    const filteredDiskLogs = diskLogs.filter((dl) => inMemoryLogs.every((l) => l.timestamp.secs_since_epoch !== dl.timestamp.secs_since_epoch))
    _logsFromDisk.set(filteredDiskLogs);
}

export function handleNewLog(l: Log) {
    _logs.update(prev => [l, ...prev]);
}

