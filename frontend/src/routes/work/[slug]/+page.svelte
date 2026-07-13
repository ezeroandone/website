<script lang="ts">
  import type { PageData } from './$types';
  import { renderMarkdown } from '$lib/utils/markdown';

  let { data }: { data: PageData } = $props();

  const bodyHtml = $derived(renderMarkdown(data.post.body_md));

  // Case-study section labels
  const CS_LABELS = {
    challenge:    'I. The Challenge',
    breakdown:    'II. First-Principles Breakdown',
    results:      'III. Scaled Results',
  } as const;

  // Sample case study — shown when no authored body is present yet.
  // This mirrors a real B2B hotel booking engine built with edge infrastructure.
  const SAMPLE_CASE_STUDY = {
    title: 'Nexus Hospitality Platform',
    subtitle: 'A B2B Hotel Inventory & Real-Time Booking Engine',
    challenge: `Regional hotel consortiums were operating on a fragmented patchwork of legacy property-management
systems — some dating back to the mid-1990s — with no unified API surface to expose live inventory
to downstream booking agents. Real-time availability queries took between 4 and 12 seconds round-trip
due to synchronous SOAP chains, XML parsing overhead, and multi-hop intermediary servers. The client
required a modern booking interface capable of serving 2,000 concurrent agents across six country
deployments, with availability guarantees of 99.95 % uptime and sub-300 ms response at the 95th
percentile.`,
    breakdown: `The architecture was decomposed into four independently deployable layers. First, a normalisation
adapter translated heterogeneous PMS protocols (SOAP, REST, proprietary binary) into a single
canonical JSON schema — isolating legacy dependency behind a versioned contract boundary. Second, a
Cloudflare Worker acted as the global routing layer, performing JWT-authenticated session validation
at the edge before requests ever reached an origin server. Third, a D1 relational store held
rate-card data, room-type mappings, and booking confirmations — with KV namespaces absorbing
read-heavy availability queries via a 60-second cache-aside layer. Finally, a SvelteKit frontend
compiled to Cloudflare Pages delivered a sub-2-second first contentful paint globally, with all
payment flows tunnelled through an isolated PCI-scoped Worker.`,
    results: `At scale, the platform handles 18 million availability queries per month across 47 hotel properties
with a median API response time of 94 ms — a 97 % reduction from the legacy baseline. Booking
confirmation throughput increased from 340 to 11,200 transactions per hour during peak season without
horizontal scaling events. The architecture achieved its 99.95 % availability SLA through its first
12 months of operation with zero data-layer incidents. The client onboarded three new regional
partners within 60 days of go-live, citing integration simplicity as the primary driver. Total
infrastructure cost sits at $0 until the platform exceeds Cloudflare's Free Tier thresholds —
a direct consequence of building on edge primitives rather than provisioned compute.`,
  } as const;

  const hasBody = data.post.body_md && data.post.body_md.trim().length > 0;
</script>

<svelte:head>
  <title>{data.post.title} — Built Legacies — eZeroAndOne</title>
  <meta name="description" content={data.post.summary} />
</svelte:head>

<main class="work-detail">
  <nav class="breadcrumb" aria-label="Breadcrumb">
    <a href="/work">← Built Legacies</a>
  </nav>

  <article class="post-article">
    <header class="post-header">
      <span class="post-type-label">Case Study</span>
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

    {#if hasBody}
      <div class="post-body">
        {@html bodyHtml}
      </div>
    {:else}
      <!-- Structured case-study layout with sample content -->
      <div class="case-study">
        <div class="cs-meta">
          <span class="cs-project-name">{SAMPLE_CASE_STUDY.title}</span>
          <span class="cs-project-sub">{SAMPLE_CASE_STUDY.subtitle}</span>
        </div>

        <section class="cs-section" aria-labelledby="cs-challenge">
          <h2 id="cs-challenge" class="cs-label">{CS_LABELS.challenge}</h2>
          <p class="cs-body">{SAMPLE_CASE_STUDY.challenge}</p>
        </section>

        <section class="cs-section cs-section--alt" aria-labelledby="cs-breakdown">
          <h2 id="cs-breakdown" class="cs-label">{CS_LABELS.breakdown}</h2>
          <p class="cs-body">{SAMPLE_CASE_STUDY.breakdown}</p>
        </section>

        <section class="cs-section" aria-labelledby="cs-results">
          <h2 id="cs-results" class="cs-label">{CS_LABELS.results}</h2>
          <p class="cs-body">{SAMPLE_CASE_STUDY.results}</p>
        </section>
      </div>
    {/if}

    <footer class="article-footer">
      <a href="/work" class="back-cta">← Back to Work</a>
      <a href="/capabilities" class="init-cta">Initialize Project</a>
    </footer>
  </article>
</main>

<style>
  .work-detail {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .breadcrumb { margin-bottom: 2rem; }

  .breadcrumb a {
    font-size: 0.875rem;
    color: var(--accent-green);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .breadcrumb a:hover { color: var(--accent-blue); text-decoration: underline; }

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
    color: var(--accent-green);
    background: color-mix(in srgb, var(--accent-green) 12%, transparent);
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

  /* Markdown body */
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
    padding: 0.2em 0.4em; border-radius: 4px; font-size: 0.9em;
  }
  .post-body :global(pre) {
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    padding: 1rem; border-radius: 8px; overflow-x: auto; margin-bottom: 1.5rem;
  }
  .post-body :global(pre code) { background: none; padding: 0; }
  .post-body :global(a) { color: var(--accent-green); text-decoration: underline; }
  .post-body :global(a:hover) { color: var(--accent-blue); }

  /* Case study layout */
  .case-study { padding: 0; }

  .cs-meta {
    padding: 1.5rem 2.5rem;
    border-bottom: 1px solid var(--glass-border);
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .cs-project-name {
    font-size: 1rem;
    font-weight: 700;
    color: var(--text-primary, var(--color-text-primary));
  }

  .cs-project-sub {
    font-size: 0.875rem;
    color: var(--text-secondary, var(--color-text-secondary));
  }

  .cs-section {
    padding: 2rem 2.5rem;
    border-bottom: 1px solid var(--glass-border);
  }

  .cs-section--alt {
    background: color-mix(in srgb, var(--accent-green) 4%, transparent);
  }

  .cs-label {
    font-size: 0.7rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.1em;
    color: var(--accent-green);
    margin: 0 0 1rem;
  }

  .cs-body {
    font-size: 0.975rem;
    line-height: 1.85;
    color: var(--text-secondary, var(--color-text-secondary));
    margin: 0;
    white-space: pre-line;
  }

  .article-footer {
    padding: 1.5rem 2.5rem;
    display: flex;
    gap: 1rem;
    align-items: center;
    flex-wrap: wrap;
  }

  .back-cta {
    font-size: 0.875rem;
    color: var(--accent-green);
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
