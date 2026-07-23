// Test the R2 proxy and diagnose the 404
const SESSION = process.env.SESSION_COOKIE;
const base = 'https://ezeroandone.io';

// 1. Test media proxy
const r1 = await fetch(`${base}/media/clients/acaca93ceca484521621c1ca0407492a/logo.webp`);
console.log('GET /media/clients/.../logo.webp:', r1.status, r1.headers.get('content-type'));
const body = await r1.text();
console.log('Body preview:', body.substring(0, 200));

// 2. Check what keys the API returns
const r2 = await fetch(`${base}/api/clients`);
const clients = await r2.json();
console.log('\nStored logo_url:', clients[0]?.logo_url);
console.log('R2 key would be:', clients[0]?.logo_url?.replace('https://media.ezeroandone.com/', ''));
