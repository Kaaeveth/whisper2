import type { Chat, ChatMessage } from "./Chat";
import type { Model } from "./LLMBackend";
import { load, type Store } from '@tauri-apps/plugin-store';
import Settings from "./Settings.svelte";
import OllamaBackend from "./backends/Ollama.svelte";
import { invoke } from "@tauri-apps/api/core";

export default class AppContext {
    private static instance?: AppContext;

    public static getInstance(): AppContext {
        if(!AppContext.instance) {
            AppContext.instance = new AppContext();
        }
        return AppContext.instance;
    }

    private static CHAT_STORE_PATH: string = "chats.json";

    constructor() {
        this.ollamaBackend = new OllamaBackend();
        this._settings = new Settings();
    }

    private isInit: boolean = false;
    private _status: AppStatus = $state({status: "ok", msg: ""});
    private ollamaBackend: OllamaBackend;
    private _models: Model[] = $state([]);

    // Store for saving and loading chats from disk
    // Initialized at startup
    private _chatStore!: Store;
    private _settings: Settings;

    private _chats: Chat[] = $state([]);

    get ollama(): OllamaBackend {
        return this.ollamaBackend;
    }

    get models(): Model[] {
        return this._models;
    }

    get chats(): Chat[] {
        return this._chats;
    }

    get settings(): Settings {
        return this._settings;
    }

    /**
     * Setup for all backends.
     * Should be called once at application startup.
     */
    async init(): Promise<void> {
        if(this.isInit) return;

        try {
            await this._settings.init();
            this._chatStore = await load(AppContext.CHAT_STORE_PATH);
            await Promise.all([
                this.loadChats(),
                this.updateModels()
            ]);

            this.isInit = true;
        } catch(e: any) {
            console.error(e);
            this.status = {msg: String(e), status: "error"};
            throw e;
        }
    }

    set status(status: AppStatus) {
        this._status.status = status?.status ?? "ok";
        this._status.msg = status?.msg ?? "";
    }

    get status(): AppStatus {
        return this._status;
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
        let snapshot: Partial<Chat> = this.convertChatToSerializable(chat);
        delete snapshot.uuid;
        this._chatStore.set(chat.uuid, snapshot);
    }

    private convertChatToSerializable(chat: Chat) {
        if(chat instanceof ReactiveChat) {
            return (chat as ReactiveChat).toPoco();
        } else {
            return $state.snapshot(chat) as Partial<Chat>;
        }
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
                chat.createdAt = new Date(chat.createdAt);
                this._chats.push(new ReactiveChat(this, chat));
            }
        }
        this._chats.sort((a,b) => b.createdAt.getTime() - a.createdAt.getTime());
    }

    async saveChatsToDisk(): Promise<void> {
        await this._chatStore.save();
        await invoke("save_chats", {
            storeName: AppContext.CHAT_STORE_PATH
        });
    }

    async importChatsFromDisk(): Promise<void> {
        await invoke("import_chats", {
            storeName: AppContext.CHAT_STORE_PATH
        });
        await this.loadChats();
    }

    async deleteAllChats(): Promise<void> {
        await this._chatStore.clear();
        await this.loadChats();
    }

    /**
     * Fetchs all available models of all backends.
     * Note that this updates AppContext.models as well,
     * which is a Svelte state.
     * @returns Available Models of all backends
     */
    async updateModels(): Promise<Model[]> {
        if(await this.ollamaBackend.running()) {
            this._models = await this.ollamaBackend.updateModels();
        }
        return this._models;
    }

    newChat(): Chat {
        const chat: Chat = new ReactiveChat(this);
        this._chats.unshift(chat);
        return chat;
    }
}

class ReactiveChat implements Chat {
    public title: string = $state("New Chat");
    public history: ChatMessage[] = $state([]);
    
    private ctx: AppContext;
    private _uuid: string;
    private _createdAt: Date;

    constructor(ctx: AppContext, chat?: Chat) {
        this.ctx = ctx;
        if(!chat){
            this._uuid = crypto.randomUUID();
            this._createdAt = new Date();
        } else {
            this._uuid = chat.uuid;
            this._createdAt = chat.createdAt;
            this.title = chat.title;
            this.history = chat.history;
        }
    }

    async save() {
        await this.ctx.saveChat(this);
    }

    async delete() {
        await this.ctx.deleteChat(this);
    }

    get uuid() {
        return this._uuid;
    }

    get createdAt(): Date {
        return this._createdAt;
    }

    toPoco(): Omit<Chat, "save"|"delete"> {
        return {
            title: $state.snapshot(this.title),
            history: $state.snapshot(this.history),
            uuid: this._uuid,
            createdAt: this._createdAt
        }
    }
}

export interface AppStatus {
    status: "ok"|"warn"|"error";
    msg: string;
}
