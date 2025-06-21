<script lang="ts">
    import { ExclamationCircleOutline } from "flowbite-svelte-icons";
    import AppContext from "./core/AppContext.svelte";

    const ctx = AppContext.getInstance();
</script>

{#if ctx.status.status !== "ok"}
<div class="flex w-80 items-center">
    <ExclamationCircleOutline
        size="xl"
        class="mr-2"
        color={ctx.status.status === "error" ? "red" : "#f0f704"}>
    </ExclamationCircleOutline>
    <p class="truncate">{ctx.status.msg}</p>
</div>
{/if}