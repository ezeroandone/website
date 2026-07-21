<!--
  Admin Careers & Applications Management Page — Task 22.3
  Requirements: 5.5, 5.6, 5.7, 7.3, 7.6, 15.4
-->

<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import type { PageProps } from './$types';
  import type { Career, Application } from './+page.server';

  let { data }: PageProps = $props();

  type CareerType = Career['type'];
  type ApplicationStatus = Application['status'];

  let careers = $state<Career[]>(data.careers);
  let applications = $state<Application[]>(data.applications);

  const TRANSITIONS: Record<ApplicationStatus, ApplicationStatus[]> = {
    Applied: ['Interviewing'],
    Interviewing: ['Offered', 'Rejected'],
    Offered: ['Hired', 'Rejected'],
    Rejected: [],
    Hired: [],
  };
  const FINAL_STATUSES: ApplicationStatus[] = ['Hired', 'Rejected'];

  let banner = $state<{ text: string; ok: boolean } | null>(null);
  function showBanner(text: string, ok: boolean) {
    banner = { text, ok };
    setTimeout(() => (banner = null), 4500);
  }

  function slugify(title: string): string {
    return title
      .toLowerCase()
      .trim()
      .replace(/[^\w\s-]/g, '')
      .replace(/[\s_-]+/g, '-')
      .replace(/^-+|-+$/g, '');
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString('en-GB', {
      day: 'numeric', month: 'short', year: 'numeric',
    });
  }

  function statusBadgeClass(status: ApplicationStatus): string {
    const map: Record<ApplicationStatus, string> = {
      Applied: 'badge badge--blue',
      Interviewing: 'badge badge--yellow',
      Offered: 'badge badge--green',
      Rejected: 'badge badge--muted',
      Hired: 'badge badge--purple',
    };
    return map[status] ?? 'badge';
  }

  function activeBadgeClass(active: boolean): string {
    return active ? 'badge badge--green' : 'badge badge--muted';
  }

  // ── Create career ─────────────────────────────────────────────────────────
  let showCreate = $state(false);
  let createTitle = $state('');
  let createSlug = $derived(slugify(createTitle));
  let createDepartment = $state('');
  let createDescMd = $state('');
  let createType = $state<CareerType>('Full-Time');
  let createActive = $state(true);
  let createBusy = $state(false);

  async function handleCreate(e: SubmitEvent) {
    e.preventDefault();
    createBusy = true;
    try {
      const res = await fetch('/api/admin/careers', {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          slug: createSlug, title: createTitle, description_md: createDescMd,
          department: createDepartment, type: createType, active: createActive,
        }),
      });
      if (!res.ok) { showBanner(`Create failed: ${await res.text()}`, false); return; }
      const created: Career = await res.json();
      careers = [created, ...careers];
      createTitle = ''; createDepartment = ''; createDescMd = '';
      createType = 'Full-Time'; createActive = true; showCreate = false;
      showBanner('Career listing created.', true);
    } catch { showBanner('Network error — career not created.', false); }
    finally { createBusy = false; }
  }

  // ── Edit career ───────────────────────────────────────────────────────────
  let editingId = $state<string | null>(null);
  let editTitle = $state('');
  let editDepartment = $state('');
  let editDescMd = $state('');
  let editType = $state<CareerType>('Full-Time');
  let editActive = $state(true);
  let editBusy = $state(false);

  function openEdit(career: Career) {
    editingId = career.id; editTitle = career.title; editDepartment = career.department;
    editDescMd = career.description_md; editType = career.type; editActive = career.active;
  }
  function cancelEdit() { editingId = null; }

  async function handleEdit(e: SubmitEvent, careerId: string) {
    e.preventDefault();
    editBusy = true;
    try {
      const res = await fetch(`/api/admin/careers/${careerId}`, {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          title: editTitle, department: editDepartment, description_md: editDescMd,
          type: editType, active: editActive,
        }),
      });
      if (!res.ok) { showBanner(`Update failed: ${await res.text()}`, false); return; }
      const updated: Career = await res.json();
      careers = careers.map((c) => (c.id === careerId ? updated : c));
      editingId = null;
      showBanner('Career updated.', true);
    } catch { showBanner('Network error — career not updated.', false); }
    finally { editBusy = false; }
  }

  async function toggleActive(career: Career) {
    try {
      const res = await fetch(`/api/admin/careers/${career.id}`, {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ active: !career.active }),
      });
      if (!res.ok) { showBanner(`Toggle failed: ${await res.text()}`, false); return; }
      const updated: Career = await res.json();
      careers = careers.map((c) => (c.id === career.id ? updated : c));
    } catch { showBanner('Network error — toggle failed.', false); }
  }

  // ── Application transitions ───────────────────────────────────────────────
  let pendingStatus = $state<Record<string, ApplicationStatus>>(
    Object.fromEntries(
      applications
        .filter((a) => TRANSITIONS[a.status].length > 0)
        .map((a) => [a.id, TRANSITIONS[a.status][0]])
    )
  );
  let transitionBusy = $state<Set<string>>(new Set());
  let appMessages = $state<Record<string, { text: string; ok: boolean }>>({});

  function setAppMsg(id: string, text: string, ok: boolean) {
    appMessages = { ...appMessages, [id]: { text, ok } };
    setTimeout(() => {
      const updated = { ...appMessages };
      delete updated[id];
      appMessages = updated;
    }, 4000);
  }

  async function applyTransition(app: Application) {
    const nextStatus = pendingStatus[app.id];
    if (!nextStatus) return;
    transitionBusy = new Set([...transitionBusy, app.id]);
    try {
      const res = await fetch(`/api/admin/applications/${app.id}/status`, {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ status: nextStatus }),
      });
      if (!res.ok) { setAppMsg(app.id, `Transition failed: ${await res.text()}`, false); return; }
      applications = applications.map((a) =>
        a.id === app.id ? { ...a, status: nextStatus, updated_at: Math.floor(Date.now() / 1000) } : a
      );
      const next = TRANSITIONS[nextStatus];
      const upd = { ...pendingStatus };
      if (next.length > 0) { upd[app.id] = next[0]; } else { delete upd[app.id]; }
      pendingStatus = upd;
      setAppMsg(app.id, `Moved to ${nextStatus}.`, true);
    } catch { setAppMsg(app.id, 'Network error.', false); }
    finally {
      const s = new Set(transitionBusy); s.delete(app.id); transitionBusy = s;
    }
  }

  // ── Hire action ───────────────────────────────────────────────────────────
  let hireAppId = $state<string | null>(null);
  let hireProbationMonths = $state(3);
  let hireBusy = $state(false);

  function openHire(appId: string) { hireAppId = appId; hireProbationMonths = 3; }

  async function handleHire(e: SubmitEvent) {
    e.preventDefault();
    if (!hireAppId) return;
    hireBusy = true;
    try {
      const res = await fetch(`/api/admin/applications/${hireAppId}/hire`, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ probation_months: hireProbationMonths }),
      });
      if (!res.ok) { showBanner(`Hire failed: ${await res.text()}`, false); return; }
      const { staff_id }: { staff_id: string } = await res.json();
      applications = applications.map((a) =>
        a.id === hireAppId ? { ...a, status: 'Hired' as const, updated_at: Math.floor(Date.now() / 1000) } : a
      );
      hireAppId = null;
      showBanner(`Hired successfully. New staff ID: ${staff_id}`, true);
    } catch { showBanner('Network error — hire failed.', false); }
    finally { hireBusy = false; }
  }

  const appsByCareer = $derived(
    careers.reduce<Record<string, Application[]>>((acc, career) => {
      acc[career.id] = applications.filter((a) => a.career_id === career.id);
      return acc;
    }, {})
  );
