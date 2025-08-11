import { Channel, invoke } from "@tauri-apps/api/core";
import { DeletableTag, type Backend, type Capability, type DeletableModel, type Model, type PromptOptions } from "$lib/core/LLMBackend";
import type { ChatMessage, ChatResponse } from "$lib/core/Chat";
import { handleError } from "$lib/Util";

export default abstract class BackendImpl implements Backend {
    abstract readonly name: string;
    private _models: Model[] = $state([]);

    async updateModels(): Promise<Model[]> {
        await invoke("update_models_in_backend", {
            backendName: this.name,
        });
        // NOTE: This is actually not(!) an array of instances [Model].
        // Only the properties are present but not the methods!
        let res: Model[] = await invoke("get_models_for_backend", {
            backendName: this.name,
        });

        let models: Model[] = [];
        for(let m of res) {
            models.push(this.buildModel(m));
        }
        console.log(res);

        this._models = models;
        return models;
    }

    buildModel(m: Model): Model {
        return new ModelImpl({
            id: m.name,
            backend: this,
            ...(m as any)
        });
    }

    get models(): Model[] {
        return this._models;
    }

    /**
     * Querys all running models and returns their names.
     * @returns Names of running models
     */
    async getRunningModels(): Promise<RuntimeInfo[]> {
        return invoke("get_running_models_in_backend", {
            backendName: this.name,
        });
    }

    async running(): Promise<boolean> {
        return invoke("is_backend_running", {
            backendName: this.name,
        });
    }

    /**
     * Starts the backend, if not already running.
     * @returns A promise which resolves when the backend has started.
     */
    async boot(): Promise<void> {
        await invoke("boot_backend", {
            backendName: this.name,
        });
    }

    /**
     * Shuts down the backend.
     * @returns The backend has finished
     */
    shutdown(): Promise<void> {
        return invoke("shutdown_backend", {
            backendName: this.name,
        });
    }
}

interface RuntimeInfo {
    size_vram: number;
    expires_at: Date;
    name: string;
}

export class ModelImpl implements Model {
    readonly name!: string;
    readonly id!: string;
    readonly size!: number;
    readonly capabilities!: Capability[];
    readonly backend!: Backend;

    // Ressource identifiers of ongoing chat completions
    // in the backend.
    protected promptGenIds: Set<number>;

    public constructor(init: Model) {
        Object.assign(this, init);
        this.promptGenIds = new Set();
    }

    isDeletable(): this is DeletableModel {
        return (this as any)[DeletableTag] === true;
    }

    async getRuntimeInfo(): Promise<RuntimeInfo> {
        try {
            let res: RuntimeInfo = await invoke("get_model_runtime_info", {
                backendName: this.backend.name,
                modelName: this.name
            });
            return res;
        } catch(e) {
            handleError(e, {level: "warn", userMsg: ""});
            throw e
        }
    }

    async loaded(): Promise<boolean> {
        return invoke("is_model_loaded", {
            backendName: this.backend.name,
            modelName: this.name
        });
    }

    async getLoadedSize(): Promise<number> {
        return invoke("get_model_loaded_size", {
            backendName: this.backend.name,
            modelName: this.name
        });
    }

    load(): Promise<void> {
        return invoke("load_model", {
            backendName: this.backend.name,
            modelName: this.name
        });
    }

    async unload(): Promise<void> {
        await this.stopAllPrompts();
        await invoke("unload_model", {
            backendName: this.backend.name,
            modelName: this.name
        });
    }

    public async stopAllPrompts() {
        for (const rid of this.promptGenIds) {
            await this.stopPrompt(rid);
        }
    }

    private async stopPrompt(rid: number) {
        await invoke("stop_prompt", { rid });
        this.promptGenIds.delete(rid);
    }

    async* prompt(content: ChatMessage, history?: ChatMessage[], options?: PromptOptions): AsyncIterable<ChatResponse> {
        history ??= [];
        let rid = -1;
        const stream = new ReadableStream({
            start: async ctrl => {
                const responseChannel = new Channel<ChatResponse>((chunk) => {
                    if(chunk.done) {
                        ctrl.close();
                    } else {
                        ctrl.enqueue(chunk);
                    }
                });
                rid = await invoke("prompt_model", {
                    backendName: this.backend.name,
                    modelName: this.name,
                    content,
                    history,
                    think: options?.think ?? false,
                    responseChannel
                });
                this.promptGenIds.add(rid);
            },
            cancel: async _ => {
                await this.stopPrompt(rid);
            }
        });

        options?.abort?.addEventListener('abort', async _ => {
            await this.stopPrompt(rid);
        });

        const reader = stream.getReader();
        try {
            while (true) {
                const { done, value } = await reader.read();
                if (done) break;
                yield value;
            }
        } finally {
            reader.releaseLock();
        }
    }
}
