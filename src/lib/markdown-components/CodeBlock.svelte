<script lang="ts" module>
    import hljs from 'highlight.js';
    import 'highlight.js/styles/atom-one-light.css';
    import DOMPurify from 'dompurify';
    import { ClipboardOutline } from 'flowbite-svelte-icons';
    import { Tooltip } from 'flowbite-svelte';
</script>

<script lang="ts">
    interface Props {
        lang: string
        text: string
    }
    const { lang, text: content }: Props = $props();

    let formatedContent: string = $state("");
    let codeElement: HTMLElement | undefined;
    let hasLang: string | undefined = $state("");

    $effect(() => {
        hasLang = lang && hljs.getLanguage(lang) as string | undefined;
    });

    $effect(() => {
        if(hasLang) {
            formatedContent = hljs.highlight(content, {language: lang, ignoreIllegals: true}).value;
        } else {
            formatedContent = DOMPurify.sanitize(content);
        }
    });

    function copyToClipboard() {
        const snippet = codeElement?.innerText;
        if(snippet)
            navigator.clipboard.writeText(snippet);
    }
</script>

<style>
    .codeblock {
        display: flex;
        flex-direction: column;
    }
    .codeblock-header {
        display: flex;
        flex-direction: row;
        justify-content: space-between;
        padding-left: 1em;
        padding-right: 1em;
        padding-top: .5em;
        padding-bottom: .2em;
        font-size: smaller;
        font-family: var(--default-font-family);
    }
    .codeblock-header button {
        display: flex;
        gap: 4px;
        align-items: center;
    }
    .codeblock-header button:hover {
        cursor: pointer;
    }
</style>

<pre>
    <div class="codeblock">
        <div class="codeblock-header hljs">
            <span>{hasLang ? lang : "Code"}</span>
            <button onclick={copyToClipboard}>
                <ClipboardOutline size="sm"></ClipboardOutline>
                <span>Copy</span>
            </button>
            <Tooltip trigger="click">Copied</Tooltip>
        </div>
        <code class="hljs" bind:this={codeElement}>{@html formatedContent}</code>
    </div>
</pre>
