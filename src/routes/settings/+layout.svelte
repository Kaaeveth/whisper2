<script lang="ts">
    import { AngleLeftOutline } from "flowbite-svelte-icons";
    import { Button, Heading } from "flowbite-svelte";
    import NavButton from "$lib/NavButton.svelte";

    let { children } = $props();
</script>

<div class="p-6 pl-8 h-full">
    <div class="flex flex-row gap-6 sticky top-4 select-none">
        <Button onclick={() => history.back()} color="alternative">
            <AngleLeftOutline></AngleLeftOutline>
        </Button>
        <Heading tag="h3">Settings</Heading>
    </div>
    <div class="flex flex-row mt-8">
        <!-- Sidebar -->
        <nav class="flex flex-col gap-2 w-35 select-none">
            <NavButton href="/settings">⚙️ General</NavButton>
            <NavButton href="/settings/models">🤖 Models</NavButton>
            <NavButton href="/settings/about">ℹ️ About</NavButton>
        </nav>
        <div class="pl-12 grow-1 max-w-2xl">
            {@render children()}
        </div>
    </div>
</div>
