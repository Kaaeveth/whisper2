<script lang="ts">
    import { Blockquote, P } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'

    interface Props {
        children?: Snippet
    }
    const { children }: Props = $props()
</script>

<Blockquote border bg class="my-4 p-4 dark:bg-gray-700">
  <P size="lg" height="relaxed">{@render children?.()}</P>
</Blockquote>
