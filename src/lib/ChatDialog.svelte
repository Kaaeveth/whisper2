<script lang="ts">
    import { Textarea, Button, Card } from "flowbite-svelte";
    import { CaretRightOutline } from "flowbite-svelte-icons";
    import type { Chat, ChatMessage } from "./core/Chat";
    import type { Model } from "./core/LLMBackend";

    interface Props {
        chat?: Chat;
        model?: Model;
        createChat: () => Chat;
    }

    let props: Props = $props();
    let inputChatMsg: string = $state("");
    let generating: boolean = $state(false);

    async function chatSubmit(e: SubmitEvent) {
        e.preventDefault();
        if(!props.model || generating || inputChatMsg.trim().length < 1) return;
        if(!props.chat) {
            props.createChat();
        }

        // TODO: handle errors
        try {
            generating = true;
            const userPrompt: ChatMessage = {
                content: inputChatMsg,
                role: "user"
            };
            const promptResponse = props.model.prompt(userPrompt, props.chat!.history);
            let answer: ChatMessage = $state({content: "", role: "assistant"});
            props.chat!.history.push(userPrompt, answer);

            inputChatMsg = "";

            for await(const res of promptResponse) {
                answer.content += res.message.content;
            }
        } finally {
            generating = false;
        }
    }
</script>

<div class="flex flex-col h-full">
    <div class="flex flex-col grow-1 overflow-y-auto gap-4 justify-start">
        {#if !props.chat}
            <p class="m-auto text-2xl font-medium text-black dark:text-white">
                Start a new chat below
            </p>
        {:else}
            {#each props.chat.history as msg}
                <div class:justify-items-end={msg.role == "user"}>
                    <Card class="p-3 dark:text-white" shadow="sm" horizontal size="lg">
                        {msg.content}
                    </Card>
                </div>
            {/each}
        {/if}
    </div>
    <!-- Message input -->
    <div class="w-130 mx-auto">
        <form onsubmit={chatSubmit}>
            <Textarea 
                disabled={!!!props.model || generating}
                clearable class="mb-4" placeholder="Ask me anything"
                rows={2}
                bind:value={inputChatMsg}>
                {#snippet footer()}
                <div class="flex items-center justify-end h-3">
                    <Button disabled={generating} pill outline type="submit" class="p-0 border-0">
                        <CaretRightOutline size="xl"></CaretRightOutline>
                    </Button>
                </div>
                {/snippet}
            </Textarea>
        </form>
        <p class="text-xs text-gray-500 dark:text-gray-400">
            Language models may hallucinate. Check important information.
        </p>
    </div>
</div>