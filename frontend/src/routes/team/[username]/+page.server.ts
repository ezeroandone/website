import type { PageServerLoad } from './$types';
import { error } from '@sveltejs/kit';
import { apiFetch, type StaffPublicProfile } from '$lib/api';

interface IdentityJwtResponse {
  identity_jwt: string;
}

export const load: PageServerLoad = async ({ params, fetch }) => {
  // Fetch the public staff profile; apiFetch throws error(404) if not found
  const profile = await apiFetch<StaffPublicProfile>(fetch, `/api/team/${params.username}`);

  // Attempt to fetch a short-lived identity JWT for the QR widget.
  // The Worker exposes POST /api/team/:username/identity-jwt (or similar) to
  // generate a 5-minute QR token.  If the endpoint is not yet available, we
  // fall back to a placeholder so the profile page still renders.
  let identityJwt: string | null = null;
  try {
    const jwtRes = await fetch(`/api/team/${params.username}/identity-jwt`, {
      method: 'POST',
      credentials: 'include',
    });
    if (jwtRes.ok) {
      const jwtData: IdentityJwtResponse = await jwtRes.json();
      identityJwt = jwtData.identity_jwt ?? null;
    }
  } catch {
    // Non-fatal — QR widget will render with null token and show a placeholder
  }

  return { profile, identityJwt };
};
