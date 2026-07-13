/**
 * Home page server load.
 *
 * Fetches a preview of the latest published content from three sections
 * (insights, work, capabilities) and the active careers count.
 * Failures are non-fatal: the page renders with empty arrays so a backend
 * outage never breaks the landing page.
 */

import type { PageServerLoad } from './$types';
import { apiFetch, type Post, type Career } from '$lib/api';

export const load: PageServerLoad = async ({ fetch }) => {
	const [insights, work, capabilities, careers] = await Promise.allSettled([
		apiFetch<Post[]>(fetch, '/api/insights'),
		apiFetch<Post[]>(fetch, '/api/work'),
		apiFetch<Post[]>(fetch, '/api/capabilities'),
		apiFetch<Career[]>(fetch, '/api/careers')
	]);

	return {
		// Show at most 3 preview cards per section on the home page
		insights: insights.status === 'fulfilled' ? insights.value.slice(0, 3) : [],
		work: work.status === 'fulfilled' ? work.value.slice(0, 3) : [],
		capabilities: capabilities.status === 'fulfilled' ? capabilities.value.slice(0, 3) : [],
		openRoles: careers.status === 'fulfilled' ? careers.value.length : 0
	};
};
