<script lang="ts">
  import type { PageData } from './$types';
  import { renderMarkdown } from '$lib/utils/markdown';

  let { data }: { data: PageData } = $props();

  const bodyHtml = $derived(renderMarkdown(data.post.body_md));

  const SECTION_LABELS = {
    analysis:    '01 / Structural Analysis',
    deconstruct: '02 / Deconstruction',
    synthesis:   '03 / Synthesis',
  } as const;
</script>

<svelte:head>
  <title>{data.post.title} — Capabilities — eZeroAndOne</title>
  <meta name="description" content={data.post.summary} />
</svelte:head>

<main class="cap-detail">

  <!-- Back breadcrumb -->
  <nav class="breadcrumb" aria-label="Breadcrumb">
    <a href="/capabilities" class="back-link">
      <span class="material-symbols-outlined" aria-hidden="true">arrow_back</span>
      Back to Capabilities
    </a>
  </nav>

  <!-- Page header -->
  <header class="cap-header">
    <span class="cap-label">Capability</span>
    <h1 class="cap-h1">{data.post.title}</h1>
    {#if data.post.author}
      <div class="author-meta">
        <span class="material-symbols-outlined" aria-hidden="true">person</span>
        <span class="author-name">{data.post.author.name}</span>
        {#if data.post.published_at}
          <time datetime={new Date(data.post.published_at * 1000).toISOString()}>
            {new Date(data.post.published_at * 1000).toLocaleDateString('en-US', {
              year: 'numeric', month: 'long', day: 'numeric'
            })}
          </time>
        {/if}
      </div>
    {/if}
  </header>

  <!-- Summary -->
  <div class="cap-summary">
    <p>{data.post.summary}</p>
  </div>

  <!-- Body: authored markdown or structured three-section fallback -->
  {#if data.post.body_md && data.post.body_md.trim().length > 0}
    <div class="post-body">
      {@html bodyHtml}
    </div>
  {:else}
    <div class="framework-grid">

      <div class="framework-col">
        <div class="framework-label">
          <span class="material-symbols-outlined" aria-hidden="true">analytics</span>
          <span>{SECTION_LABELS.analysis}</span>
        </div>
        <p>
          Every engineering challenge begins with an honest diagnosis. Before a single line of
          code is written, we conduct a rigorous structural analysis of the problem domain —
          mapping data flows, identifying bottleneck topologies, and cataloguing the brittle
          abstractions left behind by previous iterations. This stage strips away assumption
          layers until the root constraint is exposed in its simplest, most tractable form.
        </p>
      </div>

      <div class="framework-col">
        <div class="framework-label">
          <span class="material-symbols-outlined" aria-hidden="true">schema</span>
          <span>{SECTION_LABELS.deconstruct}</span>
        </div>
        <p>
          A complex system is nothing more than a collection of simple components waiting to be
          isolated. We decompose the problem into discrete, independently verifiable modules —
          authentication primitives, data access layers, event propagation channels, and
          validation gates — each solved in isolation before being composed into a coherent
          whole.
        </p>
      </div>

      <div class="framework-col">
        <div class="framework-label">
          <span class="material-symbols-outlined" aria-hidden="true">integration_instructions</span>
          <span>{SECTION_LABELS.synthesis}</span>
        </div>
        <p>
          Synthesis is where architecture becomes infrastructure. Verified modules are assembled
          at the Cloudflare edge — compiled to WebAssembly for sub-millisecond cold starts,
          routed through hardened D1 relational stores, cached via KV namespaces with
          cryptographic TTL policies, and distributed globally across 300+ points of presence.
        </p>
      </div>

    </div>
  {/if}

  <!-- CTA footer -->
  <footer class="cap-footer">
    <a href="/capabilities" class="back-cta">
      <span class="material-symbols-outlined" aria-hidden="true">arrow_back</span>
      Back to Capabilities
    </a>
    <a href="/auth/login" class="btn btn-primary">Initialize Project</a>
  </footer>

</main>

<style>
  .cap-detail {
    max-width: 1100px;
    margin: 0 auto;
    padding: 3rem 2rem 6rem;
  }

  /* Breadcrumb */
  .breadcrumb { margin-bottom: 2.5rem; }
  .back-link {
    display: inline-flex; align-items: center; gap: 0.4rem;
    font-family: var(--font-heading); font-size: 0.72rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.1em;
    color: var(--accent-blue); text-decoration: none; transition: gap 0.2s ease;
  }
  .back-link:hover { gap: 0.6rem; text-decoration: none; }
  .back-link .material-symbols-outlined { font-size: 16px; }

  /* Header */
  .cap-header {
    border-bottom: 1px solid rgba(255, 255, 255, 0.08);
    padding-bottom: 2rem; margin-bottom: 2rem;
  }
  .cap-label {
    display: inline-block; font-family: var(--font-heading); font-size: 0.65rem;
    font-weight: 700; text-transform: uppercase; letter-spacing: 0.1em;
    color: var(--accent-blue); background: rgba(0, 82, 255, 0.1);
    padding: 0.2rem 0.6rem; margin-bottom: 1rem;
  }
  .cap-h1 {
    font-size: clamp(2rem, 6vw, 5rem); font-weight: 800;
    letter-spacing: -0.04em; line-height: 1.0; color: #fff; margin: 0 0 1.25rem;
  }
  .author-meta {
    display: flex; gap: 0.75rem; align-items: center;
    font-family: var(--font-body); font-size: 0.875rem; color: var(--text-secondary);
  }
  .author-meta .material-symbols-outlined { font-size: 16px; color: var(--text-muted); }
  .author-name { font-weight: 600; color: var(--text-primary); }

  /* Summary */
  .cap-summary {
    margin-bottom: 3rem;
    border-bottom: 1px solid rgba(255,255,255,0.06);
    padding-bottom: 3rem;
  }
  .cap-summary p {
    font-family: var(--font-body); font-size: 1.15rem; line-height: 1.8;
    color: var(--text-secondary); font-style: italic; margin: 0;
  }

  /* Three-section framework grid */
  .framework-grid {
    display: grid; grid-template-columns: repeat(3, 1fr); gap: 0;
    border-top: 1px solid rgba(255,255,255,0.08);
    margin-bottom: 4rem;
  }
  .framework-col {
    padding: 2.5rem;
    border-right: 1px solid rgba(255,255,255,0.06);
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }
  .framework-col:last-child { border-right: none; }
  .framework-label {
    display: flex; align-items: center; gap: 0.5rem;
    font-family: var(--font-heading); font-size: 0.65rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.1em; color: var(--accent-blue);
    margin-bottom: 1.25rem;
  }
  .framework-label .material-symbols-outlined { font-size: 18px; }
  .framework-col p {
    font-family: var(--font-body); font-size: 0.9rem; line-height: 1.8;
    color: var(--text-secondary); margin: 0;
  }

  /* Authored markdown body */
  .post-body {
    margin-bottom: 4rem;
    font-family: var(--font-body); font-size: 1rem; line-height: 1.8;
    color: var(--text-primary);
  }
  .post-body :global(h2) { font-size: 1.5rem; margin-top: 2rem; margin-bottom: 0.75rem; }
  .post-body :global(h3) { font-size: 1.25rem; margin-top: 1.5rem; margin-bottom: 0.5rem; }
  .post-body :global(p)  { margin-bottom: 1.25rem; }
  .post-body :global(code) {
    background: rgba(255,255,255,0.06); padding: 0.2em 0.4em; font-size: 0.9em;
  }
  .post-body :global(pre) {
    background: rgba(255,255,255,0.04); padding: 1rem; overflow-x: auto; margin-bottom: 1.5rem;
  }
  .post-body :global(pre code) { background: none; padding: 0; }
  .post-body :global(a) { color: var(--accent-blue-hi); text-decoration: underline; }
  .post-body :global(a:hover) { color: #fff; }

  /* Footer CTAs */
  .cap-footer {
    border-top: 1px solid rgba(255,255,255,0.08); padding-top: 2rem;
    display: flex; gap: 1rem; align-items: center; flex-wrap: wrap;
  }
  .back-cta {
    display: inline-flex; align-items: center; gap: 0.4rem;
    font-family: var(--font-heading); font-size: 0.72rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.1em;
    color: var(--accent-blue); text-decoration: none; flex: 1; transition: gap 0.2s ease;
  }
  .back-cta:hover { gap: 0.6rem; text-decoration: none; }
  .back-cta .material-symbols-outlined { font-size: 16px; }

  /* Responsive */
  @media (max-width: 900px) {
    .framework-grid { grid-template-columns: 1fr; }
    .framework-col { border-right: none; }
  }
  @media (max-width: 640px) {
    .cap-detail { padding: 2rem 1rem 4rem; }
  }
</style>
