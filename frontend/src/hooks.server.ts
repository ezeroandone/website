/**
 * SvelteKit server hooks.
 *
 * handleFetch rewrites relative /api/* requests made during SSR so they hit
 * the local Wrangler dev server (port 8787) instead of looping back to the
 * SvelteKit Node.js server.  In production on Cloudflare Pages both the
 * Worker and the frontend share the same origin, so no rewrite is needed.
 */

import type { HandleFetch } from '@sveltejs/kit';

export const handleFetch: HandleFetch = async ({ request, fetch }) => {
	// Only rewrite in local dev — when the origin is localhost:5173.
	// In production the URL will already point to the correct Cloudflare origin.
	const url = new URL(request.url);
	const isLocalDev =
		url.hostname === 'localhost' || url.hostname === '127.0.0.1';

	if (isLocalDev && url.pathname.startsWith('/api/')) {
		const rewritten = new Request(
			`http://127.0.0.1:8787${url.pathname}${url.search}`,
			request
		);
		return fetch(rewritten);
	}

	return fetch(request);
};
