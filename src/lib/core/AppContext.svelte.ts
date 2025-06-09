import OllamaBackend from "./backends/Ollama.svelte";
import type { Chat } from "./Chat";
import type { Model } from "./LLMBackend";

export default class AppContext {
    private static instance?: AppContext;

    public static getInstance(): AppContext {
        if(!AppContext.instance) {
            AppContext.instance = new AppContext();
        }
        return AppContext.instance;
    }

    constructor() {
        this.ollamaBackend = new OllamaBackend();
    }

    private isInit: boolean = false;
    private ollamaBackend: OllamaBackend;
    private _models: Model[] = $state([]);

    chats: Chat[] = $state([]);

    get ollama(): OllamaBackend {
        return this.ollamaBackend;
    }

    get models(): Model[] {
        return this._models;
    }

    /**
     * Setup for all backends.
     * Should be called once at application startup.
     */
    async init(): Promise<void> {
        if(this.isInit) return;
        try {
            await this.updateModels();
            // TODO: load saved state (Chats, ...)
            this.isInit = true;
        } catch(e) {
            console.error("Error initializing: "+e);
        }
    }

    /**
     * Fetchs all available models of all backends.
     * Note that this updates AppContext.models as well,
     * which is a Svelte state.
     * @returns Available Models of all backends
     */
    async updateModels(): Promise<Model[]> {
        return this._models = await this.ollamaBackend.updateModels();
    }

    newChat(): Chat {
        const chat: Chat = {
            title: "New Chat",
            history: []
        }
        this.chats.push(chat);
        return chat;
    }
}
