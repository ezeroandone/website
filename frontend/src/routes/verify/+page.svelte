<script lang="ts">
  import type { PageData } from './$types';
  import GlassCard from '$lib/components/GlassCard.svelte';

  let { data }: { data: PageData } = $props();

  // ── Brand copy — QR verification output messages ─────────────────────────
  const VERIFY_COPY = {
    verified:    'Identity Cryptographically Verified.',
    clearance:   'Security Clearance: Level {level} Access Confirmed.',
    restricted:  'Internal Verification Node. Access Restricted to Authorized Administrators and Profile Owner.',
    invalid:     'Verification failed. The presented credential is invalid, expired, or has already been consumed.',
    missing:     'No verification token detected. Present a valid QR code to initialize identity verification.',
  } as const;

  // Clearance level label map
  const CLEARANCE_LABELS: Record<number, string> = {
    1: 'Level 1 — Standard Access',
    2: 'Level 2 — Elevated Access',
    3: 'Level 3 — Administrative Access',
    4: 'Level 4 — Super-Administrative Access',
  };

  // Status label map — mirrors Rust LifecycleStatus enum
  const STATUS_LABELS: Record<string, string> = {
    Probation:  'Probation',
    Confirmed:  'Confirmed',
    Inactive:   'Inactive',
  };

  function clearanceLine(level: number): string {
    return VERIFY_COPY.clearance.replace('{level}', String(level));
  }

  function formatVerifiedAt(unixSeconds: number): string {
    return new Date(unixSeconds * 1000).toLocaleString('en-US', {
      year: 'numeric', month: 'long', day: 'numeric',
      hour: '2-digit', minute: '2-digit', timeZoneName: 'short',
    });
  }
</script>

<svelte:head>
  <title>Verification Portal — eZeroAndOne</title>
  <meta name="description" content="eZeroAndOne identity verification portal. Cryptographically verify staff credentials via QR scan." />
  <!-- Prevent indexing — verification URLs contain single-use tokens -->
  <meta name="robots" content="noindex, nofollow" />
</svelte:head>

