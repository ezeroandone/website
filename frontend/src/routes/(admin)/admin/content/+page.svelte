<script lang="ts">
  /**
   * Content management — rich type-specific forms.
   * Work: featured image, category, tags, project type, technologies, team members
   * Insight: featured image, category, tags, media uploads/links, full markdown body
   * Capability: featured image, category, Material Icon picker
   */
  import type { PageProps } from './$types';
  import type { StaffAdmin } from '../staff/+page.server';

  let { data }: PageProps = $props();

  type PostType = 'insight' | 'work' | 'capability';

  interface Post {
    id: string; type: PostType; slug: string; title: string;
    summary: string; body_md: string; author_id: string | null;
    published_at: number | null; updated_at: number; published: boolean;
    featured_image_url: string; category: string; tags: string;
    project_type: string; technologies: string; material_icon: string;
  }

  interface TeamMember {
    id: string; post_id: string; staff_id: string | null;
    staff_name: string | null; staff_username: string | null; staff_avatar_url: string | null;
    ext_name: string; ext_role: string; ext_url: string; sort_order: number;
  }

  let posts = $state<Post[]>(data.posts as Post[]);
  let staffList = $state<StaffAdmin[]>(data.staff as StaffAdmin[]);
  const isSuperAdmin = $derived(data.session?.role === 'SuperAdmin');

  // ── Overlay state ─────────────────────────────────────────────────────────
  let overlay = $state<null | 'create' | string>(null);
  let statusMsg = $state<{ text: string; ok: boolean } | null>(null);

  function showStatus(text: string, ok: boolean) {
    statusMsg = { text, ok };
    setTimeout(() => (statusMsg = null), 4000);
  }

  // ── Helpers ───────────────────────────────────────────────────────────────
  const TYPE_LABEL: Record<PostType, string> = { insight: 'Insight', work: 'Work', capability: 'Capability' };
  const TYPE_ACCENT: Record<PostType, string> = {
    insight: 'var(--accent-blue)', work: 'var(--accent-green)', capability: 'var(--accent-yellow)',
  };

  function formatDate(unix: number | null): string {
    if (!unix) return '—';
    return new Date(unix * 1000).toLocaleDateString('en-GB', { day: '2-digit', month: 'short', year: 'numeric' });
  }

  function tagsArray(raw: string): string[] {
    return raw.split(',').map(t => t.trim()).filter(Boolean);
  }

  // ── Create form ───────────────────────────────────────────────────────────
  let cType = $state<PostType>('insight');
  let cTitle = $state(''); let cSummary = $state(''); let cBody = $state('');
  let cFeaturedUrl = $state(''); let cFeaturedFile = $state<File | null>(null);
  let cCategory = $state(''); let cTags = $state('');
  let cProjectType = $state(''); let cTechnologies = $state('');
  let cMaterialIcon = $state('star');
  let cBusy = $state(false); let createdPostId = $state<string | null>(null);

  // Team members for newly created work post
  let cTeam = $state<TeamMember[]>([]);
  let cTeamStaffId = $state(''); let cTeamExtName = $state('');
  let cTeamExtRole = $state(''); let cTeamExtUrl = $state('');
  let cTeamIsExternal = $state(false);

  async function handleCreate(e: SubmitEvent) {
    e.preventDefault(); cBusy = true;
    try {
      const res = await fetch('/api/admin/content', {
        method: 'POST', credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: cType, title: cTitle, summary: cSummary, body_md: cBody,
          author_id: data.session?.sub ?? null,
          featured_image_url: cFeaturedUrl, category: cCategory, tags: cTags,
          project_type: cProjectType, technologies: cTechnologies, material_icon: cMaterialIcon,
        }),
      });
      if (!res.ok) { showStatus(`Create failed: ${await res.text()}`, false); return; }
      const created: Post = await res.json();
      // Upload featured image file if chosen
      if (cFeaturedFile) await uploadCover(created.id, cFeaturedFile);
      posts = [created, ...posts];
      createdPostId = created.id;
      showStatus('Post created (draft). Add team members below, then close.', true);
      cTitle = ''; cSummary = ''; cBody = ''; cFeaturedUrl = ''; cFeaturedFile = null;
      cCategory = ''; cTags = ''; cProjectType = ''; cTechnologies = ''; cMaterialIcon = 'star';
    } catch { showStatus('Network error — post not created.', false); }
    finally { cBusy = false; }
  }

  async function uploadCover(postId: string, file: File) {
    const fd = new FormData(); fd.append('file', file);
    const r = await fetch(`/api/upload/post/${postId}/cover`, { method: 'POST', credentials: 'include', body: fd });
    if (r.ok) {
      const { url } = await r.json();
      await fetch(`/api/admin/content/${postId}`, {
        method: 'PATCH', credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ featured_image_url: url }),
      });
    }
  }

  async function addTeamMember(postId: string) {
    const body = cTeamIsExternal
      ? { ext_name: cTeamExtName, ext_role: cTeamExtRole, ext_url: cTeamExtUrl }
      : { staff_id: cTeamStaffId };
    const res = await fetch(`/api/admin/content/${postId}/team`, {
      method: 'POST', credentials: 'include',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body),
    });
    if (res.ok) {
      cTeamStaffId = ''; cTeamExtName = ''; cTeamExtRole = ''; cTeamExtUrl = '';
      await refreshTeam(postId);
    }
  }

  async function refreshTeam(postId: string) {
    const r = await fetch(`/api/admin/content/${postId}/team`, { credentials: 'include' });
    if (r.ok) cTeam = await r.json();
  }

  async function removeTeamMember(postId: string, memberId: string) {
    await fetch(`/api/admin/content/${postId}/team/${memberId}`, { method: 'DELETE', credentials: 'include' });
    await refreshTeam(postId);
  }
