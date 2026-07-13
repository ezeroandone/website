<script lang="ts">
  import type { PageData } from './$types';
  import { renderMarkdown } from '$lib/utils/markdown';

  let { data }: { data: PageData } = $props();

  const bodyHtml = $derived(renderMarkdown(data.post.body_md));

  const hasBody = data.post.body_md && data.post.body_md.trim().length > 0;

  // Sample essay — shown when no authored body is present yet.
  // Topic: Rust + WebAssembly at the cloud edge.
  const SAMPLE_ESSAY = {
    intro: `The question of cold starts is not merely a performance footnote — it is an architectural constraint
that defines the entire topology of a modern edge system. When a conventional serverless function
boots a Node.js runtime, it must initialize the V8 isolate, resolve the module graph, hydrate
closures, and establish I/O event loops before the first meaningful byte of application logic
executes. On a warm path this overhead is invisible. At scale, during traffic spikes, in
geographical regions far from origin infrastructure, or during initial deployment, the cold start
becomes a measurable and compounding tax on every downstream user experience.`,

    section1_title: 'Why the Runtime Model Matters',
    section1: `WebAssembly changes the unit of deployment from a runtime environment to a compiled binary artifact.
When Rust source code is compiled to the wasm32-unknown-unknown target, the output is a deterministic,
portable bytecode module that carries no implicit runtime dependencies. There is no garbage collector
to initialise, no class loader to warm, no interpreter to JIT-compile. The Cloudflare Workers runtime
executes this module directly within a V8 isolate with a startup time measured in microseconds rather
than milliseconds — typically under 1 ms from a cold state across all 300+ Cloudflare edge nodes
globally.

The architectural implication is significant: an edge function compiled from Rust is not a
request-handler deployed at a single origin and replicated. It is a binary artefact instantiated
simultaneously at every point of presence, executing with CPU and memory characteristics equivalent
to a native process while maintaining the security boundary of a sandboxed isolate.`,

    section2_title: 'Overhead Elimination Through the Type System',
    section2: `Rust's ownership and borrowing model eliminates an entire class of runtime overhead that JIT-compiled
languages must continuously manage. Memory is deallocated deterministically at the end of a binding's
scope — not speculatively by a garbage collection cycle triggered at an unpredictable moment during
request handling. In a high-throughput edge environment processing thousands of concurrent requests
within shared memory pools, GC pauses represent an uncontrolled latency variable. Rust removes this
variable entirely.

Beyond memory, Rust's zero-cost abstractions mean that iterator chains, pattern matching, and
trait-based polymorphism compile to machine instructions with no indirection layer. The resulting
WASM binary is typically 50–80 % smaller than an equivalent Node.js bundle, which directly
reduces transfer time to the edge node, accelerates module parsing, and lowers peak memory pressure
inside the isolate. For Cloudflare Workers, which enforce a 10 MB compressed script size limit,
this compactness is not merely an efficiency gain — it is a deployment prerequisite for complex
applications.`,

    section3_title: 'Resource Footprint at Planetary Scale',
    section3: `Consider the operational economics. A Cloudflare Worker compiled from Rust consumes approximately
2–5 MB of resident memory per isolate, compared to 30–80 MB for an equivalent Express.js application
on a Node.js runtime. Cloudflare's infrastructure can therefore co-locate an order of magnitude more
isolates per physical CPU core, translating directly to lower cost per request and more predictable
tail latency under load.

The resource footprint advantage compounds when multiplied across a global edge network. An
application serving 50 million daily active users from 300 data centres requires neither horizontal
auto-scaling groups, nor load balancer provisioning, nor capacity planning events. The binary is
already present at every node, executing within microseconds of the request arriving. The operator
pays for what runs, not for what sits idle.

This is the first-principles argument for compiling to WebAssembly at the cloud edge: not as a
fashionable technology choice, but as the logical terminus of the question — what is the minimum
viable runtime required to execute this logic correctly? In Rust compiled to WASM, the answer
is: nothing more than the logic itself.`,
  } as const;
</script>

<svelte:head>
  <title>{data.post.title} — Technical Musings — eZeroAndOne</title>
  <meta name="description" content={data.post.summary} />
</svelte:head>

