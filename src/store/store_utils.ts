import { writable, type Writable } from "svelte/store";

const writeMap = new Set<string>();
export type WriteOnce<T> = Writable<T> & { reset?: () => void }
export function writeOnce<T>(name: string, value: T) {
    const w: WriteOnce<T> = writable(value);
    const originalSetter = w.set;
    w.set = (value: T) => {
        if (writeMap.has(name)) {
            return;
        }
        writeMap.add(name);
        originalSetter(value);
    }
    w.reset = () => originalSetter(value);
    return w;
}

