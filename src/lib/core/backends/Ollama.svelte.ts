import { SvelteURL } from "svelte/reactivity";
import BackendImpl from "./Backend";
import { invoke } from "@tauri-apps/api/core";

export default class OllamaBackend extends BackendImpl {
    // This must be the same string used in the backend
    readonly name: string = "Ollama";
    private _apiUrl: URL = new SvelteURL("http://localhost:11434/api/");
    private _modelsPath: string | undefined = $state(undefined);

    async init(): Promise<void> {
        this._apiUrl.href = await invoke("ollama_get_api_url");
        this._modelsPath = await invoke("ollama_get_models_path");
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

    get modelsPath(): string|undefined {
        return this._modelsPath;
    }

    /**
     * Sets the path of the directory containing the models.
     * This will restart the Ollama backend.
     * You should also update the available models using `updateModels`
     * @see updateModels
     * @param path Path to model directory
     */
    async setModelsPath(path: string): Promise<void> {
        await invoke("ollama_set_models_path", {
            path
        });
        this._modelsPath = path;
    }
}
