/**
 * Onboarding page server load.
 *
 * Requires an authenticated session. Fetches the current onboarding step
 * from the Worker and returns { step, completed } to the page component.
 *
 * Requirements: 4.3, 4.9, 15.6
 */

import { error, redirect } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';

interface OnboardingStatus {
	step: 1 | 2 | 3;
	completed: boolean;
}

export const load: PageServerLoad = async ({ fetch, cookies }) => {
	const session = cookies.get('session');

	// No session → send to login.
	if (!session) {
		throw redirect(302, '/auth/login');
	}

	let res: Response;
	try {
		res = await fetch('/api/onboarding/status', {
			headers: { Cookie: `session=${session}` },
		});
	} catch {
		throw error(502, 'Unable to reach the server.');
	}

	if (res.status === 401 || res.status === 403) {
		// Session present but invalid/expired — back to login.
		throw redirect(302, '/auth/login');
	}

	if (!res.ok) {
		throw error(res.status, 'Failed to load onboarding status.');
	}

	const status = (await res.json()) as OnboardingStatus;

	// Already finished onboarding — redirect to dashboard.
	if (status.completed) {
		throw redirect(302, '/admin/dashboard');
	}

	return { step: status.step, completed: status.completed };
};
