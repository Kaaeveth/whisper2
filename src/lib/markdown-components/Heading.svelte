<script lang="ts">
    import type { SvelteMarkdownOptions } from '@humanspeak/svelte-markdown';
    import { Heading } from 'flowbite-svelte';
    import type { Snippet } from 'svelte';

    interface Props {
        depth: number
        raw: string
        text: string
        options: SvelteMarkdownOptions
        slug: (val: string) => string // eslint-disable-line no-unused-vars
        children?: Snippet
    }

    const { depth, raw, text, options, slug, children }: Props = $props();

    const id = $derived(options.headerIds ? options.headerPrefix + slug(text) : undefined)
</script>

{#if depth < 7}
    <Heading 
        id={id}
        class="my-4 dark:text-gray-100"
        tag={("h"+Math.max(2, depth)) as "h1" | "h2" | "h3" | "h4" | "h5" | "h6" | undefined}
    >
        {@render children?.()}
    </Heading>
{:else}
    {raw}
{/if}
