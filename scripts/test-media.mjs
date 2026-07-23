// Test various worker URLs to find where it's actually deployed
const bases = [
  'https://ezeroandone-api.workers.dev',
  'https://api.ezeroandone.io',
  'https://ezeroandone.io',
];

for (const base of bases) {
  try {
    const r = await fetch(`${base}/api/clients`, { method: 'HEAD', signal: AbortSignal.timeout(5000) });
    console.log(r.status, base + '/api/clients');
  } catch (e) {
    console.log('err:', base, e.message.substring(0, 60));
  }
}
