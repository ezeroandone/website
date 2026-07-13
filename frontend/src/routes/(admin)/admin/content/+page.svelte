<script lang="ts">
  /**
   * Admin content management page
   *
   * Renders the post list and exposes three actions:
   *   • Create  — POST /api/admin/content  (Admin+)
   *   • Edit    — PATCH /api/admin/content/:id  (Admin+)
   *   • Delete  — DELETE /api/admin/content/:id  (SuperAdmin only)
   *
   * Requirements: 10.5, 10.6, 10.7, 15.4
   */

  import type { Post } from '$lib/api';
  import type { PageProps } from './$types';

  let { data }: PageProps = $props();

  // ── Derived state ──────────────────────────────────────────────────────────
  /** Posts list, kept reactive so optimistic updates work without re-running load */
  let posts = $state<Post[]>(data.posts);

  /** Whether the logged-in user is SuperAdmin — controls Delete button visibility */
  const isSuperAdmin = $derived(data.session?.role === 'SuperAdmin');

  // ── UI state ───────────────────────────────────────────────────────────────
  type PostType = 'insight' | 'work' | 'capability';

  /** Which overlay is open: null | 'create' | post id (edit) */
  let overlay = $state<null | 'create' | string>(null);

  /** Global status message shown below the header */
  let statusMsg = $state<{ text: string; ok: boolean } | null>(null);

  function showStatus(text: string, ok: boolean) {
    statusMsg = { text, ok };
    setTimeout(() => (statusMsg = null), 4000);
  }

  // ── Create form fields ─────────────────────────────────────────────────────
  let createType = $state<PostType>('insight');
  let createTitle = $state('');
  let createSummary = $state('');
  let createBody = $state('');
  let createBusy = $state(false);

  async function handleCreate(e: SubmitEvent) {
    e.preventDefault();
    createBusy = true;
    try {
      const res = await fetch('/api/admin/content', {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          type: createType,
          title: createTitle,
          summary: createSummary,
          body_md: createBody,
          author_id: data.session?.sub ?? null,
        }),
      });
      if (!res.ok) {
        const msg = await res.text();
        showStatus(`Create failed: ${msg}`, false);
        return;
      }
      const created: Post = await res.json();
      posts = [created, ...posts];
      // Reset form
      createTitle = '';
      createSummary = '';
      createBody = '';
      createType = 'insight';
      overlay = null;
      showStatus('Post created (unpublished).', true);
    } catch (err) {
      showStatus('Network error — post not created.', false);
    } finally {
      createBusy = false;
    }
  }

  // ── Edit form fields ───────────────────────────────────────────────────────
  let editPost = $state<Post | null>(null);
  let editTitle = $state('');
  let editSummary = $state('');
  let editBody = $state('');
  let editPublished = $state(false);
  let editBusy = $state(false);

  function openEdit(post: Post) {
    editPost = post;
    editTitle = post.title;
    editSummary = post.summary;
    editBody = post.body_md;
    editPublished = post.published;
    overlay = post.id;
  }

  async function handleEdit(e: SubmitEvent) {
    e.preventDefault();
    if (!editPost) return;
    editBusy = true;
    try {
      const body: Record<string, unknown> = {
        title: editTitle,
        summary: editSummary,
        body_md: editBody,
        published: editPublished,
      };
      if (editPublished && !editPost.published) {
        // First time publishing — set published_at to now
        body.published_at = Math.floor(Date.now() / 1000);
      }
      const res = await fetch(`/api/admin/content/${editPost.id}`, {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(body),
      });
      if (!res.ok) {
        const msg = await res.text();
        showStatus(`Update failed: ${msg}`, false);
        return;
      }
      const updated: Post = await res.json();
      posts = posts.map((p) => (p.id === updated.id ? updated : p));
      overlay = null;
      editPost = null;
      showStatus('Post updated.', true);
    } catch (err) {
      showStatus('Network error — post not updated.', false);
    } finally {
      editBusy = false;
    }
  }

  // ── Delete ─────────────────────────────────────────────────────────────────
  let deletingId = $state<string | null>(null);

  async function handleDelete(post: Post) {
    if (!confirm(`Delete "${post.title}"? This action cannot be undone.`)) return;
    deletingId = post.id;
    try {
      const res = await fetch(`/api/admin/content/${post.id}`, {
        method: 'DELETE',
        credentials: 'include',
      });
      if (!res.ok) {
        const msg = await res.text();
        showStatus(`Delete failed: ${msg}`, false);
        return;
      }
      posts = posts.filter((p) => p.id !== post.id);
      showStatus('Post deleted.', true);
    } catch (err) {
      showStatus('Network error — post not deleted.', false);
    } finally {
      deletingId = null;
    }
  }

  // ── Helpers ────────────────────────────────────────────────────────────────
  const TYPE_LABEL: Record<PostType, string> = {
    insight: 'Insight',
    work: 'Work',
    capability: 'Capability',
  };

  const TYPE_ACCENT: Record<PostType, string> = {
    insight: 'var(--accent-blue)',
    work: 'var(--accent-green)',
    capability: 'var(--accent-yellow)',
  };

  function formatDate(unix: number | null): string {
    if (!unix) return '—';
    return new Date(unix * 1000).toLocaleDateString('en-GB', {
      day: '2-digit',
      month: 'short',
      year: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Content — eZeroAndOne Admin</title>
</svelte:head>

<!-- ── Page shell ──────────────────────────────────────────────────────────── -->
<section class="content-page">
  <header class="page-header">
    <div class="header-text">
      <h1 class="page-title">Content</h1>
      <p class="page-subtitle">Manage insights, work, and capabilities posts.</p>
    </div>
    <button class="btn btn-primary" onclick={() => (overlay = 'create')} type="button">
      + New post
    </button>
  </header>

  <!-- Status banner -->
  {#if statusMsg}
    <div class="status-banner" class:status-ok={statusMsg.ok} class:status-err={!statusMsg.ok} role="status">
      {statusMsg.text}
    </div>
  {/if}

  <!-- ── Post list ─────────────────────────────────────────────────────────── -->
  {#if posts.length === 0}
    <div class="empty-state">
      <p>No published posts yet. Create one to get started.</p>
    </div>
  {:else}
    <div class="post-list" role="list">
      {#each posts as post (post.id)}
        <article class="post-row glass-row" role="listitem">
          <div class="post-meta">
            <span
              class="type-badge"
              style="--badge-color: {TYPE_ACCENT[post.type as PostType]};"
            >
              {TYPE_LABEL[post.type as PostType]}
            </span>
            <span class="post-status" class:published={post.published} class:draft={!post.published}>
              {post.published ? 'Published' : 'Draft'}
            </span>
          </div>

          <div class="post-body">
            <h2 class="post-title">{post.title}</h2>
            {#if post.summary}
              <p class="post-summary">{post.summary}</p>
            {/if}
          </div>

          <dl class="post-dates">
            <div>
              <dt>Published</dt>
              <dd>{formatDate(post.published_at)}</dd>
            </div>
            <div>
              <dt>Updated</dt>
              <dd>{formatDate(post.updated_at)}</dd>
            </div>
          </dl>

          <div class="post-actions">
            <button
              class="btn btn-secondary btn-sm"
              type="button"
              onclick={() => openEdit(post)}
            >
              Edit
            </button>

            {#if isSuperAdmin}
              <button
                class="btn btn-danger btn-sm"
                type="button"
                disabled={deletingId === post.id}
                onclick={() => handleDelete(post)}
                aria-label="Delete {post.title}"
              >
                {deletingId === post.id ? 'Deleting…' : 'Delete'}
              </button>
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
  <div class="modal-backdrop" onclick={() => (overlay = null)}>
    <div class="modal glass-card" role="dialog" aria-modal="true" aria-labelledby="create-title">
      <!-- Stop click propagation so the modal itself doesn't close the overlay -->
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2 id="create-title" class="modal-title">New post</h2>
          <button class="modal-close" type="button" onclick={() => (overlay = null)} aria-label="Close">✕</button>
        </div>

        <form class="modal-form" onsubmit={handleCreate}>
          <label class="field">
            <span class="field-label">Type</span>
            <select bind:value={createType} class="input" required>
              <option value="insight">Insight</option>
              <option value="work">Work</option>
              <option value="capability">Capability</option>
            </select>
          </label>

          <label class="field">
            <span class="field-label">Title</span>
            <input
              type="text"
              bind:value={createTitle}
              class="input"
              placeholder="Post title"
              required
              maxlength="200"
            />
          </label>

          <label class="field">
            <span class="field-label">Summary</span>
            <input
              type="text"
              bind:value={createSummary}
              class="input"
              placeholder="Short summary (optional)"
              maxlength="500"
            />
          </label>

          <label class="field">
            <span class="field-label">Body (Markdown)</span>
            <textarea
              bind:value={createBody}
              class="input textarea"
              placeholder="Write your post in Markdown…"
              rows="8"
            ></textarea>
          </label>

          <div class="form-actions">
            <button type="button" class="btn btn-secondary" onclick={() => (overlay = null)}>
              Cancel
            </button>
            <button type="submit" class="btn btn-primary" disabled={createBusy}>
              {createBusy ? 'Creating…' : 'Create post'}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}

<!-- ── Edit overlay ────────────────────────────────────────────────────────── -->
{#if overlay !== null && overlay !== 'create' && editPost !== null}
  <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
  <div class="modal-backdrop" onclick={() => (overlay = null)}>
    <div class="modal glass-card" role="dialog" aria-modal="true" aria-labelledby="edit-title">
      <!-- svelte-ignore a11y_click_events_have_key_events a11y_no_static_element_interactions -->
      <div onclick={(e) => e.stopPropagation()}>
        <div class="modal-header">
          <h2 id="edit-title" class="modal-title">Edit post</h2>
          <button class="modal-close" type="button" onclick={() => (overlay = null)} aria-label="Close">✕</button>
        </div>

        <form class="modal-form" onsubmit={handleEdit}>
          <label class="field">
            <span class="field-label">Title</span>
            <input
              type="text"
              bind:value={editTitle}
              class="input"
              placeholder="Post title"
              required
              maxlength="200"
            />
          </label>

          <label class="field">
            <span class="field-label">Summary</span>
            <input
              type="text"
              bind:value={editSummary}
              class="input"
              placeholder="Short summary (optional)"
              maxlength="500"
            />
          </label>

          <label class="field">
            <span class="field-label">Body (Markdown)</span>
            <textarea
              bind:value={editBody}
              class="input textarea"
              placeholder="Write your post in Markdown…"
              rows="8"
            ></textarea>
          </label>

          <label class="field field-inline">
            <input type="checkbox" bind:checked={editPublished} class="checkbox" />
            <span class="field-label">Published</span>
          </label>

          <div class="form-actions">
            <button type="button" class="btn btn-secondary" onclick={() => (overlay = null)}>
              Cancel
            </button>
            <button type="submit" class="btn btn-primary" disabled={editBusy}>
              {editBusy ? 'Saving…' : 'Save changes'}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
{/if}

<style>
  /* ── Page layout ─────────────────────────────────────────── */
  .content-page {
    max-width: 960px;
  }

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
    font-size: 0.9rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
  }

  /* ── Status banner ───────────────────────────────────────── */
  .status-banner {
    padding: 10px 16px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 500;
    margin-bottom: 20px;
    border: 1px solid transparent;
  }

  .status-ok {
    background: color-mix(in srgb, var(--accent-green, #00ff88) 12%, transparent);
    border-color: color-mix(in srgb, var(--accent-green, #00ff88) 30%, transparent);
    color: var(--accent-green, #00ff88);
  }

  .status-err {
    background: color-mix(in srgb, var(--accent-red, #ff3366) 12%, transparent);
    border-color: color-mix(in srgb, var(--accent-red, #ff3366) 30%, transparent);
    color: var(--accent-red, #ff3366);
  }

  /* ── Empty state ─────────────────────────────────────────── */
  .empty-state {
    padding: 48px 0;
    text-align: center;
    color: var(--color-text-secondary, #888899);
    font-size: 0.95rem;
  }

  /* ── Post list ───────────────────────────────────────────── */
  .post-list {
    display: flex;
    flex-direction: column;
    gap: 12px;
  }

  .glass-row {
    background: var(--glass-bg, rgba(255, 255, 255, 0.04));
    backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    -webkit-backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
    border-radius: 14px;
    box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.4));
  }

  .post-row {
    display: grid;
    grid-template-columns: auto 1fr auto auto;
    align-items: center;
    gap: 16px;
    padding: 16px 20px;
    transition: border-color 0.2s ease;
  }

  .post-row:hover {
    border-color: var(--glass-border-strong, rgba(255, 255, 255, 0.16));
  }

  /* ── Post meta (type badge + status) ─────────────────────── */
  .post-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-start;
    gap: 6px;
    min-width: 88px;
  }

  .type-badge {
    display: inline-block;
    padding: 2px 8px;
    border-radius: 6px;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.5px;
    text-transform: uppercase;
    background: color-mix(in srgb, var(--badge-color) 18%, transparent);
    border: 1px solid color-mix(in srgb, var(--badge-color) 40%, transparent);
    color: var(--badge-color);
  }

  .post-status {
    font-size: 0.72rem;
    font-weight: 500;
    letter-spacing: 0.4px;
  }

  .post-status.published {
    color: var(--accent-green, #00ff88);
  }

  .post-status.draft {
    color: var(--color-text-muted, rgba(255, 255, 255, 0.32));
  }

  /* ── Post body ───────────────────────────────────────────── */
  .post-body {
    min-width: 0;
  }

  .post-title {
    font-size: 1rem;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 4px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .post-summary {
    font-size: 0.8rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  /* ── Post dates ──────────────────────────────────────────── */
  .post-dates {
    display: flex;
    flex-direction: column;
    gap: 4px;
    margin: 0;
    min-width: 100px;
  }

  .post-dates div {
    display: flex;
    gap: 6px;
    align-items: baseline;
  }

  .post-dates dt {
    font-size: 0.7rem;
    color: var(--color-text-muted, rgba(255, 255, 255, 0.32));
    min-width: 54px;
  }

  .post-dates dd {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
  }

  /* ── Post actions ────────────────────────────────────────── */
  .post-actions {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-shrink: 0;
  }

  /* ── Buttons ─────────────────────────────────────────────── */
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 9px 18px;
    border-radius: 10px;
    font-size: 0.875rem;
    font-weight: 600;
    border: none;
    cursor: pointer;
    transition: all 0.2s ease;
    white-space: nowrap;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-sm {
    padding: 6px 12px;
    font-size: 0.8rem;
    border-radius: 8px;
  }

  .btn-primary {
    background: var(--accent-blue, #00d4ff);
    color: #000;
  }

  .btn-primary:hover:not(:disabled) {
    box-shadow: var(--glow-blue, 0 0 20px rgba(0, 122, 255, 0.4));
    filter: brightness(1.1);
  }

  .btn-secondary {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.08));
    color: var(--color-text-primary, #f0f0f0);
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
  }

  .btn-secondary:hover:not(:disabled) {
    background: var(--glass-border-strong, rgba(255, 255, 255, 0.14));
  }

  .btn-danger {
    background: color-mix(in srgb, var(--accent-red, #ff3366) 20%, transparent);
    color: var(--accent-red, #ff3366);
    border: 1px solid color-mix(in srgb, var(--accent-red, #ff3366) 35%, transparent);
  }

  .btn-danger:hover:not(:disabled) {
    background: color-mix(in srgb, var(--accent-red, #ff3366) 30%, transparent);
    box-shadow: var(--glow-red, 0 0 20px rgba(255, 45, 85, 0.3));
  }

  /* ── Modal overlay ───────────────────────────────────────── */
  .modal-backdrop {
    position: fixed;
    inset: 0;
    background: rgba(0, 0, 0, 0.65);
    backdrop-filter: blur(4px);
    z-index: 100;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 24px;
  }

  .modal {
    width: 100%;
    max-width: 600px;
    max-height: calc(100vh - 48px);
    overflow-y: auto;
    padding: 28px;
    border-radius: 20px;
  }

  /* glass-card override — remove the hover scale on modals */
  .modal:hover {
    transform: none;
  }

  .glass-card {
    background: var(--glass-bg, rgba(255, 255, 255, 0.04));
    backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    -webkit-backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
    box-shadow: var(--glass-shadow, 0 8px 32px rgba(0, 0, 0, 0.4));
  }

  .modal-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 24px;
  }

  .modal-title {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0;
  }

  .modal-close {
    background: none;
    border: none;
    color: var(--color-text-secondary, #888899);
    font-size: 1rem;
    cursor: pointer;
    padding: 4px 8px;
    border-radius: 6px;
    transition: color 0.2s ease;
    line-height: 1;
  }

  .modal-close:hover {
    color: var(--color-text-primary, #f0f0f0);
  }

  /* ── Form fields ─────────────────────────────────────────── */
  .modal-form {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }

  .field {
    display: flex;
    flex-direction: column;
    gap: 6px;
  }

  .field-inline {
    flex-direction: row;
    align-items: center;
    gap: 10px;
  }

  .field-label {
    font-size: 0.8rem;
    font-weight: 600;
    color: var(--color-text-secondary, #888899);
    text-transform: uppercase;
    letter-spacing: 0.5px;
  }

  .input {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.06));
    border: 1px solid var(--glass-border, rgba(255, 255, 255, 0.1));
    border-radius: 10px;
    padding: 10px 14px;
    color: var(--color-text-primary, #f0f0f0);
    font-size: 0.9rem;
    font-family: inherit;
    width: 100%;
    box-sizing: border-box;
    transition: border-color 0.2s ease;
    -webkit-appearance: none;
    appearance: none;
  }

  .input:focus {
    outline: none;
    border-color: var(--accent-blue, #00d4ff);
    box-shadow: 0 0 0 3px color-mix(in srgb, var(--accent-blue, #00d4ff) 20%, transparent);
  }

  .textarea {
    resize: vertical;
    min-height: 140px;
    font-family: 'SFMono-Regular', Consolas, 'Liberation Mono', Menlo, monospace;
    font-size: 0.85rem;
    line-height: 1.6;
  }

  .checkbox {
    width: 16px;
    height: 16px;
    accent-color: var(--accent-blue, #00d4ff);
    cursor: pointer;
    flex-shrink: 0;
  }

  .form-actions {
    display: flex;
    justify-content: flex-end;
    gap: 10px;
    padding-top: 8px;
    border-top: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
    margin-top: 8px;
  }

  /* ── Responsive ──────────────────────────────────────────── */
  @media (max-width: 640px) {
    .post-row {
      grid-template-columns: 1fr;
      gap: 12px;
    }

    .post-dates {
      flex-direction: row;
      flex-wrap: wrap;
      gap: 10px;
    }

    .post-actions {
      justify-content: flex-end;
    }

    .page-header {
      flex-direction: column;
    }
  }
</style>
