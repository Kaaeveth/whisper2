<script lang="ts">
	import '../app.css';
	
	let { children } = $props();
</script>

<main class="contents">
	{@render children()}
</main>
