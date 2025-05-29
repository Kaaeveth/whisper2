<script lang="ts" module>
    const ctx = AppContext.getInstance();
    await ctx.updateModels();
</script>

<script lang="ts">
    import ChatSidebar from "$lib/ChatSidebar.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import AppContext from "$lib/core/AppContext.svelte";
    import type { Chat } from "$lib/core/Chat";
    import ChatDialog from "$lib/ChatDialog.svelte";
    import ModelSelection from "$lib/ModelSelection.svelte";

    const sidebar = new ToggableElement(true);
    
    let selectedChatIdx = $state(-1);
    const chatTitles = $derived(ctx.chats.map(c => c.title));
    const selectedChat: Chat | undefined = $derived.by(() => {
        if(selectedChatIdx > -1) {
            return ctx.chats[selectedChatIdx];
        }
    });
    let selectedModel = $state(undefined);

    function newChat() {
        /* ctx.newChat();
        selectedChatIdx = ctx.chats.length-1; */
        selectedChatIdx = -1;
    }
</script>

<div class="flex flex-row h-screen">
    <ChatSidebar 
        chatTitles={chatTitles}
        bind:selectedChatIdx
        sidebar={sidebar}
        onNewChat={newChat}>
    </ChatSidebar>
    <div class="flex flex-col gap-2 p-4 grow-1" class:pl-0={!sidebar.open}>
        <div class="min-w-xs">
            <ModelSelection models={ctx.models} bind:selectedModel></ModelSelection>
        </div>
        <ChatDialog 
           chat={selectedChat}>
       </ChatDialog>
    </div>
</div>
