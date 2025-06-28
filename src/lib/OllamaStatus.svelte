<script lang="ts">
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import AppContext from "./core/AppContext.svelte";
    import { onMount } from "svelte";
    import { Button } from "flowbite-svelte";

    let ollamaRunning = $state(true);
    let ollamaStarting = $state(false);
    const ctx = AppContext.getInstance();

    onMount(() => {
        const updateHndl = setInterval(async () => {
            ollamaRunning = await ctx.ollama.running();
            if(ollamaRunning && ollamaStarting) {
                ollamaStarting = false;
                await ctx.updateModels();
            }
        }, 5000);

        return () => {
            clearInterval(updateHndl);
        };
    });

    async function startOllama() {
        if(ollamaStarting) return;
        ollamaStarting = true;
        await ctx.ollama.boot();
    }
</script>

{#if !ollamaRunning}
<div class="flex w-80 items-center gap-2">
    <ExclamationCircleOutline
        size="xl"
        color="#f0f704">
    </ExclamationCircleOutline>
    <p class="truncate">Ollama is not running</p>
    <Button
        class="p-1.5"
        color="red"
        outline
        disabled={ollamaStarting}
        onclick={() => startOllama()}>
        Start Ollama
    </Button>
</div>
{/if}
