import { SvelteURL } from "svelte/reactivity";
import BackendImpl, { ModelImpl } from "$lib/core/backends/Backend.svelte";
import { Channel, invoke } from "@tauri-apps/api/core";
import { DeletableTag, type DeletableModel, type Model } from "../LLMBackend";

export interface OllamaPullProgress {
    status: string,
    error?: string,
    digest?: string,
    total?: number,
    completed?: number
}

export default class OllamaBackend extends BackendImpl {
    // This must be the same string used in the backend
    readonly name: string = "Ollama";
    private _apiUrl: URL = new SvelteURL("http://localhost:11434/api/");
    private _modelsPath: string | undefined = $state(undefined);

    async init(): Promise<void> {
        await this.boot();
        this._apiUrl.href = await invoke("ollama_get_api_url");
        this._modelsPath = await invoke("ollama_get_models_path");
    }

    buildModel(m: Model): Model {
        return new OllamaModel(super.buildModel(m));
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

    /**
     * Pulls a model from the Ollama registry.
     * This methods waits until the model has been downloaded
     * and throws in case of an error.
     * @param tag The tag of the model e.g. gpt-oss:latest
     * @param cb Called for events reporting the download progress
     */
    async pullModel(tag: string, cb: (ev: OllamaPullProgress) => void): Promise<void> {
        return new Promise(async (resolve, reject) => {
            let succeeded = false;
            const progressChannel = new Channel<OllamaPullProgress>(ev => {
                cb(ev);
                if (ev.status === "success") {
                    succeeded = true;
                    resolve();
                } else if ((ev.status === "done" && !succeeded) || ev.error) {
                    reject(ev.error ?? "Unknown");
                }
            });
            await invoke("ollama_pull_model", {
                tag,
                progressChannel: progressChannel
            });
        });
    }
}

export class OllamaModel extends ModelImpl implements DeletableModel {
    readonly [DeletableTag] = true;
    async delete(): Promise<void> {
        const idx = this.backend.models.findIndex(m => m.id === this.id);
        if(idx < 0) {
            // This should not happen
            throw new Error("bug: Model not found in backend");
        }

        await super.stopAllPrompts();
        await invoke("ollama_delete_model", {
            tag: this.id
        });
        await this.backend.updateModels();
    }

    public constructor(init: Model) {
        super(init);
    }
}
