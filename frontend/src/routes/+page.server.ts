/**
 * Home page server load.
 */

import type { PageServerLoad } from './$types';
import { apiFetch, type Post, type Career } from '$lib/api';

interface ClientLogo {
  id: string;
  name: string;
  logo_url: string;
  website_url: string;
  sort_order: number;
}

export const load: PageServerLoad = async ({ fetch }) => {
	const [insights, work, capabilities, careers, clients] = await Promise.allSettled([
		apiFetch<Post[]>(fetch, '/api/insights'),
		apiFetch<Post[]>(fetch, '/api/work'),
		apiFetch<Post[]>(fetch, '/api/capabilities'),
		apiFetch<Career[]>(fetch, '/api/careers'),
		apiFetch<ClientLogo[]>(fetch, '/api/clients'),
	]);

	return {
		insights:    insights.status    === 'fulfilled' ? insights.value.slice(0, 3)       : [],
		work:        work.status        === 'fulfilled' ? work.value.slice(0, 6)            : [],
		capabilities:capabilities.status=== 'fulfilled' ? capabilities.value.slice(0, 3)   : [],
		clients:     clients.status     === 'fulfilled' ? clients.value                     : [],
		openRoles:   careers.status     === 'fulfilled' ? careers.value.length              : 0,
	};
};
