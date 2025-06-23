<script lang="ts">
    import { Button, Heading, Modal } from "flowbite-svelte";
    import ModalDialog from "./ModalDialog.svelte.ts";

    const manager = ModalDialog.get();
</script>

<Modal bind:open={manager.componentState.modalOpen}>
    <Heading tag="h3">{manager.componentState.title}</Heading>
    {@render manager.componentState.content?.()}

    {#snippet footer()}
        <Button 
            color={manager.componentState.confirmColor ?? "primary"}
            onclick={() => manager.closeModal(true)}>
            {manager.componentState.confirmText}
        </Button>
        <Button color="alternative" onclick={() => manager.closeModal(false)}>
            {manager.componentState.abortText}
        </Button>
    {/snippet}
</Modal>
