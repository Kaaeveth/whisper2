<script lang="ts">
    import { P } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'

    interface Props {
        children?: Snippet
    }
    const { children }: Props = $props()
</script>

<P class="my-2">
    {@render children?.()}
</P>