<script lang="ts">
    import ChatSidebar from "$lib/ChatSidebar.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import AppContext from "$lib/core/AppContext.svelte";
    import type { Chat } from "$lib/core/Chat";
    import ChatDialog from "$lib/ChatDialog.svelte";
    import ModelSelection from "$lib/ModelSelection.svelte";
    import type { Snapshot } from "./$types";
    import StatusDisplay from "$lib/StatusDisplay.svelte";
    import OllamaStatus from "$lib/OllamaStatus.svelte";
    import Settings from "$lib/core/Settings.svelte";

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
    let selectedModel = $state(undefined);

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
            console.error("Cannot delete chat: Idx out of bounds");
            return;
        }

        await ctx.deleteChat(ctx.chats[idx]);
    }
</script>

<div class="flex flex-row h-screen">
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
            <StatusDisplay></StatusDisplay>
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
