<script lang="ts">
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import AppContext from "./core/AppContext.svelte";
    import { onMount } from "svelte";
    import { Button } from "flowbite-svelte";
    import { handleError } from "./Util";

    let ollamaRunning = $state(true);
    let ollamaStarting = $state(false);
    const ctx = AppContext.getInstance();

    async function checkOllamaHealth() {
        if(!ollamaStarting)
            ollamaRunning = await ctx.ollama.running();
        else if(ollamaRunning) {
            ollamaStarting = false;
            await ctx.updateOllamaModels();
        }
    }

    onMount(() => {
        checkOllamaHealth();
        const updateHndl = setInterval(async () => {
            await checkOllamaHealth();
        }, 5000);

        return () => {
            clearInterval(updateHndl);
        };
    });

    async function startOllama() {
        if(ollamaStarting) return;
        ollamaStarting = true;
        try {
            await ctx.ollama.boot();
            await ctx.updateOllamaModels();
            ollamaStarting = false;
            ollamaRunning = true;
        } catch(e) {
            handleError(e, {userMsg: "Could not boot Ollama"});
        }
    }
</script>

{#if !ollamaRunning}
<div class="flex w-80 items-center gap-2">
    <ExclamationCircleOutline
        size="xl"
        color="#f0f704">
    </ExclamationCircleOutline>
    <p class="truncate dark:text-gray-400">Ollama is not running</p>
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