<main class="verify-page">
  <div class="verify-shell">

    <!-- Brand header -->
    <div class="verify-brand">
      <span class="brand-e">e</span><span class="brand-accent">Zero</span><span class="brand-e">And</span><span class="brand-accent">One</span>
      <span class="brand-divider" aria-hidden="true">·</span>
      <span class="brand-portal">Verification Portal</span>
    </div>

    {#if data.state === 'success' && data.identity}
      <!-- ── Identity verified ───────────────────────────── -->
      <GlassCard accentColor="green">
        <div class="identity-card">

          <div class="verify-status verified" aria-live="polite">
            <span class="status-dot" aria-hidden="true"></span>
            <span class="status-text">{VERIFY_COPY.verified}</span>
          </div>

          <div class="identity-profile">
            {#if data.identity.photo_url}
              <img
                src={data.identity.photo_url}
                alt="{data.identity.name} — verified staff"
                class="profile-photo"
              />
            {:else}
              <div class="profile-photo-placeholder" aria-hidden="true">
                {data.identity.name.charAt(0).toUpperCase()}
              </div>
            {/if}

            <div class="profile-details">
              <h1 class="profile-name">{data.identity.name}</h1>

              <div class="status-row">
                <span class="status-label">Status</span>
                <span class="status-value status-value--{data.identity.identity_status.toLowerCase()}">
                  {STATUS_LABELS[data.identity.identity_status] ?? data.identity.identity_status}
                </span>
              </div>

              <div class="clearance-row">
                <span class="clearance-label">Clearance</span>
                <span class="clearance-value">
                  {CLEARANCE_LABELS[data.identity.clearance_level] ?? `Level ${data.identity.clearance_level}`}
                </span>
              </div>
            </div>
          </div>

          <p class="clearance-statement">
            {clearanceLine(data.identity.clearance_level)}
          </p>

          <p class="verified-at">
            Verified at {formatVerifiedAt(data.identity.verified_at)}
          </p>

          <p class="restricted-notice">
            {VERIFY_COPY.restricted}
          </p>

        </div>
      </GlassCard>

    {:else if data.state === 'missing'}
      <!-- ── No token ───────────────────────────────────── -->
      <GlassCard accentColor="yellow">
        <div class="status-card">
          <span class="status-icon" aria-hidden="true">⬜</span>
          <h1>No Credential Detected</h1>
          <p>{VERIFY_COPY.missing}</p>
        </div>
      </GlassCard>

    {:else}
      <!-- ── Invalid / expired ──────────────────────────── -->
      <GlassCard accentColor="red">
        <div class="status-card status-card--invalid">
          <span class="status-icon" aria-hidden="true" role="img" aria-label="Invalid">✕</span>
          <h1>Verification Failed</h1>
          <p>{VERIFY_COPY.invalid}</p>
        </div>
      </GlassCard>
    {/if}

    <p class="portal-footer">
      eZeroAndOne © 2026. Engineered from the baseline. Built for legacy.
    </p>
  </div>
</main>

<style>
  .verify-page {
    min-height: 100vh;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem 1rem;
    background: var(--color-bg-primary, #0a0a0f);
  }

  .verify-shell {
    width: 100%;
    max-width: 480px;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  /* ── Brand header ────────────────────────────── */
  .verify-brand {
    text-align: center;
    font-size: 1rem;
    font-weight: 700;
    letter-spacing: -0.01em;
    color: var(--text-primary, var(--color-text-primary));
    display: flex;
    align-items: center;
    justify-content: center;
    gap: 0.35rem;
    flex-wrap: wrap;
  }

  .brand-e     { color: var(--text-primary, var(--color-text-primary)); }
  .brand-accent { color: var(--accent-blue); }
  .brand-divider { color: var(--text-secondary, var(--color-text-secondary)); font-weight: 300; }
  .brand-portal {
    font-size: 0.75rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--text-secondary, var(--color-text-secondary));
  }

  /* ── Identity card ───────────────────────────── */
  .identity-card {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1.5rem;
  }

  .verify-status {
    display: flex;
    align-items: center;
    gap: 0.6rem;
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .verified { color: var(--accent-green); }

  .status-dot {
    width: 8px;
    height: 8px;
    border-radius: 50%;
    background: var(--accent-green);
    animation: pulse 2s ease-in-out infinite;
    flex-shrink: 0;
  }

  @keyframes pulse {
    0%, 100% { opacity: 1; }
    50% { opacity: 0.35; }
  }

  .identity-profile {
    display: flex;
    align-items: center;
    gap: 1.25rem;
  }

  .profile-photo {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    object-fit: cover;
    border: 3px solid var(--accent-green);
    box-shadow: 0 0 20px color-mix(in srgb, var(--accent-green) 35%, transparent);
    flex-shrink: 0;
  }

  .profile-photo-placeholder {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    border: 3px solid var(--accent-green);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: 700;
    color: var(--accent-green);
    flex-shrink: 0;
  }

  .profile-details { flex: 1; min-width: 0; }

  .profile-name {
    font-size: 1.5rem;
    font-weight: 800;
    margin: 0 0 0.6rem;
    color: var(--text-primary, var(--color-text-primary));
    letter-spacing: -0.02em;
  }

  .status-row,
  .clearance-row {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    font-size: 0.8rem;
    margin-bottom: 0.3rem;
  }

  .status-label,
  .clearance-label {
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    font-size: 0.65rem;
    color: var(--text-secondary, var(--color-text-secondary));
    width: 64px;
    flex-shrink: 0;
  }

  .status-value {
    font-weight: 600;
    padding: 2px 8px;
    border-radius: 20px;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .status-value--confirmed {
    background: color-mix(in srgb, var(--accent-green) 15%, transparent);
    color: var(--accent-green);
    border: 1px solid color-mix(in srgb, var(--accent-green) 30%, transparent);
  }

  .status-value--probation {
    background: color-mix(in srgb, var(--accent-yellow) 15%, transparent);
    color: var(--accent-yellow);
    border: 1px solid color-mix(in srgb, var(--accent-yellow) 30%, transparent);
  }

  .status-value--inactive {
    background: color-mix(in srgb, var(--text-secondary, #888) 10%, transparent);
    color: var(--text-secondary, var(--color-text-secondary));
    border: 1px solid color-mix(in srgb, var(--text-secondary, #888) 20%, transparent);
  }

  .clearance-value {
    font-size: 0.8rem;
    color: var(--text-primary, var(--color-text-primary));
    font-weight: 500;
  }

  .clearance-statement {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--accent-green);
    margin: 0;
    letter-spacing: 0.02em;
  }

  .verified-at {
    font-size: 0.72rem;
    color: var(--text-secondary, var(--color-text-secondary));
    margin: 0;
    opacity: 0.7;
  }

  .restricted-notice {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--accent-red);
    background: color-mix(in srgb, var(--accent-red) 8%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-red) 20%, transparent);
    border-radius: 6px;
    padding: 0.5rem 0.75rem;
    margin: 0;
    line-height: 1.5;
    text-align: center;
  }

  /* ── Status / error cards ────────────────────── */
  .status-card {
    padding: 2.5rem 2rem;
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
  }

  .status-icon {
    font-size: 2.5rem;
    line-height: 1;
  }

  .status-card h1 {
    font-size: 1.5rem;
    font-weight: 700;
    margin: 0;
    color: var(--text-primary, var(--color-text-primary));
  }

  .status-card p {
    font-size: 0.9rem;
    line-height: 1.6;
    color: var(--text-secondary, var(--color-text-secondary));
    margin: 0;
    max-width: 360px;
  }

  .status-card--invalid h1 { color: var(--accent-red); }

  /* ── Footer ──────────────────────────────────── */
  .portal-footer {
    text-align: center;
    font-size: 0.72rem;
    color: var(--text-secondary, var(--color-text-secondary));
    opacity: 0.5;
    margin: 0;
  }
</style>
