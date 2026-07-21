/**
 * Admin content management — server load
 *
 * Uses the admin content endpoint to get ALL posts (including drafts),
 * plus the staff list for team member pickers.
 *
 * Requirements: 10.5, 10.6, 10.7, 15.4
 */

import { apiFetch } from '$lib/api';
import type { PageServerLoad } from './$types';

export interface Post {
  id: string;
  type: 'insight' | 'work' | 'capability';
  slug: string;
  title: string;
  summary: string;
  body_md: string;
  author_id: string | null;
  published_at: number | null;
  updated_at: number;
  published: boolean;
  featured_image_url: string;
  category: string;
  tags: string;
  project_type: string;
  technologies: string;
  material_icon: string;
}

export interface StaffAdmin {
  id: string;
  email: string;
  name: string;
  username: string;
  role: string;
  onboarding_completed: boolean;
}

export const load: PageServerLoad = async ({ fetch }) => {
  // Fetch all posts (including drafts) from admin endpoint + staff list in parallel
  const [posts, staff] = await Promise.all([
    apiFetch<Post[]>(fetch, '/api/admin/content').catch(() => [] as Post[]),
    apiFetch<StaffAdmin[]>(fetch, '/api/admin/staff').catch(() => [] as StaffAdmin[]),
  ]);

  return {
    posts: posts.sort((a, b) => b.updated_at - a.updated_at),
    staff,
  };
};
