<script lang="ts">
    import { Textarea, Button, Card, Checkbox } from "flowbite-svelte";
    import AssistantResponse from "./AssistantResponse.svelte";
    import { CaretRightOutline, StopOutline } from "flowbite-svelte-icons";
    import type { Chat, ChatMessage } from "./core/Chat";
    import { generateTitle, prependAssistantContext, type Model } from "./core/LLMBackend";

    interface Props {
        chat?: Chat;
        model?: Model;
        createChat: () => Chat;
    }

    let props: Props = $props();
    let inputChatMsg: string = $state("");
    let think: boolean = $state(false); // Use "thinking" mode of the model
    let generating: boolean = $state(false); // Is a prompt being answered?
    let chatContainer: HTMLDivElement | undefined = $state();

    // Ctrl for aborting chat completions
    // NOTE: The tauri http-plugin currently doesn't close the http response stream
    //       when cancelling a request. Meaning Ollama will continue generating tokens.
    // see: https://github.com/tauri-apps/plugins-workspace/pull/2562
    let promptAbortController = new AbortController();

    const scrollToLastChatMsg = () => 
        chatContainer?.scrollTo({behavior: "smooth", top: chatContainer!.scrollHeight});

    /**
     * Submits the entered message to the model
     * and outputs the answer into the chat history.
     */
    async function submitChatPrompt(e?: SubmitEvent) {
        e?.preventDefault();
        if(!props.model || generating || inputChatMsg.trim().length < 1) return;

        let needsTitle = false;
        if(!props.chat) {
            props.createChat();
            needsTitle = true;
        }

        // Start generating a chat completion
        // TODO: handle errors & abort
        try {
            generating = true;
            scrollToLastChatMsg();

            const userPrompt: ChatMessage = {
                content: inputChatMsg,
                role: "user"
            };

            // Generate chat completion
            // We get a stream of strings, which we assemble below
            const promptResponse = props.model.prompt(
                userPrompt,
                prependAssistantContext($state.snapshot(props.chat!.history)),
                {think, abort: promptAbortController.signal}
            );
            let answer: ChatMessage = $state({
                content: "",
                role: "assistant"
            });
            props.chat!.history.push(userPrompt, answer);
            inputChatMsg = "";

            // Assemble the completion
            // Since the content is reactive, the UI gets
            // updated automatically.
            // If the completion gets cancelled, an exception gets thrown
            // but we still generate a title and save the chat.
            try {
                for await(const res of promptResponse) {
                    answer.content += res.message.content;
                    scrollToLastChatMsg();
                }
            } finally {
                // Generate title for the chat on first prompt
                if(needsTitle)
                    props.chat!.title = await generateTitle(props.model, $state.snapshot(props.chat!.history));
    
                await props.chat!.save();
            }

        } finally {
            generating = false;
            promptAbortController.abort();
            promptAbortController = new AbortController();
        }
    }

    // Submit chat message on "enter" and line-break on "shift+enter"
    function onMessageKeyDown(e: KeyboardEvent) {
        if(e.key == "Enter" && !e.shiftKey) {
            e.preventDefault();
            submitChatPrompt();
        }
    }
</script>

<div class="flex flex-col h-95/100 w-full">
    <div bind:this={chatContainer} class="flex flex-col grow-1 overflow-y-auto gap-3 w-full">
        {#if !props.chat}
            <p class="m-auto content-center grow-1 text-2xl font-medium text-black dark:text-white">
                Start a new chat below
            </p>
        {:else}
            <div class="mx-auto xl:w-5xl lg:w-3xl w-auto">
                {#each props.chat.history as msg}
                    {#if msg.role == "user"}
                        <div class="lg:justify-items-end my-10">
                            <Card class="p-3 dark:text-white flex-col! shadow-none" horizontal size="lg">
                                {msg.content}
                            </Card>
                        </div>
                    {:else}
                        <AssistantResponse content={msg.content}></AssistantResponse>
                    {/if}
                {/each}
            </div>
        {/if}
    </div>
    <!-- Message input -->
    <div class="lg:w-150 w-full mx-auto">
        <form onsubmit={submitChatPrompt}>
            <Textarea
                disabled={!!!props.model || generating}
                onkeydown={onMessageKeyDown}
                clearable class="mb-4" placeholder="Ask me anything" rows={2}
                bind:value={inputChatMsg}>
                {#snippet footer()}
                <div class="flex items-center h-3">
                    <Checkbox bind:checked={think}>Think</Checkbox>
                    {#if !generating}
                        <Button disabled={generating} pill outline
                                type="submit" class="p-0 border-0 ml-auto">
                            <CaretRightOutline size="xl"></CaretRightOutline>
                        </Button>
                    {:else}
                        <Button 
                            onclick={() => promptAbortController.abort()}
                            disabled={!generating} pill outline class="p-0 border-0 ml-auto">
                            <StopOutline size="xl"></StopOutline>
                        </Button>
                    {/if}
                </div>
                {/snippet}
            </Textarea>
            <input type="submit" hidden />
        </form>
        <p class="text-xs text-gray-500 dark:text-gray-400">
            Language models may hallucinate. Check important information.
        </p>
    </div>
</div>