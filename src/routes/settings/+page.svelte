<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import Settings from "$lib/core/Settings.svelte";
    import ModalDialog from "$lib/ModalDialog.svelte.ts";
    import { showInfo } from "$lib/Snackbar.svelte";
    import { handleError } from "$lib/Util";
    import { Button, DarkMode, Heading, ButtonGroup, Toggle, Card } from "flowbite-svelte";

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
            handleError(e, {userMsg: "Error saving chats"});
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
            handleError(e, {userMsg: "Error deleting chats"});
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
    <Card class="p-4 w-full mb-6" size="lg">
        <Heading tag="h4" class="font-medium mb-4">Theme</Heading>
        <DarkMode/>
    </Card>

    <Card class="p-4 w-full mb-6" size="lg">
        <Heading tag="h4" class="font-medium mb-4">Chats</Heading>
        <div class="flex gap-2">
            <ButtonGroup>
                <Button outline onclick={() => saveChats()}>Export</Button>
                <Button outline onclick={() => importChats()}>Import</Button>
            </ButtonGroup>
            <Button color="red" onclick={() => deleteChats()}>Delete</Button>
        </div>
        <div class="flex flex-col mt-4">
            <Toggle bind:checked={autoScroll}>Autoscroll in chat dialog</Toggle>
        </div>
    </Card>
    {#if ctx.debug}
        <Card class="p-4 w-full" size="lg">
            <Heading tag="h4">Debug</Heading>
            <Button onclick={() => showInfo("Debug")}>Show test snackbar</Button>
        </Card>
    {/if}
</div>
