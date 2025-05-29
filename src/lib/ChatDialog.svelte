<script lang="ts">
    import { Textarea, Button, Card } from "flowbite-svelte";
    import type { Chat } from "./core/Chat";
    import { CaretRightOutline } from "flowbite-svelte-icons";

    interface Props {
        chat?: Chat;
    }

    let props: Props = $props();
</script>

<div class="flex flex-col h-full">
    <div class="flex flex-col grow-1 overflow-y-auto gap-1 justify-start">
        {#if !props.chat}
            <p class="m-auto text-2xl font-medium text-black dark:text-white">
                Start a new chat below
            </p>
        {:else}
            {#each props.chat.history as msg}
                <div class:justify-items-start={msg.role == "assistant"}>
                    <Card class="p-2 sm:p-6 md:p-8" shadow="sm" horizontal size="md">
                        {msg.content}
                    </Card>
                </div>
            {/each}
        {/if}
    </div>
    <!-- Message input -->
    <div class="w-130 mx-auto">
        <form>
            <Textarea class="mb-4" placeholder="Ask me anything">
                {#snippet footer()}
                <div class="flex items-center justify-end">
                    <Button pill outline type="submit" class="p-1">
                        <CaretRightOutline size="lg"></CaretRightOutline>
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