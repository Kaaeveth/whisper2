<script lang="ts">
    import { Button, DarkMode, Heading, ButtonGroup } from "flowbite-svelte";
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
</div>