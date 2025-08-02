<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import Settings from "$lib/core/Settings.svelte";
    import ModalDialog from "$lib/ModalDialog.svelte.ts";
    import { handleError } from "$lib/Util";
    import { Button, DarkMode, Heading, ButtonGroup, Toggle } from "flowbite-svelte";

    const ctx = AppContext.getInstance();
    const dialog = ModalDialog.get();
    let autoScroll = $state(ctx.settings.get<boolean>(Settings.AUTO_SCROLL));
    $effect(() => {
        ctx.settings.set(Settings.AUTO_SCROLL, autoScroll);
    });

    async function saveChats() {
        try {
            await ctx.saveChatsToDisk();
        } catch(e) {
            handleError(e);
        }
    }

    async function importChats() {
        try {
            if (await dialog.showModal({
                title: "Import Chats",
                confirmColor: "primary",
                confirmText: "Import",
                abortText: "Abort",
                content: confirmImport
            }))
            {
                await ctx.importChatsFromDisk();
            }
        } catch(e) {
            handleError(e);
        }
    }

    async function deleteChats() {
        try {
            if (await dialog.showModal({
                title: "Delete all Chats",
                confirmColor: "red",
                confirmText: "Delete",
                abortText: "Abort",
                content: confirmDelete
            }))
            {
                await ctx.deleteAllChats();
            }
        } catch(e) {
            handleError(e);
        }
    }
</script>

{#snippet confirmDelete()}
    <p>Do you want to delete <strong>all</strong> chats?</p>
{/snippet}

{#snippet confirmImport()}
    <p>
        This will import all chats from the selected file.
        Duplicate chats will be <strong>overwritten</strong>!
    </p>
{/snippet}

<div class="w-full">
    <Heading tag="h4" class="font-medium mb-4">Theme</Heading>
    <DarkMode/>
    <div class="border-b border-gray-400 my-2"></div>

    <Heading tag="h4" class="font-medium my-4">Chats</Heading>
    <div class="flex gap-2">
        <ButtonGroup>
            <Button outline onclick={() => saveChats()}>Export</Button>
            <Button outline onclick={() => importChats()}>Import</Button>
        </ButtonGroup>
        <Button color="red" onclick={() => deleteChats()}>Delete</Button>
    </div>
    <div class="flex flex-col my-4">
        <Toggle bind:checked={autoScroll}>Autoscroll in chat dialog</Toggle>
    </div>
</div>
