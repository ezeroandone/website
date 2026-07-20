/**
 * SvelteKit server hooks.
 *
 * handleFetch rewrites relative /api/* requests so they reach the correct
 * backend target in every environment:
 *
 *   - Local dev  → http://127.0.0.1:8787  (Wrangler dev server)
 *   - Production → https://api.ezeroandone.io  (Cloudflare Worker)
 *
 * Without this rewrite, SSR-time fetch calls to /api/* would loop back to
 * the SvelteKit server itself and return 404.
 */

import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
	const url = new URL(request.url);

	if (url.pathname.startsWith('/api/')) {
		const isLocalDev =
			url.hostname === 'localhost' || url.hostname === '127.0.0.1';

		const apiBase = isLocalDev
			? 'http://127.0.0.1:8787'
			: 'https://api.ezeroandone.io';

		const rewritten = new Request(
			`${apiBase}${url.pathname}${url.search}`,
			request
		);
		return fetch(rewritten);
	}

	return fetch(request);
};
