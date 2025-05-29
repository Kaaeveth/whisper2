<script lang="ts">
    import { Button } from "flowbite-svelte";
    import { type Snippet } from "svelte";
    import { page } from "$app/state";
    import { goto } from "$app/navigation";

    interface Props {
        href: string;
        children: Snippet;
    }

    let props: Props = $props();
    const active = $derived(page.url.pathname == props.href);

    const activeClass = "bg-gray-300 dark:bg-gray-600 hover:bg-gray-400 dark:hover:bg-gray-700";
    const inActiveClass = "bg-transparent hover:bg-gray-300 dark:bg-transparent dark:hover:bg-gray-500";
</script>

<Button 
    class={`${active ? activeClass : inActiveClass} text-black dark:text-white text-base`}
    onclick={() => goto(props.href, {replaceState: true})}
>
    {@render props.children()}
</Button>