/**
 * Auth callback server load.
 *
 * Forwards the `?token` query parameter to the Worker's
 * GET /api/auth/callback endpoint. The Worker validates the token and
 * returns JSON { jwt, redirect }. This page sets the session cookie
 * directly (so it's scoped to ezeroandone.io) and redirects the user.
 *
 * Requirements: 1.6, 15.1
 */

import { error, redirect } from '@sveltejs/kit';
import type { RequestEvent } from '@sveltejs/kit';

export async function load({ url, fetch, cookies }: RequestEvent) {
	const token = url.searchParams.get('token');

	if (!token) {
		throw error(400, 'Missing token');
	}

	let res: Response;
	try {
		res = await fetch(`/api/auth/callback?token=${encodeURIComponent(token)}`, {
			method: 'GET',
		});
	} catch {
		throw error(502, 'Unable to reach the authentication service.');
	}

	if (res.ok) {
		let data: { jwt: string; redirect: string };
		try {
			data = await res.json() as { jwt: string; redirect: string };
		} catch (e) {
			throw error(500, `Failed to parse auth response: ${e}`);
		}

		if (!data.jwt) {
			throw error(500, 'Auth response missing JWT');
		}

		// Set the session cookie on the SvelteKit server so it is scoped
		// to ezeroandone.io (not api.ezeroandone.io).
		cookies.set('session', data.jwt, {
			path: '/',
			httpOnly: true,
			secure: true,
			sameSite: 'strict',
			maxAge: 86400,
		});

		throw redirect(302, data.redirect ?? '/dashboard');
	}

	if (res.status === 401) {
		throw error(401, 'This magic link is invalid or has already been used. Please request a new one.');
	}

	// Expose the actual status and body for debugging
	let body = '';
	try { body = await res.text(); } catch { body = '(unreadable)'; }
	throw error(res.status, `Authentication failed (${res.status}): ${body}`);
};
