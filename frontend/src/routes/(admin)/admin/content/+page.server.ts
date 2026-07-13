/**
 * Admin content management — server load
 *
 * Fetches all published posts by calling the three public list endpoints
 * (/api/insights, /api/work, /api/capabilities) and merging the results.
 * Unpublished posts are not exposed by the public API; they become visible
 * once an Admin toggles `published = true` via the PATCH endpoint.
 *
 * Requirements: 10.5, 10.6, 10.7, 15.4
 */

import { apiFetch } from '$lib/api';
import type { Post } from '$lib/api';
import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ fetch }) => {
  // Fetch all three content type lists in parallel.
  // Public endpoints return only published posts; the admin can create new
  // (unpublished) ones and edit/publish them from this page.
  const [insights, work, capabilities] = await Promise.all([
    apiFetch<Post[]>(fetch, '/api/insights'),
    apiFetch<Post[]>(fetch, '/api/work'),
    apiFetch<Post[]>(fetch, '/api/capabilities'),
  ]);

  // Merge and sort descending by updated_at so the most-recently-touched
  // post always appears first regardless of type.
  const posts: Post[] = [...insights, ...work, ...capabilities].sort(
    (a, b) => b.updated_at - a.updated_at
  );

  return { posts };
};
