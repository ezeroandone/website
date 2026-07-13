<script lang="ts">
	import type { Snippet } from 'svelte';

	interface Props {
		accentColor?: 'blue' | 'green' | 'red' | 'yellow';
		blur?: number;
		children?: Snippet;
		class?: string;
	}

	let {
		accentColor = 'blue',
		blur = 12,
		children,
		class: extraClass = ''
	}: Props = $props();

	const accentMap: Record<NonNullable<Props['accentColor']>, string> = {
		blue:   'var(--accent-blue)',
		green:  'var(--accent-green)',
		red:    'var(--accent-red)',
		yellow: 'var(--accent-yellow)',
	};

	const accent = $derived(accentMap[accentColor ?? 'blue']);
</script>

<div
	class="glass-card {extraClass}"
	style="--_accent:{accent}; --_blur:{blur}px;"
>
	{#if children}{@render children()}{/if}
</div>

<style>
	.glass-card {
		background:              var(--bg-card, #0d0d0d);
		border:                  1px solid var(--glass-border);
		border-radius:           16px;
		transition:              all 0.3s cubic-bezier(0.16, 1, 0.3, 1);
		will-change:             transform, box-shadow, border-color;
	}

	.glass-card:hover {
		border-left-color: var(--_accent);
		border-left-width: 3px;
		box-shadow:        0 16px 48px rgba(0, 0, 0, 0.5);
	}
</style>
