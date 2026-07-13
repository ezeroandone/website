/**
 * Admin Careers page — server load function
 *
 * Fetches all career listings (including inactive) and all applications
 * from the Worker API for the admin careers management page.
 *
 * Requirements: 5.5, 5.6, 5.7, 7.3, 15.4
 */

import { apiFetch } from '$lib/api';
import type { PageServerLoad } from './$types';

export interface Application {
  id: string;
  career_id: string;
  applicant_name: string;
  applicant_email: string;
  cover_letter: string;
  status: 'Applied' | 'Interviewing' | 'Offered' | 'Rejected' | 'Hired';
  /** Unix timestamp (seconds) */
  applied_at: number;
  /** Unix timestamp (seconds) */
  updated_at: number;
}

export interface Career {
  id: string;
  slug: string;
  title: string;
  description_md: string;
  department: string;
  type: 'Full-Time' | 'Part-Time' | 'Contract' | 'Internship';
  active: boolean;
  /** Unix timestamp (seconds) */
  created_at: number;
}

export const load: PageServerLoad = async ({ fetch }) => {
  // Fetch all careers (incl. inactive) — Requirement 5.7
  // Fetch all applications ordered by applied_at DESC — Requirement 7.3
  const [careers, applications] = await Promise.all([
    apiFetch<Career[]>(fetch, '/api/admin/careers'),
    apiFetch<Application[]>(fetch, '/api/admin/applications'),
  ]);

  return { careers, applications };
};
