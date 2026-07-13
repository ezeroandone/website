<script lang="ts">
	import type { Snippet } from 'svelte';
	import { browser } from '$app/environment';

	interface Props {
		children?: Snippet;
		class?: string;
	}

	let { children, class: extraClass = '' }: Props = $props();

	let compact = $state(false);

	function floatingHeader(node: HTMLElement) {
		if (!browser) return {};

		function onScroll() {
			compact = window.scrollY > 50;
		}

		onScroll();
		window.addEventListener('scroll', onScroll, { passive: true });
		return { destroy() { window.removeEventListener('scroll', onScroll); } };
	}
</script>

<header
	class="floating-header {extraClass}"
	class:header--compact={compact}
	use:floatingHeader
	role="banner"
>
	{#if children}{@render children()}{/if}
</header>

<style>
	.floating-header {
		position:            fixed;
		top:                 0;
		left:                0;
		right:               0;
		z-index:             100;
		height:              var(--header-height, 64px);
		display:             flex;
		align-items:         center;
		padding:             0 2rem;

		/* Liquid glass */
		background:          rgba(0, 0, 0, 0.75);
		backdrop-filter:     blur(12px);
		-webkit-backdrop-filter: blur(12px);
		border-bottom:       1px solid rgba(255, 255, 255, 0.10);

		transition:          background 0.3s cubic-bezier(0.16, 1, 0.3, 1),
		                     border-color 0.3s ease,
		                     box-shadow 0.3s ease;
	}

	.header--compact {
		background:   rgba(0, 0, 0, 0.92);
		border-color: rgba(255, 255, 255, 0.06);
		box-shadow:   0 1px 0 rgba(255, 255, 255, 0.04);
	}
</style>
