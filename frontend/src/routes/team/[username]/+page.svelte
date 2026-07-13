<script lang="ts">
  import { onMount } from 'svelte';
  import type { PageData } from './$types';
  import GlassCard from '$lib/components/GlassCard.svelte';

  let { data }: { data: PageData } = $props();

  // The URL encoded into the QR code.
  // When identityJwt is present (server-side generated), it encodes the live
  // verify URL.  Otherwise a static profile URL is used as a safe fallback.
  const qrUrl = $derived(
    data.identityJwt
      ? `https://ezeroandone.io/verify?token=${data.identityJwt}`
      : `https://ezeroandone.io/team/${data.profile.username}`
  );

  // QR code SVG string, populated client-side
  let qrSvg = $state<string>('');
  let qrError = $state<string>('');

  onMount(async () => {
    try {
      // Dynamic import keeps qrcode out of the SSR bundle
      const QRCode = await import('qrcode');
      qrSvg = await QRCode.toString(qrUrl, {
        type: 'svg',
        margin: 2,
        color: {
          dark: '#00d4ff',   // accent-blue (dark theme)
          light: '#00000000' // transparent background
        }
      });
    } catch (err) {
      qrError = 'QR code generation failed';
      console.error('[QR widget]', err);
    }
  });
</script>

<svelte:head>
  <title>{data.profile.name} - eZeroAndOne Team</title>
  <meta name="description" content="{data.profile.name} — {data.profile.job_title} at eZeroAndOne" />
</svelte:head>

