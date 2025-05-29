export type Role = "system"|"user"|"assistant"|"tool";
export type Base64 = string;

export interface ChatMessage {
    role: Role;
    content: string;
    images: Base64[];
}

export interface ChatRequest {
    prompt: ChatMessage;
}

export interface ChatResponse {
    done: boolean;
    message: ChatMessage;
}

export interface Chat {
    title: string;
    history: ChatMessage[];
}