</script>

<svelte:head>
  <title>Careers — eZeroAndOne Admin</title>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<section class="careers-page">
  <header class="page-header">
    <div class="page-header-text">
      <h1 class="page-title">Careers</h1>
      <p class="page-subtitle">
        {careers.length} listing{careers.length !== 1 ? 's' : ''} ·
        {applications.length} application{applications.length !== 1 ? 's' : ''}
      </p>
    </div>
    <button class="btn btn-primary" type="button" onclick={() => (showCreate = !showCreate)} aria-expanded={showCreate}>
      {showCreate ? 'Cancel' : '+ New listing'}
    </button>
  </header>

  {#if banner}
    <div class="banner" class:banner--ok={banner.ok} class:banner--err={!banner.ok} role="status" aria-live="polite">
      {banner.text}
    </div>
  {/if}

  {#if showCreate}
    <GlassCard accentColor="green">
      <div class="section-pad">
        <h2 class="section-title">New career listing</h2>
        <form class="career-form" onsubmit={handleCreate}>
          <div class="form-row">
            <label class="field">
              <span class="field-label">Title</span>
              <input type="text" class="input" bind:value={createTitle} placeholder="e.g. Senior Rust Engineer" required maxlength="200" />
            </label>
            <label class="field">
              <span class="field-label">Slug (auto-generated)</span>
              <input type="text" class="input input--readonly" value={createSlug} readonly aria-readonly="true" tabindex={-1} />
            </label>
          </div>
          <div class="form-row">
            <label class="field">
              <span class="field-label">Department</span>
              <input type="text" class="input" bind:value={createDepartment} placeholder="e.g. Engineering" required maxlength="100" />
            </label>
            <label class="field">
              <span class="field-label">Type</span>
              <select class="input" bind:value={createType} required>
                <option value="Full-Time">Full-Time</option>
                <option value="Part-Time">Part-Time</option>
                <option value="Contract">Contract</option>
                <option value="Internship">Internship</option>
              </select>
            </label>
          </div>
          <label class="field">
            <span class="field-label">Description (Markdown)</span>
            <textarea class="input textarea" bind:value={createDescMd} placeholder="Describe the role…" rows="5"></textarea>
          </label>
          <div class="form-footer">
            <label class="field-inline">
              <input type="checkbox" class="checkbox" bind:checked={createActive} />
              <span class="field-label">Active (visible on public careers page)</span>
            </label>
            <div class="form-actions">
              <button type="button" class="btn btn-secondary" onclick={() => (showCreate = false)}>Cancel</button>
              <button type="submit" class="btn btn-primary" disabled={createBusy}>{createBusy ? 'Creating…' : 'Create listing'}</button>
            </div>
          </div>
        </form>
      </div>
    </GlassCard>
  {/if}

  {#if careers.length === 0}
    <GlassCard accentColor="green">
      <div class="empty-state">
        <span class="material-icons-outlined empty-icon" aria-hidden="true">work_outline</span>
        <p>No career listings yet. Create one to get started.</p>
      </div>
    </GlassCard>
  {:else}
    <GlassCard accentColor="green">
      <div class="table-wrapper" role="region" aria-label="Career listings">
        <table class="data-table">
          <thead>
            <tr>
              <th scope="col">Title</th>
              <th scope="col">Department</th>
              <th scope="col">Type</th>
              <th scope="col">Status</th>
              <th scope="col">Created</th>
              <th scope="col" class="col-actions">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each careers as career (career.id)}
              <tr>
                <td class="cell-title">{career.title}</td>
                <td class="cell-muted">{career.department}</td>
                <td><span class="badge badge--blue">{career.type}</span></td>
                <td><span class={activeBadgeClass(career.active)}>{career.active ? 'Active' : 'Inactive'}</span></td>
                <td class="cell-date">
                  <time datetime={new Date(career.created_at * 1000).toISOString()}>{formatDate(career.created_at)}</time>
                </td>
                <td class="cell-actions">
                  <div class="action-row">
                    <button class="btn btn-sm btn-secondary" type="button" onclick={() => openEdit(career)} aria-label="Edit {career.title}">Edit</button>
                    <button
                      class="btn btn-sm"
                      class:btn-toggle-on={career.active}
                      class:btn-toggle-off={!career.active}
                      type="button"
                      onclick={() => toggleActive(career)}
                      aria-label="{career.active ? 'Deactivate' : 'Activate'} {career.title}"
                    >{career.active ? 'Deactivate' : 'Activate'}</button>
                  </div>
                </td>
              </tr>
              {#if editingId === career.id}
                <tr class="edit-row">
                  <td colspan="6">
                    <form class="inline-edit-form" onsubmit={(e) => handleEdit(e, career.id)}>
                      <div class="form-row">
                        <label class="field">
                          <span class="field-label">Title</span>
                          <input type="text" class="input" bind:value={editTitle} required maxlength="200" />
                        </label>
                        <label class="field">
                          <span class="field-label">Department</span>
                          <input type="text" class="input" bind:value={editDepartment} required maxlength="100" />
                        </label>
                        <label class="field">
                          <span class="field-label">Type</span>
                          <select class="input" bind:value={editType} required>
                            <option value="Full-Time">Full-Time</option>
                            <option value="Part-Time">Part-Time</option>
                            <option value="Contract">Contract</option>
                            <option value="Internship">Internship</option>
                          </select>
                        </label>
                      </div>
                      <label class="field">
                        <span class="field-label">Description (Markdown)</span>
                        <textarea class="input textarea" bind:value={editDescMd} rows="4"></textarea>
                      </label>
                      <div class="form-footer">
                        <label class="field-inline">
                          <input type="checkbox" class="checkbox" bind:checked={editActive} />
                          <span class="field-label">Active</span>
                        </label>
                        <div class="form-actions">
                          <button type="button" class="btn btn-sm btn-secondary" onclick={cancelEdit}>Cancel</button>
                          <button type="submit" class="btn btn-sm btn-primary" disabled={editBusy}>{editBusy ? 'Saving…' : 'Save changes'}</button>
                        </div>
                      </div>
                    </form>
                  </td>
                </tr>
              {/if}
            {/each}
          </tbody>
        </table>
      </div>
    </GlassCard>
  {/if}

  <div class="section-divider"></div>
  <h2 class="section-heading">Applications</h2>

  {#if applications.length === 0}
    <GlassCard accentColor="blue">
      <div class="empty-state">
        <span class="material-icons-outlined empty-icon" aria-hidden="true">assignment</span>
        <p>No applications yet.</p>
      </div>
    </GlassCard>
  {:else}
    {#each careers as career (career.id)}
      {#if appsByCareer[career.id]?.length > 0}
        <div class="career-group">
          <h3 class="career-group-title">{career.title}</h3>
          <GlassCard accentColor="blue">
            <div class="table-wrapper" role="region" aria-label="Applications for {career.title}">
              <table class="data-table">
                <thead>
                  <tr>
                    <th scope="col">Applicant</th>
                    <th scope="col">Status</th>
                    <th scope="col">Applied</th>
                    <th scope="col">Updated</th>
                    <th scope="col" class="col-actions">Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {#each appsByCareer[career.id] as app (app.id)}
                    <tr class:row--busy={transitionBusy.has(app.id)}>
                      <td class="cell-applicant">
                        <span class="applicant-name">{app.applicant_name}</span>
                        <span class="applicant-email">{app.applicant_email}</span>
                      </td>
                      <td><span class={statusBadgeClass(app.status)}>{app.status}</span></td>
                      <td class="cell-date">
                        <time datetime={new Date(app.applied_at * 1000).toISOString()}>{formatDate(app.applied_at)}</time>
                      </td>
                      <td class="cell-date">
                        <time datetime={new Date(app.updated_at * 1000).toISOString()}>{formatDate(app.updated_at)}</time>
                      </td>
                      <td class="cell-actions">
                        {#if !FINAL_STATUSES.includes(app.status) && TRANSITIONS[app.status].length > 0}
                          <div class="action-row">
                            <label class="sr-only" for="status-{app.id}">Move {app.applicant_name} to</label>
                            <select
                              id="status-{app.id}"
                              class="input input--sm"
                              bind:value={pendingStatus[app.id]}
                              disabled={transitionBusy.has(app.id)}
                            >
                              {#each TRANSITIONS[app.status] as next}
                                <option value={next}>{next}</option>
                              {/each}
                            </select>
                            <button
                              class="btn btn-sm btn-secondary"
                              type="button"
                              disabled={transitionBusy.has(app.id)}
                              onclick={() => applyTransition(app)}
                              aria-label="Apply status transition for {app.applicant_name}"
                            >{transitionBusy.has(app.id) ? '…' : 'Apply'}</button>
                            {#if app.status === 'Offered'}
                              <button
                                class="btn btn-sm btn-hire"
                                type="button"
                                onclick={() => openHire(app.id)}
                                aria-label="Hire {app.applicant_name}"
                              >Hire</button>
                            {/if}
                          </div>
                        {:else}
                          <span class="cell-muted">—</span>
                        {/if}
                        {#if appMessages[app.id]}
                          <p
                            class="action-msg"
                            class:action-msg--ok={appMessages[app.id].ok}
                            class:action-msg--err={!appMessages[app.id].ok}
                            role="status"
                            aria-live="polite"
                          >{appMessages[app.id].text}</p>
                        {/if}
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </GlassCard>
        </div>
      {/if}
    {/each}

    {#if applications.some((a) => !careers.find((c) => c.id === a.career_id))}
      <div class="career-group">
        <h3 class="career-group-title">Other / Archived Listings</h3>
        <GlassCard accentColor="blue">
          <div class="table-wrapper">
            <table class="data-table">
              <thead>
                <tr>
                  <th scope="col">Applicant</th>
                  <th scope="col">Status</th>
                  <th scope="col">Applied</th>
                  <th scope="col">Updated</th>
                  <th scope="col">Actions</th>
                </tr>
              </thead>
              <tbody>
                {#each applications.filter((a) => !careers.find((c) => c.id === a.career_id)) as app (app.id)}
                  <tr>
                    <td class="cell-applicant">
                      <span class="applicant-name">{app.applicant_name}</span>
                      <span class="applicant-email">{app.applicant_email}</span>
                    </td>
                    <td><span class={statusBadgeClass(app.status)}>{app.status}</span></td>
                    <td class="cell-date">{formatDate(app.applied_at)}</td>
                    <td class="cell-date">{formatDate(app.updated_at)}</td>
                    <td class="cell-muted">—</td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </GlassCard>
      </div>
    {/if}
  {/if}
</section>

{#if hireAppId !== null}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => (hireAppId = null)}>
    <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
    <div class="modal" role="dialog" aria-modal="true" aria-labelledby="hire-dialog-title" tabindex="-1" onclick={(e) => e.stopPropagation()}>
      <div class="modal-header">
        <h2 id="hire-dialog-title" class="modal-title">Hire applicant</h2>
        <button class="modal-close" type="button" onclick={() => (hireAppId = null)} aria-label="Close hire dialog">✕</button>
      </div>
      <p class="modal-body-text">Set the probation period (3–6 months). This will create a new staff record and send an onboarding email.</p>
      <form class="modal-form" onsubmit={handleHire}>
        <label class="field">
          <span class="field-label">Probation months</span>
          <input type="number" class="input" bind:value={hireProbationMonths} min="3" max="6" step="1" required />
        </label>
        <div class="form-actions">
          <button type="button" class="btn btn-secondary" onclick={() => (hireAppId = null)}>Cancel</button>
          <button type="submit" class="btn btn-hire" disabled={hireBusy}>{hireBusy ? 'Processing…' : 'Confirm hire'}</button>
        </div>
      </form>
    </div>
  </div>
{/if}

<style>
  .careers-page { max-width: 1100px; }

  .page-header {
    display: flex;
    align-items: flex-start;
    justify-content: space-between;
    gap: 16px;
    margin-bottom: 24px;
    flex-wrap: wrap;
  }
  .page-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 4px;
    letter-spacing: -0.5px;
  }
  .page-subtitle { font-size: 0.875rem; color: var(--color-text-secondary, #888899); margin: 0; }

  .banner {
    padding: 10px 16px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 20px;
    border: 1px solid transparent;
  }
  .banner--ok {
    background: color-mix(in srgb, var(--accent-green, #34c759) 12%, transparent);
    border-color: color-mix(in srgb, var(--accent-green, #34c759) 30%, transparent);
    color: var(--accent-green, #34c759);
  }
  .banner--err {
    background: color-mix(in srgb, var(--accent-red, #ff2d55) 12%, transparent);
    border-color: color-mix(in srgb, var(--accent-red, #ff2d55) 30%, transparent);
    color: var(--accent-red, #ff2d55);
  }

  .section-pad { padding: 24px; }
  .section-title { font-size: 1rem; font-weight: 700; color: var(--color-text-primary, #f0f0f0); margin: 0 0 18px; }
  .section-divider { height: 1px; background: var(--glass-border, rgba(255,255,255,0.08)); margin: 36px 0 20px; }
  .section-heading { font-size: 1.25rem; font-weight: 700; color: var(--color-text-primary, #f0f0f0); margin: 0 0 16px; }

  .career-group { margin-bottom: 28px; }
  .career-group-title {
    font-size: 0.9rem;
    font-weight: 600;
    color: var(--color-text-secondary, #888899);
    margin: 0 0 8px;
    text-transform: uppercase;
    letter-spacing: 0.04em;
  }

  .table-wrapper { overflow-x: auto; -webkit-overflow-scrolling: touch; }
  .data-table { width: 100%; border-collapse: collapse; font-size: 0.875rem; color: var(--color-text-primary, #f0f0f0); }
  .data-table thead tr { border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.12)); }
  .data-table th {
    padding: 12px 16px;
    text-align: left;
    font-weight: 600;
    font-size: 0.72rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-secondary, #888899);
    white-space: nowrap;
  }
  .data-table td {
    padding: 13px 16px;
    vertical-align: middle;
    border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.06));
  }
  .data-table tbody tr:last-child td { border-bottom: none; }
  .data-table tbody tr:hover:not(.edit-row) { background: var(--bg-elevated, rgba(255,255,255,0.03)); }
  .row--busy { opacity: 0.65; pointer-events: none; }

  .cell-title { font-weight: 600; max-width: 260px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .cell-muted { color: var(--color-text-secondary, #888899); font-size: 0.85rem; }
  .cell-date { white-space: nowrap; color: var(--color-text-secondary, #888899); font-size: 0.8rem; }
  .col-actions { min-width: 180px; }
  .cell-actions { vertical-align: top; padding-top: 10px; }
  .action-row { display: flex; flex-wrap: wrap; align-items: center; gap: 8px; }
  .cell-applicant { display: flex; flex-direction: column; gap: 2px; }
  .applicant-name { font-weight: 600; }
  .applicant-email { font-size: 0.75rem; color: var(--color-text-secondary, #888899); }

  .edit-row td { background: var(--bg-elevated, rgba(255,255,255,0.04)); padding: 20px 24px; }
  .inline-edit-form { display: flex; flex-direction: column; gap: 14px; }

  .badge {
    display: inline-block;
    padding: 3px 9px;
    border-radius: 20px;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    white-space: nowrap;
  }
  .badge--red { background: rgba(255,45,85,0.15); color: var(--accent-red, #ff2d55); border: 1px solid rgba(255,45,85,0.25); }
  .badge--blue { background: rgba(0,122,255,0.15); color: var(--accent-blue, #007aff); border: 1px solid rgba(0,122,255,0.25); }
  .badge--green { background: rgba(52,199,89,0.15); color: var(--accent-green, #34c759); border: 1px solid rgba(52,199,89,0.25); }
  .badge--yellow { background: rgba(255,214,10,0.15); color: var(--accent-yellow, #ffd60a); border: 1px solid rgba(255,214,10,0.25); }
  .badge--purple { background: rgba(175,82,222,0.15); color: #af52de; border: 1px solid rgba(175,82,222,0.25); }
  .badge--muted { background: var(--bg-elevated, rgba(255,255,255,0.08)); color: var(--color-text-muted, rgba(255,255,255,0.35)); border: 1px solid var(--glass-border); }

  .career-form { display: flex; flex-direction: column; gap: 14px; }
  .form-row { display: grid; grid-template-columns: repeat(auto-fit, minmax(200px, 1fr)); gap: 14px; }
  .field { display: flex; flex-direction: column; gap: 5px; }
  .field-label { font-size: 0.75rem; font-weight: 600; color: var(--color-text-secondary, #888899); text-transform: uppercase; letter-spacing: 0.05em; }
  .field-inline { display: flex; flex-direction: row; align-items: center; gap: 8px; }
  .field-inline .field-label { text-transform: none; font-size: 0.85rem; color: var(--color-text-primary, #f0f0f0); }

  .input {
    background: var(--bg-elevated, rgba(255,255,255,0.06));
    border: 1px solid var(--glass-border, rgba(255,255,255,0.1));
    border-radius: 10px;
    padding: 9px 13px;
    color: var(--color-text-primary, #f0f0f0);
    font-size: 0.875rem;
    font-family: inherit;
    width: 100%;
    box-sizing: border-box;
    transition: border-color 0.2s ease;
    -webkit-appearance: none;
    appearance: none;
  }
  .input:focus {
    outline: none;
    border-color: var(--accent-blue, #007aff);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent-blue, #007aff) 20%, transparent);
  }
  .input--readonly { opacity: 0.55; cursor: default; font-family: 'SFMono-Regular', Consolas, monospace; font-size: 0.8rem; }
  .input--sm { padding: 5px 9px; font-size: 0.8rem; border-radius: 8px; width: auto; }
  .textarea { resize: vertical; min-height: 110px; font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace; font-size: 0.82rem; line-height: 1.6; }
  .checkbox { width: 16px; height: 16px; accent-color: var(--accent-blue, #007aff); cursor: pointer; flex-shrink: 0; }
  .form-footer { display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 12px; padding-top: 6px; }
  .form-actions { display: flex; gap: 10px; align-items: center; }

  .empty-state { display: flex; flex-direction: column; align-items: center; gap: 12px; padding: 40px 24px; color: var(--color-text-secondary, #888899); font-size: 0.95rem; }
  .empty-icon { font-size: 2.25rem; }

  .action-msg { margin: 4px 0 0; font-size: 0.75rem; font-weight: 500; }
  .action-msg--ok { color: var(--accent-green, #34c759); }
  .action-msg--err { color: var(--accent-red, #ff2d55); }

  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 9px 18px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 600;
    border: 1px solid transparent;
    cursor: pointer;
    transition: all 0.18s ease;
    white-space: nowrap;
    line-height: 1.4;
  }
  .btn:disabled { opacity: 0.45; cursor: not-allowed; }
  .btn-sm { padding: 5px 13px; font-size: 0.78rem; border-radius: 8px; }
  .btn-primary { background: var(--accent-blue, #007aff); color: #000; border-color: var(--accent-blue, #007aff); }
  .btn-primary:hover:not(:disabled) { filter: brightness(1.1); box-shadow: 0 0 16px color-mix(in srgb, var(--accent-blue, #007aff) 45%, transparent); }
  .btn-secondary { background: var(--bg-elevated, rgba(255,255,255,0.08)); color: var(--color-text-primary, #f0f0f0); border-color: var(--glass-border, rgba(255,255,255,0.1)); }
  .btn-secondary:hover:not(:disabled) { background: rgba(255,255,255,0.12); }
  .btn-hire { background: rgba(175,82,222,0.2); color: #af52de; border-color: rgba(175,82,222,0.35); }
  .btn-hire:hover:not(:disabled) { background: rgba(175,82,222,0.3); box-shadow: 0 0 12px rgba(175,82,222,0.3); }
  .btn-toggle-on { background: rgba(255,45,85,0.15); color: var(--accent-red, #ff2d55); border-color: rgba(255,45,85,0.3); }
  .btn-toggle-on:hover:not(:disabled) { background: rgba(255,45,85,0.25); }
  .btn-toggle-off { background: rgba(52,199,89,0.15); color: var(--accent-green, #34c759); border-color: rgba(52,199,89,0.3); }
  .btn-toggle-off:hover:not(:disabled) { background: rgba(52,199,89,0.25); }

  .modal-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.65); backdrop-filter: blur(4px); z-index: 100; display: flex; align-items: center; justify-content: center; padding: 24px; }
  .modal {
    background: var(--glass-bg, rgba(30,30,40,0.95));
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--glass-border, rgba(255,255,255,0.1));
    box-shadow: 0 8px 40px rgba(0,0,0,0.6);
    border-radius: 20px;
    padding: 28px;
    width: 100%;
    max-width: 400px;
  }
  .modal-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 12px; }
  .modal-title { font-size: 1.15rem; font-weight: 700; color: var(--color-text-primary, #f0f0f0); margin: 0; }
  .modal-close { background: none; border: none; color: var(--color-text-secondary, #888899); font-size: 1rem; cursor: pointer; padding: 4px 8px; border-radius: 6px; line-height: 1; transition: color 0.2s; }
  .modal-close:hover { color: var(--color-text-primary, #f0f0f0); }
  .modal-body-text { font-size: 0.875rem; color: var(--color-text-secondary, #888899); margin: 0 0 20px; line-height: 1.5; }
  .modal-form { display: flex; flex-direction: column; gap: 16px; }

  .sr-only { position: absolute; width: 1px; height: 1px; padding: 0; margin: -1px; overflow: hidden; clip: rect(0,0,0,0); white-space: nowrap; border: 0; }

  @media (max-width: 680px) {
    .page-header { flex-direction: column; }
    .form-row { grid-template-columns: 1fr; }
    .form-footer { flex-direction: column; align-items: flex-start; }
    .data-table th, .data-table td { padding: 10px 12px; }
  }
</style>
