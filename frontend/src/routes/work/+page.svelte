<script lang="ts">
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  function techs(raw: string): string[] {
    return raw ? raw.split(',').map((t: string) => t.trim()).filter(Boolean) : [];
  }

  function formatDate(unix: number | null): string {
    if (!unix) return '';
    return new Date(unix * 1000).toLocaleDateString('en-GB', {
      month: 'short', year: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Built Legacies — eZeroAndOne Work</title>
  <meta name="description" content="Proven technical engineering deployed in production environments. Portfolio of websites, web apps and digital products built by eZeroAndOne." />
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<main class="work-page">
  <header class="page-header">
    <div class="header-eyebrow">
      <span class="material-icons-outlined" aria-hidden="true">rocket_launch</span>
      Portfolio
    </div>
    <h1>Built Legacies</h1>
    <p class="subtitle">Proven technical engineering deployed in production environments.</p>
  </header>

  {#if data.posts.length === 0}
    <div class="empty-state">
      <span class="material-icons-outlined" aria-hidden="true">inventory_2</span>
      <p>No published work yet. Check back soon.</p>
    </div>
  {:else}
    <div class="posts-grid">
      {#each data.posts as post}
        <article class="post-card">
          <a href="/work/{post.slug}" class="card-link" aria-label="View {post.title}">

            <!-- Thumbnail -->
            <div class="card-thumb">
              {#if post.featured_image_url}
                <img
                  src={post.featured_image_url}
                  alt="Screenshot of {post.title}"
                  class="thumb-img"
                  loading="lazy"
                  width="640"
                  height="360"
                />
              {:else}
                <div class="thumb-placeholder" aria-hidden="true">
                  <span class="material-icons-outlined">web</span>
                </div>
              {/if}
              {#if post.category}
                <span class="card-category">{post.category}</span>
              {/if}
            </div>

            <!-- Content -->
            <div class="card-content">
              {#if post.project_type}
                <span class="project-type">{post.project_type}</span>
              {/if}
              <h2>{post.title}</h2>
              <p class="summary">{post.summary}</p>

              {#if post.technologies}
                <div class="tech-row">
                  {#each techs(post.technologies).slice(0, 4) as tech}
                    <span class="tech-chip">{tech}</span>
                  {/each}
                  {#if techs(post.technologies).length > 4}
                    <span class="tech-chip tech-chip--more">+{techs(post.technologies).length - 4}</span>
                  {/if}
                </div>
              {/if}

              <div class="card-footer">
                {#if post.author}
                  <span class="author-name">{post.author.name}</span>
                {/if}
                {#if post.published_at}
                  <time class="pub-date" datetime={new Date(post.published_at * 1000).toISOString()}>
                    {formatDate(post.published_at)}
                  </time>
                {/if}
                <span class="card-arrow" aria-hidden="true">→</span>
              </div>
            </div>
          </a>
        </article>
      {/each}
    </div>
  {/if}
</main>

<style>
  .work-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 3rem 1.25rem 5rem;
  }

  /* ── Header ──────────────────────────────────────────────────── */
  .page-header {
    text-align: center;
    margin-bottom: 3.5rem;
  }

  .header-eyebrow {
    display: inline-flex;
    align-items: center;
    gap: 6px;
    font-size: 0.72rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: var(--accent-green, #00ff88);
    margin-bottom: 0.75rem;
  }

  .header-eyebrow .material-icons-outlined { font-size: 1rem; }

  .page-header h1 {
    font-size: clamp(2.5rem, 6vw, 4rem);
    font-weight: 800;
    margin: 0 0 0.75rem;
    color: var(--color-text-primary, #f0f0f0);
    letter-spacing: -0.03em;
    line-height: 1.05;
  }

  .subtitle {
    font-size: 1.1rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
    max-width: 520px;
    margin: 0 auto;
    line-height: 1.6;
  }

  /* ── Empty state ─────────────────────────────────────────────── */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 5rem 0;
    color: rgba(255,255,255,0.3);
  }

  .empty-state .material-icons-outlined { font-size: 3rem; }

  /* ── Grid ────────────────────────────────────────────────────── */
  .posts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 1.75rem;
  }

  /* ── Card ────────────────────────────────────────────────────── */
  .post-card {
    background: var(--glass-bg, rgba(255,255,255,0.04));
    backdrop-filter: blur(16px) saturate(180%);
    -webkit-backdrop-filter: blur(16px) saturate(180%);
    border: 1px solid var(--glass-border, rgba(255,255,255,0.08));
    border-radius: 16px;
    overflow: hidden;
    transition: transform 0.2s ease, border-color 0.2s ease, box-shadow 0.2s ease;
  }

  .post-card:hover {
    transform: translateY(-3px);
    border-color: rgba(0,255,136,0.25);
    box-shadow: 0 8px 32px rgba(0,255,136,0.08);
  }

  .card-link {
    display: flex;
    flex-direction: column;
    text-decoration: none;
    color: inherit;
    height: 100%;
  }

  /* Thumbnail */
  .card-thumb {
    position: relative;
    width: 100%;
    aspect-ratio: 16/9;
    overflow: hidden;
    background: rgba(255,255,255,0.04);
    flex-shrink: 0;
  }

  .thumb-img {
    width: 100%;
    height: 100%;
    object-fit: cover;
    display: block;
    transition: transform 0.35s ease;
  }

  .post-card:hover .thumb-img { transform: scale(1.03); }

  .thumb-placeholder {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    color: rgba(255,255,255,0.15);
  }

  .thumb-placeholder .material-icons-outlined { font-size: 3rem; }

  .card-category {
    position: absolute;
    bottom: 10px;
    left: 12px;
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.07em;
    color: var(--accent-green, #00ff88);
    background: rgba(5,5,12,0.75);
    border: 1px solid rgba(0,255,136,0.2);
    border-radius: 20px;
    padding: 2px 9px;
    backdrop-filter: blur(4px);
  }

  /* Content */
  .card-content {
    padding: 1.25rem 1.5rem 1.5rem;
    display: flex;
    flex-direction: column;
    gap: 0;
    flex: 1;
  }

  .project-type {
    font-size: 0.68rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-blue, #00d4ff);
    margin-bottom: 0.4rem;
    display: block;
  }

  .card-content h2 {
    font-size: 1.1rem;
    font-weight: 700;
    margin: 0 0 0.5rem;
    color: var(--color-text-primary, #f0f0f0);
    line-height: 1.3;
    letter-spacing: -0.01em;
  }

  .summary {
    font-size: 0.85rem;
    line-height: 1.6;
    color: var(--color-text-secondary, #888899);
    margin: 0 0 0.9rem;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
    flex: 1;
  }

  .tech-row {
    display: flex;
    flex-wrap: wrap;
    gap: 5px;
    margin-bottom: 1rem;
  }

  .tech-chip {
    font-size: 0.65rem;
    font-weight: 600;
    color: rgba(255,255,255,0.5);
    background: rgba(255,255,255,0.06);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 4px;
    padding: 2px 7px;
  }

  .tech-chip--more {
    color: rgba(0,212,255,0.6);
    background: rgba(0,212,255,0.06);
    border-color: rgba(0,212,255,0.15);
  }

  .card-footer {
    display: flex;
    align-items: center;
    gap: 0.5rem;
    padding-top: 0.75rem;
    border-top: 1px solid rgba(255,255,255,0.07);
  }

  .author-name {
    font-size: 0.75rem;
    font-weight: 600;
    color: rgba(255,255,255,0.4);
    flex: 1;
  }

  .pub-date {
    font-size: 0.72rem;
    color: rgba(255,255,255,0.25);
    white-space: nowrap;
  }

  .card-arrow {
    font-size: 0.85rem;
    color: var(--accent-green, #00ff88);
    margin-left: auto;
    opacity: 0;
    transform: translateX(-4px);
    transition: opacity 0.2s, transform 0.2s;
  }

  .post-card:hover .card-arrow {
    opacity: 1;
    transform: translateX(0);
  }

  @media (max-width: 640px) {
    .posts-grid { grid-template-columns: 1fr; }
    .page-header h1 { font-size: 2.25rem; }
  }
</style>
