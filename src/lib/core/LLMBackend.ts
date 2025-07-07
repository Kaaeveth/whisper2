import type { ChatMessage, ChatResponse } from "./Chat";

export interface Backend {
    readonly name: string;

    updateModels(): Promise<Model[]>;
    running(): Promise<boolean>;

    boot(): Promise<void>;
    shutdown(): Promise<void>;
}

export type Capability = "completion"|"vision"|"tools";

export interface Model {
    name: string;
    id: string;
    size: number;
    capabilities: Capability[];
    backend: Backend;

    /**
     * Gets whether the model is loaded into memory.
     * @returns True if loaded
     */
    loaded(): Promise<boolean>;

    /**
     * Gets the size of the model in memory or -1
     * if its not loaded.
     */
    getLoadedSize(): Promise<number>;

    /**
     * Loads the model into memory for usage.
     * Not necessary to call before calling prompt.
     * @see prompt
     */
    load(): Promise<void>;

    /**
     * Unloads the model.
     * Calling prompt afterwards loads the model again.
     */
    unload(): Promise<void>;

    prompt(content: ChatMessage, history?: ChatMessage[], options?: PromptOptions): AsyncIterable<ChatResponse>;
}

export interface PromptOptions {
    think?: boolean;
    abort?: AbortSignal;
}

/**
 * Generates a title for a chat history.
 * The `history` needs to have at least two entries!
 * @throws Error if the `history` has less than two entries.
 * @param model The model to use
 * @param history The chat history to generate a title for
 * @returns A title
 */
export async function generateTitle(model: Model, history: ChatMessage[]): Promise<string> {
    if(history.length < 2) {
        throw new Error("Chat history needs at least two entries");
    }
    
    let title = "";
    const instruction: ChatMessage[] = [
        {
            role: "system",
            content:
            `You generate a brief title for the given user prompt and chat history.
            The title should not be longer than five words and not empty.
            The title must not contain any Markdown or HTML.
            The title must only contain plain text!
            You must only output the title and nothing more!`
        },
        ...history
    ];
    for await(const chunk of model.prompt(instruction[0], instruction)) {
        title += chunk.message.content;
    }
    return title;//.split(' ').slice(0,5).join(' ');
}

export function prependAssistantContext(history: ChatMessage[]): ChatMessage[] {
    const assistant: ChatMessage = {
        role: "system",
        content: 
`You are a helpful, honest, and knowledgeable AI assistant.
You respond clearly, directly, and professionally, without unnecessary flattery or emotional language.
You aim to explain complex topics in a concise and understandable way, while being transparent about what you know and what you don't.
If you're unsure about something, say so clearly — do not guess.
Be respectful, analytical, and solution-oriented.

Always format your output using Markdown when appropriate:
- Use bullet points for lists.
- Use headings to organize content.
- Use fenced code blocks (\`\`\`language) for code.
- Use tables when comparing options or presenting structured data.

If the user prefers a different tone (e.g., more casual, more technical, more formal), adapt accordingly.
Always assume the user is technically capable unless stated otherwise.

Speak in English unless the user requests another language.
`
    };
    return [assistant, ...history];
}
