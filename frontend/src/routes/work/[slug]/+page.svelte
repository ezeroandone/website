<script lang="ts">
  import { renderMarkdown } from '$lib/utils/markdown';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  const post = $derived(data.post);
  const team = $derived(data.team ?? []);
  const bodyHtml = $derived(post.body_md?.trim() ? renderMarkdown(post.body_md) : '');

  const tags = $derived(
    post.tags ? post.tags.split(',').map((t: string) => t.trim()).filter(Boolean) : []
  );
  const techs = $derived(
    post.technologies ? post.technologies.split(',').map((t: string) => t.trim()).filter(Boolean) : []
  );

  function formatDate(unix: number | null): string {
    if (!unix) return '';
    return new Date(unix * 1000).toLocaleDateString('en-GB', {
      day: 'numeric', month: 'long', year: 'numeric',
    });
  }

  // Category → capability slug (lowercase hyphenated)
  function categorySlug(cat: string): string {
    return cat.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');
  }
</script>

<svelte:head>
  <title>{post.title} — Work — eZeroAndOne</title>
  <meta name="description" content={post.summary} />
  {#if post.featured_image_url}
    <meta property="og:image" content={post.featured_image_url} />
  {/if}
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<main class="work-detail">

  <!-- ── Breadcrumb ─────────────────────────────────────────── -->
  <nav class="breadcrumb" aria-label="Breadcrumb">
    <a href="/work">← Built Legacies</a>
    {#if post.category}
      <span class="bc-sep" aria-hidden="true">/</span>
      <a href="/capabilities#{categorySlug(post.category)}">{post.category}</a>
    {/if}
  </nav>

  <!-- ── Hero image ─────────────────────────────────────────── -->
  {#if post.featured_image_url}
    <div class="hero-image-wrap">
      <img
        src={post.featured_image_url}
        alt="Screenshot of {post.title}"
        class="hero-image"
        width="1280"
        height="720"
      />
      <div class="hero-overlay" aria-hidden="true"></div>
    </div>
  {/if}

  <article class="post-article">

    <!-- ── Header ───────────────────────────────────────────── -->
    <header class="post-header">
      <div class="header-meta">
        {#if post.category}
          <a
            href="/capabilities#{categorySlug(post.category)}"
            class="service-pill"
            title="View {post.category} capability"
          >
            <span class="material-icons-outlined" aria-hidden="true">category</span>
            {post.category}
          </a>
        {/if}
        {#if post.project_type}
          <span class="type-pill">{post.project_type}</span>
        {/if}
      </div>

      <h1>{post.title}</h1>

      <p class="post-summary">{post.summary}</p>

      <div class="header-footer">
        {#if post.name}
          <div class="author-row">
            {#if post.avatar_url}
              <img src={post.avatar_url} alt={post.name} class="author-avatar" width="32" height="32" />
            {:else}
              <div class="author-avatar-placeholder" aria-hidden="true">
                {post.name.charAt(0).toUpperCase()}
              </div>
            {/if}
            <div class="author-details">
              <a href="/team/{post.username}" class="author-name">{post.name}</a>
              {#if post.job_title}
                <span class="author-title">{post.job_title}</span>
              {/if}
            </div>
          </div>
        {/if}
        {#if post.published_at}
          <time
            class="publish-date"
            datetime={new Date(post.published_at * 1000).toISOString()}
          >
            {formatDate(post.published_at)}
          </time>
        {/if}
      </div>
    </header>

    <!-- ── Body content ─────────────────────────────────────── -->
    {#if bodyHtml}
      <div class="post-body">
        {@html bodyHtml}
      </div>
    {/if}

    <!-- ── Meta sidebar strip ───────────────────────────────── -->
    <div class="meta-strip">
      {#if techs.length > 0}
        <div class="meta-group">
          <span class="meta-label">
            <span class="material-icons-outlined" aria-hidden="true">code</span>
            Technologies
          </span>
          <div class="tech-tags">
            {#each techs as tech}
              <span class="tech-tag">{tech}</span>
            {/each}
          </div>
        </div>
      {/if}

      {#if tags.length > 0}
        <div class="meta-group">
          <span class="meta-label">
            <span class="material-icons-outlined" aria-hidden="true">label</span>
            Tags
          </span>
          <div class="tag-list">
            {#each tags as tag}
              <span class="tag">{tag}</span>
            {/each}
          </div>
        </div>
      {/if}
    </div>

    <!-- ── Team ─────────────────────────────────────────────── -->
    {#if team.length > 0}
      <section class="team-section" aria-labelledby="team-heading">
        <h2 id="team-heading" class="section-heading">
          <span class="material-icons-outlined" aria-hidden="true">group</span>
          Team on this project
        </h2>
        <div class="team-grid">
          {#each team as member}
            {#if member.staff_id && member.staff_username}
              <a
                href="/team/{member.staff_username}"
                class="team-card team-card--link"
                aria-label="View {member.staff_name}'s profile"
              >
                <div class="team-avatar">
                  {#if member.staff_avatar_url}
                    <img
                      src={member.staff_avatar_url}
                      alt={member.staff_name ?? ''}
                      width="56"
                      height="56"
                    />
                  {:else}
                    <span class="avatar-initials" aria-hidden="true">
                      {(member.staff_name ?? '?').charAt(0).toUpperCase()}
                    </span>
                  {/if}
                </div>
                <div class="team-info">
                  <span class="team-name">{member.staff_name}</span>
                  {#if member.staff_job_title}
                    <span class="team-role">{member.staff_job_title}</span>
                  {/if}
                </div>
                <span class="material-icons-outlined team-link-icon" aria-hidden="true">arrow_forward</span>
              </a>
            {:else}
              <!-- External contributor -->
              <div class="team-card">
                <div class="team-avatar team-avatar--ext">
                  <span class="avatar-initials" aria-hidden="true">
                    {(member.ext_name || '?').charAt(0).toUpperCase()}
                  </span>
                </div>
                <div class="team-info">
                  <span class="team-name">
                    {#if member.ext_url}
                      <a href={member.ext_url} target="_blank" rel="noopener noreferrer" class="ext-link">
                        {member.ext_name}
                        <span class="material-icons-outlined" aria-hidden="true" style="font-size:0.85rem">open_in_new</span>
                      </a>
                    {:else}
                      {member.ext_name}
                    {/if}
                  </span>
                  {#if member.ext_role}
                    <span class="team-role">{member.ext_role}</span>
                  {/if}
                </div>
              </div>
            {/if}
          {/each}
        </div>
      </section>
    {/if}

    <!-- ── Footer CTAs ──────────────────────────────────────── -->
    <footer class="article-footer">
      <a href="/work" class="back-cta">← Back to Work</a>
      <a href="/capabilities" class="init-cta">Initialize a Project</a>
    </footer>

  </article>
</main>

<style>
  .work-detail {
    max-width: 900px;
    margin: 0 auto;
    padding: 2rem 1.25rem 4rem;
  }

  /* ── Breadcrumb ────────────────────────────────────────────── */
  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    margin-bottom: 1.75rem;
    flex-wrap: wrap;
  }

  .breadcrumb a {
    font-size: 0.8rem;
    color: var(--accent-green, #00ff88);
    text-decoration: none;
    transition: color 0.2s;
  }

  .breadcrumb a:hover { color: var(--accent-blue, #00d4ff); text-decoration: underline; }

  .bc-sep {
    font-size: 0.75rem;
    color: rgba(255,255,255,0.25);
  }

  /* ── Hero ──────────────────────────────────────────────────── */
  .hero-image-wrap {
    position: relative;
    border-radius: 16px;
    overflow: hidden;
    margin-bottom: 2rem;
    border: 1px solid var(--glass-border, rgba(255,255,255,0.08));
    aspect-ratio: 16/9;
    background: rgba(255,255,255,0.04);
  }

  .hero-image {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
  }

  .hero-overlay {
    position: absolute;
    inset: 0;
    background: linear-gradient(to bottom, transparent 55%, rgba(5,5,12,0.85) 100%);
    pointer-events: none;
  }

  /* ── Article card ──────────────────────────────────────────── */
  .post-article {
    background: var(--glass-bg, rgba(255,255,255,0.04));
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--glass-border, rgba(255,255,255,0.08));
    border-radius: 18px;
    overflow: hidden;
  }

  /* ── Header ────────────────────────────────────────────────── */
  .post-header {
    padding: 2rem 2.5rem 1.75rem;
    border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.08));
  }

  .header-meta {
    display: flex;
    gap: 8px;
    align-items: center;
    flex-wrap: wrap;
    margin-bottom: 1rem;
  }

  .service-pill {
    display: inline-flex;
    align-items: center;
    gap: 4px;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-green, #00ff88);
    background: color-mix(in srgb, var(--accent-green, #00ff88) 12%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-green, #00ff88) 30%, transparent);
    border-radius: 20px;
    padding: 3px 10px 3px 7px;
    text-decoration: none;
    transition: background 0.2s, border-color 0.2s;
  }

  .service-pill:hover {
    background: color-mix(in srgb, var(--accent-green, #00ff88) 20%, transparent);
    text-decoration: none;
  }

  .service-pill .material-icons-outlined { font-size: 0.9rem; }

  .type-pill {
    display: inline-flex;
    align-items: center;
    font-size: 0.72rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--accent-blue, #00d4ff);
    background: color-mix(in srgb, var(--accent-blue, #00d4ff) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-blue, #00d4ff) 25%, transparent);
    border-radius: 20px;
    padding: 3px 10px;
  }

  .post-header h1 {
    font-size: clamp(1.6rem, 4vw, 2.5rem);
    font-weight: 800;
    margin: 0 0 0.9rem;
    color: var(--color-text-primary, #f0f0f0);
    letter-spacing: -0.025em;
    line-height: 1.15;
  }

  .post-summary {
    font-size: 1.05rem;
    line-height: 1.7;
    color: var(--color-text-secondary, #888899);
    margin: 0 0 1.5rem;
    font-style: italic;
  }

  .header-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    flex-wrap: wrap;
  }

  .author-row {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .author-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    object-fit: cover;
    border: 1px solid rgba(255,255,255,0.12);
    flex-shrink: 0;
  }

  .author-avatar-placeholder {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: rgba(0,194,255,0.12);
    border: 1px solid rgba(0,194,255,0.25);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.85rem;
    font-weight: 700;
    color: #00C2FF;
    flex-shrink: 0;
  }

  .author-details { display: flex; flex-direction: column; }

  .author-name {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    text-decoration: none;
  }

  .author-name:hover { color: var(--accent-blue, #00d4ff); text-decoration: underline; }

  .author-title {
    font-size: 0.72rem;
    color: rgba(255,255,255,0.4);
  }

  .publish-date {
    font-size: 0.78rem;
    color: rgba(255,255,255,0.3);
  }

  /* ── Body ───────────────────────────────────────────────────── */
  .post-body {
    padding: 2rem 2.5rem;
    font-size: 1rem;
    line-height: 1.85;
    color: var(--color-text-secondary, #888899);
    border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.08));
  }

  .post-body :global(h2) { font-size: 1.4rem; margin-top: 2rem; margin-bottom: 0.6rem; color: var(--color-text-primary,#f0f0f0); }
  .post-body :global(h3) { font-size: 1.15rem; margin-top: 1.5rem; margin-bottom: 0.4rem; color: var(--color-text-primary,#f0f0f0); }
  .post-body :global(p)  { margin-bottom: 1.2rem; }
  .post-body :global(a)  { color: var(--accent-green,#00ff88); text-decoration: underline; }
  .post-body :global(a:hover) { color: var(--accent-blue,#00d4ff); }
  .post-body :global(code) { background: rgba(255,255,255,0.08); padding: 0.15em 0.4em; border-radius: 4px; font-size: 0.9em; }
  .post-body :global(pre) { background: rgba(255,255,255,0.06); padding: 1rem; border-radius: 10px; overflow-x: auto; margin-bottom: 1.5rem; }
  .post-body :global(pre code) { background: none; padding: 0; }
  .post-body :global(ul), .post-body :global(ol) { padding-left: 1.5rem; margin-bottom: 1.2rem; }
  .post-body :global(li) { margin-bottom: 0.4rem; }

  /* ── Meta strip ─────────────────────────────────────────────── */
  .meta-strip {
    display: flex;
    flex-wrap: wrap;
    gap: 1.5rem;
    padding: 1.5rem 2.5rem;
    border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.08));
  }

  .meta-group { display: flex; flex-direction: column; gap: 8px; }

  .meta-label {
    display: flex;
    align-items: center;
    gap: 5px;
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: rgba(255,255,255,0.35);
  }

  .meta-label .material-icons-outlined { font-size: 0.95rem; }

  .tech-tags, .tag-list {
    display: flex;
    flex-wrap: wrap;
    gap: 6px;
  }

  .tech-tag {
    display: inline-block;
    font-size: 0.72rem;
    font-weight: 600;
    color: var(--accent-blue, #00d4ff);
    background: color-mix(in srgb, var(--accent-blue, #00d4ff) 10%, transparent);
    border: 1px solid color-mix(in srgb, var(--accent-blue, #00d4ff) 25%, transparent);
    border-radius: 6px;
    padding: 3px 9px;
  }

  .tag {
    display: inline-block;
    font-size: 0.7rem;
    color: rgba(255,255,255,0.45);
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 6px;
    padding: 2px 8px;
  }

  /* ── Team section ───────────────────────────────────────────── */
  .team-section {
    padding: 2rem 2.5rem;
    border-bottom: 1px solid var(--glass-border, rgba(255,255,255,0.08));
  }

  .section-heading {
    display: flex;
    align-items: center;
    gap: 8px;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: rgba(255,255,255,0.35);
    margin: 0 0 1.25rem;
  }

  .section-heading .material-icons-outlined { font-size: 1rem; }

  .team-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
    gap: 12px;
  }

  .team-card {
    display: flex;
    align-items: center;
    gap: 12px;
    padding: 14px 16px;
    background: rgba(255,255,255,0.04);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 12px;
    transition: background 0.2s, border-color 0.2s;
    text-decoration: none;
    color: inherit;
    position: relative;
  }

  .team-card--link:hover {
    background: rgba(0,212,255,0.06);
    border-color: rgba(0,212,255,0.25);
    text-decoration: none;
  }

  .team-avatar {
    width: 46px;
    height: 46px;
    border-radius: 50%;
    flex-shrink: 0;
    overflow: hidden;
    background: rgba(0,194,255,0.1);
    border: 1px solid rgba(0,194,255,0.2);
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .team-avatar img {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .team-avatar--ext {
    background: rgba(255,255,255,0.06);
    border-color: rgba(255,255,255,0.1);
  }

  .avatar-initials {
    font-size: 1.1rem;
    font-weight: 700;
    color: #00C2FF;
  }

  .team-info {
    display: flex;
    flex-direction: column;
    min-width: 0;
    flex: 1;
  }

  .team-name {
    font-size: 0.85rem;
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .team-role {
    font-size: 0.72rem;
    color: rgba(255,255,255,0.4);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
    margin-top: 2px;
  }

  .team-link-icon {
    font-size: 0.9rem;
    color: rgba(0,212,255,0.4);
    flex-shrink: 0;
    transition: color 0.2s;
  }

  .team-card--link:hover .team-link-icon { color: #00d4ff; }

  .ext-link {
    color: inherit;
    text-decoration: none;
    display: inline-flex;
    align-items: center;
    gap: 3px;
  }

  .ext-link:hover { color: var(--accent-green, #00ff88); }

  /* ── Footer ─────────────────────────────────────────────────── */
  .article-footer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 1rem;
    padding: 1.5rem 2.5rem;
    flex-wrap: wrap;
  }

  .back-cta {
    font-size: 0.875rem;
    color: var(--accent-green, #00ff88);
    text-decoration: none;
  }

  .back-cta:hover { text-decoration: underline; }

  .init-cta {
    display: inline-block;
    padding: 0.6rem 1.5rem;
    background: var(--accent-blue, #00d4ff);
    color: #000;
    font-size: 0.875rem;
    font-weight: 700;
    border-radius: 10px;
    text-decoration: none;
    transition: filter 0.2s, box-shadow 0.2s;
  }

  .init-cta:hover {
    filter: brightness(1.1);
    box-shadow: 0 0 20px rgba(0,212,255,0.35);
    text-decoration: none;
  }

  @media (max-width: 640px) {
    .post-header { padding: 1.5rem; }
    .post-body { padding: 1.5rem; }
    .meta-strip { padding: 1.25rem 1.5rem; }
    .team-section { padding: 1.5rem; }
    .article-footer { padding: 1.25rem 1.5rem; }
    .team-grid { grid-template-columns: 1fr; }
  }
</style>
