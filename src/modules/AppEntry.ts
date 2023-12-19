export const APP_TYPE = {
    Application: "Application",
    Link: "Link",
    Directory: "Directory",
} as const;

export const ENCODING = {
    UTF8: { kind: 'UTF8' },
    OTHER(value: string): AppEntry["encoding"] {
        return { kind: 'Other', value } as const
    },
} as const;

export const APP_VISIBILITY = {
    LOCAL: 'local',
    SHARED: 'shared',
} as const;

export type AppVisibility = typeof APP_VISIBILITY[keyof typeof APP_VISIBILITY];
export type AppType = typeof APP_TYPE[keyof typeof APP_TYPE];
export type Encoding = { kind: 'UTF8' } | { kind: 'Other', value: string };
export type AllApps = {
    userApps: AppEntryResponse[],
    sharedApps: AppEntryResponse[],
}

export type AppEntryResponse = {
    appType: AppType;
    encoding: 'UTF8' | 'Other';
    name: string;
    comment: Nullish<string>;
    icon: Nullish<string>;
    exec: string;
    terminal: boolean;
    categories: string[];
    absolutePath: string,
}
export class AppEntry {
    appType: AppType = APP_TYPE.Application;
    encoding: Encoding = { kind: 'UTF8' };
    name: string = "";
    comment: Nullish<string>;
    icon: Nullish<string>;
    exec: string = "";
    terminal: boolean = false;
    categories: string[] = [];

    // its a little messy, but i'm using this as
    // local or shared value on create for now
    // Adding type to make this more obvious until
    // cleanup is done
    absolutePath: string | AppVisibility = "";

    static toResponse(entry: AppEntry): AppEntryResponse {
        return {
            appType: entry.appType,
            encoding: entry.encoding.kind,
            name: entry.name,
            comment: entry.comment,
            icon: entry.icon,
            exec: entry.exec,
            terminal: entry.terminal,
            categories: entry.categories,
            absolutePath: entry.absolutePath,
        }
    }

    // TODO see how Other actually gets deserialized
    static create(response: AppEntryResponse): AppEntry {
        const newEntry = new AppEntry();
        newEntry.encoding = ((): AppEntry['encoding'] => {
            if (response.encoding === 'UTF8') {
                return ENCODING.UTF8;
            }
            if (response.encoding === 'Other') {
                return ENCODING.OTHER('');
            }
            return ENCODING.OTHER('');
        })();
        newEntry.appType = response.appType;
        newEntry.name = response.name;
        newEntry.comment = response.comment;
        newEntry.icon = response.icon;
        newEntry.exec = response.exec;
        newEntry.terminal = response.terminal;
        newEntry.categories = response.categories;
        newEntry.absolutePath = response.absolutePath;
        return newEntry
    }
}
