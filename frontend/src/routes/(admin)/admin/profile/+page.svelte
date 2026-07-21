<!--
  Admin Profile Settings Page
  Allows the authenticated user to update their own profile:
  name, job_title, bio, and avatar_url.
  Uses PATCH /api/onboarding/profile (Requirements: 4.4)
-->

<script lang="ts">
  import { onMount } from 'svelte';
  import GlassCard from '$lib/components/GlassCard.svelte';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  // Form state — initialised from server-loaded profile
  let name = $state(data.profile.name ?? '');
  let jobTitle = $state(data.profile.job_title ?? '');
  let bio = $state(data.profile.bio ?? '');
  let avatarUrl = $state(data.profile.avatar_url ?? '');

  let busy = $state(false);
  let status = $state<{ text: string; ok: boolean } | null>(null);

  function showStatus(text: string, ok: boolean) {
    status = { text, ok };
    setTimeout(() => (status = null), 5000);
  }

  async function handleSave(e: SubmitEvent) {
    e.preventDefault();
    busy = true;
    try {
      const res = await fetch('/api/onboarding/profile', {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          name: name.trim(),
          job_title: jobTitle.trim(),
          bio: bio.trim(),
          avatar_url: avatarUrl.trim(),
        }),
      });

      if (!res.ok) {
        const body = await res.text();
        showStatus(body || 'Failed to save profile. Please try again.', false);
        return;
      }

      showStatus('Profile updated successfully.', true);
    } catch {
      showStatus('Network error — please try again.', false);
    } finally {
      busy = false;
    }
  }

  // Derived: profile page URL for QR
  const profileUrl = $derived(
    data.profile.username
      ? `https://ezeroandone.io/team/${data.profile.username}`
      : null
  );

  // QR code — rendered client-side into the #qr-container div
  onMount(async () => {
    if (!profileUrl) return;
    try {
      const QRCode = await import('qrcode');
      const svg = await QRCode.toString(profileUrl, {
        type: 'svg',
        margin: 2,
        color: {
          dark: '#00d4ff',
          light: '#00000000',
        },
      });
      const container = document.getElementById('qr-container');
      if (container) container.innerHTML = svg;
    } catch (err) {
      console.error('[QR]', err);
    }
  });
</script>

