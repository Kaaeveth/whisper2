<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import { showInfo, showWarning } from "$lib/Snackbar.svelte";
    import { formatByteSize, handleError } from "$lib/Util";
    import { Card, Heading, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
    import { Input, Label, Button } from "flowbite-svelte";
    import { RefreshOutline } from "flowbite-svelte-icons";
    import { open } from '@tauri-apps/plugin-dialog';

    const ctx = AppContext.getInstance();
    let updateModels = $state(ctx.updateModels());

    let ollamaUrl = $state(ctx.ollama.apiUrl.href);
    let ollamaModelsPath = $state(ctx.ollama.modelsPath);
    let ollamaModelsPathPlaceholder = $derived(ollamaModelsPath ? "" : "Ollama default");
    let updatingConfig = $state(false);

    async function updateOllamaUrl() {
        try {
            const url = new URL(ollamaUrl);
            await ctx.ollama.setUrl(url);
            await ctx.updateOllamaModels();
            showInfo("Endpoint updated");
        } catch(e) {
            handleError(e, {userMsg: "Error updating Ollama endpoint"});
        }
    }

    async function updateOllamaModelsPath() {
        try {
            if(ollamaModelsPath) {
                await ctx.ollama.setModelsPath(ollamaModelsPath);
                await ctx.updateOllamaModels();
                showInfo("Path updated");
            } else {
                showWarning("Models path must not be empty");
            }
        } catch(e) {
            handleError(e, {userMsg: "Error updating Ollama models path"});
        }
    }

    async function updateOllamaConfig() {
        updatingConfig = true;
        if (ollamaUrl !== ctx.ollama.apiUrl.href) {
            await updateOllamaUrl();
        }
        if(ollamaModelsPath !== ctx.ollama.modelsPath) {
            await updateOllamaModelsPath();
        }
        updatingConfig = false;
    }

    async function selectModelsPath() {
        ollamaModelsPath = await open({
            multiple: false,
            directory: true
        }) ?? undefined;
    }
</script>

<div class="w-full">
    <Card class="p-4 w-full mb-4" size="lg">
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
                    {:else}
                        <TableBodyCell colspan={4}>Empty</TableBodyCell>
                    {/each}
                {:catch error}
                    <TableBodyCell colspan={4}>{error}</TableBodyCell>
                {/await}
            </TableBody>
        </Table>
    </Card>

    <Card class="p-4 w-full" size="lg">
        <Heading tag="h4" class="font-medium mb-4">Ollama</Heading>
        <div class="grid gap-3 gap-y-2 grid-cols-3">
            <Label for="ollamaEndpoint">Endpoint</Label>
            <Input bind:value={ollamaUrl} class="col-span-3" type="url" id="ollamaEndpoint" />

            <Label for="ollamaEndpoint" class="col-span-3">Models Path</Label>
            <Input bind:value={ollamaModelsPath} placeholder={ollamaModelsPathPlaceholder} class="col-span-2" type="text" id="ollamaModelsPath" />
            <Button outline onclick={() => selectModelsPath()}>Select</Button>

            <Button onclick={() => updateOllamaConfig()} disabled={updatingConfig} class="w-60">Update</Button>
        </div>
        <div class="grid gap-3 gap-y-2 grid-cols-3 mt-4">
            <Label for="ollamaAddModel">Add model</Label>
            <Input class="row-start-2 col-span-2" placeholder="gpt-oss:latest" type="text" id="ollamaAddModel" />
            <Button disabled class="row-start-2 col-start-3">Pull</Button>
        </div>
    </Card>
</div>
