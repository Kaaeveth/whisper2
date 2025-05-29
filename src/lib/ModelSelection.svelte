<script lang="ts">
    import type { Model } from "./core/LLMBackend";
    import { Dropdown, Button, DropdownGroup, DropdownItem } from "flowbite-svelte";
    import { ChevronDownOutline } from "flowbite-svelte-icons";
    import { scale } from "svelte/transition";

    interface Props {
        models: Model[];
        selectedModel?: Model;
    }

    let {
        selectedModel = $bindable(),
        ...props
    }: Props = $props();

    let selectedModelName = $derived(selectedModel?.name ?? "Select Model");
</script>

<Button outline color="alternative" class="p-2">
    {selectedModelName}<ChevronDownOutline class="ms-2 h-6 w-6 dark:text-white" />
</Button>
<Dropdown class="overflow-y-auto py-1" transition={scale} transitionParams={{ duration: 100 }}>
  <DropdownGroup>
    {#each props.models as model}
        <DropdownItem onclick={() => selectedModel = model} class="w-full text-base font-semibold">
            {model.name}
        </DropdownItem>
    {/each}
  </DropdownGroup>
  <a href="/settings/models" class="text-primary-600 dark:text-primary-500 -mb-1 flex items-center bg-gray-50 px-3 py-2 text-sm font-medium hover:bg-gray-100 hover:underline dark:bg-gray-700 dark:hover:bg-gray-600">
    Add new models
  </a>
</Dropdown>
