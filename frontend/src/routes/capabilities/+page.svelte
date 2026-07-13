<script lang="ts">
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>Architectural Capabilities — eZeroAndOne</title>
  <meta name="description" content="Modular solutions designed for absolute performance at scale. Full-stack architecture, API integration, and edge infrastructure from first principles." />
</svelte:head>

<main class="capabilities-page">
  <header class="page-header">
    <h1>Architectural Capabilities</h1>
    <p class="subtitle">Modular solutions designed for absolute performance at scale.</p>
  </header>

  <div class="posts-grid">
    {#each data.posts as post}
      <article class="post-card">
        <a href="/capabilities/{post.slug}" class="card-link">
          <div class="card-content">
            <span class="post-type">{post.type}</span>
            <h2>{post.title}</h2>
            <p class="summary">{post.summary}</p>
            {#if post.author}
              <div class="author-info">
                <span class="author-name">{post.author.name}</span>
              </div>
            {/if}
          </div>
        </a>
      </article>
    {/each}
  </div>
</main>

<style>
  .capabilities-page {
    max-width: 1200px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .page-header {
    text-align: center;
    margin-bottom: 3rem;
  }

  .page-header h1 {
    font-size: 3rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    color: var(--color-text-primary);
  }

  .subtitle {
    font-size: 1.25rem;
    color: var(--color-text-secondary);
    margin: 0;
  }

  .posts-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(320px, 1fr));
    gap: 2rem;
  }

  .post-card {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur)) saturate(var(--glass-saturate));
    border: 1px solid var(--glass-border);
    border-radius: 12px;
    overflow: hidden;
    transition: transform 0.2s ease, box-shadow 0.2s ease;
  }

  .post-card:hover {
    transform: scale(1.02);
    box-shadow: var(--glow-yellow);
  }

  .card-link {
    display: block;
    text-decoration: none;
    color: inherit;
  }

  .card-content {
    padding: 1.5rem;
  }

  .post-type {
    display: inline-block;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    color: var(--accent-yellow);
    font-weight: 600;
    margin-bottom: 0.5rem;
  }

  .post-card h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0 0 0.75rem 0;
    color: var(--color-text-primary);
  }

  .summary {
    font-size: 1rem;
    line-height: 1.6;
    color: var(--color-text-secondary);
    margin: 0 0 1rem 0;
  }

  .author-info {
    padding-top: 0.75rem;
    border-top: 1px solid var(--color-border);
  }

  .author-name {
    font-size: 0.875rem;
    color: var(--color-text-muted);
  }
</style>
