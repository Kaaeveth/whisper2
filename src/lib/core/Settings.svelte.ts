import { load, type Store } from "@tauri-apps/plugin-store";

export default class Settings {
    private _store?: Store;
    // reactive settings store for automatic updates
    private _settings: Record<string, any> = $state({});

    /**
     * Loads the underlying store.
     * Needs to be called before any other method.
     */
    public async init() {
        this._store = await load("settings.json");
        for(const key of await this._store.keys()) {
            this._settings[key] = await this._store.get(key);
        }
    }

    /**
     * Returns the setting for `key`.
     * Wrapping the return value in `$derived`
     * makes the setting reactive to changes made with `set`.
     * @param key Key of the setting
     * @returns The setting or undefined of not found
     */
    public get<T>(key: string): T|undefined {        
        return this._settings[key];
    }

    /**
     * Sets the setting `val` with the `key`.
     * This is reactive if a value retrieved with `get`
     * is wrapped with $derived.
     * @param key Key of the setting
     * @param val Setting value
     */
    public async set<T>(key: string, val: T): Promise<void> {
        await this._store?.set(key, val);
        this._settings[key] = val;
    }

    get store() {
        return this._settings;
    }

    public static AUTO_SCROLL: string = "autoScroll";
}