<main class="insight-detail">
  <nav class="breadcrumb" aria-label="Breadcrumb">
    <a href="/insights">← Technical Musings</a>
  </nav>

  <article class="post-article">
    <header class="post-header">
      <span class="post-type-label">Insight</span>
      <h1>{data.post.title}</h1>
      {#if data.post.author}
        <div class="author-meta">
          {#if data.post.author.avatar_url}
            <img src={data.post.author.avatar_url} alt={data.post.author.name} class="author-avatar" />
          {/if}
          <div>
            <span class="author-name">{data.post.author.name}</span>
            <span class="author-role">{data.post.author.job_title}</span>
          </div>
          {#if data.post.published_at}
            <time datetime={new Date(data.post.published_at * 1000).toISOString()} class="publish-date">
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
      <!-- Sample essay shown until authored content is published -->
      <div class="essay-body">
        <p class="essay-intro">{SAMPLE_ESSAY.intro}</p>

        <h2>{SAMPLE_ESSAY.section1_title}</h2>
        <p>{SAMPLE_ESSAY.section1}</p>

        <h2>{SAMPLE_ESSAY.section2_title}</h2>
        <p>{SAMPLE_ESSAY.section2}</p>

        <h2>{SAMPLE_ESSAY.section3_title}</h2>
        <p>{SAMPLE_ESSAY.section3}</p>
      </div>
    {/if}

    <footer class="article-footer">
      <a href="/insights" class="back-cta">← Back to Insights</a>
    </footer>
  </article>
</main>

<style>
  .insight-detail {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .breadcrumb { margin-bottom: 2rem; }

  .breadcrumb a {
    font-size: 0.875rem;
    color: var(--accent-blue);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .breadcrumb a:hover { color: var(--accent-green); text-decoration: underline; }

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
    color: var(--accent-blue);
    background: color-mix(in srgb, var(--accent-blue) 12%, transparent);
    border-radius: 4px;
    padding: 0.2rem 0.5rem;
    margin-bottom: 0.75rem;
  }

  .post-header h1 {
    font-size: clamp(1.75rem, 4vw, 2.75rem);
    font-weight: 800;
    margin: 0 0 1.25rem;
    color: var(--text-primary, var(--color-text-primary));
    letter-spacing: -0.02em;
    line-height: 1.15;
  }

  .author-meta {
    display: flex;
    align-items: center;
    gap: 0.75rem;
    font-size: 0.875rem;
    color: var(--text-secondary, var(--color-text-secondary));
    flex-wrap: wrap;
  }

  .author-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid var(--glass-border);
  }

  .author-name {
    font-weight: 600;
    display: block;
    color: var(--text-primary, var(--color-text-primary));
  }

  .author-role {
    font-size: 0.8rem;
    color: var(--text-secondary, var(--color-text-secondary));
    display: block;
  }

  .publish-date {
    margin-left: auto;
    font-size: 0.8rem;
    color: var(--text-secondary, var(--color-text-secondary));
    opacity: 0.7;
  }

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

  .post-body :global(h2) { font-size: 1.5rem; margin-top: 2.5rem; margin-bottom: 0.75rem; color: var(--text-primary, var(--color-text-primary)); }
  .post-body :global(h3) { font-size: 1.25rem; margin-top: 2rem; margin-bottom: 0.5rem; }
  .post-body :global(p)  { margin-bottom: 1.5rem; }
  .post-body :global(code) {
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    padding: 0.2em 0.4em; border-radius: 4px; font-size: 0.9em;
  }
  .post-body :global(pre) {
    background: var(--color-bg-elevated, rgba(255,255,255,0.08));
    padding: 1rem; border-radius: 8px; overflow-x: auto; margin-bottom: 1.5rem;
  }
  .post-body :global(pre code) { background: none; padding: 0; }
  .post-body :global(a) { color: var(--accent-blue); text-decoration: underline; }
  .post-body :global(a:hover) { color: var(--accent-green); }

  /* Essay (sample) body */
  .essay-body {
    padding: 2rem 2.5rem;
    font-size: 1rem;
    line-height: 1.85;
    color: var(--text-secondary, var(--color-text-secondary));
  }

  .essay-intro {
    font-size: 1.05rem;
    font-style: italic;
    border-left: 3px solid var(--accent-blue);
    padding-left: 1.25rem;
    margin-bottom: 2rem;
    color: var(--text-primary, var(--color-text-primary));
    white-space: pre-line;
  }

  .essay-body h2 {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--text-primary, var(--color-text-primary));
    margin: 2.5rem 0 0.75rem;
    letter-spacing: -0.01em;
  }

  .essay-body p {
    margin: 0 0 1.5rem;
    white-space: pre-line;
  }

  .article-footer {
    padding: 1.5rem 2.5rem;
    border-top: 1px solid var(--glass-border);
  }

  .back-cta {
    font-size: 0.875rem;
    color: var(--accent-blue);
    text-decoration: none;
  }

  .back-cta:hover { text-decoration: underline; }
</style>
