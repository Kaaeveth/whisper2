<script lang="ts">
    import { Sidebar, SidebarItem, SidebarGroup, Button } from "flowbite-svelte";
    import { ToolsOutline, TrashBinOutline } from "flowbite-svelte-icons";
    import SidebarButton from "$lib/SidebarButton.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import { goto } from "$app/navigation";
    import ModalDialog, { type ShowModalOptions } from "./ModalDialog.svelte.ts";

    interface Props {
        chatTitles: string[];
        onNewChat?: () => void;
        onDeleteChat?: (idx: number) => void;
        selectedChatIdx?: number;
        sidebar: ToggableElement;
    };

    let {
        selectedChatIdx = $bindable(0),
        onNewChat = () => {},
        ...props
    }: Props = $props();

    function onChatClicked(idx: number) {
        selectedChatIdx = idx;
    }

    // == Deleting chats ==
    const modalOptions: ShowModalOptions = {
        confirmText: "Delete",
        confirmColor: "red",
        abortText: "Abort",
        title: "Delete Chat?",
        content: deletePopup
    }
    let deleteChatIdx = $state(-1); // Chat idx to be deleted
    let deletePromptChatTitle = $derived(props.chatTitles[deleteChatIdx]); // Title of chat to be deleted

    async function promptDeleteChat(idx: number) {
        if(idx < 0) return;
        deleteChatIdx = idx;
        if(await ModalDialog.get().showModal(modalOptions) && props.onDeleteChat)
            props.onDeleteChat(deleteChatIdx);
        deleteChatIdx = -1;
    }
</script>

<style>
</style>

{#snippet deletePopup()}
    <p>The chat <i>{deletePromptChatTitle}</i> will be deleted forever.</p>
{/snippet}

{#if props.sidebar.open}
<Sidebar backdrop={false} isOpen={props.sidebar.open} activateClickOutside={false} closeSidebar={() => props.sidebar.open = !props.sidebar.open}
         class="z-50" divClass="flex flex-col h-screen" position="static" alwaysOpen={true}
    >
    <SidebarGroup class="mb-4">
        <div class="flex flex-row gap-2">
            <SidebarButton sidebar={props.sidebar}></SidebarButton>
            <Button onclick={(_) => onNewChat()} pill class="grow-1">New Chat</Button>
        </div>
    </SidebarGroup>
    <SidebarGroup class="space-y-2 overflow-y-auto">
        {#each props.chatTitles as title, idx}
            <SidebarItem
                activeClass="flex items-center p-2 text-base font-normal text-white bg-primary-500 dark:bg-primary-900 rounded-lg dark:text-white hover:bg-gray-800 dark:hover:bg-gray-700"
                spanClass="ms-3 truncate"
                class="sideitem"
                aClass="justify-between"
                label={title}
                active={selectedChatIdx == idx}
                onclick={(_) => onChatClicked(idx)}
            >
                {#snippet subtext()}
                    <TrashBinOutline 
                        class="hidden dots"
                        color={"red"}
                        onclick={() => promptDeleteChat(idx)}>
                    </TrashBinOutline>
                {/snippet}
            </SidebarItem>
        {/each}
    </SidebarGroup>
    <div class="mt-auto">
        <SidebarGroup border>
            <SidebarItem label="Settings" onclick={() => goto("/settings")}>
                {#snippet icon()}
                    <ToolsOutline class="h-5 w-5 text-gray-500 transition duration-75 group-hover:text-gray-900 dark:text-gray-400 dark:group-hover:text-white" />
                {/snippet}
            </SidebarItem>
        </SidebarGroup>
    </div>
</Sidebar>
{:else}
<div class="px-3 py-4">
    <SidebarButton sidebar={props.sidebar}></SidebarButton>
</div>
{/if}