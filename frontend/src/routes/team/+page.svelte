<script lang="ts">
  import type { PageData } from './$types';
  import GlassCard from '$lib/components/GlassCard.svelte';

  let { data }: { data: PageData } = $props();
</script>

<svelte:head>
  <title>The Engineers — eZeroAndOne Team</title>
  <meta name="description" content="The minds breaking down complex paradigms into executable code. Meet the eZeroAndOne engineering team." />
</svelte:head>

<main class="team-page">
  <header class="page-header">
    <h1>The Engineers</h1>
    <p class="subtitle">The minds breaking down complex paradigms into executable code.</p>
  </header>

  <div class="staff-grid">
    {#each data.staff as member}
      <a href="/team/{member.username}" class="staff-link">
        <GlassCard accentColor="blue">
          <div class="staff-card-content">
            {#if member.avatar_url}
              <img
                src={member.avatar_url}
                alt="{member.name}'s avatar"
                class="avatar"
                loading="lazy"
              />
            {:else}
              <div class="avatar-placeholder" aria-hidden="true">
                {member.name.charAt(0).toUpperCase()}
              </div>
            {/if}
            <div class="staff-info">
              <h2 class="staff-name">{member.name}</h2>
              <p class="staff-title">{member.job_title}</p>
              {#if member.bio}
                <p class="staff-bio">{member.bio}</p>
              {/if}
            </div>
          </div>
        </GlassCard>
      </a>
    {/each}
  </div>

  {#if data.staff.length === 0}
    <div class="empty-state">
      <p>No engineers listed yet. Check back soon.</p>
    </div>
  {/if}
</main>

<style>
  .team-page {
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

  .staff-grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
    gap: 2rem;
  }

  .staff-link {
    display: block;
    text-decoration: none;
    color: inherit;
  }

  .staff-card-content {
    padding: 1.5rem;
    display: flex;
    flex-direction: column;
    align-items: center;
    text-align: center;
    gap: 1rem;
  }

  .avatar {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    object-fit: cover;
    border: 2px solid var(--glass-border);
  }

  .avatar-placeholder {
    width: 80px;
    height: 80px;
    border-radius: 50%;
    background: var(--color-bg-elevated);
    border: 2px solid var(--glass-border);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 2rem;
    font-weight: 700;
    color: var(--accent-blue);
  }

  .staff-info {
    display: flex;
    flex-direction: column;
    gap: 0.25rem;
  }

  .staff-name {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: var(--color-text-primary);
  }

  .staff-title {
    font-size: 0.875rem;
    color: var(--accent-blue);
    font-weight: 500;
    margin: 0;
  }

  .staff-bio {
    font-size: 0.875rem;
    line-height: 1.5;
    color: var(--color-text-secondary);
    margin: 0.5rem 0 0 0;
    display: -webkit-box;
    -webkit-line-clamp: 3;
    line-clamp: 3;
    -webkit-box-orient: vertical;
    overflow: hidden;
  }

  .empty-state {
    text-align: center;
    padding: 4rem;
    color: var(--color-text-muted);
  }
</style>
