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
    readonly name: string;
    readonly id: string;
    readonly size: number;
    readonly capabilities: Capability[];
    readonly backend: Backend;

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

    prompt(content: ChatMessage, history?: ChatMessage[], think?: boolean): AsyncIterable<ChatResponse>;
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
            Output only the title, nothing more!`
        },
        ...history
    ];
    for await(const chunk of model.prompt(instruction[0], instruction, false)) {
        title += chunk.message.content;
    }
    return title;//.split(' ').slice(0,5).join(' ');
}

export function prependAssistantContext(history: ChatMessage[]): ChatMessage[] {
    console.log(history);
    const assistant: ChatMessage = {
        role: "system",
        content: 
        `You are a helpful assistant!
        You may use Github-flavored Markdown in your answers at your own discretion (except for images)!`
    };
    return [assistant, ...history];
}
