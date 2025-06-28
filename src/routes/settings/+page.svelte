<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import Settings from "$lib/core/Settings.svelte";
    import { Button, DarkMode, Heading, ButtonGroup, Checkbox } from "flowbite-svelte";

    const ctx = AppContext.getInstance();
    let autoScroll = $state(ctx.settings.get<boolean>(Settings.AUTO_SCROLL));
    $effect(() => {
        ctx.settings.set(Settings.AUTO_SCROLL, autoScroll);
    });
</script>

<div class="w-full">
    <Heading tag="h4" class="font-medium mb-4">Theme</Heading>
    <DarkMode/>
    <div class="border-b border-gray-400 my-2"></div>

    <Heading tag="h4" class="font-medium my-4">Chats</Heading>
    <div class="flex gap-2">
        <ButtonGroup>
            <Button outline>Export</Button>
            <Button outline>Import</Button>
        </ButtonGroup>
        <Button color="red">Delete</Button>
    </div>
    <div class="flex flex-col my-4">
        <Checkbox bind:checked={autoScroll}>Autoscroll in chat dialog</Checkbox>
    </div>
</div>