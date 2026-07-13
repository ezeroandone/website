import type { PageServerLoad } from './$types';
import { apiFetch, type Post } from '$lib/api';

export const load: PageServerLoad = async ({ fetch, params }) => {
  const post = await apiFetch<Post>(fetch, `/api/work/${params.slug}`);
  return { post };
};
