<script lang="ts">
  import type { PageProps } from './$types';
  import type { ClientLogo } from './+page.server';

  let { data }: PageProps = $props();
  let clients = $state<ClientLogo[]>(data.clients);

  let banner = $state<{ text: string; ok: boolean } | null>(null);
  function showBanner(text: string, ok: boolean) {
    banner = { text, ok };
    setTimeout(() => (banner = null), 4000);
  }

  // ── Create ────────────────────────────────────────────────────────────────
  let showCreate = $state(false);
  let cName = $state(''); let cWebsite = $state(''); let cSortOrder = $state(0);
  let cFile = $state<File | null>(null); let cLogoUrl = $state('');
  let cBusy = $state(false);

  async function handleCreate(e: SubmitEvent) {
    e.preventDefault(); cBusy = true;
    try {
      const res = await fetch('/api/admin/clients', {
        method: 'POST', credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: cName, logo_url: cLogoUrl, website_url: cWebsite, sort_order: cSortOrder }),
      });
      if (!res.ok) { showBanner(`Create failed: ${await res.text()}`, false); return; }
      const created: ClientLogo = await res.json();
      // Upload logo file if provided
      if (cFile) {
        const fd = new FormData(); fd.append('file', cFile);
        const up = await fetch(`/api/upload/client/${created.id}/logo`, { method: 'POST', credentials: 'include', body: fd });
        if (up.ok) {
          const { url } = await up.json();
          const patch = await fetch(`/api/admin/clients/${created.id}`, {
            method: 'PATCH', credentials: 'include',
            headers: { 'Content-Type': 'application/json' },
            body: JSON.stringify({ logo_url: url }),
          });
          if (patch.ok) created.logo_url = (await patch.json()).logo_url;
        }
      }
      clients = [...clients, created].sort((a, b) => a.sort_order - b.sort_order);
      cName = ''; cWebsite = ''; cSortOrder = 0; cFile = null; cLogoUrl = ''; showCreate = false;
      showBanner('Client logo added.', true);
    } catch { showBanner('Network error.', false); }
    finally { cBusy = false; }
  }

  // ── Edit ──────────────────────────────────────────────────────────────────
  let editId = $state<string | null>(null);
  let eName = $state(''); let eWebsite = $state(''); let eSortOrder = $state(0);
  let eLogoUrl = $state(''); let eFile = $state<File | null>(null); let eBusy = $state(false);

  function openEdit(c: ClientLogo) {
    editId = c.id; eName = c.name; eWebsite = c.website_url;
    eSortOrder = c.sort_order; eLogoUrl = c.logo_url; eFile = null;
  }

  async function handleEdit(e: SubmitEvent) {
    e.preventDefault(); if (!editId) return; eBusy = true;
    try {
      if (eFile) {
        const fd = new FormData(); fd.append('file', eFile);
        const up = await fetch(`/api/upload/client/${editId}/logo`, { method: 'POST', credentials: 'include', body: fd });
        if (up.ok) { const { url } = await up.json(); eLogoUrl = url; }
      }
      const res = await fetch(`/api/admin/clients/${editId}`, {
        method: 'PATCH', credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ name: eName, logo_url: eLogoUrl, website_url: eWebsite, sort_order: eSortOrder }),
      });
      if (!res.ok) { showBanner(`Update failed: ${await res.text()}`, false); return; }
      const updated: ClientLogo = await res.json();
      clients = clients.map(c => c.id === updated.id ? updated : c).sort((a, b) => a.sort_order - b.sort_order);
      editId = null; showBanner('Updated.', true);
    } catch { showBanner('Network error.', false); }
    finally { eBusy = false; }
  }

  async function toggleActive(c: ClientLogo) {
    const res = await fetch(`/api/admin/clients/${c.id}`, {
      method: 'PATCH', credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ active: !c.active }),
    });
    if (res.ok) { const updated: ClientLogo = await res.json(); clients = clients.map(x => x.id === updated.id ? updated : x); }
  }

  async function handleDelete(c: ClientLogo) {
    if (!confirm(`Delete "${c.name}"?`)) return;
    const res = await fetch(`/api/admin/clients/${c.id}`, { method: 'DELETE', credentials: 'include' });
    if (res.ok) { clients = clients.filter(x => x.id !== c.id); showBanner('Deleted.', true); }
    else showBanner('Delete failed.', false);
  }
