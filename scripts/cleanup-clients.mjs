/**
 * cleanup-clients.mjs
 * - Deletes duplicate entries that have no logo_url (the failed first-run creates)
 * - Renames "Client Brand" → "eZeroAndOne (Light)" and "Client Logo" → "Ojis Travels"
 *   based on what those files actually are
 */

const SESSION  = process.env.SESSION_COOKIE;
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';

const h = (extra = {}) => ({
  'Content-Type': 'application/json',
  Cookie: `session=${SESSION}`,
  Origin: BASE_URL,
  ...extra,
});

const res  = await fetch(`${BASE_URL}/api/admin/clients`, { headers: h() });
const all  = await res.json();

// Split into: has logo vs no logo
const noLogo   = all.filter(c => !c.logo_url);
const hasLogo  = all.filter(c => c.logo_url);

console.log(`Found ${noLogo.length} clients without logo (to delete), ${hasLogo.length} with logo (to keep)\n`);

// 1. Delete the empty-logo duplicates
for (const c of noLogo) {
  const r = await fetch(`${BASE_URL}/api/admin/clients/${c.id}`, {
    method: 'DELETE', headers: h(),
  });
  console.log(`DELETE "${c.name}" (${c.id.substring(0,8)}) → ${r.status}`);
  await new Promise(r => setTimeout(r, 300));
}

// 2. Rename the generic names to their real identities
// Logo-light.png.webp = eZeroAndOne light/white logo variant
// logo.webp = Ojis Travels logo (saved from their site without a clear name)
const renames = {
  'Client Brand': 'eZeroAndOne',   // Logo-light.png.webp — the light variant of the e01 logo
  'Client Logo':  'Ojis Travels',  // logo.webp — Ojis Travels logo
};

for (const client of hasLogo) {
  const newName = renames[client.name];
  if (!newName) continue;

  const r = await fetch(`${BASE_URL}/api/admin/clients/${client.id}`, {
    method: 'PATCH',
    headers: h(),
    body: JSON.stringify({
      name: newName,
      website_url: newName === 'Ojis Travels' ? 'https://ojistravels.com' : 'https://ezeroandone.io',
    }),
  });
  const d = await r.json();
  console.log(`RENAME "${client.name}" → "${d.name}" (${r.status})`);
  await new Promise(r => setTimeout(r, 300));
}

// 3. Also fix the duplicate eZeroAndOne — keep only the one with the logo, delete the other
const ezeroAll = hasLogo.filter(c => c.name === 'eZeroAndOne');
if (ezeroAll.length > 1) {
  // Keep the one with the higher sort_order or just keep first, delete rest
  const toDelete = ezeroAll.slice(1);
  for (const c of toDelete) {
    const r = await fetch(`${BASE_URL}/api/admin/clients/${c.id}`, {
      method: 'DELETE', headers: h(),
    });
    console.log(`DELETE duplicate eZeroAndOne (${c.id.substring(0,8)}) → ${r.status}`);
  }
}

console.log('\nDone. Final state:');
const final = await fetch(`${BASE_URL}/api/admin/clients`, { headers: h() });
const finalList = await final.json();
finalList.forEach(c => console.log(`  [${c.sort_order}] ${c.name.padEnd(42)} ${c.logo_url ? '✅' : '❌ no logo'}`));
