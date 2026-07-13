<script lang="ts">
	/**
	 * Magic-link login page.
	 *
	 * Submits the user's email to POST /api/auth/request and
	 * renders appropriate feedback per Requirements 1.6 and 15.1.
	 */

	type FeedbackState =
		| { kind: 'idle' }
		| { kind: 'loading' }
		| { kind: 'success' }
		| { kind: 'error'; message: string };

	let email = $state('');
	let feedback = $state<FeedbackState>({ kind: 'idle' });

	async function handleSubmit(e: Event) {
		e.preventDefault();
		feedback = { kind: 'loading' };

		try {
			const res = await fetch('/api/auth/request', {
				method: 'POST',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ email })
			});

			if (res.ok) {
				feedback = { kind: 'success' };
				return;
			}

			// Map status codes to user-friendly messages.
			// 403: domain not allowed — do NOT reveal which domains are approved.
			if (res.status === 403) {
				feedback = { kind: 'error', message: 'This email address is not authorized.' };
			} else if (res.status === 429) {
				feedback = { kind: 'error', message: 'Too many requests, please wait before trying again.' };
			} else {
				feedback = { kind: 'error', message: 'Something went wrong. Please try again later.' };
			}
		} catch {
			feedback = { kind: 'error', message: 'Unable to reach the server. Check your connection.' };
		}
	}

	const isLoading = $derived(feedback.kind === 'loading');
</script>

<svelte:head>
	<title>Sign in — eZeroAndOne</title>
</svelte:head>

<main class="login-page">
	<div class="login-card">
		<div class="brand">
			<span class="brand-zero">e</span><span class="brand-accent">Zero</span><span class="brand-zero">And</span><span class="brand-accent">One</span><span class="brand-tld">.io</span>
		</div>

		<h1 class="heading">Staff Portal</h1>
		<p class="subheading">Enter your corporate email to receive a magic link.</p>

		{#if feedback.kind === 'success'}
			<div class="feedback feedback--success" role="status" aria-live="polite">
				<svg aria-hidden="true" width="20" height="20" viewBox="0 0 20 20" fill="none">
					<circle cx="10" cy="10" r="9" stroke="currentColor" stroke-width="1.5"/>
					<path d="M6.5 10.5l2.5 2.5 4.5-5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
				</svg>
				<span>Check your email for a magic link.</span>
			</div>
		{:else}
			<form onsubmit={handleSubmit} novalidate>
				<div class="field">
					<label for="email" class="label">Email address</label>
					<input
						id="email"
						type="email"
						class="input"
						bind:value={email}
						placeholder="you@ezeroandone.com"
						autocomplete="email"
						required
						disabled={isLoading}
						aria-describedby={feedback.kind === 'error' ? 'login-error' : undefined}
					/>
				</div>

				{#if feedback.kind === 'error'}
					<p id="login-error" class="feedback feedback--error" role="alert" aria-live="assertive">
						{feedback.message}
					</p>
				{/if}

				<button type="submit" class="btn-submit" disabled={isLoading || !email.trim()}>
					{#if isLoading}
						<span class="spinner" aria-hidden="true"></span>
						<span>Sending…</span>
					{:else}
						<span>Send magic link</span>
					{/if}
				</button>
			</form>
		{/if}
	</div>
</main>

<style>
	.login-page {
		display: flex;
		align-items: center;
		justify-content: center;
		min-height: 100vh;
		padding: 1.5rem;
		background-color: var(--color-bg-primary, #0a0a0f);
	}

	.login-card {
		width: 100%;
		max-width: 420px;
		padding: 2.5rem 2rem;
		background: var(--glass-bg, rgba(255, 255, 255, 0.04));
		backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
		-webkit-backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
		border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
		border-radius: 20px;
		box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.4));
	}

	/* Brand wordmark */
	.brand {
		font-size: 1.1rem;
		font-weight: 700;
		letter-spacing: -0.02em;
		margin-bottom: 1.75rem;
		text-align: center;
	}

	.brand-zero {
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
	}

	.brand-accent {
		color: var(--accent-blue, #00d4ff);
	}

	.brand-tld {
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.56));
	}

	.heading {
		margin: 0 0 0.5rem;
		font-size: 1.5rem;
		font-weight: 700;
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
		text-align: center;
	}

	.subheading {
		margin: 0 0 1.75rem;
		font-size: 0.9rem;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.56));
		text-align: center;
		line-height: 1.5;
	}

	/* Form */
	form {
		display: flex;
		flex-direction: column;
		gap: 1rem;
	}

	.field {
		display: flex;
		flex-direction: column;
		gap: 0.375rem;
	}

	.label {
		font-size: 0.8125rem;
		font-weight: 500;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.56));
	}

	.input {
		padding: 0.65rem 0.875rem;
		background: var(--color-bg-elevated, rgba(255, 255, 255, 0.08));
		border: 1px solid var(--color-border, rgba(255, 255, 255, 0.08));
		border-radius: 10px;
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
		font-size: 0.9375rem;
		outline: none;
		transition:
			border-color 0.2s ease,
			box-shadow 0.2s ease;
	}

	.input::placeholder {
		color: var(--color-text-muted, rgba(255, 255, 255, 0.32));
	}

	.input:focus {
		border-color: var(--accent-blue, #00d4ff);
		box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent-blue, #00d4ff) 20%, transparent);
	}

	.input:disabled {
		opacity: 0.5;
		cursor: not-allowed;
	}

	/* Submit button */
	.btn-submit {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		background: var(--accent-blue, #00d4ff);
		color: #000;
		font-size: 0.9375rem;
		font-weight: 600;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition:
			opacity 0.2s ease,
			transform 0.15s ease,
			box-shadow 0.2s ease;
	}

	.btn-submit:hover:not(:disabled) {
		opacity: 0.9;
		box-shadow: 0 0 18px color-mix(in srgb, var(--accent-blue, #00d4ff) 50%, transparent);
	}

	.btn-submit:active:not(:disabled) {
		transform: scale(0.98);
	}

	.btn-submit:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* Spinner */
	.spinner {
		display: inline-block;
		width: 16px;
		height: 16px;
		border: 2px solid rgba(0, 0, 0, 0.3);
		border-top-color: #000;
		border-radius: 50%;
		animation: spin 0.7s linear infinite;
		flex-shrink: 0;
	}

	@keyframes spin {
		to { transform: rotate(360deg); }
	}

	/* Feedback messages */
	.feedback {
		display: flex;
		align-items: flex-start;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		border-radius: 10px;
		font-size: 0.875rem;
		line-height: 1.5;
	}

	.feedback--success {
		background: color-mix(in srgb, var(--accent-green, #00ff88) 12%, transparent);
		border: 1px solid color-mix(in srgb, var(--accent-green, #00ff88) 30%, transparent);
		color: var(--accent-green, #00ff88);
	}

	.feedback--error {
		background: color-mix(in srgb, var(--accent-red, #ff3366) 12%, transparent);
		border: 1px solid color-mix(in srgb, var(--accent-red, #ff3366) 30%, transparent);
		color: var(--accent-red, #ff3366);
	}
</style>