</script>

<svelte:head>
  <title>Client Logos — eZeroAndOne Admin</title>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<section class="clients-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">Client Logos</h1>
      <p class="page-subtitle">Manage brand marks displayed on the site. {clients.length} brand{clients.length !== 1 ? 's' : ''}.</p>
    </div>
    <button class="btn btn-primary" type="button" onclick={() => (showCreate = !showCreate)}>
      <span class="material-icons-outlined" aria-hidden="true">add</span>
      {showCreate ? 'Cancel' : 'Add brand'}
    </button>
  </header>

  {#if banner}
    <div class="banner" class:ok={banner.ok} class:err={!banner.ok} role="status">{banner.text}</div>
  {/if}

  {#if showCreate}
  <div class="card glass-card section-pad">
    <h2 class="section-title">Add brand logo</h2>
    <form class="form" onsubmit={handleCreate}>
      <div class="form-row">
        <label class="field"><span class="field-label">Brand name</span><input type="text" bind:value={cName} class="input" required maxlength="120" /></label>
        <label class="field"><span class="field-label">Website URL</span><input type="url" bind:value={cWebsite} class="input" placeholder="https://…" /></label>
        <label class="field"><span class="field-label">Sort order</span><input type="number" bind:value={cSortOrder} class="input" min="0" step="1" /></label>
      </div>
      <div class="field">
        <span class="field-label">Logo image</span>
        <div class="img-row">
          <input type="file" accept="image/*" class="input" onchange={(e) => { const t = e.currentTarget as HTMLInputElement; cFile = t.files?.[0] ?? null; }} />
          <span class="img-or">or URL</span>
          <input type="url" bind:value={cLogoUrl} class="input" placeholder="https://…" />
        </div>
      </div>
      <div class="form-actions">
        <button type="button" class="btn btn-secondary" onclick={() => (showCreate = false)}>Cancel</button>
        <button type="submit" class="btn btn-primary" disabled={cBusy}>{cBusy ? 'Adding…' : 'Add brand'}</button>
      </div>
    </form>
  </div>
  {/if}

  {#if clients.length === 0}
    <div class="empty-state">
      <span class="material-icons-outlined empty-icon">business</span>
      <p>No client logos yet. Add one to get started.</p>
    </div>
  {:else}
    <div class="logo-grid">
      {#each clients as c (c.id)}
        <div class="logo-card glass-card" class:inactive={!c.active}>
          <div class="logo-img-wrap">
            {#if c.logo_url}
              <img src={c.logo_url} alt={c.name} class="logo-img" />
            {:else}
              <span class="logo-placeholder">{c.name.charAt(0).toUpperCase()}</span>
            {/if}
          </div>
          <div class="logo-info">
            <span class="logo-name">{c.name}</span>
            {#if c.website_url}
              <a href={c.website_url} target="_blank" rel="noopener" class="logo-url">{c.website_url}</a>
            {/if}
            <span class="logo-order">Order: {c.sort_order}</span>
          </div>
          <div class="logo-actions">
            <span class="badge" class:active={c.active} class:inactive-badge={!c.active}>{c.active ? 'Active' : 'Hidden'}</span>
            <button class="icon-action" type="button" onclick={() => openEdit(c)} title="Edit" aria-label="Edit {c.name}">
              <span class="material-icons-outlined">edit</span>
            </button>
            <button class="icon-action" type="button" onclick={() => toggleActive(c)} title="{c.active ? 'Hide' : 'Show'}" aria-label="{c.active ? 'Hide' : 'Show'} {c.name}">
              <span class="material-icons-outlined">{c.active ? 'visibility_off' : 'visibility'}</span>
            </button>
            <button class="icon-action icon-action--danger" type="button" onclick={() => handleDelete(c)} title="Delete" aria-label="Delete {c.name}">
              <span class="material-icons-outlined">delete</span>
            </button>
          </div>

          {#if editId === c.id}
          <form class="edit-form" onsubmit={handleEdit}>
            <div class="form-row">
              <label class="field"><span class="field-label">Name</span><input type="text" bind:value={eName} class="input" required /></label>
              <label class="field"><span class="field-label">Website</span><input type="url" bind:value={eWebsite} class="input" /></label>
              <label class="field"><span class="field-label">Order</span><input type="number" bind:value={eSortOrder} class="input" min="0" /></label>
            </div>
            <div class="field">
              <span class="field-label">Logo</span>
              <div class="img-row">
                <input type="file" accept="image/*" class="input" onchange={(e) => { const t = e.currentTarget as HTMLInputElement; eFile = t.files?.[0] ?? null; }} />
                <span class="img-or">or URL</span>
                <input type="url" bind:value={eLogoUrl} class="input" />
              </div>
            </div>
            <div class="form-actions">
              <button type="button" class="btn btn-secondary btn-sm" onclick={() => (editId = null)}>Cancel</button>
              <button type="submit" class="btn btn-primary btn-sm" disabled={eBusy}>{eBusy ? 'Saving…' : 'Save'}</button>
            </div>
          </form>
          {/if}
        </div>
      {/each}
    </div>
  {/if}
</section>

<style>
  .clients-page { max-width: 1000px; }
  .page-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 16px; margin-bottom: 28px; flex-wrap: wrap; }
  .page-title { font-size: 2rem; font-weight: 700; color: var(--color-text-primary,#f0f0f0); margin: 0 0 4px; letter-spacing: -0.5px; }
  .page-subtitle { font-size: 0.9rem; color: var(--color-text-secondary,#888899); margin: 0; }

  .banner { padding: 10px 16px; border-radius: 10px; font-size: 0.875rem; font-weight: 500; margin-bottom: 20px; border: 1px solid transparent; }
  .ok { background: color-mix(in srgb, #34c759 12%, transparent); border-color: color-mix(in srgb, #34c759 30%, transparent); color: #34c759; }
  .err { background: color-mix(in srgb, #ff2d55 12%, transparent); border-color: color-mix(in srgb, #ff2d55 30%, transparent); color: #ff2d55; }

  .glass-card { background: var(--glass-bg, rgba(255,255,255,0.04)); border: 1px solid var(--glass-border, rgba(255,255,255,0.08)); border-radius: 16px; }
  .section-pad { padding: 24px; margin-bottom: 24px; }
  .section-title { font-size: 1rem; font-weight: 700; color: var(--color-text-primary,#f0f0f0); margin: 0 0 18px; }

  .form { display: flex; flex-direction: column; gap: 14px; }
  .form-row { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; }
  .field { display: flex; flex-direction: column; gap: 5px; }
  .field-label { font-size: 0.73rem; font-weight: 600; color: var(--color-text-secondary,#888899); text-transform: uppercase; letter-spacing: 0.05em; }
  .input { background: rgba(255,255,255,0.06); border: 1px solid rgba(255,255,255,0.1); border-radius: 10px; padding: 9px 13px; color: var(--color-text-primary,#f0f0f0); font-size: 0.875rem; font-family: inherit; width: 100%; box-sizing: border-box; -webkit-appearance: none; appearance: none; }
  .input:focus { outline: none; border-color: var(--accent-blue,#00d4ff); box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent-blue,#00d4ff) 20%, transparent); }
  .img-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
  .img-row .input { flex: 1; min-width: 140px; }
  .img-or { font-size: 0.73rem; color: var(--color-text-muted, rgba(255,255,255,0.3)); flex-shrink: 0; }
  .form-actions { display: flex; justify-content: flex-end; gap: 10px; padding-top: 8px; border-top: 1px solid rgba(255,255,255,0.08); }

  .btn { display: inline-flex; align-items: center; gap: 5px; padding: 9px 18px; border-radius: 10px; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.2s; }
  .btn:disabled { opacity: 0.5; cursor: not-allowed; }
  .btn-sm { padding: 6px 12px; font-size: 0.78rem; border-radius: 8px; }
  .btn-primary { background: var(--accent-blue,#00d4ff); color: #000; }
  .btn-primary:hover:not(:disabled) { filter: brightness(1.1); }
  .btn-secondary { background: rgba(255,255,255,0.08); color: var(--color-text-primary,#f0f0f0); border: 1px solid rgba(255,255,255,0.1); }
  .btn-secondary:hover:not(:disabled) { background: rgba(255,255,255,0.14); }

  .empty-state { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 60px 24px; color: var(--color-text-secondary,#888899); }
  .empty-icon { font-size: 3rem; color: var(--color-text-muted, rgba(255,255,255,0.2)); }

  .logo-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 16px; }

  .logo-card { padding: 20px; display: flex; flex-direction: column; gap: 14px; transition: border-color 0.2s; }
  .logo-card:hover { border-color: rgba(255,255,255,0.16); }
  .logo-card.inactive { opacity: 0.5; }

  .logo-img-wrap { height: 72px; display: flex; align-items: center; justify-content: center; background: rgba(255,255,255,0.03); border-radius: 10px; overflow: hidden; }
  .logo-img { max-height: 60px; max-width: 100%; object-fit: contain; filter: brightness(0) invert(1); opacity: 0.85; }
  .logo-placeholder { font-size: 2rem; font-weight: 700; color: var(--color-text-secondary,#888899); }

  .logo-info { display: flex; flex-direction: column; gap: 3px; min-width: 0; }
  .logo-name { font-weight: 600; color: var(--color-text-primary,#f0f0f0); font-size: 0.95rem; }
  .logo-url { font-size: 0.75rem; color: var(--accent-blue,#00d4ff); text-decoration: none; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .logo-url:hover { text-decoration: underline; }
  .logo-order { font-size: 0.7rem; color: var(--color-text-muted, rgba(255,255,255,0.3)); }

  .logo-actions { display: flex; align-items: center; gap: 8px; flex-wrap: wrap; }
  .badge { padding: 2px 8px; border-radius: 20px; font-size: 0.68rem; font-weight: 600; text-transform: uppercase; letter-spacing: 0.04em; }
  .badge.active { background: rgba(52,199,89,0.15); color: #34c759; border: 1px solid rgba(52,199,89,0.3); }
  .badge.inactive-badge { background: rgba(255,255,255,0.07); color: var(--color-text-muted, rgba(255,255,255,0.35)); border: 1px solid rgba(255,255,255,0.1); }

  .icon-action { background: rgba(255,255,255,0.07); border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; padding: 6px; cursor: pointer; color: var(--color-text-secondary,#888899); transition: all 0.18s; display: flex; align-items: center; }
  .icon-action:hover { background: rgba(255,255,255,0.14); color: var(--color-text-primary,#f0f0f0); }
  .icon-action .material-icons-outlined { font-size: 1.1rem; }
  .icon-action--danger:hover { background: rgba(255,45,85,0.18); color: #ff2d55; border-color: rgba(255,45,85,0.3); }

  .edit-form { display: flex; flex-direction: column; gap: 12px; padding-top: 14px; border-top: 1px solid rgba(255,255,255,0.08); }

  @media (max-width: 600px) {
    .logo-grid { grid-template-columns: 1fr; }
    .page-header { flex-direction: column; }
  }
</style>
