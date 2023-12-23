import { writable } from "svelte/store";

const writeMap = new Set<string>();
export function writeOnce<T>(name: string, value: T) {
    const w = writable(value);
    const originalSetter = w.set;
    w.set = (value: T) => {
        console.log('attempting writing')
        if (writeMap.has(name)) {
            return;
        }
        console.log('writing')
        writeMap.add(name);
        originalSetter(value);
    }
    return w;
}

