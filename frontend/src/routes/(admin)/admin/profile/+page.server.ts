/**
 * Admin profile page server load
 *
 * Fetches the current user's full profile from the API so the edit form
 * can be pre-populated with existing values.
 */

import type { PageServerLoad } from './$types';
import { apiFetch } from '$lib/api';

interface MyProfile {
  id: string;
  email: string;
  username: string;
  name: string;
  job_title: string;
  bio: string;
  avatar_url: string;
  role: string;
}

export const load: PageServerLoad = async ({ fetch }) => {
  const profile = await apiFetch<MyProfile>(fetch, '/api/me');
  return { profile };
};
