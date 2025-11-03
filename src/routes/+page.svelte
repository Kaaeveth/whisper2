<script lang="ts">
    import ChatSidebar from "$lib/ChatSidebar.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import AppContext from "$lib/core/AppContext.svelte";
    import type { Chat } from "$lib/core/Chat";
    import ChatDialog from "$lib/ChatDialog.svelte";
    import ModelSelection from "$lib/ModelSelection.svelte";
    import type { Snapshot } from "./$types";
    import OllamaStatus from "$lib/OllamaStatus.svelte";
    import Settings from "$lib/core/Settings.svelte";
    import { handleError } from "$lib/Util";

    const ctx = AppContext.getInstance();
    const sidebar = new ToggableElement(true);
    let autoScroll = $derived(ctx.settings.get<boolean>(Settings.AUTO_SCROLL));

    // State about all chats and the current selected one.
    // Every child component uses theses states.
    let selectedChatIdx = $state(-1);
    const chatTitles = $derived(ctx.chats.map(c => c.title));
    const selectedChat: Chat | undefined = $derived.by(() => {
        if(selectedChatIdx > -1) {
            return ctx.chats[selectedChatIdx];
        }
    });

    // Load and save the selected model to disk when it changes
    // We also update the selected model whenever the available models change
    const modelName = ctx.settings.get<string>(Settings.SELECTED_MODEL);
    let selectedModel = $derived(ctx.models.find(m => m.name === modelName));
    $effect(() => {
        if (!selectedModel) return;
        ctx.settings.set(Settings.SELECTED_MODEL, selectedModel.name);
    });

    export const snapshot: Snapshot = {
        capture: () => ({selectedChatIdx, selectedModel}),
        restore(snapshot) {
            selectedModel = snapshot.selectedModel;
            selectedChatIdx = snapshot.selectedChatIdx;
        }
    };

    function clearSelectedChat() {
        selectedChatIdx = -1;
    }

    function newChat(): Chat {
        const chat = ctx.newChat();
        // The first chat is the newly created one
        selectedChatIdx = 0;
        return chat;
    }

    async function deleteChat(idx: number) {
        if(idx >= ctx.chats.length || idx < 0) {
            handleError("Cannot delete chat: Idx out of bounds");
            return;
        }

        await ctx.deleteChat(ctx.chats[idx]);
    }
</script>

<div class="flex flex-row h-full">
    <ChatSidebar
        chatTitles={chatTitles}
        bind:selectedChatIdx
        sidebar={sidebar}
        onNewChat={clearSelectedChat}
        onDeleteChat={deleteChat}>
    </ChatSidebar>
    <div class="flex flex-col gap-2 p-4 grow-1" class:pl-0={!sidebar.open}>
        <div class="flex min-w-xs gap-5">
            <ModelSelection models={ctx.models} bind:selectedModel></ModelSelection>
            <OllamaStatus></OllamaStatus>
        </div>
        <ChatDialog
           chat={selectedChat}
           model={selectedModel}
           autoScroll={autoScroll}
           createChat={newChat}>
       </ChatDialog>
    </div>
</div>
