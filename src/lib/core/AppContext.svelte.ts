import OllamaBackend from "./backends/Ollama.svelte";
import type { Chat } from "./Chat";
import type { Model } from "./LLMBackend";
import { load, Store } from '@tauri-apps/plugin-store';

type ChatStorage = Omit<Chat, "uuid" | "save" | "delete">;

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

    // Store for saving and loading chats from disk
    // Initialized at startup
    private _chatStore!: Store;

    private _chats: Chat[] = $state([]);

    get ollama(): OllamaBackend {
        return this.ollamaBackend;
    }

    get models(): Model[] {
        return this._models;
    }

    get chats() : Chat[] {
        return this._chats;
    }

    /**
     * Setup for all backends.
     * Should be called once at application startup.
     */
    async init(): Promise<void> {
        if(this.isInit) return;

        try {
            this._chatStore = await load("chats.json");
            await this.updateModels();
            await this.loadChats();
            this.isInit = true;
        } catch(e) {
            console.error("Error initializing: "+e);
            throw e;
        }
    }

    /**
     * Saves all current chats to disk.
     */
    async saveChats() {
        for(const chat of this._chats) {
            await this.saveChat(chat);
        }
    }

    /**
     * Saves the given chat to disk.
     * @param chat The chat to save
     */
    async saveChat(chat: Chat) {
        const snapshot: Partial<Chat> = $state.snapshot(chat) as Partial<Chat>;
        delete snapshot.uuid;
        this._chatStore.set(chat.uuid, snapshot);
    }

    /**
     * Deletes the given chat from the context and disk.
     * Does nothing if the chat is not in `this.chats`.
     * @param chat The chat to delete
     */
    async deleteChat(chat: Chat) {
        await this._chatStore.delete(chat.uuid);
        const idx = this._chats.findIndex(c => c.uuid === chat.uuid);
        if(idx < 0) return;
        this._chats.splice(idx, 1);
    }

    /**
     * Load all chats from disk into `this.chats`.
     */
    async loadChats() {
        this._chats = [];
        for(const chatId of await this._chatStore.keys()) {
            let chat = await this._chatStore.get<Chat>(chatId);
            if(chat) {
                chat.uuid = chatId;
                this._chats.push(chat);
            }
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
        const chat: Chat = $state({
            uuid: crypto.randomUUID(),
            title: "New Chat",
            history: [],
            save: () => this.saveChat(chat),
            delete: () => this.deleteChat(chat)
        });
        this._chats.push(chat);
        return chat;
    }
}
