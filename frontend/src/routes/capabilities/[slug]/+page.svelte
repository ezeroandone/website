<script lang="ts">
  import type { PageData } from './$types';
  import { renderMarkdown } from '$lib/utils/markdown';

  let { data }: { data: PageData } = $props();

  const bodyHtml = $derived(renderMarkdown(data.post.body_md));

  // Section labels used in the structured capability layout
  const SECTION_LABELS = {
    analysis:     'I. Structural Analysis — The Problem',
    deconstruct:  'II. Deconstruction — The Simple Components',
    synthesis:    'III. Synthesis — The Final Edge Infrastructure',
  } as const;
</script>

<svelte:head>
  <title>{data.post.title} — Architectural Capabilities — eZeroAndOne</title>
  <meta name="description" content={data.post.summary} />
</svelte:head>

<main class="capability-detail">
  <nav class="breadcrumb" aria-label="Breadcrumb">
    <a href="/capabilities">← Capabilities</a>
  </nav>

  <article class="post-article">
    <header class="post-header">
      <span class="post-type-label">Capability</span>
      <h1>{data.post.title}</h1>
      {#if data.post.author}
        <div class="author-meta">
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

    <div class="post-summary">
      <p>{data.post.summary}</p>
    </div>

    <!-- If the post body_md contains content, render it directly.
         Otherwise fall back to the structured three-section framework. -->
    {#if data.post.body_md && data.post.body_md.trim().length > 0}
      <div class="post-body">
        {@html bodyHtml}
      </div>
    {:else}
      <!-- Three-section framework template (used when no body is authored yet) -->
      <div class="framework-sections">
        <section class="framework-section" aria-labelledby="section-analysis">
          <h2 id="section-analysis" class="section-marker">{SECTION_LABELS.analysis}</h2>
          <p class="section-body">
            Every engineering challenge begins with an honest diagnosis. Before a single line of
            code is written, we conduct a rigorous structural analysis of the problem domain —
            mapping data flows, identifying bottleneck topologies, and cataloguing the brittle
            abstractions left behind by previous iterations. This stage strips away assumption
            layers until the root constraint is exposed in its simplest, most tractable form.
          </p>
        </section>

        <section class="framework-section" aria-labelledby="section-deconstruct">
          <h2 id="section-deconstruct" class="section-marker">{SECTION_LABELS.deconstruct}</h2>
          <p class="section-body">
            A complex system is nothing more than a collection of simple components waiting to be
            isolated. We decompose the problem into discrete, independently verifiable modules —
            authentication primitives, data access layers, event propagation channels, and
            validation gates — each solved in isolation before being composed into a coherent
            whole. This modular methodology eliminates cross-cutting failures and ensures each
            component carries zero unintended side effects into the larger assembly.
          </p>
        </section>

        <section class="framework-section" aria-labelledby="section-synthesis">
          <h2 id="section-synthesis" class="section-marker">{SECTION_LABELS.synthesis}</h2>
          <p class="section-body">
            Synthesis is where architecture becomes infrastructure. Verified modules are assembled
            at the Cloudflare edge — compiled to WebAssembly for sub-millisecond cold starts,
            routed through hardened D1 relational stores, cached via KV namespaces with
            cryptographic TTL policies, and distributed globally across 300+ points of presence.
            The result is a production system that does not merely function but endures — a
            digital legacy built to withstand the full spectrum of load, attack surface, and
            organizational evolution.
          </p>
        </section>
      </div>
    {/if}

    <!-- Back CTA -->
    <footer class="article-footer">
      <a href="/capabilities" class="back-cta">← Back to Capabilities</a>
      <a href="/auth/login" class="init-cta">Initialize Project</a>
    </footer>
  </article>
</main>

<style>
  .capability-detail {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .breadcrumb {
    margin-bottom: 2rem;
  }

  .breadcrumb a {
    font-size: 0.875rem;
    color: var(--accent-yellow);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .breadcrumb a:hover {
    color: var(--accent-red);
    text-decoration: underline;
  }

  .post-article {
    background: var(--glass-bg);
    backdrop-filter: blur(var(--glass-blur)) saturate(180%);
    -webkit-backdrop-filter: blur(var(--glass-blur)) saturate(180%);
    border: 1px solid var(--glass-border);
    border-radius: 16px;
    overflow: hidden;
  }

  .post-header {
    padding: 2.5rem 2.5rem 1.5rem;
    border-bottom: 1px solid var(--glass-border);
  }

  .post-type-label {
    display: inline-block;
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--accent-yellow);
    background: color-mix(in srgb, var(--accent-yellow) 12%, transparent);
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    margin-bottom: 0.75rem;
  }

  .post-header h1 {
    font-size: clamp(1.75rem, 4vw, 2.75rem);
    font-weight: 800;
    margin: 0 0 1rem;
    color: var(--text-primary, var(--color-text-primary));
    letter-spacing: -0.02em;
    line-height: 1.15;
  }

  .author-meta {
    display: flex;
    gap: 1rem;
    align-items: center;
    font-size: 0.875rem;
    color: var(--text-secondary, var(--color-text-secondary));
  }

  .author-name { font-weight: 600; }

  .post-summary {
    padding: 1.75rem 2.5rem;
    border-bottom: 1px solid var(--glass-border);
  }

  .post-summary p {
    font-size: 1.15rem;
    line-height: 1.7;
    color: var(--text-secondary, var(--color-text-secondary));
    font-style: italic;
    margin: 0;
  }

  /* Dynamic body (markdown) */
  .post-body {
    padding: 2rem 2.5rem;
    font-size: 1rem;
    line-height: 1.8;
    color: var(--text-primary, var(--color-text-primary));
  }

  .post-body :global(h2) { font-size: 1.5rem; margin-top: 2rem; margin-bottom: 0.75rem; }
  .post-body :global(h3) { font-size: 1.25rem; margin-top: 1.5rem; margin-bottom: 0.5rem; }
  .post-body :global(p)  { margin-bottom: 1.25rem; }
  .post-body :global(code) {
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    padding: 0.2em 0.4em;
    border-radius: 4px;
    font-size: 0.9em;
  }
  .post-body :global(pre) {
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    padding: 1rem;
    border-radius: 8px;
    overflow-x: auto;
    margin-bottom: 1.5rem;
  }
  .post-body :global(pre code) { background: none; padding: 0; }
  .post-body :global(a) { color: var(--accent-yellow); text-decoration: underline; }
  .post-body :global(a:hover) { color: var(--accent-red); }

  /* Framework sections */
  .framework-sections {
    padding: 2rem 2.5rem;
    display: flex;
    flex-direction: column;
    gap: 2.5rem;
  }

  .framework-section {
    border-left: 3px solid var(--accent-yellow);
    padding-left: 1.5rem;
  }

  .section-marker {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-yellow);
    margin: 0 0 0.75rem;
  }

  .section-body {
    font-size: 1rem;
    line-height: 1.8;
    color: var(--text-secondary, var(--color-text-secondary));
    margin: 0;
  }

  /* Footer CTAs */
  .article-footer {
    padding: 1.5rem 2.5rem;
    border-top: 1px solid var(--glass-border);
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .back-cta {
    font-size: 0.875rem;
    color: var(--accent-yellow);
    text-decoration: none;
    flex: 1;
  }

  .back-cta:hover { text-decoration: underline; }

  .init-cta {
    display: inline-block;
    padding: 0.6rem 1.5rem;
    background: var(--accent-blue);
    color: #fff;
    font-size: 0.875rem;
    font-weight: 600;
    border-radius: 8px;
    text-decoration: none;
    transition: opacity 0.2s ease;
  }

  .init-cta:hover { opacity: 0.88; text-decoration: none; }
</style>
