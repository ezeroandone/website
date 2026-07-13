/**
 * Admin staff management page server load — Task 22.2
 *
 * Fetches all staff members from the Worker API (GET /api/admin/staff) and
 * provides them to the page component.  The admin layout guard already ensures
 * the user is authenticated and holds at minimum the Admin role.
 *
 * Requirements: 8.3, 8.5, 15.4
 */

import type { PageServerLoad } from './$types';
import { apiFetch } from '$lib/api';

/**
 * StaffAdmin — the full admin view of a staff member
 * Matches the Rust StaffAdmin model from design section 3.1.
 * Excludes signing_public_key per Requirement 8.3.
 */
export interface StaffAdmin {
  id: string;
  email: string;
  username: string;
  name: string;
  job_title: string;
  bio: string;
  avatar_url: string;
  role: 'SuperAdmin' | 'Admin' | 'Staff';
  onboarding_completed: boolean;
  created_at: number; // Unix timestamp (seconds)
  updated_at: number; // Unix timestamp (seconds)
  /** Lifecycle status from staff_lifecycle join */
  status?: 'Probation' | 'Confirmed' | 'Inactive';
}

export const load: PageServerLoad = async ({ fetch }) => {
  const staff = await apiFetch<StaffAdmin[]>(fetch, '/api/admin/staff');

  return {
    staff,
  };
};
