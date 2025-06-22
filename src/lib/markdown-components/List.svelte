<script lang="ts">
    import { List } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'

    interface Props {
        ordered?: boolean
        start?: number
        children?: Snippet
    }

    const { ordered = false, start = 1, children }: Props = $props()
</script>

<List tag={ordered ? "ol" : "ul"}>
    {@render children?.()}
</List>