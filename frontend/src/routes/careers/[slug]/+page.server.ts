import type { PageServerLoad } from './$types';
import { apiFetch, type Career } from '$lib/api';

export const load: PageServerLoad = async ({ params, fetch }) => {
  // apiFetch throws error(404) via the SvelteKit error boundary when not found
  const career = await apiFetch<Career>(fetch, `/api/careers/${params.slug}`);
  return { career };
};
