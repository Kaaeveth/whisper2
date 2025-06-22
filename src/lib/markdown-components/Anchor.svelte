<script lang="ts">
    import { A } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'
    import { openUrl } from "@tauri-apps/plugin-opener";

    interface Props {
        href?: string
        title?: string
        children?: Snippet
    }
    const { href = '', title = undefined, children }: Props = $props()

    function openLink(e: MouseEvent) {
        e.preventDefault();
        if(href.length > 0) openUrl(href);
    }
</script>

<A onclick={openLink} href={href}>
    {@render children?.()}
</A>