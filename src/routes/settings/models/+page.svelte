<script lang="ts" module>
    import AppContext from "$lib/core/AppContext.svelte";
    import { formatByteSize } from "$lib/Util";
    import { Card, Button, Heading, P, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell } from "flowbite-svelte";
    import { RefreshOutline } from "flowbite-svelte-icons";
    import OllamaConfig from "$lib/OllamaConfig.svelte";

    const ctx = AppContext.getInstance();
    let updateModels = $state(ctx.updateModels());
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
    <OllamaConfig></OllamaConfig>
</div>
