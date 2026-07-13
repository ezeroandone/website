<script lang="ts">
	/**
	 * Auth callback page.
	 *
	 * This page is only rendered if the server load throws an error.
	 * Under normal circumstances, the server load redirects and the user
	 * never sees this component.
	 */
</script>

<svelte:head>
	<title>Signing in… — eZeroAndOne</title>
</svelte:head>

<main class="callback-page">
	<div class="spinner-container">
		<div class="spinner" aria-label="Signing you in…"></div>
		<p class="status-text">Verifying your magic link…</p>
	</div>
</main>

<style>
	.callback-page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		background-color: var(--color-bg-primary, #0a0a0f);
	}

	.spinner-container {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 1.5rem;
	}

	.spinner {
		width: 48px;
		height: 48px;
		border: 3px solid rgba(255, 255, 255, 0.1);
		border-top-color: var(--accent-blue, #00d4ff);
		border-radius: 50%;
		animation: spin 0.8s linear infinite;
	}

	@keyframes spin {
		to {
			transform: rotate(360deg);
		}
	}

	.status-text {
		margin: 0;
		font-size: 0.9375rem;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.56));
		letter-spacing: 0.01em;
	}
</style>
