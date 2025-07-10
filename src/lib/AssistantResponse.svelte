<script lang="ts">
    import { Hr, Li, Spinner, TableBody, TableBodyRow } from "flowbite-svelte";
    import SvelteMarkdown from "@humanspeak/svelte-markdown";
    import type { Renderers } from "@humanspeak/svelte-markdown";
    import Heading from "./markdown-components/Heading.svelte";
    import CodeBlock from "./markdown-components/CodeBlock.svelte";
    import Anchor from "./markdown-components/Anchor.svelte";
    import Blockquote from "./markdown-components/Blockquote.svelte";
    import List from "./markdown-components/List.svelte";
    import Code from "./markdown-components/Code.svelte";
    import TableCell from "./markdown-components/TableCell.svelte";
    import Table from "./markdown-components/Table.svelte";
    import Paragraph from "./markdown-components/Paragraph.svelte";
    import TableHead from "./markdown-components/TableHead.svelte";

    interface Props {
        content: string;
    }
    const props: Props = $props();

    const renderers: Partial<Renderers> = {
        link: Anchor,
        heading: Heading,
        paragraph: Paragraph,
        hr: Hr,
        code: CodeBlock,
        image: Anchor,
        blockquote: Blockquote,
        list: List,
        codespan: Code,
        listitem: Li,
        orderedlistitem: Li,
        unorderedlistitem: Li,
        table: Table,
        tablebody: TableBody,
        tablecell: TableCell,
        tablehead: TableHead,
        tablerow: TableBodyRow
    }
</script>

<md class="contents">
    {#if props.content.length > 0}
        <SvelteMarkdown
          source={props.content}
          renderers={renderers}>
        </SvelteMarkdown>
    {:else}
        <div class="flex justify-center content-center flex-wrap h-24">
            <Spinner></Spinner>
        </div>
    {/if}
</md>
