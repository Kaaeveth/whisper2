import { Channel, invoke } from "@tauri-apps/api/core";
import type { Backend, Capability, Model, PromptOptions } from "$lib/core/LLMBackend";
import type { ChatMessage, ChatResponse } from "$lib/core/Chat";

export default abstract class BackendImpl implements Backend {
    abstract readonly name: string;

    async updateModels(): Promise<Model[]> {
        await invoke("update_models_in_backend", {
            backendName: this.name,
        });
        let res: any[] = await invoke("get_models_for_backend", {
            backendName: this.name,
        });

        let models: ModelImpl[] = [];
        for(let m of res) {
            models.push(new ModelImpl({
                id: m.model,
                backend: this,
                ...m
            }));
        }
        console.log(res);

        return models;
    }

    /**
     * Querys all running models and returns their names.
     * @returns Names of running models
     */
    async getRunningModels(): Promise<RunningInfo[]> {
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
        try{
            await invoke("boot_backend", {
                backendName: this.name,
            });
        } catch (e) {
            console.error(e);
            throw e;
        }
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

interface RunningInfo {
    size_vram: number;
    expires_at: Date;
    name: string;
}

interface PromptEvent {
    type: "stop"|"message";
    data: ChatResponse
}

export class ModelImpl implements Model {
    readonly name!: string;
    readonly id!: string;
    readonly size!: number;
    readonly capabilities!: Capability[];
    readonly backend!: Backend;

    public constructor(init: ModelImpl) {
        Object.assign(this, init);
    }

    async getRunningInfo(): Promise<RunningInfo|undefined> {
        try {
            let res: RunningInfo = await invoke("get_model_runtime_info", {
                backendName: this.backend.name,
                modelName: this.name
            });
            return res;
        } catch(e) {
            console.warn(e);
            return undefined;
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

    unload(): Promise<void> {
        return invoke("unload_model", {
            backendName: this.backend.name,
            modelName: this.name
        });
    }

    async* prompt(content: ChatMessage, history?: ChatMessage[], options?: PromptOptions): AsyncIterable<ChatResponse> {
        history ??= [];
        let rid = -1;
        const stream = new ReadableStream({
            start: async ctrl => {
                const responseChannel = new Channel<PromptEvent>((chunk) => {
                    if(chunk.type == "stop" || chunk.data.done) {
                        ctrl.close();
                    } else {
                        ctrl.enqueue(chunk.data);
                    }
                });
                try{
                    rid = await invoke("prompt_model", {
                        backendName: this.backend.name,
                        modelName: this.name,
                        content,
                        history,
                        think: options?.think ?? false,
                        responseChannel
                    });
                } catch(e) {
                    console.error(e);
                }
            },
            cancel: async _ => {
                await invoke("stop_prompt", {rid});
            }
        });

        options?.abort?.addEventListener('abort', async _ => {
            await invoke("stop_prompt", {rid});
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
