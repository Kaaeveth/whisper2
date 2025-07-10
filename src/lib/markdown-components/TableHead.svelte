<script lang="ts">
    import { TableHead } from "flowbite-svelte";
    import type { Snippet } from "svelte";

    interface Props {
        children?: Snippet
    }
    const { children }: Props = $props()
</script>

<TableHead defaultRow={false}>{@render children?.()}</TableHead>