</script>

<script context="module" lang="ts">
  // Material Icons commonly used for capabilities
  export const CAPABILITY_ICONS = [
    'rocket_launch','bolt','hub','psychology','cloud','code','security',
    'analytics','palette','devices','settings','build','integration_instructions',
    'storage','speed','visibility','public','api','memory','architecture',
  ];
</script>

<svelte:head>
  <title>Content — eZeroAndOne Admin</title>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<!-- ── Page ──────────────────────────────────────────────────────────────── -->
<section class="content-page">
  <header class="page-header">
    <div>
      <h1 class="page-title">Content</h1>
      <p class="page-subtitle">Manage insights, work, and capabilities posts.</p>
    </div>
    <button class="btn btn-primary" onclick={() => { overlay = 'create'; createdPostId = null; cTeam = []; }} type="button">
      <span class="material-icons-outlined" aria-hidden="true">add</span> New post
    </button>
  </header>

  {#if statusMsg}
    <div class="status-banner" class:ok={statusMsg.ok} class:err={!statusMsg.ok} role="status">{statusMsg.text}</div>
  {/if}

  {#if posts.length === 0}
    <div class="empty-state"><p>No posts yet. Create one to get started.</p></div>
  {:else}
    <div class="post-list" role="list">
      {#each posts as post (post.id)}
        <article class="post-row glass-row" role="listitem">
          {#if post.featured_image_url}
            <div class="post-thumb" style="background-image: url('{post.featured_image_url}')"></div>
          {:else if post.type === 'capability' && post.material_icon}
            <div class="post-thumb post-thumb--icon">
              <span class="material-icons-outlined">{post.material_icon}</span>
            </div>
          {:else}
            <div class="post-thumb post-thumb--placeholder"></div>
          {/if}

          <div class="post-meta">
            <span class="type-badge" style="--c: {TYPE_ACCENT[post.type]};">{TYPE_LABEL[post.type]}</span>
            <span class="post-status" class:pub={post.published}>{post.published ? 'Published' : 'Draft'}</span>
            {#if post.category}<span class="cat-tag">{post.category}</span>{/if}
          </div>

          <div class="post-body">
            <h2 class="post-title">{post.title}</h2>
            {#if post.summary}<p class="post-summary">{post.summary}</p>{/if}
            {#if post.tags}
              <div class="tags-row">
                {#each tagsArray(post.tags) as tag}<span class="tag">{tag}</span>{/each}
              </div>
            {/if}
          </div>

          <dl class="post-dates">
            <div><dt>Published</dt><dd>{formatDate(post.published_at)}</dd></div>
            <div><dt>Updated</dt><dd>{formatDate(post.updated_at)}</dd></div>
          </dl>

          <div class="post-actions">
            <button class="btn btn-secondary btn-sm" type="button" onclick={() => openEdit(post)}>Edit</button>
            {#if isSuperAdmin}
              <button class="btn btn-danger btn-sm" type="button" onclick={() => handleDelete(post)}>Delete</button>
            {/if}
          </div>
        </article>
      {/each}
    </div>
  {/if}
</section>

<!-- ── Create overlay ──────────────────────────────────────────────────────── -->
{#if overlay === 'create'}
<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={() => { overlay = null; }}>
  <div class="modal glass-card" role="dialog" aria-modal="true" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2 class="modal-title">New post</h2>
      <button class="modal-close" type="button" onclick={() => (overlay = null)} aria-label="Close">✕</button>
    </div>

    {#if !createdPostId}
    <form class="modal-form" onsubmit={handleCreate}>
      <!-- Type selector -->
      <label class="field">
        <span class="field-label">Type</span>
        <select bind:value={cType} class="input" required>
          <option value="insight">Insight</option>
          <option value="work">Work</option>
          <option value="capability">Capability</option>
        </select>
      </label>

      <div class="form-row">
        <label class="field">
          <span class="field-label">Title</span>
          <input type="text" bind:value={cTitle} class="input" placeholder="Post title" required maxlength="200" />
        </label>
        <label class="field">
          <span class="field-label">Category</span>
          <input type="text" bind:value={cCategory} class="input" placeholder="e.g. Case Study" maxlength="100" />
        </label>
      </div>

      <label class="field">
        <span class="field-label">Summary</span>
        <input type="text" bind:value={cSummary} class="input" placeholder="Short excerpt" maxlength="500" />
      </label>

      <!-- Featured image -->
      <div class="field">
        <span class="field-label">Featured image</span>
        <div class="img-input-row">
          <input type="file" accept="image/*" class="input" onchange={(e) => { const t = e.currentTarget as HTMLInputElement; cFeaturedFile = t.files?.[0] ?? null; }} />
          <span class="img-or">or URL</span>
          <input type="url" bind:value={cFeaturedUrl} class="input" placeholder="https://…" />
        </div>
      </div>

      <label class="field">
        <span class="field-label">Tags <small>(comma-separated)</small></span>
        <input type="text" bind:value={cTags} class="input" placeholder="branding, strategy, design" />
      </label>

      <!-- Work-specific -->
      {#if cType === 'work'}
      <div class="form-row">
        <label class="field">
          <span class="field-label">Project type</span>
          <input type="text" bind:value={cProjectType} class="input" placeholder="e.g. Web App, Brand Identity" />
        </label>
        <label class="field">
          <span class="field-label">Technologies <small>(comma-separated)</small></span>
          <input type="text" bind:value={cTechnologies} class="input" placeholder="React, Rust, Cloudflare" />
        </label>
      </div>
      {/if}

      <!-- Capability-specific icon picker -->
      {#if cType === 'capability'}
      <div class="field">
        <span class="field-label">Material Icon</span>
        <div class="icon-grid">
          {#each CAPABILITY_ICONS as icon}
            <button type="button" class="icon-btn" class:selected={cMaterialIcon === icon}
              onclick={() => (cMaterialIcon = icon)} title={icon}>
              <span class="material-icons-outlined">{icon}</span>
            </button>
          {/each}
        </div>
        <input type="text" bind:value={cMaterialIcon} class="input" placeholder="or type any icon name" style="margin-top:8px" />
      </div>
      {/if}

      <label class="field">
        <span class="field-label">Body (Markdown)</span>
        <textarea bind:value={cBody} class="input textarea" placeholder="Write in Markdown…" rows="8"></textarea>
      </label>

      <div class="form-actions">
        <button type="button" class="btn btn-secondary" onclick={() => (overlay = null)}>Cancel</button>
        <button type="submit" class="btn btn-primary" disabled={cBusy}>{cBusy ? 'Creating…' : 'Create post'}</button>
      </div>
    </form>
    {:else}
    <!-- Post created — show team member section for work posts -->
    <div class="team-section">
      <p class="success-note">✓ Post created. {cType === 'work' ? 'Add team members below.' : 'Done!'}</p>
      {#if cType === 'work'}
      <h3 class="section-sub">Team members</h3>
      {#if cTeam.length > 0}
        <ul class="team-list">
          {#each cTeam as m (m.id)}
            <li class="team-item">
              {#if m.staff_id}
                <span>{m.staff_name ?? m.staff_username}</span>
              {:else}
                <span>{m.ext_name} — {m.ext_role}</span>
              {/if}
              <button class="btn btn-danger btn-sm" type="button" onclick={() => removeTeamMember(createdPostId!, m.id)}>Remove</button>
            </li>
          {/each}
        </ul>
      {/if}

      <div class="team-add-row">
        <label class="field-inline">
          <input type="checkbox" bind:checked={cTeamIsExternal} class="checkbox" />
          <span class="field-label">External contractor</span>
        </label>
      </div>
      {#if !cTeamIsExternal}
        <div class="form-row">
          <label class="field">
            <span class="field-label">Staff member</span>
            <select bind:value={cTeamStaffId} class="input">
              <option value="">— select staff —</option>
              {#each staffList as s}
                <option value={s.id}>{s.name || s.email}</option>
              {/each}
            </select>
          </label>
          <button class="btn btn-secondary" type="button" onclick={() => createdPostId && cTeamStaffId && addTeamMember(createdPostId)}>Add</button>
        </div>
      {:else}
        <div class="form-row">
          <label class="field"><span class="field-label">Name</span><input type="text" bind:value={cTeamExtName} class="input" /></label>
          <label class="field"><span class="field-label">Role</span><input type="text" bind:value={cTeamExtRole} class="input" /></label>
          <label class="field"><span class="field-label">URL</span><input type="url" bind:value={cTeamExtUrl} class="input" /></label>
          <button class="btn btn-secondary" type="button" onclick={() => createdPostId && addTeamMember(createdPostId)}>Add</button>
        </div>
      {/if}
      {/if}
      <div class="form-actions" style="margin-top:20px">
        <button class="btn btn-primary" type="button" onclick={() => (overlay = null)}>Done</button>
      </div>
    </div>
    {/if}
  </div>
</div>
{/if}

<script lang="ts">
  // Edit state
  let editPost = $state<Post | null>(null);
  let eTitle = $state(''); let eSummary = $state(''); let eBody = $state('');
  let ePublished = $state(false); let eFeaturedUrl = $state(''); let eFeaturedFile = $state<File | null>(null);
  let eCategory = $state(''); let eTags = $state('');
  let eProjectType = $state(''); let eTechnologies = $state('');
  let eMaterialIcon = $state('');
  let eBusy = $state(false);
  let eTeam = $state<TeamMember[]>([]);
  let eTeamStaffId = $state(''); let eTeamExtName = $state(''); let eTeamExtRole = $state('');
  let eTeamExtUrl = $state(''); let eTeamIsExternal = $state(false);
  let deletingId = $state<string | null>(null);

  async function openEdit(post: Post) {
    editPost = post; eTitle = post.title; eSummary = post.summary;
    eBody = post.body_md; ePublished = post.published; eFeaturedUrl = post.featured_image_url;
    eCategory = post.category; eTags = post.tags;
    eProjectType = post.project_type; eTechnologies = post.technologies;
    eMaterialIcon = post.material_icon || 'star';
    eFeaturedFile = null; eTeam = [];
    overlay = post.id;
    if (post.type === 'work') {
      const r = await fetch(`/api/admin/content/${post.id}/team`, { credentials: 'include' });
      if (r.ok) eTeam = await r.json();
    }
  }

  async function handleEdit(e: SubmitEvent) {
    e.preventDefault();
    if (!editPost) return;
    eBusy = true;
    try {
      const body: Record<string, unknown> = {
        title: eTitle, summary: eSummary, body_md: eBody, published: ePublished,
        featured_image_url: eFeaturedUrl, category: eCategory, tags: eTags,
        project_type: eProjectType, technologies: eTechnologies, material_icon: eMaterialIcon,
      };
      if (ePublished && !editPost.published) body.published_at = Math.floor(Date.now() / 1000);
      const res = await fetch(`/api/admin/content/${editPost.id}`, {
        method: 'PATCH', credentials: 'include',
        headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body),
      });
      if (!res.ok) { showStatus(`Update failed: ${await res.text()}`, false); return; }
      if (eFeaturedFile) await uploadCover(editPost.id, eFeaturedFile);
      const updated: Post = await res.json();
      posts = posts.map(p => p.id === updated.id ? updated : p);
      overlay = null; editPost = null;
      showStatus('Post updated.', true);
    } catch { showStatus('Network error.', false); }
    finally { eBusy = false; }
  }

  async function handleDelete(post: Post) {
    if (!confirm(`Delete "${post.title}"? Cannot be undone.`)) return;
    deletingId = post.id;
    try {
      const res = await fetch(`/api/admin/content/${post.id}`, { method: 'DELETE', credentials: 'include' });
      if (!res.ok) { showStatus(`Delete failed: ${await res.text()}`, false); return; }
      posts = posts.filter(p => p.id !== post.id);
      showStatus('Post deleted.', true);
    } catch { showStatus('Network error.', false); }
    finally { deletingId = null; }
  }

  async function addEditTeamMember(postId: string) {
    const body = eTeamIsExternal
      ? { ext_name: eTeamExtName, ext_role: eTeamExtRole, ext_url: eTeamExtUrl }
      : { staff_id: eTeamStaffId };
    const res = await fetch(`/api/admin/content/${postId}/team`, {
      method: 'POST', credentials: 'include',
      headers: { 'Content-Type': 'application/json' }, body: JSON.stringify(body),
    });
    if (res.ok) {
      eTeamStaffId = ''; eTeamExtName = ''; eTeamExtRole = ''; eTeamExtUrl = '';
      const r = await fetch(`/api/admin/content/${postId}/team`, { credentials: 'include' });
      if (r.ok) eTeam = await r.json();
    }
  }

  async function removeEditTeamMember(postId: string, memberId: string) {
    await fetch(`/api/admin/content/${postId}/team/${memberId}`, { method: 'DELETE', credentials: 'include' });
    const r = await fetch(`/api/admin/content/${postId}/team`, { credentials: 'include' });
    if (r.ok) eTeam = await r.json();
  }
</script>

<!-- ── Edit overlay ────────────────────────────────────────────────────────── -->
{#if overlay !== null && overlay !== 'create' && editPost !== null}
<!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
<div class="modal-backdrop" onclick={() => (overlay = null)}>
  <div class="modal glass-card" role="dialog" aria-modal="true" onclick={(e) => e.stopPropagation()}>
    <div class="modal-header">
      <h2 class="modal-title">Edit — {editPost.title}</h2>
      <button class="modal-close" type="button" onclick={() => (overlay = null)} aria-label="Close">✕</button>
    </div>
    <form class="modal-form" onsubmit={handleEdit}>
      <div class="form-row">
        <label class="field">
          <span class="field-label">Title</span>
          <input type="text" bind:value={eTitle} class="input" required maxlength="200" />
        </label>
        <label class="field">
          <span class="field-label">Category</span>
          <input type="text" bind:value={eCategory} class="input" maxlength="100" />
        </label>
      </div>
      <label class="field">
        <span class="field-label">Summary</span>
        <input type="text" bind:value={eSummary} class="input" maxlength="500" />
      </label>
      <div class="field">
        <span class="field-label">Featured image</span>
        <div class="img-input-row">
          <input type="file" accept="image/*" class="input" onchange={(e) => { const t = e.currentTarget as HTMLInputElement; eFeaturedFile = t.files?.[0] ?? null; }} />
          <span class="img-or">or URL</span>
          <input type="url" bind:value={eFeaturedUrl} class="input" placeholder="https://…" />
        </div>
        {#if eFeaturedUrl}<img src={eFeaturedUrl} alt="preview" class="img-preview" />{/if}
      </div>
      <label class="field">
        <span class="field-label">Tags</span>
        <input type="text" bind:value={eTags} class="input" placeholder="comma-separated" />
      </label>
      {#if editPost.type === 'work'}
      <div class="form-row">
        <label class="field"><span class="field-label">Project type</span><input type="text" bind:value={eProjectType} class="input" /></label>
        <label class="field"><span class="field-label">Technologies</span><input type="text" bind:value={eTechnologies} class="input" /></label>
      </div>
      <!-- Team members -->
      <div class="field">
        <span class="field-label">Team members</span>
        {#if eTeam.length > 0}
          <ul class="team-list">
            {#each eTeam as m (m.id)}
              <li class="team-item">
                <span>{m.staff_id ? (m.staff_name ?? m.staff_username) : m.ext_name + (m.ext_role ? ' — ' + m.ext_role : '')}</span>
                <button class="btn btn-danger btn-sm" type="button" onclick={() => removeEditTeamMember(editPost!.id, m.id)}>Remove</button>
              </li>
            {/each}
          </ul>
        {/if}
        <label class="field-inline" style="margin-top:8px">
          <input type="checkbox" bind:checked={eTeamIsExternal} class="checkbox" />
          <span class="field-label">External contractor</span>
        </label>
        {#if !eTeamIsExternal}
          <div class="form-row" style="margin-top:6px">
            <select bind:value={eTeamStaffId} class="input">
              <option value="">— select staff —</option>
              {#each staffList as s}<option value={s.id}>{s.name || s.email}</option>{/each}
            </select>
            <button class="btn btn-secondary btn-sm" type="button" onclick={() => addEditTeamMember(editPost!.id)}>Add</button>
          </div>
        {:else}
          <div class="form-row" style="margin-top:6px">
            <input type="text" bind:value={eTeamExtName} class="input" placeholder="Name" />
            <input type="text" bind:value={eTeamExtRole} class="input" placeholder="Role" />
            <input type="url" bind:value={eTeamExtUrl} class="input" placeholder="URL" />
            <button class="btn btn-secondary btn-sm" type="button" onclick={() => addEditTeamMember(editPost!.id)}>Add</button>
          </div>
        {/if}
      </div>
      {/if}

      {#if editPost.type === 'capability'}
      <div class="field">
        <span class="field-label">Material Icon</span>
        <div class="icon-grid">
          {#each CAPABILITY_ICONS as icon}
            <button type="button" class="icon-btn" class:selected={eMaterialIcon === icon} onclick={() => (eMaterialIcon = icon)} title={icon}>
              <span class="material-icons-outlined">{icon}</span>
            </button>
          {/each}
        </div>
        <input type="text" bind:value={eMaterialIcon} class="input" style="margin-top:8px" />
      </div>
      {/if}

      <label class="field">
        <span class="field-label">Body (Markdown)</span>
        <textarea bind:value={eBody} class="input textarea" rows="10"></textarea>
      </label>

      <label class="field field-inline">
        <input type="checkbox" bind:checked={ePublished} class="checkbox" />
        <span class="field-label">Published</span>
      </label>

      <div class="form-actions">
        <button type="button" class="btn btn-secondary" onclick={() => (overlay = null)}>Cancel</button>
        <button type="submit" class="btn btn-primary" disabled={eBusy}>{eBusy ? 'Saving…' : 'Save changes'}</button>
      </div>
    </form>
  </div>
</div>
{/if}

<style>
  .content-page { max-width: 1000px; }
  .page-header { display: flex; align-items: flex-start; justify-content: space-between; gap: 16px; margin-bottom: 28px; flex-wrap: wrap; }
  .page-title { font-size: 2rem; font-weight: 700; color: var(--color-text-primary,#f0f0f0); margin: 0 0 4px; letter-spacing: -0.5px; }
  .page-subtitle { font-size: 0.9rem; color: var(--color-text-secondary,#888899); margin: 0; }
  .status-banner { padding: 10px 16px; border-radius: 10px; font-size: 0.875rem; font-weight: 500; margin-bottom: 20px; border: 1px solid transparent; }
  .ok { background: color-mix(in srgb, var(--accent-green,#00ff88) 12%, transparent); border-color: color-mix(in srgb, var(--accent-green,#00ff88) 30%, transparent); color: var(--accent-green,#00ff88); }
  .err { background: color-mix(in srgb, var(--accent-red,#ff3366) 12%, transparent); border-color: color-mix(in srgb, var(--accent-red,#ff3366) 30%, transparent); color: var(--accent-red,#ff3366); }
  .empty-state { padding: 48px 0; text-align: center; color: var(--color-text-secondary,#888899); }

  .post-list { display: flex; flex-direction: column; gap: 10px; }
  .glass-row {
    background: var(--glass-bg, rgba(255,255,255,0.04));
    border: 1px solid var(--glass-border, rgba(255,255,255,0.08));
    border-radius: 14px;
  }
  .post-row { display: grid; grid-template-columns: 64px auto 1fr auto auto; align-items: center; gap: 14px; padding: 12px 16px; transition: border-color 0.2s; }
  .post-row:hover { border-color: var(--glass-border-strong, rgba(255,255,255,0.16)); }

  .post-thumb { width: 64px; height: 64px; border-radius: 10px; background-size: cover; background-position: center; flex-shrink: 0; }
  .post-thumb--placeholder { background: var(--bg-elevated, rgba(255,255,255,0.06)); }
  .post-thumb--icon { background: var(--bg-elevated, rgba(255,255,255,0.06)); display: flex; align-items: center; justify-content: center; }
  .post-thumb--icon .material-icons-outlined { font-size: 2rem; color: var(--accent-yellow,#ffd60a); }

  .post-meta { display: flex; flex-direction: column; align-items: flex-start; gap: 5px; min-width: 90px; }
  .type-badge { padding: 2px 8px; border-radius: 6px; font-size: 0.68rem; font-weight: 600; letter-spacing: 0.4px; text-transform: uppercase; background: color-mix(in srgb, var(--c) 18%, transparent); border: 1px solid color-mix(in srgb, var(--c) 40%, transparent); color: var(--c); }
  .post-status { font-size: 0.7rem; font-weight: 500; color: var(--color-text-muted, rgba(255,255,255,0.32)); }
  .post-status.pub { color: var(--accent-green,#00ff88); }
  .cat-tag { font-size: 0.68rem; color: var(--color-text-secondary,#888899); background: rgba(255,255,255,0.06); padding: 1px 6px; border-radius: 4px; }

  .post-body { min-width: 0; }
  .post-title { font-size: 0.95rem; font-weight: 600; color: var(--color-text-primary,#f0f0f0); margin: 0 0 3px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .post-summary { font-size: 0.78rem; color: var(--color-text-secondary,#888899); margin: 0 0 4px; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .tags-row { display: flex; flex-wrap: wrap; gap: 4px; }
  .tag { font-size: 0.65rem; background: rgba(255,255,255,0.07); color: var(--color-text-secondary,#888899); padding: 1px 6px; border-radius: 4px; }

  .post-dates { display: flex; flex-direction: column; gap: 3px; margin: 0; min-width: 96px; }
  .post-dates div { display: flex; gap: 5px; }
  .post-dates dt { font-size: 0.68rem; color: var(--color-text-muted, rgba(255,255,255,0.32)); min-width: 52px; }
  .post-dates dd { font-size: 0.72rem; color: var(--color-text-secondary,#888899); margin: 0; }
  .post-actions { display: flex; gap: 8px; align-items: center; flex-shrink: 0; }

  /* Buttons */
  .btn { display: inline-flex; align-items: center; gap: 5px; padding: 9px 18px; border-radius: 10px; font-size: 0.875rem; font-weight: 600; border: none; cursor: pointer; transition: all 0.2s; white-space: nowrap; }
  .btn:disabled { opacity: 0.5; cursor: not-allowed; }
  .btn-sm { padding: 6px 12px; font-size: 0.78rem; border-radius: 8px; }
  .btn-primary { background: var(--accent-blue,#00d4ff); color: #000; }
  .btn-primary:hover:not(:disabled) { filter: brightness(1.1); }
  .btn-secondary { background: rgba(255,255,255,0.08); color: var(--color-text-primary,#f0f0f0); border: 1px solid rgba(255,255,255,0.1); }
  .btn-secondary:hover:not(:disabled) { background: rgba(255,255,255,0.13); }
  .btn-danger { background: color-mix(in srgb, var(--accent-red,#ff3366) 18%, transparent); color: var(--accent-red,#ff3366); border: 1px solid color-mix(in srgb, var(--accent-red,#ff3366) 35%, transparent); }

  /* Modal */
  .modal-backdrop { position: fixed; inset: 0; background: rgba(0,0,0,0.65); backdrop-filter: blur(4px); z-index: 100; display: flex; align-items: center; justify-content: center; padding: 24px; }
  .modal { width: 100%; max-width: 680px; max-height: calc(100vh - 48px); overflow-y: auto; padding: 28px; border-radius: 20px; }
  .glass-card { background: var(--glass-bg, rgba(20,20,30,0.96)); backdrop-filter: blur(16px); border: 1px solid var(--glass-border, rgba(255,255,255,0.1)); box-shadow: 0 8px 40px rgba(0,0,0,0.5); }
  .modal-header { display: flex; align-items: center; justify-content: space-between; margin-bottom: 22px; }
  .modal-title { font-size: 1.2rem; font-weight: 700; color: var(--color-text-primary,#f0f0f0); margin: 0; white-space: nowrap; overflow: hidden; text-overflow: ellipsis; }
  .modal-close { background: none; border: none; color: var(--color-text-secondary,#888899); font-size: 1rem; cursor: pointer; padding: 4px 8px; border-radius: 6px; }

  /* Form */
  .modal-form { display: flex; flex-direction: column; gap: 14px; }
  .form-row { display: grid; grid-template-columns: repeat(auto-fit, minmax(180px, 1fr)); gap: 12px; align-items: end; }
  .field { display: flex; flex-direction: column; gap: 5px; }
  .field-inline { flex-direction: row; align-items: center; gap: 10px; }
  .field-label { font-size: 0.75rem; font-weight: 600; color: var(--color-text-secondary,#888899); text-transform: uppercase; letter-spacing: 0.05em; }
  .field-label small { text-transform: none; font-weight: 400; }
  .input { background: rgba(255,255,255,0.06); border: 1px solid rgba(255,255,255,0.1); border-radius: 10px; padding: 9px 13px; color: var(--color-text-primary,#f0f0f0); font-size: 0.875rem; font-family: inherit; width: 100%; box-sizing: border-box; transition: border-color 0.2s; -webkit-appearance: none; appearance: none; }
  .input:focus { outline: none; border-color: var(--accent-blue,#00d4ff); box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent-blue,#00d4ff) 20%, transparent); }
  .textarea { resize: vertical; min-height: 140px; font-family: monospace; font-size: 0.83rem; line-height: 1.6; }
  .checkbox { width: 16px; height: 16px; accent-color: var(--accent-blue,#00d4ff); cursor: pointer; flex-shrink: 0; }
  .form-actions { display: flex; justify-content: flex-end; gap: 10px; padding-top: 8px; border-top: 1px solid rgba(255,255,255,0.08); margin-top: 4px; }

  .img-input-row { display: flex; align-items: center; gap: 10px; flex-wrap: wrap; }
  .img-input-row .input { flex: 1; min-width: 160px; }
  .img-or { font-size: 0.75rem; color: var(--color-text-muted, rgba(255,255,255,0.3)); flex-shrink: 0; }
  .img-preview { width: 100%; max-height: 120px; object-fit: cover; border-radius: 8px; margin-top: 6px; }

  /* Icon grid */
  .icon-grid { display: flex; flex-wrap: wrap; gap: 6px; margin-bottom: 4px; }
  .icon-btn { background: rgba(255,255,255,0.06); border: 1px solid rgba(255,255,255,0.1); border-radius: 8px; padding: 8px; cursor: pointer; transition: all 0.15s; color: var(--color-text-secondary,#888899); }
  .icon-btn:hover { background: rgba(255,255,255,0.12); color: var(--color-text-primary,#f0f0f0); }
  .icon-btn.selected { background: color-mix(in srgb, var(--accent-blue,#00d4ff) 20%, transparent); border-color: var(--accent-blue,#00d4ff); color: var(--accent-blue,#00d4ff); }

  /* Team section */
  .team-section { display: flex; flex-direction: column; gap: 14px; }
  .success-note { color: var(--accent-green,#00ff88); font-size: 0.9rem; margin: 0; }
  .section-sub { font-size: 0.95rem; font-weight: 700; color: var(--color-text-primary,#f0f0f0); margin: 0; }
  .team-list { list-style: none; padding: 0; margin: 0; display: flex; flex-direction: column; gap: 6px; }
  .team-item { display: flex; align-items: center; justify-content: space-between; padding: 8px 12px; background: rgba(255,255,255,0.05); border-radius: 8px; font-size: 0.85rem; color: var(--color-text-primary,#f0f0f0); }
  .team-add-row { display: flex; align-items: center; gap: 10px; }

  @media (max-width: 700px) {
    .post-row { grid-template-columns: 1fr; }
    .post-dates { flex-direction: row; flex-wrap: wrap; gap: 10px; }
    .post-actions { justify-content: flex-end; }
  }
</style>