<svelte:head>
  <title>My Profile — eZeroAndOne Admin</title>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<section class="profile-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">My Profile</h1>
      <p class="page-subtitle">Update your name, title, bio, and avatar.</p>
    </div>
    {#if profileUrl}
      <a href={profileUrl} target="_blank" rel="noopener noreferrer" class="btn btn-secondary">
        <span class="material-icons-outlined" aria-hidden="true">open_in_new</span>
        View public profile
      </a>
    {/if}
  </header>

  {#if status}
    <div
      class="status-banner"
      class:ok={status.ok}
      class:err={!status.ok}
      role="status"
      aria-live="polite"
    >
      {status.text}
    </div>
  {/if}

  <div class="profile-layout">
    <!-- ── Edit form ─────────────────────────────────── -->
    <GlassCard accentColor="blue">
      <div class="card-pad">
        <form class="profile-form" onsubmit={handleSave}>
          <!-- Avatar preview + URL -->
          <div class="avatar-section">
            <div class="avatar-preview" aria-hidden="true">
              {#if avatarUrl}
                <img src={avatarUrl} alt="Avatar preview" class="avatar-img" />
              {:else}
                <span class="avatar-placeholder">
                  {(name || data.profile.email).charAt(0).toUpperCase()}
                </span>
              {/if}
            </div>
            <div class="avatar-fields">
              <label class="field">
                <span class="field-label">Avatar URL</span>
                <input
                  type="url"
                  class="input"
                  bind:value={avatarUrl}
                  placeholder="https://…"
                />
              </label>
              <p class="field-hint">Paste a direct image URL (JPG, PNG, WebP).</p>
            </div>
          </div>

          <div class="form-row">
            <label class="field">
              <span class="field-label">Full name</span>
              <input
                type="text"
                class="input"
                bind:value={name}
                placeholder="Your full name"
                maxlength="120"
                required
              />
            </label>
            <label class="field">
              <span class="field-label">Job title</span>
              <input
                type="text"
                class="input"
                bind:value={jobTitle}
                placeholder="e.g. Senior Engineer"
                maxlength="120"
              />
            </label>
          </div>

          <label class="field">
            <span class="field-label">Bio</span>
            <textarea
              class="input textarea"
              bind:value={bio}
              placeholder="A short bio about yourself…"
              rows="5"
              maxlength="1000"
            ></textarea>
          </label>

          <!-- Read-only identity fields -->
          <div class="form-row">
            <label class="field">
              <span class="field-label">Email</span>
              <input type="email" class="input input--readonly" value={data.profile.email} readonly aria-readonly="true" />
            </label>
            <label class="field">
              <span class="field-label">Username</span>
              <input type="text" class="input input--readonly" value={`@${data.profile.username}`} readonly aria-readonly="true" />
            </label>
          </div>

          <label class="field">
            <span class="field-label">Role</span>
            <input type="text" class="input input--readonly" value={data.profile.role} readonly aria-readonly="true" />
          </label>

          <div class="form-actions">
            <button type="submit" class="btn btn-primary" disabled={busy}>
              {#if busy}
                <span class="material-icons-outlined spin" aria-hidden="true">sync</span>
                Saving…
              {:else}
                <span class="material-icons-outlined" aria-hidden="true">save</span>
                Save changes
              {/if}
            </button>
          </div>
        </form>
      </div>
    </GlassCard>

    <!-- ── QR code card ──────────────────────────────── -->
    {#if profileUrl}
      <GlassCard accentColor="green">
        <div class="card-pad qr-section">
          <h2 class="card-title">Public Profile QR</h2>
          <p class="card-sub">Scan to open your public team profile.</p>

          <!-- QR SVG injected here by onMount -->
          <div class="qr-wrap" id="qr-container" aria-label="QR code for your public profile">
            <div class="qr-loading">
              <span class="material-icons-outlined" aria-hidden="true">qr_code_2</span>
            </div>
          </div>

          <a href={profileUrl} class="profile-link" target="_blank" rel="noopener noreferrer">
            {profileUrl}
          </a>
        </div>
      </GlassCard>
    {/if}
  </div>
</section>

<style>
  .profile-page { max-width: 900px; }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 28px;
    flex-wrap: wrap;
  }

  .page-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 4px;
    letter-spacing: -0.5px;
  }

  .page-subtitle {
    font-size: 0.875rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
  }

  .status-banner {
    padding: 10px 16px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 20px;
    border: 1px solid transparent;
  }

  .ok {
    background: color-mix(in srgb, #00ff88 12%, transparent);
    border-color: color-mix(in srgb, #00ff88 30%, transparent);
    color: #00ff88;
  }

  .err {
    background: color-mix(in srgb, #ff3366 12%, transparent);
    border-color: color-mix(in srgb, #ff3366 30%, transparent);
    color: #ff3366;
  }

  .profile-layout {
    display: grid;
    grid-template-columns: 1fr 280px;
    gap: 20px;
    align-items: start;
  }

  @media (max-width: 720px) {
    .profile-layout { grid-template-columns: 1fr; }
  }

  .card-pad { padding: 24px; }

  .profile-form {
    display: flex;
    flex-direction: column;
    gap: 18px;
  }

  /* Avatar */
  .avatar-section {
    display: flex;
    align-items: flex-start;
    gap: 16px;
  }

  .avatar-preview {
    width: 72px;
    height: 72px;
    border-radius: 50%;
    flex-shrink: 0;
    background: rgba(0, 194, 255, 0.1);
    border: 2px solid rgba(0, 194, 255, 0.25);
    overflow: hidden;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .avatar-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .avatar-placeholder {
    font-size: 1.6rem;
    font-weight: 700;
    color: #00C2FF;
  }

  .avatar-fields {
    flex: 1;
    min-width: 0;
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-hint {
    font-size: 0.72rem;
    color: rgba(255, 255, 255, 0.3);
    margin: 0;
  }

  /* Form */
  .form-row {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 14px;
  }

  @media (max-width: 560px) {
    .form-row { grid-template-columns: 1fr; }
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 5px;
  }

  .field-label {
    font-size: 0.75rem;
    font-weight: 600;
    color: var(--color-text-secondary, #888899);
    text-transform: uppercase;
    letter-spacing: 0.05em;
  }

  .input {
    background: rgba(255, 255, 255, 0.06);
    border: 1px solid rgba(255, 255, 255, 0.1);
    border-radius: 10px;
    padding: 9px 13px;
    color: #f0f0f0;
    font-size: 0.875rem;
    font-family: inherit;
    width: 100%;
    box-sizing: border-box;
    transition: border-color 0.2s;
    appearance: none;
    -webkit-appearance: none;
  }

  .input:focus {
    outline: none;
    border-color: #00C2FF;
    box-shadow: 0 0 0 3px rgba(0, 194, 255, 0.15);
  }

  .input--readonly {
    opacity: 0.5;
    cursor: default;
  }

  .textarea {
    resize: vertical;
    min-height: 110px;
    line-height: 1.6;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    padding-top: 4px;
  }

  /* Buttons */
  .btn {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    padding: 9px 18px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 600;
    border: 1px solid transparent;
    cursor: pointer;
    transition: all 0.18s;
    white-space: nowrap;
    text-decoration: none;
  }

  .btn:disabled { opacity: 0.45; cursor: not-allowed; }

  .btn-primary {
    background: #00C2FF;
    color: #000;
    border-color: #00C2FF;
  }

  .btn-primary:hover:not(:disabled) {
    filter: brightness(1.1);
    box-shadow: 0 0 16px rgba(0, 194, 255, 0.4);
  }

  .btn-secondary {
    background: rgba(255, 255, 255, 0.08);
    color: #f0f0f0;
    border-color: rgba(255, 255, 255, 0.1);
  }

  .btn-secondary:hover { background: rgba(255, 255, 255, 0.13); }

  .btn .material-icons-outlined { font-size: 1.1rem; }

  @keyframes spin { to { transform: rotate(360deg); } }
  .spin { animation: spin 0.8s linear infinite; }

  /* QR Card */
  .qr-section {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 14px;
    text-align: center;
  }

  .card-title {
    font-size: 1rem;
    font-weight: 700;
    color: #f0f0f0;
    margin: 0;
    align-self: flex-start;
  }

  .card-sub {
    font-size: 0.78rem;
    color: rgba(255, 255, 255, 0.45);
    margin: 0;
    line-height: 1.5;
    align-self: flex-start;
    text-align: left;
  }

  .qr-wrap {
    width: 180px;
    height: 180px;
    display: flex;
    align-items: center;
    justify-content: center;
    background: rgba(0, 0, 0, 0.3);
    border-radius: 14px;
    padding: 10px;
    border: 1px solid rgba(0, 194, 255, 0.2);
    margin: 4px 0;
  }

  .qr-wrap :global(svg) {
    width: 100%;
    height: 100%;
  }

  .qr-loading {
    display: flex;
    align-items: center;
    justify-content: center;
    opacity: 0.3;
  }

  .qr-loading .material-icons-outlined {
    font-size: 3.5rem;
    color: #00C2FF;
  }

  .profile-link {
    font-size: 0.72rem;
    color: #00C2FF;
    word-break: break-all;
    text-decoration: none;
    opacity: 0.7;
  }

  .profile-link:hover { opacity: 1; text-decoration: underline; }
</style>
