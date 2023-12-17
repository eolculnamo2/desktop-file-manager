import { writable, readonly } from "svelte/store";
import type { AppEntry } from "../modules/AppEntry";

export enum PageName {
    INDEX,
    VIEW_APP_ENTRY,
    ADD_APP_ENTRY,
}

export type Pages =
    | { page: PageName.INDEX }
    | { page: PageName.VIEW_APP_ENTRY, entry: AppEntry }
    | { page: PageName.ADD_APP_ENTRY }

const _currentPage = writable<Pages>({ page: PageName.INDEX });
export const currentPage = readonly(_currentPage);

export const goToPage = (nextPage: Pages) => {
    _currentPage.set(nextPage);
    window.scrollTo(0, 0);
}

