/**
 * Cloudflare Pages Function — proxies all /api/* requests to the Worker.
 * This works on the free plan, unlike _redirects proxy rules.
 */
export async function onRequest(context) {
  const url = new URL(context.request.url);
  const targetUrl = 'https://api.ezeroandone.io' + url.pathname + url.search;

  const request = new Request(targetUrl, {
    method: context.request.method,
    headers: context.request.headers,
    body: context.request.body,
    redirect: 'follow',
  });

  return fetch(request);
}
