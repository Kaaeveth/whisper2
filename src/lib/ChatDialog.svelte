<script lang="ts">
    import { Textarea, Button, Card, Checkbox, Spinner } from "flowbite-svelte";
    import { CaretRightOutline } from "flowbite-svelte-icons";
    import type { Chat, ChatMessage } from "./core/Chat";
    import { generateTitle, prependAssistantContext, type Model } from "./core/LLMBackend";
    import MarkdownIt from "markdown-it";

    interface Props {
        chat?: Chat;
        model?: Model;
        createChat: () => Chat;
    }

    /** 
     * NOTE: We currently render markdown with each new token
     * returned by a model, which is quite inefficient.
     * We should replace this with an incremental md parser in the future.
     * For now, however, this solution is still relatively fast
     * and not too resource heavy, especially for small responses.
     * */
    const md = MarkdownIt();

    let props: Props = $props();
    let inputChatMsg: string = $state("");
    let think: boolean = $state(false); // Use "thinking" mode of the model
    let generating: boolean = $state(false); // Is a prompt being answered?
    let chatContainer: HTMLDivElement | undefined = $state();

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
                think
            );
            let answer: ChatMessage = $state({
                content: "",
                role: "assistant"
            });
            props.chat!.history.push(userPrompt, answer);
            inputChatMsg = "";

            // Assemble the completion
            // Since the content is reactive, the UI gets
            // updated automatically
            for await(const res of promptResponse) {
                answer.content += res.message.content;
                scrollToLastChatMsg();
            }

            // Generate title for the chat on first prompt
            if(needsTitle)
                props.chat!.title = await generateTitle(props.model, $state.snapshot(props.chat!.history));
        } finally {
            generating = false;
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

<div class="flex flex-col h-95/100 lg:w-4xl w-auto mx-auto">
    <div bind:this={chatContainer} class="flex flex-col grow-1 overflow-y-auto gap-3 justify-start">
        {#if !props.chat}
            <p class="m-auto text-2xl font-medium text-black dark:text-white">
                Start a new chat below
            </p>
        {:else}
            {#each props.chat.history as msg}
                <div class:justify-items-end={msg.role == "user"}>
                    <Card class="p-3 dark:text-white flex-col!" shadow="sm" horizontal size="lg">
                        {#if msg.content.length > 0}
                            {@html md.render(msg.content)}
                        {:else}
                            <Spinner class="mx-auto"></Spinner>
                        {/if}
                    </Card>
                </div>
            {/each}
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
                    <Button disabled={generating} pill outline
                            type="submit" class="p-0 border-0 ml-auto">
                        <CaretRightOutline size="xl"></CaretRightOutline>
                    </Button>
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