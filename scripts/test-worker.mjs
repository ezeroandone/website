// Test which endpoints work on the deployed worker
const base = 'https://ezeroandone.io';
const tests = [
  '/api/clients',
  '/api/work',
  '/api/team',
  '/api/capabilities',
  '/media/clients/56662d4662c43c0ce7e07e2be318aa14/logo.webp',
];

for (const path of tests) {
  try {
    const r = await fetch(base + path, { method: 'HEAD', signal: AbortSignal.timeout(6000) });
    const ct = r.headers.get('content-type') ?? '';
    console.log(r.status, path, ct.substring(0, 40));
  } catch (e) {
    console.log('ERR', path, e.message.substring(0, 60));
  }
}
