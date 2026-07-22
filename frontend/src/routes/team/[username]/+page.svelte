<script lang="ts">
  import { onMount } from 'svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // Always encode the plain profile URL — no JWT required for public page
  const qrUrl = `https://ezeroandone.io/team/${data.profile.username}`;

  // Rewrite media URLs to use the worker proxy
  const avatarSrc = $derived(
    data.profile.avatar_url
      ? data.profile.avatar_url.replace('https://media.ezeroandone.com/', '/media/')
      : ''
  );

  // QR code SVG string, populated client-side
  let qrSvg = $state<string>('');
  let qrError = $state<string>('');

  onMount(async () => {
    try {
      const QRCode = await import('qrcode');
      qrSvg = await QRCode.toString(qrUrl, {
        type: 'svg',
        width: 240,
        margin: 2,
        color: {
          dark: '#00C2FF',
          light: '#00000000',
        },
      });
    } catch (err) {
      qrError = 'QR generation failed';
      console.error('[QR widget]', err);
    }
  });
</script>

<svelte:head>
  <title>{data.profile.name} — eZeroAndOne.io Team</title>
  <meta name="description" content="{data.profile.name} · {data.profile.job_title} at eZeroAndOne.io" />
</svelte:head>

<main class="profile-page">
  <a href="/team" class="back-link">← Team</a>

  <div class="profile-grid">
    <!-- LEFT: identity card -->
    <div class="id-card">
      <div class="id-card-header">
        <div class="org-badge">eZeroAndOne.io</div>
        <div class="staff-badge">Staff ID</div>
      </div>

      <div class="avatar-wrap">
        {#if avatarSrc}
          <img src={avatarSrc} alt={data.profile.name} class="avatar" />
        {:else}
          <div class="avatar-initial" aria-hidden="true">
            {data.profile.name.charAt(0).toUpperCase()}
          </div>
        {/if}
        <div class="avatar-glow" aria-hidden="true"></div>
      </div>

      <h1 class="profile-name">{data.profile.name}</h1>
      <p class="profile-role">{data.profile.job_title}</p>
      <p class="profile-handle">@{data.profile.username}</p>

      {#if data.profile.bio}
        <p class="profile-bio">{data.profile.bio}</p>
      {/if}
    </div>

    <!-- RIGHT: QR verification -->
    <div class="qr-panel">
      <div class="qr-panel-header">
        <span class="qr-icon material-symbols-outlined" aria-hidden="true">verified_user</span>
        <div>
          <h2 class="qr-title">Identity Verification</h2>
          <p class="qr-sub">Scan to confirm this person works at eZeroAndOne.io</p>
        </div>
      </div>

      <div class="qr-code-wrap" aria-label="QR code for identity verification">
        {#if qrSvg}
          <!-- eslint-disable-next-line svelte/no-at-html-tags -->
          {@html qrSvg}
        {:else if qrError}
          <p class="qr-err" role="alert">{qrError}</p>
        {:else}
          <div class="qr-spinner-wrap" aria-busy="true">
            <div class="qr-spinner" aria-hidden="true"></div>
          </div>
        {/if}
      </div>

      <p class="qr-url">{qrUrl}</p>

      <div class="qr-status">
        <span class="status-dot" aria-hidden="true"></span>
        Scan to verify identity
      </div>
    </div>
  </div>
</main>

<style>
  .profile-page {
    max-width: 960px;
    margin: 0 auto;
    padding: 2.5rem 1.5rem 4rem;
  }

  .back-link {
    display: inline-flex;
    align-items: center;
    gap: 0.35rem;
    margin-bottom: 2.5rem;
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 0.8rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: rgba(255,255,255,0.45);
    text-decoration: none;
    transition: color 0.2s;
  }

  .back-link:hover {
    color: #00C2FF;
    text-decoration: none;
  }

  /* ── Two-column grid ─────────────────────────── */
  .profile-grid {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 2rem;
    align-items: start;
  }

  @media (max-width: 680px) {
    .profile-grid { grid-template-columns: 1fr; }
  }

  /* ── ID Card (left panel) ────────────────────── */
  .id-card {
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    padding: 2rem 1.75rem 2.5rem;
    background: rgba(255,255,255,0.03);
    border-radius: 20px;
    border: 1px solid rgba(255,255,255,0.08);
    position: relative;
    overflow: hidden;
  }

  /* Subtle gradient border effect */
  .id-card::before {
    content: '';
    position: absolute;
    inset: 0;
    border-radius: 20px;
    padding: 1px;
    background: linear-gradient(135deg, rgba(0,194,255,0.25) 0%, transparent 50%, rgba(0,194,255,0.1) 100%);
    -webkit-mask: linear-gradient(#fff 0 0) content-box, linear-gradient(#fff 0 0);
    -webkit-mask-composite: xor;
    mask-composite: exclude;
    pointer-events: none;
  }

  .id-card-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    width: 100%;
    margin-bottom: 2rem;
  }

  .org-badge {
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 0.72rem;
    font-weight: 800;
    letter-spacing: -0.02em;
    color: #00C2FF;
  }

  .staff-badge {
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 0.6rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: rgba(255,255,255,0.25);
    background: rgba(255,255,255,0.06);
    padding: 2px 8px;
    border-radius: 100px;
    border: 1px solid rgba(255,255,255,0.08);
  }

  /* ── Avatar ────────────────────────────────────── */
  .avatar-wrap {
    position: relative;
    margin-bottom: 1.5rem;
  }

  .avatar {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    object-fit: cover;
    border: 3px solid rgba(0,194,255,0.5);
    position: relative;
    z-index: 1;
    display: block;
  }

  .avatar-initial {
    width: 120px;
    height: 120px;
    border-radius: 50%;
    background: rgba(0,194,255,0.1);
    border: 3px solid rgba(0,194,255,0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 3rem;
    font-weight: 800;
    color: #00C2FF;
    position: relative;
    z-index: 1;
  }

  .avatar-glow {
    position: absolute;
    inset: -8px;
    border-radius: 50%;
    background: radial-gradient(circle, rgba(0,194,255,0.25) 0%, transparent 70%);
    pointer-events: none;
    animation: glow-pulse 3s ease-in-out infinite;
  }

  @keyframes glow-pulse {
    0%, 100% { opacity: 0.6; transform: scale(1); }
    50% { opacity: 1; transform: scale(1.05); }
  }

  /* ── Profile text ──────────────────────────────── */
  .profile-name {
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 1.75rem;
    font-weight: 800;
    letter-spacing: -0.04em;
    color: #ffffff;
    margin: 0 0 0.35rem;
    line-height: 1.1;
  }

  .profile-role {
    font-family: var(--font-body, 'Outfit', sans-serif);
    font-size: 0.9rem;
    font-weight: 500;
    color: #00C2FF;
    margin: 0 0 0.35rem;
  }

  .profile-handle {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.75rem;
    color: rgba(255,255,255,0.35);
    margin: 0 0 1.25rem;
  }

  .profile-bio {
    font-family: var(--font-body, 'Outfit', sans-serif);
    font-size: 0.875rem;
    line-height: 1.7;
    color: rgba(255,255,255,0.5);
    margin: 0;
    padding-top: 1.25rem;
    border-top: 1px solid rgba(255,255,255,0.07);
    width: 100%;
    text-align: left;
  }

  /* ── QR Panel (right panel) ───────────────────── */
  .qr-panel {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1.5rem;
    padding: 2rem 1.75rem 2.5rem;
    background: rgba(0,194,255,0.03);
    border-radius: 20px;
    border: 1px solid rgba(0,194,255,0.12);
  }

  .qr-panel-header {
    display: flex;
    align-items: flex-start;
    gap: 1rem;
    width: 100%;
  }

  .qr-icon {
    font-size: 1.75rem;
    color: #00C2FF;
    flex-shrink: 0;
    margin-top: 2px;
  }

  .qr-title {
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 1.1rem;
    font-weight: 700;
    color: #ffffff;
    margin: 0 0 0.25rem;
    letter-spacing: -0.02em;
  }

  .qr-sub {
    font-size: 0.8rem;
    color: rgba(255,255,255,0.45);
    margin: 0;
    line-height: 1.5;
  }

  /* ── QR code display ───────────────────────────── */
  .qr-code-wrap {
    width: 240px;
    height: 240px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0,0,0,0.4);
    border-radius: 16px;
    border: 1px solid rgba(0,194,255,0.2);
    padding: 12px;
    box-shadow: 0 0 30px rgba(0,194,255,0.08);
  }

  .qr-code-wrap :global(svg) {
    width: 100%;
    height: 100%;
    border-radius: 8px;
  }

  .qr-spinner-wrap {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100%;
    height: 100%;
  }

  .qr-spinner {
    width: 36px;
    height: 36px;
    border: 3px solid rgba(0,194,255,0.15);
    border-top-color: #00C2FF;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  @keyframes spin { to { transform: rotate(360deg); } }

  .qr-err {
    font-size: 0.8rem;
    color: rgba(255,51,102,0.8);
    text-align: center;
    margin: 0;
  }

  /* ── QR URL + status ───────────────────────────── */
  .qr-url {
    font-family: 'JetBrains Mono', 'Fira Code', monospace;
    font-size: 0.65rem;
    color: rgba(0,194,255,0.5);
    word-break: break-all;
    text-align: center;
    margin: 0;
    padding: 0.5rem 0.75rem;
    background: rgba(0,0,0,0.3);
    border-radius: 8px;
    width: 100%;
    box-sizing: border-box;
  }

  .qr-status {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 0.72rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    color: rgba(255,255,255,0.4);
    text-transform: uppercase;
  }

  .status-dot {
    display: inline-block;
    width: 6px;
    height: 6px;
    border-radius: 50%;
    background: #00C2FF;
    animation: pulse-dot 2.5s ease-in-out infinite;
    flex-shrink: 0;
  }

  @keyframes pulse-dot {
    0%, 100% { opacity: 1; transform: scale(1); }
    50% { opacity: 0.35; transform: scale(0.7); }
  }
</style>
