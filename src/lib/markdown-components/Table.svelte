<script lang="ts">
    import { Table } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'

    interface Props {
        children?: Snippet
    }
    const { children }: Props = $props()
</script>

<Table striped hoverable border={false}>
    {@render children?.()}
</Table>
