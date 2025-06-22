<script lang="ts">
    interface Props {
        raw: string
    }

    const { raw }: Props = $props()
</script>

<code class="hljs">{raw.replace(/`/g, '')}</code>