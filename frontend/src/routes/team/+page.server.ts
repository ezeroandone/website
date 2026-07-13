import type { PageServerLoad } from './$types';
import { apiFetch, type StaffPublicProfile } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
  const staff = await apiFetch<StaffPublicProfile[]>(fetch, '/api/team');
  return { staff };
};
