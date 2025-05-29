<script lang="ts">
    import { Sidebar, SidebarItem, SidebarGroup, Button, uiHelpers, sidebar } from "flowbite-svelte";
    import { ToolsOutline } from "flowbite-svelte-icons";
    import SidebarButton from "$lib/SidebarButton.svelte";
    import ToggableElement from "$lib/ToggableElement.svelte";
    import { goto } from "$app/navigation";

    interface Props {
        chatTitles: string[];
        onNewChat?: Function;
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
</script>

<style>
</style>

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
    <SidebarGroup>
        {#each props.chatTitles as title, idx}
            <SidebarItem
                activeClass="flex items-center p-2 text-base font-normal text-white bg-primary-600 dark:bg-primary-700 rounded-lg dark:text-white hover:bg-primary-800 dark:hover:bg-primary-800"
                label={title}
                active={selectedChatIdx == idx}
                onclick={(_) => onChatClicked(idx)}
            />
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