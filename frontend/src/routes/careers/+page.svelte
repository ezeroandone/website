<script lang="ts">
  import type { PageData } from './$types';
  import GlassCard from '$lib/components/GlassCard.svelte';

  let { data }: { data: PageData } = $props();

  // Colour-code by employment type
  const typeAccent: Record<string, 'red' | 'blue' | 'green' | 'yellow'> = {
    'Full-Time': 'blue',
    'Part-Time': 'green',
    'Contract': 'yellow',
    'Internship': 'red',
  };

  function accentFor(type: string): 'red' | 'blue' | 'green' | 'yellow' {
    return typeAccent[type] ?? 'blue';
  }

  function formatDate(unixSeconds: number): string {
    return new Date(unixSeconds * 1000).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Active Operations — Careers at eZeroAndOne</title>
  <meta name="description" content="Join the core team building lasting systems. Open engineering positions at eZeroAndOne." />
</svelte:head>

<main class="careers-page">
  <header class="page-header">
    <h1>Active Operations</h1>
    <p class="subtitle">Join the core team building lasting systems.</p>
  </header>

  {#if data.careers.length > 0}
    <div class="careers-list">
      {#each data.careers as career}
        <a href="/careers/{career.slug}" class="career-link">
          <GlassCard accentColor={accentFor(career.type)}>
            <div class="career-card-content">
              <div class="career-main">
                <div class="career-badges">
                  <span class="badge badge-type badge-{accentFor(career.type)}">
                    {career.type}
                  </span>
                  {#if career.department}
                    <span class="badge badge-dept">{career.department}</span>
                  {/if}
                </div>
                <h2 class="career-title">{career.title}</h2>
              </div>
              <div class="career-meta">
                <time datetime={new Date(career.created_at * 1000).toISOString()}>
                  Posted {formatDate(career.created_at)}
                </time>
                <span class="apply-cta" aria-hidden="true">Apply →</span>
              </div>
            </div>
          </GlassCard>
        </a>
      {/each}
    </div>
  {:else}
    <div class="empty-state">
      <p>No active operations at the moment. The next deployment cycle opens soon.</p>
    </div>
  {/if}
</main>

<style>
  .careers-page {
    max-width: 900px;
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

  .careers-list {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .career-link {
    display: block;
    text-decoration: none;
    color: inherit;
  }

  .career-card-content {
    padding: 1.5rem;
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  @media (max-width: 560px) {
    .career-card-content {
      flex-direction: column;
      align-items: flex-start;
    }
  }

  .career-main {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
  }

  .career-badges {
    display: flex;
    gap: 0.5rem;
    flex-wrap: wrap;
  }

  .badge {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    border: 1px solid currentColor;
  }

  .badge-blue   { color: var(--accent-blue); }
  .badge-green  { color: var(--accent-green); }
  .badge-yellow { color: var(--accent-yellow); }
  .badge-red    { color: var(--accent-red); }

  .badge-dept {
    color: var(--color-text-muted);
    border-color: var(--color-border);
  }

  .career-title {
    font-size: 1.35rem;
    font-weight: 600;
    margin: 0;
    color: var(--color-text-primary);
  }

  .career-meta {
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 0.4rem;
    white-space: nowrap;
  }

  .career-meta time {
    font-size: 0.8rem;
    color: var(--color-text-muted);
  }

  .apply-cta {
    font-size: 0.875rem;
    font-weight: 600;
    color: var(--accent-blue);
  }

  .empty-state {
    text-align: center;
    padding: 4rem;
    color: var(--color-text-muted);
  }
</style>
