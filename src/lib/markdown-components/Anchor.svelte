<script lang="ts">
    import { A } from 'flowbite-svelte';
    import type { Snippet } from 'svelte'
    import { openUrl } from "@tauri-apps/plugin-opener";
    import ModalDialog, { type ShowModalOptions } from '$lib/ModalDialog.svelte.ts';

    interface Props {
        href?: string
        title?: string
        children?: Snippet
    }
    const { href = '', title = undefined, children }: Props = $props()

    const modalOptions: ShowModalOptions = {
        confirmText: "Open",
        abortText: "Abort",
        title: "Open link?",
        content: popupContent
    }

    async function openLink(e: MouseEvent) {
        e.preventDefault();
        if(href.length > 0 && await ModalDialog.get().showModal(modalOptions))
            openUrl(href);
    }
</script>

{#snippet popupContent()}
    <p>Do you want to open <strong>{href}</strong> in your default browser?</p>
{/snippet}

<A onclick={openLink} href={href}>
    {@render children?.()}
</A>