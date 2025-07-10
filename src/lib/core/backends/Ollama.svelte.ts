import { SvelteURL } from "svelte/reactivity";
import BackendImpl from "./Backend";
import { invoke } from "@tauri-apps/api/core";

export default class OllamaBackend extends BackendImpl {
    // This must be the same string used in the backend
    readonly name: string = "Ollama";
    private _apiUrl: URL = new SvelteURL("http://localhost:11434/api/");

    async init(): Promise<void> {
        const url: string = await invoke("ollama_get_api_url");
        this._apiUrl.href = url;
    }

    /**
     * Sets the base URL of the Ollama API.
     * If the URL is syntactically invalid, the URL won't be
     * changed and an exception will be raised.
     * @param url New Ollama backend URL
     */
    async setUrl(url: URL): Promise<void> {
        await invoke("ollama_set_api_url", {
            url: url.href
        });
        this._apiUrl.href = url.href;
    }

    get apiUrl(): URL {
        return this._apiUrl;
    }
}
