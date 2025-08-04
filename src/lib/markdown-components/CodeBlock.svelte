<script lang="ts" module>
    import hljs from 'highlight.js/lib/core';
    import { ClipboardOutline } from 'flowbite-svelte-icons';
    import { Tooltip } from 'flowbite-svelte';
    import type { LanguageFn } from 'highlight.js';
    import { handleError } from '$lib/Util';

    const getLanguagePath = (lang: string) => `../../../node_modules/highlight.js/es/languages/${lang}.js`;
    // Note that the following lines will be replaced at build time by rollup.
    // Vite requires a string literal here.
    const languages = import.meta.glob("../../../node_modules/highlight.js/es/languages/*.js", {
        import: "default"
    }) as Record<string, () => Promise<LanguageFn>>;
</script>

<script lang="ts">
    interface Props {
        lang: string
        text: string
    }
    const { lang, text: content }: Props = $props();

    let formatedContent: string = $state("");
    let codeElement: HTMLElement | undefined;
    let codeLanguage: string | undefined = $state("");

    $effect(() => {
        if(!hljs.getLanguage(lang)) {
            // Try to load the language dynamically
            const loadLanguage = languages[getLanguagePath(lang)];
            if(loadLanguage !== undefined) {
                loadLanguage().then(module => {
                    hljs.registerLanguage(lang, module);
                    codeLanguage = lang;
                }).catch(e => handleError(e, {userMsg: "Error loading language"}));
            }
        } else {
            codeLanguage = lang;
        }
    });

    $effect(() => {
        if(codeLanguage) {
            formatedContent = hljs.highlight(content, {language: lang, ignoreIllegals: true}).value;
        } else {
            formatedContent = content;
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
        <div class="codeblock-header hljs dark:!text-white">
            <span>{codeLanguage ? lang : "Code"}</span>
            <button onclick={copyToClipboard}>
                <ClipboardOutline size="sm"></ClipboardOutline>
                <span>Copy</span>
            </button>
            <Tooltip trigger="click">Copied</Tooltip>
        </div>
        <code class="hljs" bind:this={codeElement}>{#if codeLanguage}{@html formatedContent}{:else}{formatedContent}{/if}</code>
    </div>
</pre>
