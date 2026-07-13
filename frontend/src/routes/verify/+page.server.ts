/**
 * Verification Portal — server load.
 *
 * Reads the ?token query param and forwards it to GET /api/verify on the
 * Worker. The Worker validates the QR JWT and returns the identity response.
 *
 * On success: returns identity data to the page component for display.
 * On failure: returns an error state — never reveals whether the token was
 * expired, invalid, or the staff member doesn't exist (Requirement 9.7).
 *
 * Requirements: 9.4, 9.5, 9.6, 9.7
 */

import type { PageServerLoad } from './$types';
import type { IdentityResponse } from '$lib/api';

export interface VerifyPageData {
  state: 'success' | 'invalid' | 'missing';
  identity: IdentityResponse | null;
}

export const load: PageServerLoad = async ({ url, fetch }): Promise<VerifyPageData> => {
  const token = url.searchParams.get('token');

  if (!token) {
    return { state: 'missing', identity: null };
  }

  try {
    const res = await fetch(`/api/verify?token=${encodeURIComponent(token)}`);

    if (res.ok) {
      const identity = (await res.json()) as IdentityResponse;
      return { state: 'success', identity };
    }

    // 401 or any non-2xx → treat as invalid without leaking reason
    return { state: 'invalid', identity: null };
  } catch {
    return { state: 'invalid', identity: null };
  }
};
