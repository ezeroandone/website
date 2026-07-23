/**
 * SvelteKit server hooks.
 *
 * handle: proxies all incoming /api/* requests from the browser to the
 *         Cloudflare Worker at api.ezeroandone.io. This covers both
 *         client-side fetches and SSR fetches in production.
 *
 * handleFetch: rewrites relative /api/* requests made during SSR so they
 *              hit the correct backend in every environment.
 */

import type { Handle, HandleFetch } from '@sveltejs/kit';

const API_BASE = 'https://api.ezeroandone.io';

/** Proxy all /api/* requests arriving at the Pages server to the Worker. */
export const handle: Handle = async ({ event, resolve }) => {
	if (event.url.pathname.startsWith('/api/')) {
		const targetUrl = `${API_BASE}${event.url.pathname}${event.url.search}`;

		const proxyRequest = new Request(targetUrl, {
			method: event.request.method,
			headers: event.request.headers,
			body: ['GET', 'HEAD'].includes(event.request.method)
				? undefined
				: event.request.body,
			duplex: 'half',
		} as RequestInit & { duplex: string });

		return fetch(proxyRequest);
	}

	return resolve(event);
};

/** Rewrite SSR-time /api/* fetches to the correct backend. */
export const handleFetch: HandleFetch = async ({ request, fetch }) => {
	const url = new URL(request.url);

	if (url.pathname.startsWith('/api/')) {
		const isLocalDev =
			url.hostname === 'localhost' || url.hostname === '127.0.0.1';

		const apiBase = isLocalDev
			? 'http://127.0.0.1:8787'
			: API_BASE;

		const rewritten = new Request(
			`${apiBase}${url.pathname}${url.search}`,
			request
		);
		return fetch(rewritten);
	}

	return fetch(request);
};
