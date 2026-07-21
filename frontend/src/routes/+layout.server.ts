/**
 * Root layout server load.
 *
 * Checks if the user has a valid session cookie and passes `isLoggedIn`
 * to the layout so the Staff Portal button can redirect to /admin/dashboard
 * instead of /auth/login when the user is already authenticated.
 */

import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ cookies }) => {
  const session = cookies.get('session');
  // We only check presence — the admin layout does the full JWT decode + role check.
  return {
    isLoggedIn: !!session,
  };
};
