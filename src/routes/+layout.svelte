<script lang="ts">
    import AppContext from '$lib/core/AppContext.svelte';
    import ModalDialog from '$lib/ModalDialog.svelte';
	import '../app.css';
	
	let { children } = $props();

	const ctx = AppContext.getInstance();
</script>

<main class="contents">
	{#await ctx.init()}
		<div class="w-full h-screen flex">
			<p class="m-auto font-medium text-3xl">
				Loading...
			</p>
		</div>
	{:then}
		{@render children()}
	{:catch e}
		<p>Error initializing: {e}</p>
	{/await}
	<ModalDialog></ModalDialog>
</main>
