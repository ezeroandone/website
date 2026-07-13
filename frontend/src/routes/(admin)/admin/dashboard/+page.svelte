<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import type { PageProps } from './$types';

  // data comes from the admin layout server load (session is on layout data)
  let { data }: PageProps = $props();

  const quickLinks = [
    {
      href: '/admin/staff',
      title: 'Staff Registry',
      description: 'Manage engineer profiles, assign roles, and advance lifecycle status.',
      accent: 'blue',
      icon: '👤',
    },
    {
      href: '/admin/careers',
      title: 'Active Operations',
      description: 'Publish deployment listings and process the full applicant pipeline.',
      accent: 'green',
      icon: '💼',
    },
    {
      href: '/admin/content',
      title: 'Content Engine',
      description: 'Author and publish insights, case studies, and capability deep-dives.',
      accent: 'yellow',
      icon: '📄',
    },
  ] as const satisfies ReadonlyArray<{
    href: string;
    title: string;
    description: string;
    accent: 'red' | 'blue' | 'green' | 'yellow';
    icon: string;
  }>;
</script>

<svelte:head>
  <title>Dashboard — eZeroAndOne Admin</title>
</svelte:head>

<section class="dashboard">
  <header class="dashboard-header">
    <h1 class="dashboard-title">Command Centre</h1>
    <p class="dashboard-subtitle">Authenticated node: <strong>{data.session?.email ?? 'Admin'}</strong></p>
  </header>

  <div class="cards-grid">
    {#each quickLinks as link}
      <a href={link.href} class="card-link" aria-label="Navigate to {link.title}">
        <GlassCard accentColor={link.accent}>
          <div class="card-body">
            <span class="card-icon" aria-hidden="true">{link.icon}</span>
            <div class="card-text">
              <h2 class="card-title">{link.title}</h2>
              <p class="card-description">{link.description}</p>
            </div>
          </div>
        </GlassCard>
      </a>
    {/each}
  </div>
</section>

<style>
  .dashboard {
    max-width: 960px;
  }

  .dashboard-header {
    margin-bottom: 32px;
  }

  .dashboard-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 6px;
    letter-spacing: -0.5px;
  }

  .dashboard-subtitle {
    font-size: 0.95rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
  }

  .dashboard-subtitle strong {
    color: var(--color-text-primary, #f0f0f0);
    font-weight: 600;
  }

  /* ── Cards ───────────────────────────────────────────────── */
  .cards-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(260px, 1fr));
    gap: 20px;
  }

  .card-link {
    display: block;
    text-decoration: none;
    color: inherit;
  }

  .card-link:hover {
    text-decoration: none;
  }

  .card-body {
    display: flex;
    align-items: flex-start;
    gap: 16px;
    padding: 24px;
  }

  .card-icon {
    font-size: 1.75rem;
    flex-shrink: 0;
    line-height: 1;
  }

  .card-text {
    flex: 1;
    min-width: 0;
  }

  .card-title {
    font-size: 1.1rem;
    font-weight: 700;
    margin: 0 0 6px;
    color: var(--color-text-primary, #f0f0f0);
  }

  .card-description {
    font-size: 0.85rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
    line-height: 1.5;
  }
</style>
