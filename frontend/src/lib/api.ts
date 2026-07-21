/**
 * Shared API client utilities for eZeroAndOne.io
 *
 * Provides TypeScript interfaces matching the Rust backend models and a
 * generic `apiFetch` wrapper that forwards the session cookie and converts
 * non-OK responses into SvelteKit errors.
 */

import { error } from '@sveltejs/kit';

// ---------------------------------------------------------------------------
// Shared response interfaces (design section 12.7)
// ---------------------------------------------------------------------------

export interface StaffPublicProfile {
  username: string;
  name: string;
  /** snake_case alias kept for Rust JSON interop */
  job_title: string;
  bio: string;
  /** snake_case alias kept for Rust JSON interop */
  avatar_url: string;
}

export interface Post {
  id: string;
  type: 'insight' | 'work' | 'capability';
  slug: string;
  title: string;
  summary: string;
  body_md: string;
  /** Nullable — a post may not yet have an assigned author */
  author_id: string | null;
  /** Unix timestamp (seconds); null when not yet published */
  published_at: number | null;
  /** Unix timestamp (seconds) of the last update */
  updated_at: number;
  published: boolean;
  // Rich metadata (added in 0002_post_meta migration)
  featured_image_url: string;
  category: string;
  tags: string;
  project_type: string;
  technologies: string;
  material_icon: string;
  /** Populated when the API returns the author joined to the post */
  author?: StaffPublicProfile;
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

export interface ApplicationSubmission {
  applicantName: string;
  applicantEmail: string;
  coverLetter: string;
}

export interface IdentityResponse {
  name: string;
  photo_url: string;
  identity_status: string;
  clearance_level: number;
  /** Unix timestamp (seconds) */
  verified_at: number;
}

// ---------------------------------------------------------------------------
// Generic fetch wrapper
// ---------------------------------------------------------------------------

/**
 * Makes an authenticated request to the backend API.
 *
 * Pass SvelteKit's enhanced `fetch` (from a `load` function or form action)
 * so that session cookies are forwarded automatically during SSR.  On a
 * non-OK response the function throws a SvelteKit `error(status, message)`
 * so the nearest `+error.svelte` boundary will handle it.
 *
 * @param fetchFn  - The `fetch` implementation provided by SvelteKit's load context.
 * @param path     - API path, e.g. `/api/insights`.
 * @param options  - Optional `RequestInit` overrides (method, body, headers, …).
 * @returns        - The JSON-decoded response body cast to `T`.
 *
 * @example
 * // In a +page.server.ts load function:
 * export const load: PageServerLoad = async ({ fetch }) => {
 *   const posts = await apiFetch<Post[]>(fetch, '/api/insights');
 *   return { posts };
 * };
 */
export async function apiFetch<T>(
  fetchFn: typeof fetch,
  path: string,
  options: RequestInit = {}
): Promise<T> {
  const response = await fetchFn(path, {
    ...options,
    credentials: 'include',
  });

  if (!response.ok) {
    const message = await response.text();
    throw error(response.status, message || response.statusText);
  }

  return (await response.json()) as T;
}
