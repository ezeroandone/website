<script lang="ts">
  import { page } from '$app/stores';
  import { theme } from '$lib/stores/theme';
  import type { LayoutProps } from './$types';

  let { data, children }: LayoutProps = $props();

  const navItems = [
    { href: '/admin/dashboard', label: 'Dashboard', icon: 'dashboard'  },
    { href: '/admin/staff',     label: 'Staff',     icon: 'group'      },
    { href: '/admin/careers',   label: 'Careers',   icon: 'work'       },
    { href: '/admin/content',   label: 'Content',   icon: 'edit_note'  },
    { href: '/admin/clients',   label: 'Clients',   icon: 'business'   },
  ] as const;
</script>

<svelte:head>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<div class="admin-shell">
  <!-- ── Sidebar ──────────────────────────────────────────── -->
  <aside class="sidebar">
    <div class="sidebar-brand">
      <a href="/admin/dashboard" class="brand-link" aria-label="Admin dashboard">
        <span class="brand-mark">e<span class="brand-accent">0</span>&amp;<span class="brand-accent">1</span></span>
        <span class="brand-sub">Admin</span>
      </a>
    </div>

    <nav class="sidebar-nav" aria-label="Admin navigation">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-link"
          class:active={$page.url.pathname.startsWith(item.href)}
          aria-current={$page.url.pathname.startsWith(item.href) ? 'page' : undefined}
        >
          <span class="material-icons-outlined nav-icon" aria-hidden="true">{item.icon}</span>
          <span class="nav-label">{item.label}</span>
          {#if $page.url.pathname.startsWith(item.href)}
            <span class="active-pip" aria-hidden="true"></span>
          {/if}
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <div class="user-info">
        <div class="user-avatar" aria-hidden="true">
          {data.session.email.charAt(0).toUpperCase()}
        </div>
        <div class="user-details">
          <span class="user-email">{data.session.email}</span>
          <span class="user-role">{data.session.role}</span>
        </div>
      </div>
      <a href="/api/auth/logout" class="logout-btn" title="Sign out">
        <span class="material-icons-outlined" aria-hidden="true">logout</span>
      </a>
    </div>
  </aside>

  <!-- ── Main content ──────────────────────────────────────── -->
  <main class="admin-main">
    <div class="admin-content">
      {@render children()}
    </div>
  </main>
</div>

<style>
  /* ── Shell ───────────────────────────────────────────────── */
  .admin-shell {
    display: flex;
    height: 100vh;
    overflow: hidden;
    background: #000;
    color: #f0f0f0;
    /* Reset body padding-top set for public header */
    margin-top: calc(-1 * var(--header-height, 64px));
  }

  /* ── Sidebar ─────────────────────────────────────────────── */
  .sidebar {
    width: 224px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    background: #050508;
    border-right: 1px solid rgba(255,255,255,0.06);
    overflow: hidden;
  }

  /* Brand */
  .sidebar-brand {
    padding: 22px 18px 18px;
    border-bottom: 1px solid rgba(255,255,255,0.06);
  }

  .brand-link {
    display: flex;
    align-items: baseline;
    gap: 8px;
    text-decoration: none;
  }

  .brand-mark {
    font-family: var(--font-heading, 'Inter Tight', sans-serif);
    font-size: 1.2rem;
    font-weight: 800;
    letter-spacing: -0.04em;
    color: #fff;
  }

  .brand-accent { color: var(--accent-blue-hi, #00C2FF); }

  .brand-sub {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.12em;
    color: rgba(255,255,255,0.3);
  }

  /* Nav */
  .sidebar-nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 14px 10px;
    overflow-y: auto;
    scrollbar-width: none;
  }

  .sidebar-nav::-webkit-scrollbar { display: none; }

  .nav-link {
    position: relative;
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 9px 12px;
    border-radius: 8px;
    color: rgba(255,255,255,0.45);
    text-decoration: none;
    font-size: 0.875rem;
    font-weight: 500;
    transition: background 0.15s, color 0.15s;
    overflow: hidden;
  }

  .nav-link:hover {
    background: rgba(255,255,255,0.06);
    color: rgba(255,255,255,0.85);
    text-decoration: none;
  }

  .nav-link.active {
    background: rgba(0,194,255,0.1);
    color: #00C2FF;
  }

  .nav-icon {
    font-size: 1.15rem;
    flex-shrink: 0;
    opacity: 0.7;
    transition: opacity 0.15s;
  }

  .nav-link:hover .nav-icon,
  .nav-link.active .nav-icon { opacity: 1; }

  .nav-label { flex: 1; }

  .active-pip {
    width: 3px;
    height: 16px;
    background: #00C2FF;
    border-radius: 3px;
    flex-shrink: 0;
  }

  /* Footer */
  .sidebar-footer {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 12px 14px;
    border-top: 1px solid rgba(255,255,255,0.06);
  }

  .user-info {
    display: flex;
    align-items: center;
    gap: 10px;
    flex: 1;
    min-width: 0;
  }

  .user-avatar {
    width: 30px;
    height: 30px;
    border-radius: 50%;
    background: rgba(0,194,255,0.15);
    border: 1px solid rgba(0,194,255,0.3);
    display: flex;
    align-items: center;
    justify-content: center;
    font-size: 0.75rem;
    font-weight: 700;
    color: #00C2FF;
    flex-shrink: 0;
  }

  .user-details {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .user-email {
    font-size: 0.72rem;
    color: rgba(255,255,255,0.5);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-role {
    font-size: 0.6rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #00C2FF;
  }

  .logout-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: 6px;
    color: rgba(255,255,255,0.3);
    text-decoration: none;
    transition: background 0.15s, color 0.15s;
    flex-shrink: 0;
  }

  .logout-btn:hover {
    background: rgba(255,51,102,0.12);
    color: #ff3366;
    text-decoration: none;
  }

  .logout-btn .material-icons-outlined { font-size: 1.1rem; }

  /* ── Main area ───────────────────────────────────────────── */
  .admin-main {
    flex: 1;
    overflow-y: auto;
    background: #080810;
    /* Thin themed scrollbar */
    scrollbar-width: thin;
    scrollbar-color: rgba(255,255,255,0.1) transparent;
  }

  .admin-main::-webkit-scrollbar { width: 4px; }
  .admin-main::-webkit-scrollbar-track { background: transparent; }
  .admin-main::-webkit-scrollbar-thumb {
    background: rgba(255,255,255,0.1);
    border-radius: 4px;
  }
  .admin-main::-webkit-scrollbar-thumb:hover {
    background: rgba(255,255,255,0.2);
  }

  .admin-content {
    padding: 32px 36px;
    max-width: 1200px;
  }

  /* ── Global admin form theming ───────────────────────────── */
  /* Propagates to all child pages via the layout */
  :global(.admin-shell input),
  :global(.admin-shell textarea),
  :global(.admin-shell select) {
    color-scheme: dark;
    background: #0d0d18;
    border: 1px solid rgba(255,255,255,0.1);
    color: #f0f0f0;
    border-radius: 8px;
    padding: 9px 13px;
    font-size: 0.875rem;
    font-family: inherit;
    width: 100%;
    box-sizing: border-box;
    appearance: none;
    -webkit-appearance: none;
    transition: border-color 0.2s;
  }

  :global(.admin-shell input:focus),
  :global(.admin-shell textarea:focus),
  :global(.admin-shell select:focus) {
    outline: none;
    border-color: #00C2FF;
    box-shadow: 0 0 0 3px rgba(0,194,255,0.15);
  }

  :global(.admin-shell input::placeholder),
  :global(.admin-shell textarea::placeholder) {
    color: rgba(255,255,255,0.25);
  }

  :global(.admin-shell select option) {
    background: #0d0d18;
    color: #f0f0f0;
  }
</style>
