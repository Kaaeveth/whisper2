<script lang="ts">
    import AppContext from "$lib/core/AppContext.svelte";
    import { formatByteSize, handleError } from "$lib/Util";
    import { Card, Button, Heading, P, Table, TableBody, TableBodyCell, TableBodyRow, TableHead, TableHeadCell, Dropdown, DropdownItem } from "flowbite-svelte";
    import { DotsVerticalOutline, RefreshOutline, TrashBinOutline } from "flowbite-svelte-icons";
    import OllamaConfig from "$lib/OllamaConfig.svelte";
    import { type DeletableModel, type Model } from "$lib/core/LLMBackend";
    import { showInfo } from "$lib/Snackbar.svelte";
    import { showModal } from "$lib/ModalDialog.svelte";

    const ctx = AppContext.getInstance();
    let updateModels = $state(ctx.updateModels());

    // Mapping of model names onto whether they are loaded into memory
    type LoadState = "Loading"|"Loaded"|"Unloading"|"Unloaded";
    let modelLoadStates: Record<string, LoadState> = $state({});
    $effect(() => {
        updateModels.then(ml => {
            for(const model of ml) {
                model.loaded().then(loaded =>
                    modelLoadStates[model.name] = loaded ? "Loaded" : "Unloaded");
            }
        });
    });

    let modelForDeletion: Model|undefined = $state();
    async function deleteModel(model: DeletableModel) {
        try {
            modelForDeletion = model;
            if (await showModal({
                title: "Delete model",
                confirmColor: "red",
                content: confirmDeleteModelContent,
                confirmText: "Delete",
                abortText: "Abort"
            })) {
                await model.delete();
                showInfo(`Deleted '${model.name}'`);
            }
        } catch(e) {
            handleError(e, {userMsg: "Could not delete model: "+model.name});
        }
    }

    async function changeModelLoad(m: Model, load: boolean) {
        try {
            showInfo(`${load ? "Loading" : "Unloading"} '${m.name}' ...`);
            modelLoadStates[m.name] = load ? "Loading" : "Unloading";
            if(load) {
                await m.load();
            } else {
                await m.unload();
            }
            modelLoadStates[m.name] = load ? "Loaded" : "Unloaded";
            showInfo(`${load ? "Loaded" : "Unloaded"} '${m.name}' ...`);
        } catch(e) {
            handleError(e, {userMsg: `Could not ${load ? "Load": "Unload"} model`});
        }
    }
</script>

{#snippet confirmDeleteModelContent()}
	<p>Do you want to delete <strong>{modelForDeletion?.name}</strong>?</p>
{/snippet}

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
                <TableHeadCell class="dark:text-white"></TableHeadCell>
            </TableHead>
            <TableBody>
                {#await updateModels}
                    <TableBodyCell colspan={5}>Loading...</TableBodyCell>
                {:then}
                    {#each ctx.models as model}
                    {@const loaded = modelLoadStates[model.name] ?? "Unloaded"}
                    <TableBodyRow>
                        <TableBodyCell class="dark:text-white">{model.name}</TableBodyCell>
                        <TableBodyCell class="dark:text-white">{model.backend.name}</TableBodyCell>
                        <TableBodyCell class="dark:text-white">{formatByteSize(model.size)}</TableBodyCell>
                        <TableBodyCell class="dark:text-white">{loaded}</TableBodyCell>
                        <TableBodyCell class="dark:text-white">
                            {@const id = model.id.replaceAll(/-|:/g,"")}
                            <DotsVerticalOutline id={id} class="dots-menu" ariaLabel="model actions"></DotsVerticalOutline>
                            <Dropdown simple triggeredBy={'#'+id} placement="right" offset={20}>
                                <DropdownItem
                                    class={["w-full", !(loaded === "Loaded" || loaded === "Unloaded") && 'hidden']}
                                    onclick={() => changeModelLoad(model, loaded === "Unloaded")}>
                                    {loaded === "Loaded" ? "Unload" : "Load"}
                                </DropdownItem>
                                {#if model.isDeletable()}
                                    <DropdownItem class="flex gap-1 w-full" onclick={() => deleteModel(model)}>
                                        <span class="my-auto text-red-500">Delete</span>
                                        <TrashBinOutline color="red"></TrashBinOutline>
                                    </DropdownItem>
                                {/if}
                            </Dropdown>
                        </TableBodyCell>
                    </TableBodyRow>
                    {:else}
                        <TableBodyCell colspan={5}>Empty</TableBodyCell>
                    {/each}
                {:catch error}
                    <TableBodyCell colspan={5}>{error}</TableBodyCell>
                {/await}
            </TableBody>
        </Table>
    </Card>
    <OllamaConfig></OllamaConfig>
</div>
