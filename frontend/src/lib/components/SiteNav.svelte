<script lang="ts">
  import FloatingHeader from './FloatingHeader.svelte';
  import { page } from '$app/stores';

  const navLinks = [
    { href: '/capabilities', label: 'Capabilities' },
    { href: '/work',         label: 'Work'         },
    { href: '/insights',     label: 'Insights'     },
    { href: '/careers',      label: 'Careers'      },
    { href: '/team',         label: 'Team'         },
  ];

  // If the user has a session cookie, send them straight to the dashboard.
  // This is checked client-side via a prop passed from the layout server load.
  interface Props {
    isLoggedIn?: boolean;
  }
  let { isLoggedIn = false }: Props = $props();

  const portalHref = $derived(isLoggedIn ? '/admin/dashboard' : '/auth/login');
</script>

<FloatingHeader>
  <nav class="nav-inner">
    <a href="/" class="brand" aria-label="eZeroAndOne.io home">
      e<span class="brand-accent">0</span>&amp;<span class="brand-accent">1</span><span class="brand-tld">.io</span>
    </a>
    <ul class="nav-links" role="list">
      {#each navLinks as link}
        <li>
          <a
            href={link.href}
            aria-current={$page.url.pathname.startsWith(link.href) ? 'page' : undefined}
          >{link.label}</a>
        </li>
      {/each}
    </ul>
    <div class="nav-actions">
      <a href={portalHref} class="btn btn-secondary nav-signin">
        <span class="material-symbols-outlined" aria-hidden="true">lock</span>
        Staff Portal
      </a>
      <a href="/capabilities" class="btn btn-primary nav-cta">
        Initialize Project
      </a>
    </div>
  </nav>
</FloatingHeader>

<style>
  .nav-inner {
    display: flex; align-items: center; gap: 2rem;
    max-width: 1200px; margin: 0 auto; width: 100%;
  }
  .brand {
    font-family: var(--font-heading); font-size: 1.25rem; font-weight: 800;
    color: #fff; text-decoration: none; letter-spacing: -0.04em; flex-shrink: 0;
  }
  .brand-accent { color: var(--accent-blue-hi, #00d4ff); }
  .nav-links {
    display: flex; list-style: none; margin: 0; padding: 0; gap: 1.75rem; flex: 1;
  }
  .nav-links a {
    font-family: var(--font-body); color: rgba(255,255,255,0.5); font-size: 0.9rem;
    font-weight: 500; text-decoration: none; letter-spacing: 0.02em;
    transition: color 0.2s ease;
  }
  .nav-links a:hover,
  .nav-links a[aria-current="page"] { color: #fff; text-decoration: none; }
  .brand-tld { color: rgba(255,255,255,0.35); font-size: 0.85em; }
  .nav-actions { display: flex; align-items: center; gap: 0.75rem; flex-shrink: 0; }
  .nav-signin { padding: 0.45rem 1rem; font-size: 0.75rem; display: flex; align-items: center; gap: 0.35rem; }
  .nav-cta    { padding: 0.45rem 1.25rem; font-size: 0.75rem; }
  @media (max-width: 640px) {
    .nav-links { display: none; }
    .nav-signin { display: none; }
  }
</style>
