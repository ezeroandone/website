/**
 * GET /media/[...path]
 *
 * SvelteKit server route that proxies R2 media asset requests to the
 * Cloudflare Worker. This is necessary because the SvelteKit adapter-cloudflare
 * generates a single _worker.js that handles all routes, making Pages Functions
 * and _redirects proxy rules unavailable.
 *
 * The Worker's GET /media/:key handler reads from the EZO_MEDIA R2 bucket
 * and returns the raw bytes with the correct Content-Type and Cache-Control.
 */

import type { RequestHandler } from './$types';

const WORKER_BASE = 'https://api.ezeroandone.io';

export const GET: RequestHandler = async ({ params, request }) => {
  const key = params.path;

  if (!key || key.includes('..')) {
    return new Response('Not Found', { status: 404 });
  }

  const upstream = `${WORKER_BASE}/media/${key}`;

  try {
    const res = await fetch(upstream, {
      headers: {
        // Forward cache-friendly headers
        'if-none-match':     request.headers.get('if-none-match')     ?? '',
        'if-modified-since': request.headers.get('if-modified-since') ?? '',
      },
    });

    if (!res.ok) {
      return new Response('Not Found', { status: 404 });
    }

    const body      = await res.arrayBuffer();
    const mime      = res.headers.get('content-type')  ?? 'application/octet-stream';
    const cacheCtrl = res.headers.get('cache-control') ?? 'public, max-age=31536000, immutable';
    const etag      = res.headers.get('etag')          ?? '';

    const headers: Record<string, string> = {
      'Content-Type':                mime,
      'Cache-Control':               cacheCtrl,
      'Access-Control-Allow-Origin': '*',
    };
    if (etag) headers['ETag'] = etag;

    return new Response(body, { status: 200, headers });

  } catch {
    return new Response('Not Found', { status: 404 });
  }
};
