<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { CloseSidebarAltOutline } from "flowbite-svelte-icons";
    import type ToggableElement from "$lib/ToggableElement.svelte";

    const props: {sidebar: ToggableElement} = $props();
</script>

<Button color="alternative" outline={false} class="p-1" onclick={() => props.sidebar.toggle()}>
    <CloseSidebarAltOutline class="h-7 w-7"></CloseSidebarAltOutline>
</Button>