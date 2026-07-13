<script lang="ts">
	import '../app.css';
	import { theme } from '$lib/stores/theme';

	let { children } = $props();
</script>

<svelte:head>
	<title>eZeroAndOne</title>
</svelte:head>

<!-- Root layout: applies theme, wraps all pages -->
<div class="app-shell" data-theme="dark">
	{@render children()}
</div>

<style>
	.app-shell {
		min-height: 100vh;
		display: flex;
		flex-direction: column;
	}
</style>
