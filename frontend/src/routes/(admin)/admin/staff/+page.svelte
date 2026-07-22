<!--
  Admin Staff Management Page — Task 22.2
  Requirements: 8.3, 8.5, 15.4

  Renders a staff table with:
  - Confirm-probation action  → POST /api/admin/staff/:id/confirm   (Requirement 8.3)
  - Update-role action         → PATCH /api/admin/staff/:id/role    (Requirement 8.5)

  SuperAdmin role assignment is gated on the acting user also being SuperAdmin
  (Requirement 8.7).
-->

<script lang="ts">
  import GlassCard from '$lib/components/GlassCard.svelte';
  import type { PageProps } from './$types';
  import type { StaffAdmin } from './+page.server';

  let { data }: PageProps = $props();

  // ── Brand copy — lifecycle state messages (brand spec §3.E) ─────────────
  const LIFECYCLE_COPY: Record<string, string> = {
    Applied:   'Submission Received. Your technical parameters are undergoing initial integrity parsing.',
    Probation: 'Onboarding Phase Active. Assigned internal domain credential provisioning initialized.',
    Confirmed: 'Tenure Confirmed. Full administrative rights, project keys, and organizational permissions unlocked.',
  };

  // Role levels mirror the Rust RBAC definition (Requirement 3.1)
  const ROLE_LEVEL: Record<string, number> = {
    Public: 1,
    Staff: 2,
    Admin: 3,
    SuperAdmin: 4,
  };

  const VALID_ROLES: StaffAdmin['role'][] = ['Staff', 'Admin', 'SuperAdmin'];

  // State: per-row role selector value — keyed by staff id
  let pendingRoles = $state<Record<string, StaffAdmin['role']>>(
    Object.fromEntries(data.staff.map((s) => [s.id, s.role]))
  );

  // Action status messages — keyed by staff id
  let actionMessages = $state<Record<string, { type: 'success' | 'error'; text: string }>>({});
  let loadingIds = $state<Set<string>>(new Set());

  // ── Helpers ──────────────────────────────────────────────────────────

  function isLoading(id: string): boolean {
    return loadingIds.has(id);
  }

  function setLoading(id: string, on: boolean) {
    loadingIds = new Set(loadingIds);
    if (on) {
      loadingIds.add(id);
    } else {
      loadingIds.delete(id);
    }
  }

  function setMessage(id: string, type: 'success' | 'error', text: string) {
    actionMessages = { ...actionMessages, [id]: { type, text } };
    // Auto-clear after 4 seconds
    setTimeout(() => {
      actionMessages = { ...actionMessages };
      delete actionMessages[id];
    }, 4000);
  }

  // Acting user's role from the layout session data (provided by +layout.server.ts)
  const actingRole: string = data.session?.role ?? 'Admin';
  const actingLevel = ROLE_LEVEL[actingRole] ?? 3;

  // Determine which roles the acting user can assign (Requirement 8.7)
  function assignableRoles(staff: StaffAdmin): StaffAdmin['role'][] {
    // SuperAdmin assignment requires the acting user to also be SuperAdmin
    return VALID_ROLES.filter(
      (r) => r !== 'SuperAdmin' || actingLevel >= ROLE_LEVEL['SuperAdmin']
    );
  }

  // ── Actions ──────────────────────────────────────────────────────────

  /**
   * Confirm a staff member's probation → Confirmed (Requirement 8.3)
   * POST /api/admin/staff/:id/confirm
   */
  async function confirmProbation(staffId: string) {
    if (isLoading(staffId)) return;
    setLoading(staffId, true);

    try {
      const res = await fetch(`/api/admin/staff/${staffId}/confirm`, {
        method: 'POST',
        credentials: 'include',
      });

      if (res.ok) {
        // Optimistically update the local status
        const idx = data.staff.findIndex((s) => s.id === staffId);
        if (idx !== -1) {
          data.staff[idx] = { ...data.staff[idx], status: 'Confirmed' };
        }
        setMessage(staffId, 'success', 'Status confirmed.');
      } else {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        setMessage(staffId, 'error', body.error ?? 'Confirm failed.');
      }
    } catch {
      setMessage(staffId, 'error', 'Network error — please try again.');
    } finally {
      setLoading(staffId, false);
    }
  }

  /**
   * Update a staff member's role (Requirement 8.5)
   * PATCH /api/admin/staff/:id/role  — body: { role }
   */
  async function updateRole(staffId: string) {
    if (isLoading(staffId)) return;
    const newRole = pendingRoles[staffId];
    if (!newRole) return;

    setLoading(staffId, true);

    try {
      const res = await fetch(`/api/admin/staff/${staffId}/role`, {
        method: 'PATCH',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ role: newRole }),
      });

      if (res.ok) {
        const idx = data.staff.findIndex((s) => s.id === staffId);
        if (idx !== -1) {
          data.staff[idx] = { ...data.staff[idx], role: newRole };
        }
        setMessage(staffId, 'success', `Role updated to ${newRole}.`);
      } else {
        const body = await res.json().catch(() => ({ error: res.statusText }));
        setMessage(staffId, 'error', body.error ?? 'Role update failed.');
      }
    } catch {
      setMessage(staffId, 'error', 'Network error — please try again.');
    } finally {
      setLoading(staffId, false);
    }
  }

  // ── Display helpers ──────────────────────────────────────────────────

  async function downloadQR(member: StaffAdmin) {
    const profileUrl = `https://ezeroandone.io/team/${member.username}`;
    try {
      const QRCode = await import('qrcode');
      const dataUrl = await QRCode.toDataURL(profileUrl, {
        width: 400,
        margin: 2,
        color: { dark: '#000000', light: '#ffffff' },
      });
      const link = document.createElement('a');
      link.href = dataUrl;
      link.download = `qr-${member.username || member.id}.png`;
      link.click();
    } catch (err) {
      console.error('[QR download]', err);
    }
  }

  function roleBadgeClass(role: StaffAdmin['role']): string {
    return {
      SuperAdmin: 'badge badge--red',
      Admin: 'badge badge--blue',
      Staff: 'badge badge--green',
    }[role] ?? 'badge';
  }

  function statusBadgeClass(status?: string): string {
    return {
      Confirmed: 'badge badge--green',
      Probation: 'badge badge--yellow',
      Inactive: 'badge badge--muted',
    }[status ?? ''] ?? 'badge badge--muted';
  }

  function formatDate(ts: number): string {
    return new Date(ts * 1000).toLocaleDateString('en-GB', {
      day: 'numeric',
      month: 'short',
      year: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>Staff Management — eZeroAndOne Admin</title>
  <link href="https://fonts.googleapis.com/icon?family=Material+Icons+Outlined" rel="stylesheet" />
</svelte:head>

<section class="staff-page">
  <!-- ── Page header ──────────────────────────────────────── -->
  <header class="page-header">
    <div class="page-header-text">
      <h1 class="page-title">Staff Registry</h1>
      <p class="page-subtitle">
        {data.staff.length} engineer{data.staff.length !== 1 ? 's' : ''} on the manifest
      </p>
    </div>
  </header>

  <!-- ── Empty state ──────────────────────────────────────── -->
  {#if data.staff.length === 0}
    <GlassCard accentColor="blue">
      <div class="empty-state">
        <span class="material-icons-outlined empty-icon" aria-hidden="true">group</span>
        <p>No staff members yet.</p>
      </div>
    </GlassCard>
  {:else}
    <!-- ── Staff table ───────────────────────────────────── -->
    <GlassCard accentColor="blue">
      <div class="table-wrapper" role="region" aria-label="Staff members">
        <table class="staff-table">
          <thead>
            <tr>
              <th scope="col">Member</th>
              <th scope="col">Role</th>
              <th scope="col">Status</th>
              <th scope="col">Joined</th>
              <th scope="col" class="col-actions">Actions</th>
            </tr>
          </thead>
          <tbody>
            {#each data.staff as member (member.id)}
              <tr class:row--loading={isLoading(member.id)}>
                <!-- Member info -->
                <td class="cell-member">
                  <div class="member-info">
                    {#if member.avatar_url}
                      <img
                        class="member-avatar"
                        src={member.avatar_url}
                        alt=""
                        aria-hidden="true"
                        width="36"
                        height="36"
                      />
                    {:else}
                      <div class="member-avatar member-avatar--placeholder" aria-hidden="true">
                        {(member.name || member.email).charAt(0).toUpperCase()}
                      </div>
                    {/if}
                    <div class="member-details">
                      <span class="member-name">{member.name || '—'}</span>
                      <span class="member-email">{member.email}</span>
                      {#if member.username}
                        <span class="member-username">@{member.username}</span>
                      {/if}
                    </div>
                  </div>
                </td>

                <!-- Role badge -->
                <td>
                  <span class={roleBadgeClass(member.role)}>{member.role}</span>
                </td>

                <!-- Lifecycle status badge with lifecycle copy as tooltip -->
                <td>
                  <span
                    class={statusBadgeClass(member.status)}
                    title={LIFECYCLE_COPY[member.status ?? ''] ?? member.status}
                  >
                    {member.status ?? 'Unknown'}
                  </span>
                </td>

                <!-- Joined date -->
                <td class="cell-date">
                  <time datetime={new Date(member.created_at * 1000).toISOString()}>
                    {formatDate(member.created_at)}
                  </time>
                </td>

                <!-- Actions -->
                <td class="cell-actions">
                  <div class="action-group">
                    <!-- Confirm probation (only shown when status = Probation) -->
                    {#if member.status === 'Probation'}
                      <button
                        class="btn btn--confirm"
                        onclick={() => confirmProbation(member.id)}
                        disabled={isLoading(member.id)}
                        aria-label="Confirm probation for {member.name || member.email}"
                      >
                        {isLoading(member.id) ? '…' : 'Confirm'}
                      </button>
                    {/if}

                    <!-- Role selector + update button -->
                    <div class="role-update">
                      <label class="sr-only" for="role-{member.id}">
                        Change role for {member.name || member.email}
                      </label>
                      <select
                        id="role-{member.id}"
                        class="role-select"
                        bind:value={pendingRoles[member.id]}
                        disabled={isLoading(member.id)}
                        aria-label="Select new role for {member.name || member.email}"
                      >
                        {#each assignableRoles(member) as role}
                          <option value={role}>{role}</option>
                        {/each}
                      </select>
                      <button
                        class="btn btn--update"
                        onclick={() => updateRole(member.id)}
                        disabled={isLoading(member.id) || pendingRoles[member.id] === member.role}
                        aria-label="Apply role change for {member.name || member.email}"
                      >
                        {isLoading(member.id) ? '…' : 'Apply'}
                      </button>
                    </div>

                    {#if member.username}
                      <button
                        class="btn btn--qr"
                        type="button"
                        onclick={() => downloadQR(member)}
                        aria-label="Download QR code for {member.name || member.email}"
                        title="Download identity QR code"
                      >
                        <span class="material-icons-outlined" style="font-size:1rem">qr_code_2</span>
                        QR
                      </button>
                    {/if}
                  </div>
                  </div>

                  <!-- Inline action feedback message -->
                  {#if actionMessages[member.id]}
                    <p
                      class="action-msg action-msg--{actionMessages[member.id].type}"
                      role="status"
                      aria-live="polite"
                    >
                      {actionMessages[member.id].text}
                    </p>
                  {/if}
                </td>
              </tr>
            {/each}
          </tbody>
        </table>
      </div>
    </GlassCard>
  {/if}
</section>

<style>
  /* ── Page layout ───────────────────────────────────────── */
  .staff-page {
    max-width: 1100px;
  }

  .page-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    margin-bottom: 28px;
  }

  .page-title {
    font-size: 2rem;
    font-weight: 700;
    color: var(--color-text-primary, #f0f0f0);
    margin: 0 0 4px;
    letter-spacing: -0.5px;
  }

  .page-subtitle {
    font-size: 0.875rem;
    color: var(--color-text-secondary, #888899);
    margin: 0;
  }

  /* ── Empty state ───────────────────────────────────────── */
  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    padding: 48px 24px;
    color: var(--color-text-secondary, #888899);
    font-size: 0.95rem;
  }

  .empty-icon {
    font-size: 2.5rem;
  }

  /* ── Table wrapper ─────────────────────────────────────── */
  .table-wrapper {
    overflow-x: auto;
    -webkit-overflow-scrolling: touch;
  }

  .staff-table {
    width: 100%;
    border-collapse: collapse;
    font-size: 0.875rem;
    color: var(--color-text-primary, #f0f0f0);
  }

  .staff-table thead tr {
    border-bottom: 1px solid var(--glass-border, rgba(255, 255, 255, 0.12));
  }

  .staff-table th {
    padding: 14px 16px;
    text-align: left;
    font-weight: 600;
    font-size: 0.75rem;
    text-transform: uppercase;
    letter-spacing: 0.06em;
    color: var(--color-text-secondary, #888899);
    white-space: nowrap;
  }

  .staff-table td {
    padding: 14px 16px;
    vertical-align: middle;
    border-bottom: 1px solid var(--glass-border, rgba(255, 255, 255, 0.06));
  }

  .staff-table tbody tr:last-child td {
    border-bottom: none;
  }

  .staff-table tbody tr:hover {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.03));
  }

  .row--loading {
    opacity: 0.6;
    pointer-events: none;
  }

  /* ── Member cell ───────────────────────────────────────── */
  .member-info {
    display: flex;
    align-items: center;
    gap: 12px;
    min-width: 0;
  }

  .member-avatar {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    object-fit: cover;
    flex-shrink: 0;
    border: 1px solid var(--glass-border);
  }

  .member-avatar--placeholder {
    display: flex;
    align-items: center;
    justify-content: center;
    background: var(--bg-elevated, rgba(255, 255, 255, 0.08));
    font-weight: 700;
    font-size: 0.85rem;
    color: var(--accent-blue);
  }

  .member-details {
    display: flex;
    flex-direction: column;
    min-width: 0;
  }

  .member-name {
    font-weight: 600;
    color: var(--color-text-primary, #f0f0f0);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .member-email {
    font-size: 0.75rem;
    color: var(--color-text-secondary, #888899);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .member-username {
    font-size: 0.7rem;
    color: var(--color-text-muted, rgba(255, 255, 255, 0.32));
  }

  /* ── Badges ────────────────────────────────────────────── */
  .badge {
    display: inline-block;
    padding: 3px 9px;
    border-radius: 20px;
    font-size: 0.7rem;
    font-weight: 600;
    letter-spacing: 0.04em;
    text-transform: uppercase;
    white-space: nowrap;
  }

  .badge--red {
    background: rgba(255, 45, 85, 0.15);
    color: var(--accent-red);
    border: 1px solid rgba(255, 45, 85, 0.25);
  }

  .badge--blue {
    background: rgba(0, 122, 255, 0.15);
    color: var(--accent-blue);
    border: 1px solid rgba(0, 122, 255, 0.25);
  }

  .badge--green {
    background: rgba(52, 199, 89, 0.15);
    color: var(--accent-green);
    border: 1px solid rgba(52, 199, 89, 0.25);
  }

  .badge--yellow {
    background: rgba(255, 214, 10, 0.15);
    color: var(--accent-yellow);
    border: 1px solid rgba(255, 214, 10, 0.25);
  }

  .badge--muted {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.08));
    color: var(--color-text-muted);
    border: 1px solid var(--glass-border);
  }

  /* ── Date cell ─────────────────────────────────────────── */
  .cell-date {
    white-space: nowrap;
    color: var(--color-text-secondary, #888899);
    font-size: 0.8rem;
  }

  /* ── Actions cell ──────────────────────────────────────── */
  .col-actions {
    min-width: 240px;
  }

  .cell-actions {
    vertical-align: top;
    padding-top: 12px;
    padding-bottom: 12px;
  }

  .action-group {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 8px;
  }

  .role-update {
    display: flex;
    align-items: center;
    gap: 6px;
  }

  .role-select {
    background: var(--bg-elevated, rgba(255, 255, 255, 0.08));
    border: 1px solid var(--glass-border);
    border-radius: 8px;
    color: var(--color-text-primary, #f0f0f0);
    font-size: 0.8rem;
    padding: 5px 8px;
    cursor: pointer;
    outline: none;
    transition: border-color 0.2s;
  }

  .role-select:focus {
    border-color: var(--accent-blue);
  }

  /* ── Buttons ───────────────────────────────────────────── */
  .btn {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    padding: 5px 14px;
    border-radius: 8px;
    font-size: 0.78rem;
    font-weight: 600;
    cursor: pointer;
    border: 1px solid transparent;
    transition: all 0.18s ease;
    white-space: nowrap;
    line-height: 1.4;
  }

  .btn:disabled {
    opacity: 0.45;
    cursor: not-allowed;
  }

  .btn--confirm {
    background: rgba(52, 199, 89, 0.18);
    color: var(--accent-green);
    border-color: rgba(52, 199, 89, 0.3);
  }

  .btn--confirm:not(:disabled):hover {
    background: rgba(52, 199, 89, 0.28);
    box-shadow: 0 0 10px rgba(52, 199, 89, 0.3);
  }

  .btn--update {
    background: rgba(0, 122, 255, 0.18);
    color: var(--accent-blue);
    border-color: rgba(0, 122, 255, 0.3);
  }

  .btn--update:not(:disabled):hover {
    background: rgba(0, 122, 255, 0.28);
    box-shadow: 0 0 10px rgba(0, 122, 255, 0.3);
  }

  .btn--qr {
    background: rgba(0, 194, 255, 0.1);
    color: #00C2FF;
    border-color: rgba(0, 194, 255, 0.25);
    gap: 4px;
  }

  .btn--qr:hover {
    background: rgba(0, 194, 255, 0.2);
    box-shadow: 0 0 10px rgba(0, 194, 255, 0.3);
  }

  /* ── Feedback messages ─────────────────────────────────── */
  .action-msg {
    margin: 6px 0 0;
    font-size: 0.75rem;
    font-weight: 500;
    line-height: 1.4;
  }

  .action-msg--success {
    color: var(--accent-green);
  }

  .action-msg--error {
    color: var(--accent-red);
  }

  /* ── Accessibility ─────────────────────────────────────── */
  .sr-only {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }
</style>
