<script lang="ts" module>
    import MarkdownIt from "markdown-it";

    /** 
     * NOTE: We currently render markdown with each new token
     * returned by a model, which is quite inefficient.
     * We should replace this with an incremental md parser in the future.
     * For now, however, this solution is still relatively fast
     * and not too resource heavy, especially for small responses.
     * */
    const md = MarkdownIt({linkify: true});
</script>

<script lang="ts">
    import { Spinner } from "flowbite-svelte";
    import { openUrl } from "@tauri-apps/plugin-opener";

    interface Props {
        content: string;
    }
    const props: Props = $props();

    /**
     * Handler for opening links in an external browser
     * instead of the webview window.
     * One would usually set the target attribute of the anchor element,
     * however, we cannot configure markdown-it (without any additional plugins)
     * to add attributes.
     * @param node
     */
    function setupAnchorHandling(node: HTMLDivElement) {
        node.addEventListener('click', async (e: MouseEvent) => {
            if(!(e.target instanceof HTMLAnchorElement)) return;

            e.preventDefault();
            const elem = e.target as HTMLAnchorElement;
            await openUrl(elem.href);
        });
    }
</script>

<style lang="postcss">
@reference "tailwindcss";
@reference "../app.css";

md :global {
    @apply dark:text-white;
    @apply text-gray-900;

    h1, h2, h3, h4, h5, h6 {
        @apply font-bold;
    }
    h1 {
        @apply text-5xl;
    }
    h2 {
        @apply text-4xl;
    }
    h3 {
        @apply text-3xl;
    }
    h4 {
        @apply text-2xl;
    }
    h5 {
        @apply text-xl;
    }
    h6 {
        @apply text-lg;
    }
    p {
        @apply leading-normal;
        @apply text-left;
        @apply whitespace-normal;
        @apply text-base;
        @apply tracking-normal;
        @apply font-normal;
    }
    hr {
        @apply bg-gray-100;
        @apply mx-auto;
        @apply w-9/10;
        @apply my-4;
        //@apply dark:bg-gray-700;
    }
    a {
        @apply inline-flex;
        @apply items-center;
        @apply hover:underline;
        @apply text-primary-600;
        @apply dark:text-primary-500;
    }
}
</style>

<md class="contents" use:setupAnchorHandling>
    {#if props.content.length > 0}
        {@html md.render(props.content)}
    {:else}
        <div class="flex justify-center content-center flex-wrap h-24">
            <Spinner></Spinner>
        </div>
    {/if}
</md>