<main class="profile-page">
  <a href="/team" class="back-link">← Back to Team</a>

  <div class="profile-layout">
    <!-- Left: profile card -->
    <GlassCard accentColor="blue" class="profile-card">
      <div class="profile-header">
        {#if data.profile.avatar_url}
          <img
            src={data.profile.avatar_url}
            alt="{data.profile.name}'s photo"
            class="avatar"
          />
        {:else}
          <div class="avatar-placeholder" aria-hidden="true">
            {data.profile.name.charAt(0).toUpperCase()}
          </div>
        {/if}

        <div class="profile-meta">
          <h1 class="profile-name">{data.profile.name}</h1>
          <p class="profile-title">{data.profile.job_title}</p>
          <p class="profile-handle">@{data.profile.username}</p>
        </div>
      </div>

      {#if data.profile.bio}
        <div class="profile-bio">
          <h2>About</h2>
          <p>{data.profile.bio}</p>
        </div>
      {/if}
    </GlassCard>

    <!-- Right: QR identity widget -->
    <GlassCard accentColor="green" class="qr-card">
      <div class="qr-widget">
        <div class="qr-header">
          <h2>Identity Verification</h2>
          <p class="qr-subtitle">
            Scan to cryptographically verify this staff member's identity and security clearance level.
          </p>
          <p class="qr-vault-notice" aria-label="Access restricted">
            Internal Verification Node. Access Restricted to Authorized Administrators and Profile Owner.
          </p>
        </div>

        <div class="qr-code-container" aria-label="QR code for identity verification">
          {#if qrSvg}
            <!-- eslint-disable-next-line svelte/no-at-html-tags -->
            {@html qrSvg}
          {:else if qrError}
            <div class="qr-placeholder qr-error" role="alert">
              <span class="qr-error-icon" aria-hidden="true">⚠</span>
              <p>{qrError}</p>
            </div>
          {:else}
            <div class="qr-placeholder qr-loading" aria-busy="true">
              <div class="qr-spinner" aria-hidden="true"></div>
              <p>Generating QR code…</p>
            </div>
          {/if}
        </div>

        <div class="qr-footer">
          {#if data.identityJwt}
            <p class="qr-note qr-live">
              <span class="qr-badge" aria-label="Live">●</span>
              Identity Cryptographically Verified · Live token expires in 5 minutes
            </p>
          {:else}
            <p class="qr-note qr-fallback">
              Identity token unavailable — showing profile link
            </p>
          {/if}
          <p class="qr-clearance">
            Security Clearance: Level 1 Access Confirmed
          </p>
          <p class="qr-url">{qrUrl}</p>
        </div>
      </div>
    </GlassCard>
  </div>
</main>

<style>
  .profile-page {
    max-width: 960px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .back-link {
    display: inline-block;
    margin-bottom: 2rem;
    font-size: 0.875rem;
    color: var(--accent-blue);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .back-link:hover {
    color: var(--accent-green);
    text-decoration: underline;
  }

  .profile-layout {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    align-items: start;
  }

  @media (max-width: 640px) {
    .profile-layout {
      grid-template-columns: 1fr;
    }
  }

  /* Profile card */
  .profile-header {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 2rem 2rem 1.5rem;
    gap: 1rem;
  }

  .avatar {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    object-fit: cover;
    border: 3px solid var(--accent-blue);
    box-shadow: 0 0 20px rgba(0, 212, 255, 0.3);
  }

  .avatar-placeholder {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    background: var(--color-bg-elevated);
    border: 3px solid var(--accent-blue);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 3rem;
    font-weight: 700;
    color: var(--accent-blue);
    box-shadow: 0 0 20px rgba(0, 212, 255, 0.3);
  }

  .profile-meta {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .profile-name {
    font-size: 1.75rem;
    font-weight: 700;
    margin: 0;
    color: var(--color-text-primary);
  }

  .profile-title {
    font-size: 1rem;
    color: var(--accent-blue);
    font-weight: 500;
    margin: 0;
  }

  .profile-handle {
    font-size: 0.875rem;
    color: var(--color-text-muted);
    margin: 0;
    font-family: monospace;
  }

  .profile-bio {
    padding: 0 2rem 2rem;
    border-top: 1px solid var(--glass-border);
    margin-top: 0.5rem;
    padding-top: 1.5rem;
  }

  .profile-bio h2 {
    font-size: 1rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--color-text-muted);
    margin: 0 0 0.75rem 0;
  }

  .profile-bio p {
    font-size: 1rem;
    line-height: 1.7;
    color: var(--color-text-secondary);
    margin: 0;
  }

  /* QR widget */
  .qr-widget {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
    align-items: center;
  }

  .qr-header {
    text-align: center;
  }

  .qr-header h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0 0 0.5rem 0;
    color: var(--color-text-primary);
  }

  .qr-subtitle {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    margin: 0;
    line-height: 1.4;
  }

  .qr-code-container {
    width: 200px;
    height: 200px;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .qr-code-container :global(svg) {
    width: 200px;
    height: 200px;
    border-radius: 8px;
  }

  .qr-placeholder {
    width: 200px;
    height: 200px;
    border-radius: 8px;
    border: 1px dashed var(--glass-border);
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.75rem;
    text-align: center;
  }

  .qr-loading {
    color: var(--color-text-muted);
  }

  .qr-error {
    color: var(--accent-red);
    border-color: var(--accent-red);
  }

  .qr-error-icon {
    font-size: 2rem;
  }

  .qr-placeholder p {
    font-size: 0.8rem;
    margin: 0;
    padding: 0 0.5rem;
  }

  /* Simple CSS spinner */
  .qr-spinner {
    width: 32px;
    height: 32px;
    border: 3px solid var(--glass-border);
    border-top-color: var(--accent-green);
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .qr-footer {
    text-align: center;
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    width: 100%;
  }

  .qr-note {
    font-size: 0.8rem;
    margin: 0;
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.4rem;
  }

  .qr-live {
    color: var(--accent-green);
  }

  .qr-badge {
    font-size: 0.6rem;
    animation: pulse 2s ease-in-out infinite;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.3; }
  }

  .qr-fallback {
    color: var(--color-text-muted);
  }

  .qr-vault-notice {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--accent-red);
    background: color-mix(in srgb, var(--accent-red) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-red) 25%, transparent);
    border-radius: 6px;
    padding: 0.35rem 0.6rem;
    margin-top: 0.5rem;
    text-align: center;
    line-height: 1.4;
  }

  .qr-clearance {
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: var(--accent-green);
    margin: 0;
    text-align: center;
  }

  .qr-url {
    font-size: 0.7rem;
    font-family: monospace;
    color: var(--color-text-muted);
    word-break: break-all;
    margin: 0;
    padding: 0.5rem;
    background: var(--color-bg-elevated);
    border-radius: 6px;
  }
</style>
