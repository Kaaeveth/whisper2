<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import { formatByteSize, handleError } from "$lib/Util";
    import { Heading, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
    import { Input, Label, Button } from "flowbite-svelte";
    import { RefreshOutline } from "flowbite-svelte-icons";

    const ctx = AppContext.getInstance();
    let updateModels = $state(ctx.updateModels());

    let ollamaUrl = $state(ctx.ollama.apiUrl.href);
    async function updateOllamaUrl() {
        try {
            const url = new URL(ollamaUrl);
            await ctx.ollama.setUrl(url);
            await ctx.updateOllamaModels();
        } catch(e) {
            handleError(e);
        }
    }
</script>

<div class="w-full">
    <div class="flex gap-2 mb-4">
        <Heading tag="h4" class="font-medium">Models</Heading>
        <Button onclick={() => updateModels = ctx.updateModels()} class="p-1! ml-auto" color="alternative" size="xl">
            <RefreshOutline class="h-6 w-6"></RefreshOutline>
        </Button>
    </div>
    <Table striped hoverable border={false}>
        <TableHead>
            <TableHeadCell class="dark:text-white">Model name</TableHeadCell>
            <TableHeadCell class="dark:text-white">Provider</TableHeadCell>
            <TableHeadCell class="dark:text-white">Size</TableHeadCell>
            <TableHeadCell class="dark:text-white">Status</TableHeadCell>
        </TableHead>
        <TableBody>
            {#await updateModels}
                <TableBodyCell colspan={4}>Loading...</TableBodyCell>
            {:then}
                {#each ctx.models as model}
                <TableBodyRow>
                    <TableBodyCell class="dark:text-white">{model.name}</TableBodyCell>
                    <TableBodyCell class="dark:text-white">{model.backend.name}</TableBodyCell>
                    <TableBodyCell class="dark:text-white">{formatByteSize(model.size)}</TableBodyCell>
                    <TableBodyCell class="dark:text-white">
                        {#await model.loaded() then loaded}
                            {loaded ? "Loaded" : "Not loaded"}
                        {/await}
                    </TableBodyCell>
                </TableBodyRow>
                {/each}
            {:catch error}
                <TableBodyCell colspan={4}>{error}</TableBodyCell>
            {/await}
        </TableBody>
    </Table>

    <div class="border-b border-gray-400 my-4"></div>

    <Heading tag="h4" class="font-medium mb-4">Ollama</Heading>
    <div class="grid gap-3 gap-y-2 grid-cols-3 w-3/4">
        <Label for="ollamaEndpoint">Endpoint</Label>
        <Input bind:value={ollamaUrl} class="row-start-2 col-span-2" type="url" id="ollamaEndpoint" />
        <Button onclick={() => updateOllamaUrl()} class="row-start-2">Update</Button>

        <!-- <Label class="row-start-3" for="ollamaAddModel">Add model</Label>
        <Input class="row-start-4 col-span-2" placeholder="mistral:latest" type="url" id="ollamaAddModel" />
        <Button class="row-start-4" disabled>Pull</Button> -->
    </div>
</div>
