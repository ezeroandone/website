/**
 * Admin clients page — server load
 * Fetches all client logos (active and inactive) for management.
 */

import { apiFetch } from '$lib/api';
import type { PageServerLoad } from './$types';

export interface ClientLogo {
  id: string;
  name: string;
  logo_url: string;
  website_url: string;
  sort_order: number;
  active: boolean;
  created_at: number;
}

export const load: PageServerLoad = async ({ fetch }) => {
  const clients = await apiFetch<ClientLogo[]>(fetch, '/api/admin/clients').catch(() => [] as ClientLogo[]);
  return { clients };
};
