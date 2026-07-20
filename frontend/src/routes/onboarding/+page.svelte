<script lang="ts">
	/**
	 * Three-step onboarding wizard.
	 *
	 * Step 1 — Profile details  (PATCH /api/onboarding/profile)
	 * Step 2 — Signing key      (POST  /api/onboarding/signing-key)
	 * Step 3 — Company briefing (POST  /api/onboarding/complete)
	 *
	 * Requirements: 4.3, 4.4, 4.5, 4.8, 4.9, 15.6
	 */

	import { goto } from '$app/navigation';

	// Data from the server load function.
	let { data } = $props<{ data: { step: 1 | 2 | 3; completed: boolean } }>();

	// Active step — initialise from server-side value so the wizard resumes
	// at the correct step on page refresh. The cast suppresses a spurious
	// Svelte reactive-capture warning; `currentStep` is intentionally local
	// mutable state seeded once from the server.
	// eslint-disable-next-line svelte/prefer-derived
	let currentStep = $state<1 | 2 | 3>(data.step as 1 | 2 | 3);

	// -------------------------------------------------------------------------
	// Step 1 — Profile
	// -------------------------------------------------------------------------
	let profileName = $state('');
	let profileJobTitle = $state('');
	let profileBio = $state('');
	let profileAvatarUrl = $state('');

	// -------------------------------------------------------------------------
	// Step 2 — Signing key
	// -------------------------------------------------------------------------
	let signingKeyPem = $state('');

	// -------------------------------------------------------------------------
	// Shared UI state
	// -------------------------------------------------------------------------
	type Status = 'idle' | 'loading' | 'error';
	let stepStatus = $state<Status>('idle');
	let errorMessage = $state('');

	function setError(msg: string) {
		stepStatus = 'error';
		errorMessage = msg;
	}

	function clearStatus() {
		stepStatus = 'idle';
		errorMessage = '';
	}

	// -------------------------------------------------------------------------
	// Step handlers
	// -------------------------------------------------------------------------

	async function submitProfile(e: Event) {
		e.preventDefault();
		clearStatus();
		stepStatus = 'loading';

		try {
			const res = await fetch('/api/onboarding/profile', {
				method: 'PATCH',
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({
					name: profileName.trim(),
					job_title: profileJobTitle.trim(),
					bio: profileBio.trim(),
					avatar_url: profileAvatarUrl.trim(),
				}),
			});

			if (!res.ok) {
				const body = await res.text();
				setError(body || 'Failed to save profile. Please try again.');
				return;
			}

			stepStatus = 'idle';
			currentStep = 2;
		} catch {
			setError('Unable to reach the server. Check your connection.');
		}
	}

	async function submitSigningKey(e: Event) {
		e.preventDefault();
		clearStatus();
		stepStatus = 'loading';

		try {
			const res = await fetch('/api/onboarding/signing-key', {
				method: 'POST',
				credentials: 'include',
				headers: { 'Content-Type': 'application/json' },
				body: JSON.stringify({ public_key_pem: signingKeyPem.trim() }),
			});

			if (res.status === 400) {
				const body = await res.text();
				let msg = 'Invalid public key format.';
				try {
					const json = JSON.parse(body);
					if (typeof json?.error === 'string') msg = json.error;
				} catch {
					// use default
				}
				setError(msg);
				return;
			}

			if (!res.ok) {
				setError('Failed to provision signing key. Please try again.');
				return;
			}

			stepStatus = 'idle';
			currentStep = 3;
		} catch {
			setError('Unable to reach the server. Check your connection.');
		}
	}

	async function completeOnboarding() {
		clearStatus();
		stepStatus = 'loading';

		try {
			const res = await fetch('/api/onboarding/complete', {
				method: 'POST',
				credentials: 'include',
			});

			if (!res.ok) {
				setError('Failed to complete onboarding. Please try again.');
				return;
			}

			// Onboarding done — navigate to dashboard.
			await goto('/admin/dashboard');
		} catch {
			setError('Unable to reach the server. Check your connection.');
		}
	}

	const isLoading = $derived(stepStatus === 'loading');
</script>

<svelte:head>
	<title>Onboarding — eZeroAndOne</title>
</svelte:head>

