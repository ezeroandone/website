<script lang="ts">
  import { page } from '$app/stores';
  import { theme } from '$lib/stores/theme';
  import type { LayoutProps } from './$types';

  let { data, children }: LayoutProps = $props();

  const navItems = [
    { href: '/admin/dashboard', label: 'Dashboard', icon: '⬛' },
    { href: '/admin/staff',     label: 'Staff',     icon: '👤' },
    { href: '/admin/careers',   label: 'Careers',   icon: '💼' },
    { href: '/admin/content',   label: 'Content',   icon: '📄' },
  ] as const;
</script>

<div class="admin-shell" data-theme={$theme}>
  <!-- Sidebar navigation -->
  <aside class="sidebar">
    <div class="sidebar-brand">
      <span class="brand-mark">eZO</span>
      <span class="brand-label">Admin</span>
    </div>

    <nav class="sidebar-nav" aria-label="Admin navigation">
      {#each navItems as item}
        <a
          href={item.href}
          class="nav-link"
          class:active={$page.url.pathname.startsWith(item.href)}
          aria-current={$page.url.pathname.startsWith(item.href) ? 'page' : undefined}
        >
          <span class="nav-icon" aria-hidden="true">{item.icon}</span>
          <span class="nav-label">{item.label}</span>
        </a>
      {/each}
    </nav>

    <div class="sidebar-footer">
      <span class="user-email">{data.session.email}</span>
      <span class="user-role">{data.session.role}</span>
    </div>
  </aside>

  <!-- Main content area -->
  <main class="admin-main">
    {@render children()}
  </main>
</div>

<style>
  .admin-shell {
    display: flex;
    min-height: 100vh;
    background-color: var(--color-bg-primary, #0a0a0f);
    color: var(--color-text-primary, #f0f0f0);
  }

  /* ── Sidebar ─────────────────────────────────────────────── */
  .sidebar {
    width: 220px;
    flex-shrink: 0;
    display: flex;
    flex-direction: column;
    background: var(--glass-bg, rgba(255, 255, 255, 0.04));
    backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    -webkit-backdrop-filter: blur(var(--glass-blur, 16px)) saturate(180%);
    border-right: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
  }

  .sidebar-brand {
    display: flex;
    align-items: baseline;
    gap: 6px;
    padding: 24px 20px 16px;
    border-bottom: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
  }

  .brand-mark {
    font-size: 1.25rem;
    font-weight: 700;
    color: var(--accent-blue, #00d4ff);
    letter-spacing: -0.5px;
  }

  .brand-label {
    font-size: 0.75rem;
    font-weight: 500;
    color: var(--color-text-secondary, #888899);
    text-transform: uppercase;
    letter-spacing: 1px;
  }

  /* ── Navigation links ────────────────────────────────────── */
  .sidebar-nav {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 12px 10px;
  }

  .nav-link {
    display: flex;
    align-items: center;
    gap: 10px;
    padding: 10px 12px;
    border-radius: 10px;
    color: var(--color-text-secondary, #888899);
    text-decoration: none;
    font-size: 0.9rem;
    font-weight: 500;
    transition: all 0.2s ease;
  }

  .nav-link:hover {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.08));
    color: var(--color-text-primary, #f0f0f0);
    text-decoration: none;
  }

  .nav-link.active {
    background: color-mix(in srgb, var(--accent-blue, #00d4ff) 15%, transparent);
    color: var(--accent-blue, #00d4ff);
    box-shadow: inset 0 0 0 1px color-mix(in srgb, var(--accent-blue, #00d4ff) 30%, transparent);
  }

  .nav-icon {
    font-size: 1rem;
    width: 20px;
    text-align: center;
  }

  .nav-label {
    flex: 1;
  }

  /* ── Sidebar footer ──────────────────────────────────────── */
  .sidebar-footer {
    display: flex;
    flex-direction: column;
    gap: 2px;
    padding: 14px 20px;
    border-top: 1px solid var(--glass-border, rgba(255, 255, 255, 0.08));
  }

  .user-email {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #888899);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .user-role {
    font-size: 0.65rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.8px;
    color: var(--accent-green, #00ff88);
  }

  /* ── Main content ────────────────────────────────────────── */
  .admin-main {
    flex: 1;
    overflow-y: auto;
    padding: 32px;
    background-color: var(--color-bg-primary, #0a0a0f);
  }
</style>
