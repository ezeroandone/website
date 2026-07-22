const SESSION = process.env.SESSION_COOKIE;
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';

const TEST_ID = 'eee0dc48e9eab5e5f5dc5dc8d2f4dde6';

async function test(label, body) {
  const res = await fetch(`${BASE_URL}/api/admin/content/${TEST_ID}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', Cookie: `session=${SESSION}` },
    body: JSON.stringify(body),
  });
  const text = await res.text();
  console.log(`[${label}] ${res.status}: ${text.substring(0, 200)}`);
}

// Test 1: just title (should work if PATCH handler is reached)
await test('title only', { title: 'Dotun Taylor' });

// Test 2: published bool only
await test('published bool', { published: true });

// Test 3: published_at int only
await test('published_at', { published_at: 1753000000 });

// Test 4: both together
await test('published + published_at', { published: true, published_at: 1753000000 });

// Also try GET to see if the post exists
const get = await fetch(`${BASE_URL}/api/admin/content`, {
  headers: { Cookie: `session=${SESSION}` }
});
const posts = await get.json();
const match = posts.find(p => p.id === TEST_ID);
console.log('\nPost found in admin list:', !!match, match ? `title="${match.title}" published=${match.published}` : '');
