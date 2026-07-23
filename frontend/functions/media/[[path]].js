/**
 * Cloudflare Pages Function — proxies all /media/* requests to the Worker.
 * The Worker's GET /media/:key handler reads from the EZO_MEDIA R2 bucket
 * and serves client logos, avatars, and other uploaded assets.
 */
export async function onRequest(context) {
  const url = new URL(context.request.url);
  const targetUrl = 'https://api.ezeroandone.io' + url.pathname + url.search;

  const request = new Request(targetUrl, {
    method: context.request.method,
    headers: context.request.headers,
    body: context.request.method !== 'GET' && context.request.method !== 'HEAD'
      ? context.request.body
      : undefined,
    redirect: 'follow',
  });

  return fetch(request);
}
