import { invoke } from "@tauri-apps/api/core";
import { fetch } from "@tauri-apps/plugin-http";
import type { Backend, Capability, Model } from "$lib/core/LLMBackend";
import type { ChatMessage, ChatResponse } from "$lib/core/Chat";
import readNdJson from "../NDJsonReader";
import { SvelteURL } from "svelte/reactivity";

export default class OllamaBackend implements Backend {
    readonly name: string = "Ollama";
    readonly apiUrl: URL = new SvelteURL("http://localhost:11434/api");

    async callBackend(endpoint: string, fetchParams?: RequestInit) {
        // Remove Origin to disable CORS
        // We do this since we cannot set the allowed origins for Ollama
        fetchParams ??= {};
        fetchParams.headers ??= {};
        (fetchParams.headers as any).Origin = "";

        const res = await fetch(this.apiUrl+endpoint, fetchParams);
        if(!res.ok) {
            throw new Error(`[OllamaBackend] Calling backend returned ${res.status}: ${res.statusText}`);
        }
        return res;
    }

    async updateModels(): Promise<Model[]> {
        const res = await this.callBackend("/tags");
        const payload = await res.json();
        return await Promise.all(payload.models.map(async (m: any) => {
            const modelDetailReq = await this.callBackend("/show", {
                body: JSON.stringify({model: m.name}),
                method: 'post'
            });
            const modelDetail = await modelDetailReq.json();
            
            const model = new OllamaModel({
                id: m.model,
                backend: this,
                capabilities: modelDetail.capabilities,
                ...m
            });
            return model;
        }));
    }

    /**
     * Querys all running models and returns their names.
     * @returns Names of running models
     */
    async getRunningModels(): Promise<RunningInfo[]> {
        const res = await this.callBackend("/ps");
        const payload = await res.json();
        return payload.models;
    }

    async running(): Promise<boolean> {
        try {
            const res = await fetch(this.apiUrl, {
                method: "head",
                cache: "no-store",
                signal: AbortSignal.timeout(5000)
            });
            return res.status < 500 && res.status != 403;
        } catch {
            return false;
        }
    }

    /**
     * Starts the Ollama service, if not already running.
     * @returns A promise which resolves when Ollama has started.
     */
    boot(): Promise<void> {
        return invoke("execute", {
            cmd: "ollama app",
            args: ["serve"]
        });
    }

    /**
     * Shuts down the Ollama service.
     * @returns The Ollama service has finished
     */
    shutdown(): Promise<void> {
        return invoke("terminate", {
            process: "ollama app"
        });
    }
}

interface RunningInfo {
    size_vram: number;
    expires_at: Date;
    name: string;
}

export class OllamaModel implements Model {
    readonly name!: string;
    readonly id!: string;
    readonly size!: number;
    readonly capabilities!: Capability[];
    readonly backend!: Backend;

    // Cached runtime information of the model
    private runningInfo?: RunningInfo;

    public constructor(init: OllamaModel) {
        Object.assign(this, init);
    }

    async getRunningInfo(): Promise<RunningInfo|undefined> {
        const currentTime = Date.now();
        if(this.runningInfo && this.runningInfo.expires_at.getUTCMilliseconds() > currentTime) {
            return this.runningInfo;
        }

        const loaded = await (this.backend as OllamaBackend).getRunningModels();
        return this.runningInfo = loaded.find(n => n.name == this.name);
    }
    
    async loaded(): Promise<boolean> {
        const info = await this.getRunningInfo();
        return info !== undefined;
    }

    async getLoadedSize(): Promise<number> {
        const info = await this.getRunningInfo();
        return info?.size_vram ?? -1;
    }

    /**
     * @inheritdoc
     * We can't explicitly load a model with Ollama.
     */
    load(): Promise<void> {
        return Promise.resolve();
    }

    /**
     * @inheritdoc
     * We can't explicitly unload a model with Ollama.
     */
    unload(): Promise<void> {
        return Promise.resolve();
    }

    async* prompt(content: ChatMessage, history?: ChatMessage[]): AsyncIterable<ChatResponse> {
        history ??= [];
        const res = await (this.backend as OllamaBackend).callBackend("/chat", {
            method: "POST",
            headers: {'content-type': "application/json"},
            body: JSON.stringify({
                model: this.id,
                messages: [
                    ...history,
                    content
                ]
            })
        });

        if(res.body === undefined) {
            throw new Error("[OllamaModel] Missing body in response");
        }

        for await(const chunk of readNdJson<ChatResponse>(res.body!)) {
            yield chunk;
            if(chunk.done) break;
        }
    }
}
