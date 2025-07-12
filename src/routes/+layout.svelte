<script lang="ts">
    import AppContext from '$lib/core/AppContext.svelte';
    import ModalDialog from '$lib/ModalDialog.svelte';
    import { DarkMode } from 'flowbite-svelte';
	import '../app.css';
    import { onMount } from 'svelte';
	
	let { children } = $props();

	// Update dark mode for global css imports
	const isDark = () => document.documentElement.attributes.getNamedItem("class")?.value?.includes("dark") ?? false;
	let darkMode = $state(isDark()); // This is only an initial value, not the actual selected theme

	onMount(() => {
		darkMode = isDark(); // Only now the document root is loaded with the actual theme
		const darkModeObserver = new MutationObserver(mut => {
			const dark = mut.find(m => m.attributeName === "class");
			if(!dark) return;
			darkMode = isDark();
		});
		darkModeObserver.observe(document.documentElement, {attributes: true, attributeOldValue: true});

		return () => {
			darkModeObserver.disconnect();
		};
	});

	const ctx = AppContext.getInstance();
</script>

<svelte:head>
    {#if darkMode}
        <link rel="stylesheet" href="/atom-one-dark.min.css">
    {:else}
        <link rel="stylesheet" href="/atom-one-light.min.css">
    {/if}
</svelte:head>

<main class="contents">
	{#await ctx.init()}
		<div class="w-full h-screen flex">
			<p class="m-auto font-medium text-3xl dark:text-gray-100">
				Loading...
			</p>
		</div>
	{:then}
		{@render children()}
	{:catch e}
		<p>Error initializing: {e}</p>
	{/await}
	<ModalDialog></ModalDialog>
	<div class="hidden">
		<!-- Included only for its update logic -->
		<DarkMode></DarkMode>
	</div>
</main>
