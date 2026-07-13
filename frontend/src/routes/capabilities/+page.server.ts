import type { PageServerLoad } from './$types';
import { apiFetch, type Post } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const posts = await apiFetch<Post[]>(fetch, '/api/capabilities');
  return { posts };
};
