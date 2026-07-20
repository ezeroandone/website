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

	const apiUrl = `/api/auth/callback?token=${encodeURIComponent(token)}`;
	console.log('[callback] fetching', apiUrl);

	let res: Response;
	try {
		res = await fetch(apiUrl, { method: 'GET' });
	} catch (e) {
		console.error('[callback] fetch threw:', e);
		throw error(502, 'Unable to reach the authentication service.');
	}

	console.log('[callback] worker status:', res.status);

	if (res.ok) {
		let raw: string;
		try {
			raw = await res.text();
		} catch (e) {
			throw error(500, `Failed to read auth response body: ${e}`);
		}

		console.log('[callback] worker body:', raw);

		let data: { jwt: string; redirect: string };
		try {
			data = JSON.parse(raw) as { jwt: string; redirect: string };
		} catch (e) {
			throw error(500, `Failed to parse auth response as JSON: ${e} — body was: ${raw}`);
		}

		if (!data.jwt) {
			throw error(500, `Auth response missing JWT — parsed: ${JSON.stringify(data)}`);
		}

		console.log('[callback] jwt prefix:', data.jwt.slice(0, 20), '  redirect:', data.redirect);

		cookies.set('session', data.jwt, {
			path: '/',
			httpOnly: true,
			secure: true,
			sameSite: 'strict',
			maxAge: 86400,
		});

		console.log('[callback] cookie set, redirecting to', data.redirect ?? '/admin/dashboard');
		throw redirect(302, data.redirect ?? '/admin/dashboard');
	}

	// Non-2xx — capture body for the error message
	let body = '';
	try { body = await res.text(); } catch { body = '(unreadable)'; }

	console.error('[callback] worker error response:', res.status, body);

	if (res.status === 401) {
		throw error(401, 'This magic link is invalid or has already been used. Please request a new one.');
	}

	throw error(res.status, `Authentication failed (${res.status}): ${body}`);
};
