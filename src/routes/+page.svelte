<script lang="ts">
    import ChatSidebar from "$lib/ChatSidebar.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import AppContext from "$lib/core/AppContext.svelte";
    import type { Chat } from "$lib/core/Chat";
    import ChatDialog from "$lib/ChatDialog.svelte";
    import ModelSelection from "$lib/ModelSelection.svelte";
    import type { Snapshot } from "./$types";

    const ctx = AppContext.getInstance();
    const sidebar = new ToggableElement(true);
    
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
        selectedChatIdx = ctx.chats.length-1; 
        return chat;
    }
</script>

<div class="flex flex-row h-screen">
    <ChatSidebar 
        chatTitles={chatTitles}
        bind:selectedChatIdx
        sidebar={sidebar}
        onNewChat={clearSelectedChat}>
    </ChatSidebar>
    <div class="flex flex-col gap-2 p-4 grow-1" class:pl-0={!sidebar.open}>
        <div class="min-w-xs">
            <ModelSelection models={ctx.models} bind:selectedModel></ModelSelection>
        </div>
        <ChatDialog 
           chat={selectedChat}
           model={selectedModel}
           createChat={newChat}>
       </ChatDialog>
    </div>
</div>
