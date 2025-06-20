export type Role = "system"|"user"|"assistant"|"tool";
export type Base64 = string;

export interface ChatMessage {
    role: Role;
    content: string;
    images?: Base64[];
    // Internal thoughts of the model
    thinking?: string
}

export interface ChatRequest {
    prompt: ChatMessage;
}

export interface ChatResponse {
    done: boolean;
    message: ChatMessage;
}

export interface Chat {
    uuid: string;
    title: string;
    history: ChatMessage[];
    save: () => Promise<void>;
    delete: () => Promise<void>;
}