<main class="onboarding-page">
	<div class="wizard-card">
		<!-- Progress indicator -->
		<div class="progress-bar" aria-label="Onboarding progress">
			{#each [1, 2, 3] as step (step)}
				<div
					class="progress-step"
					class:progress-step--complete={currentStep > step}
					class:progress-step--active={currentStep === step}
					aria-current={currentStep === step ? 'step' : undefined}
				>
					<div class="step-dot">
						{#if currentStep > step}
							<!-- Checkmark SVG for completed steps -->
							<svg width="12" height="12" viewBox="0 0 12 12" fill="none" aria-hidden="true">
								<path d="M2.5 6.5l2.5 2.5 4.5-5" stroke="currentColor" stroke-width="1.5" stroke-linecap="round" stroke-linejoin="round"/>
							</svg>
						{:else}
							{step}
						{/if}
					</div>
					<span class="step-label">
						{step === 1 ? 'Profile' : step === 2 ? 'Signing Key' : 'Briefing'}
					</span>
				</div>
				{#if step < 3}
					<div class="progress-connector" class:progress-connector--filled={currentStep > step}></div>
				{/if}
			{/each}
		</div>

		<!-- ------------------------------------------------------------------ -->
		<!-- Step 1: Profile details                                             -->
		<!-- ------------------------------------------------------------------ -->
		{#if currentStep === 1}
			<section class="wizard-step" aria-labelledby="step1-heading">
				<h1 id="step1-heading" class="step-heading">Set up your profile</h1>
				<p class="step-description">
					Let the team know who you are. You can update these details later from your profile settings.
				</p>

				<form onsubmit={submitProfile} novalidate class="step-form">
					<div class="field">
						<label for="name" class="label">Full name <span class="required" aria-hidden="true">*</span></label>
						<input
							id="name"
							type="text"
							class="input"
							bind:value={profileName}
							placeholder="Ada Lovelace"
							required
							disabled={isLoading}
						/>
					</div>

					<div class="field">
						<label for="job_title" class="label">Job title <span class="required" aria-hidden="true">*</span></label>
						<input
							id="job_title"
							type="text"
							class="input"
							bind:value={profileJobTitle}
							placeholder="Software Engineer"
							required
							disabled={isLoading}
						/>
					</div>

					<div class="field">
						<label for="bio" class="label">Bio</label>
						<textarea
							id="bio"
							class="input textarea"
							bind:value={profileBio}
							placeholder="A short description about yourself…"
							rows="3"
							disabled={isLoading}
						></textarea>
					</div>

					<div class="field">
						<label for="avatar_url" class="label">Avatar URL</label>
						<input
							id="avatar_url"
							type="url"
							class="input"
							bind:value={profileAvatarUrl}
							placeholder="https://example.com/avatar.png"
							disabled={isLoading}
						/>
					</div>

					{#if stepStatus === 'error'}
						<p class="error-msg" role="alert">{errorMessage}</p>
					{/if}

					<button
						type="submit"
						class="btn-primary"
						disabled={isLoading || !profileName.trim() || !profileJobTitle.trim()}
					>
						{#if isLoading}
							<span class="spinner" aria-hidden="true"></span>
							<span>Saving…</span>
						{:else}
							Continue
						{/if}
					</button>
				</form>
			</section>

		<!-- ------------------------------------------------------------------ -->
		<!-- Step 2: Signing key provisioning                                   -->
		<!-- ------------------------------------------------------------------ -->
		{:else if currentStep === 2}
			<section class="wizard-step" aria-labelledby="step2-heading">
				<h1 id="step2-heading" class="step-heading">Provision your signing key</h1>
				<p class="step-description">
					Paste your Ed25519 or ECDSA P-256 public key in PEM format. This key is used to
					cryptographically identify you on the platform.
				</p>

				<form onsubmit={submitSigningKey} novalidate class="step-form">
					<div class="field">
						<label for="public_key_pem" class="label">
							Public key (PEM) <span class="required" aria-hidden="true">*</span>
						</label>
						<textarea
							id="public_key_pem"
							class="input textarea textarea--mono"
							bind:value={signingKeyPem}
							placeholder="-----BEGIN PUBLIC KEY-----&#10;…&#10;-----END PUBLIC KEY-----"
							rows="6"
							required
							disabled={isLoading}
							spellcheck="false"
							autocapitalize="off"
						></textarea>
						<p class="hint">
							Generate a key pair with:&nbsp;
							<code>openssl genpkey -algorithm ed25519</code>
						</p>
					</div>

					{#if stepStatus === 'error'}
						<p class="error-msg" role="alert">{errorMessage}</p>
					{/if}

					<button
						type="submit"
						class="btn-primary"
						disabled={isLoading || !signingKeyPem.trim()}
					>
						{#if isLoading}
							<span class="spinner" aria-hidden="true"></span>
							<span>Verifying…</span>
						{:else}
							Continue
						{/if}
					</button>
				</form>
			</section>

		<!-- ------------------------------------------------------------------ -->
		<!-- Step 3: Company briefing acknowledgement                           -->
		<!-- ------------------------------------------------------------------ -->
		{:else if currentStep === 3}
			<section class="wizard-step" aria-labelledby="step3-heading">
				<h1 id="step3-heading" class="step-heading">Company briefing</h1>
				<p class="step-description">
					Please read the following before accessing the platform.
				</p>

				<div class="briefing-card">
					<h2 class="briefing-heading">Welcome to eZeroAndOne.io</h2>
					<p>
						As a member of the eZeroAndOne team you are expected to uphold our standards of
						professional conduct, data security, and confidentiality. By proceeding you
						acknowledge that:
					</p>
					<ul>
						<li>All client and company data is confidential and must be handled with care.</li>
						<li>Your account credentials must not be shared with any third party.</li>
						<li>Any suspected security incident must be reported immediately to a SuperAdmin.</li>
						<li>Platform usage is subject to our internal acceptable use policy.</li>
					</ul>
				</div>

				{#if stepStatus === 'error'}
					<p class="error-msg" role="alert">{errorMessage}</p>
				{/if}

				<button
					class="btn-primary"
					onclick={completeOnboarding}
					disabled={isLoading}
				>
					{#if isLoading}
						<span class="spinner" aria-hidden="true"></span>
						<span>Completing…</span>
					{:else}
						I acknowledge — go to dashboard
					{/if}
				</button>
			</section>
		{/if}
	</div>
</main>

<style>
	/* -----------------------------------------------------------------------
	   Layout
	----------------------------------------------------------------------- */
	.onboarding-page {
		display: flex;
		align-items: flex-start;
		justify-content: center;
		min-height: 100vh;
		padding: 3rem 1.5rem;
		background-color: var(--color-bg-primary, #0a0a0f);
	}

	.wizard-card {
		width: 100%;
		max-width: 520px;
		padding: 2.5rem 2rem;
		background: var(--glass-bg, rgba(255, 255, 255, 0.04));
		backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
		-webkit-backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
		border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
		border-radius: 20px;
		box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.4));
	}

	/* -----------------------------------------------------------------------
	   Progress bar
	----------------------------------------------------------------------- */
	.progress-bar {
		display: flex;
		align-items: center;
		margin-bottom: 2.25rem;
	}

	.progress-step {
		display: flex;
		flex-direction: column;
		align-items: center;
		gap: 0.375rem;
		flex-shrink: 0;
	}

	.step-dot {
		width: 32px;
		height: 32px;
		border-radius: 50%;
		border: 2px solid var(--color-border, rgba(255, 255, 255, 0.12));
		background: var(--color-bg-elevated, rgba(255, 255, 255, 0.06));
		display: flex;
		align-items: center;
		justify-content: center;
		font-size: 0.75rem;
		font-weight: 600;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.5));
		transition: background 0.2s ease, border-color 0.2s ease, color 0.2s ease;
	}

	.progress-step--active .step-dot {
		border-color: var(--accent-blue, #00d4ff);
		background: color-mix(in srgb, var(--accent-blue, #00d4ff) 15%, transparent);
		color: var(--accent-blue, #00d4ff);
	}

	.progress-step--complete .step-dot {
		border-color: var(--accent-green, #00ff88);
		background: color-mix(in srgb, var(--accent-green, #00ff88) 15%, transparent);
		color: var(--accent-green, #00ff88);
	}

	.step-label {
		font-size: 0.6875rem;
		font-weight: 500;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.45));
		letter-spacing: 0.03em;
		text-transform: uppercase;
	}

	.progress-step--active .step-label {
		color: var(--accent-blue, #00d4ff);
	}

	.progress-step--complete .step-label {
		color: var(--accent-green, #00ff88);
	}

	.progress-connector {
		flex: 1;
		height: 2px;
		margin: 0 0.5rem;
		margin-bottom: 1.1rem; /* vertically align with dot centre */
		background: var(--color-border, rgba(255, 255, 255, 0.1));
		transition: background 0.3s ease;
	}

	.progress-connector--filled {
		background: var(--accent-green, #00ff88);
	}

	/* -----------------------------------------------------------------------
	   Step content
	----------------------------------------------------------------------- */
	.wizard-step {
		display: flex;
		flex-direction: column;
		gap: 1.25rem;
	}

	.step-heading {
		margin: 0;
		font-size: 1.375rem;
		font-weight: 700;
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
	}

	.step-description {
		margin: 0;
		font-size: 0.9rem;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.56));
		line-height: 1.6;
	}

	/* -----------------------------------------------------------------------
	   Form elements
	----------------------------------------------------------------------- */
	.step-form {
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

	.required {
		color: var(--accent-red, #ff3366);
	}

	.input {
		padding: 0.65rem 0.875rem;
		background: var(--color-bg-elevated, rgba(255, 255, 255, 0.08));
		border: 1px solid var(--color-border, rgba(255, 255, 255, 0.08));
		border-radius: 10px;
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
		font-size: 0.9375rem;
		font-family: inherit;
		outline: none;
		resize: vertical;
		transition: border-color 0.2s ease, box-shadow 0.2s ease;
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

	.textarea {
		resize: vertical;
		min-height: 72px;
	}

	.textarea--mono {
		font-family: 'Fira Code', 'Cascadia Code', 'Consolas', monospace;
		font-size: 0.8125rem;
		line-height: 1.6;
	}

	.hint {
		margin: 0.25rem 0 0;
		font-size: 0.8rem;
		color: var(--color-text-muted, rgba(255, 255, 255, 0.36));
	}

	.hint code {
		font-family: 'Fira Code', 'Cascadia Code', monospace;
		color: var(--accent-yellow, #ffd700);
		font-size: 0.75rem;
	}

	/* -----------------------------------------------------------------------
	   Briefing card (step 3)
	----------------------------------------------------------------------- */
	.briefing-card {
		padding: 1.25rem 1.375rem;
		background: var(--color-bg-elevated, rgba(255, 255, 255, 0.05));
		border: 1px solid var(--color-border, rgba(255, 255, 255, 0.08));
		border-radius: 12px;
		font-size: 0.875rem;
		color: var(--color-text-secondary, rgba(255, 255, 255, 0.7));
		line-height: 1.7;
	}

	.briefing-heading {
		margin: 0 0 0.75rem;
		font-size: 1rem;
		font-weight: 600;
		color: var(--color-text-primary, rgba(255, 255, 255, 0.92));
	}

	.briefing-card p {
		margin: 0 0 0.75rem;
	}

	.briefing-card ul {
		margin: 0;
		padding-left: 1.25rem;
	}

	.briefing-card li {
		margin-bottom: 0.4rem;
	}

	/* -----------------------------------------------------------------------
	   Primary button
	----------------------------------------------------------------------- */
	.btn-primary {
		display: flex;
		align-items: center;
		justify-content: center;
		gap: 0.5rem;
		padding: 0.75rem 1rem;
		margin-top: 0.5rem;
		background: var(--accent-blue, #00d4ff);
		color: #000;
		font-size: 0.9375rem;
		font-weight: 600;
		border: none;
		border-radius: 10px;
		cursor: pointer;
		transition: opacity 0.2s ease, transform 0.15s ease, box-shadow 0.2s ease;
	}

	.btn-primary:hover:not(:disabled) {
		opacity: 0.9;
		box-shadow: 0 0 18px color-mix(in srgb, var(--accent-blue, #00d4ff) 50%, transparent);
	}

	.btn-primary:active:not(:disabled) {
		transform: scale(0.98);
	}

	.btn-primary:disabled {
		opacity: 0.4;
		cursor: not-allowed;
	}

	/* -----------------------------------------------------------------------
	   Spinner
	----------------------------------------------------------------------- */
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
		to {
			transform: rotate(360deg);
		}
	}

	/* -----------------------------------------------------------------------
	   Error message
	----------------------------------------------------------------------- */
	.error-msg {
		margin: 0;
		padding: 0.75rem 1rem;
		background: color-mix(in srgb, var(--accent-red, #ff3366) 12%, transparent);
		border: 1px solid color-mix(in srgb, var(--accent-red, #ff3366) 30%, transparent);
		border-radius: 10px;
		font-size: 0.875rem;
		color: var(--accent-red, #ff3366);
		line-height: 1.5;
	}
</style>
