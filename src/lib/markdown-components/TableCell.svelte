<script lang="ts">
    import { TableBodyCell, TableHeadCell } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'

    interface Props {
        header: boolean
        align: 'left' | 'center' | 'right' | 'justify' | 'char' | null | undefined
        children?: Snippet
    }

    const { header, align, children }: Props = $props()

    // Convert alignment to style object if alignment is specified
    //const style = $derived(align ? `text-align: ${align}` : undefined)
</script>

{#if header}
    <TableHeadCell>{@render children?.()}</TableHeadCell>
{:else}
    <TableBodyCell>{@render children?.()}</TableBodyCell>
{/if}
