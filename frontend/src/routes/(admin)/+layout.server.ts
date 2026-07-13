/**
 * Admin layout server guard — Requirements 15.4, 15.5
 *
 * Runs on the server before any admin route is rendered.
 * Reads the `session` cookie, decodes the HS256 JWT payload, checks that
 * `role` maps to a level ≥ Admin (3), and redirects to /auth/login when
 * the cookie is absent, the token is malformed / expired, or the role is
 * insufficient.
 *
 * NOTE: We only decode (base64url) here — we do NOT re-verify the HMAC
 * signature on the frontend because the JWT secret is a Worker secret and
 * must never be exposed to the Pages project.  Full cryptographic
 * verification is enforced by the Worker on every /api/* call.  This guard
 * exists solely to give users a fast, friendly redirect rather than hitting
 * a 401 from the API.
 */

import { redirect } from '@sveltejs/kit';
import type { LayoutServerLoad } from './$types';

// Role hierarchy — must mirror the Rust RBAC definition in requirements §3
const ROLE_LEVEL: Record<string, number> = {
  Public: 1,
  Staff: 2,
  Admin: 3,
  SuperAdmin: 4,
};

const ADMIN_REQUIRED_LEVEL = ROLE_LEVEL['Admin']; // 3

interface SessionClaims {
  sub: string;
  email: string;
  role: string;
  onboarded: boolean;
  exp: number;
}

/**
 * Decode the payload segment of a JWT string without verifying the signature.
 * Returns null for any malformed input.
 */
function decodeJwtPayload(token: string): SessionClaims | null {
  const parts = token.split('.');
  if (parts.length !== 3) return null;

  try {
    // base64url → base64 → JSON
    const base64 = parts[1].replace(/-/g, '+').replace(/_/g, '/');
    // atob is available in the Cloudflare Pages runtime
    const json = atob(base64);
    return JSON.parse(json) as SessionClaims;
  } catch {
    return null;
  }
}

export const load: LayoutServerLoad = async ({ cookies }) => {
  const token = cookies.get('session');

  // No cookie present → redirect to login (Requirement 15.5)
  if (!token) {
    redirect(302, '/auth/login');
  }

  const claims = decodeJwtPayload(token);

  // Malformed token → redirect
  if (!claims) {
    redirect(302, '/auth/login');
  }

  // Expired token → redirect
  const nowSeconds = Math.floor(Date.now() / 1000);
  if (claims.exp <= nowSeconds) {
    redirect(302, '/auth/login');
  }

  // Insufficient role → redirect (Requirement 15.4: requires Admin or above)
  const userLevel = ROLE_LEVEL[claims.role] ?? 0;
  if (userLevel < ADMIN_REQUIRED_LEVEL) {
    redirect(302, '/auth/login');
  }

  // Pass minimal safe session data to admin layout and child pages
  return {
    session: {
      sub: claims.sub,
      email: claims.email,
      role: claims.role,
      onboarded: claims.onboarded,
    },
  };
};
