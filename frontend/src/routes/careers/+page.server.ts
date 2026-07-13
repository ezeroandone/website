import type { PageServerLoad } from './$types';
import { apiFetch, type Career } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const careers = await apiFetch<Career[]>(fetch, '/api/careers');
  return { careers };
};
