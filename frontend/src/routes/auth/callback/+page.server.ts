/**
 * Auth callback server load.
 *
 * Forwards the `?token` query parameter to the Worker's
 * GET /api/auth/callback endpoint. The Worker validates the token,
 * sets the session cookie, and returns a redirect response.
 *
 * Requirements: 1.6, 15.1
 */

import { error, redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export async function load({ url, fetch, cookies }: RequestEvent) {
	const token = url.searchParams.get('token');

	// A missing token is a client error — render the error boundary.
	if (!token) {
		throw error(400, 'Missing token');
	}

	// Forward the session cookie (if any) so the Worker can detect
	// an already-authenticated session, then pass the token for exchange.
	const session = cookies.get('session');
	const headers: Record<string, string> = {};
	if (session) {
		headers['Cookie'] = `session=${session}`;
	}

	let res: Response;
	try {
		res = await fetch(`/api/auth/callback?token=${encodeURIComponent(token)}`, {
			method: 'GET',
			headers,
			// Do NOT follow redirects automatically — the Worker sets the
			// session cookie on the redirect response and we need to capture it.
			redirect: 'manual',
		});
	} catch {
		throw error(502, 'Unable to reach the authentication service.');
	}

	// The Worker responds with a redirect (302) on success and sets
	// Set-Cookie: session=<jwt>; HttpOnly; Secure; SameSite=Strict.
	// SvelteKit's `fetch` with redirect:'manual' exposes a 0-status
	// opaque redirect, but Cloudflare Pages fetch will surface a real
	// 3xx.  Handle both cases.
	if (res.status === 302 || res.status === 301 || res.status === 303) {
		const location = res.headers.get('location') ?? '/dashboard';
		throw redirect(302, location);
	}

	// 200 means the Worker returned data without a redirect (unlikely but
	// handle gracefully by sending the user to /dashboard).
	if (res.ok) {
		throw redirect(302, '/dashboard');
	}

	// 401 → invalid or expired token.
	if (res.status === 401) {
		throw error(401, 'This magic link is invalid or has already been used. Please request a new one.');
	}

	// Any other failure.
	throw error(res.status, 'Authentication failed. Please try again.');
};
