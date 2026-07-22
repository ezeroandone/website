/**
 * upload-clients.mjs
 * Creates client logo records and uploads the logo files from /logos/ to R2.
 *
 * Usage:
 *   node scripts/upload-clients.mjs
 * (SESSION_COOKIE and BASE_URL env vars must be set)
 */

import { readFileSync, readdirSync } from 'fs';
import { join, extname, basename, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const ROOT      = join(__dirname, '..');
const LOGOS_DIR = join(ROOT, 'logos');

const SESSION  = process.env.SESSION_COOKIE ?? '';
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';
if (!SESSION) { console.error('Set SESSION_COOKIE'); process.exit(1); }

// Map filename → { name, website, sort_order }
const CLIENTS = {
  'Landscape-logo.webp': {
    name: 'eZeroAndOne',
    website: 'https://ezeroandone.io',
    sort_order: 1,
  },
  'LANGE-AND-GRANT-LOGO-300x67.webp': {
    name: 'Lange & Grant',
    website: '',
    sort_order: 2,
  },
  'Logo-light.png.webp': {
    name: 'Client Brand',
    website: '',
    sort_order: 3,
  },
  'logo.webp': {
    name: 'Client Logo',
    website: '',
    sort_order: 4,
  },
  'Palton-Morgan-Logo-300x48.webp': {
    name: 'Palton Morgan',
    website: 'https://paltonmorgan.com',
    sort_order: 5,
  },
  'Plusworld-Logo-300x81.webp': {
    name: 'Plusworld',
    website: '',
    sort_order: 6,
  },
  'Tejuosho-Ultra-Modern-Shopping-Centre-Logo-300x141.webp': {
    name: 'Tejuosho Ultra Modern Shopping Centre',
    website: '',
    sort_order: 7,
  },
  'unicorn logo.webp': {
    name: 'Unicorn',
    website: 'https://unicornhrconsulting.com',
    sort_order: 8,
  },
};

function authHeaders(extra = {}) {
  return { Cookie: `session=${SESSION}`, ...extra };
}

async function createClient(name, website, sort_order) {
  const res = await fetch(`${BASE_URL}/api/admin/clients`, {
    method: 'POST',
    headers: { ...authHeaders(), 'Content-Type': 'application/json' },
    body: JSON.stringify({ name, logo_url: '', website_url: website, sort_order }),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`Create failed "${name}": ${res.status} ${text.substring(0,200)}`);
  return JSON.parse(text);
}

async function uploadLogo(clientId, filePath, filename) {
  const fileBuffer = readFileSync(filePath);
  const ext = extname(filename).toLowerCase().replace('.', '');
  // webp mime
  const mime = ext === 'webp' ? 'image/webp'
             : ext === 'png'  ? 'image/png'
             : ext === 'jpg' || ext === 'jpeg' ? 'image/jpeg'
             : 'application/octet-stream';

  // Use the Blob + FormData approach for multipart upload
  const blob = new Blob([fileBuffer], { type: mime });
  const form = new FormData();
  form.append('file', blob, filename);

  const res = await fetch(`${BASE_URL}/api/upload/client/${clientId}/logo`, {
    method: 'POST',
    headers: authHeaders(), // no Content-Type — let fetch set multipart boundary
    body: form,
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`Upload failed: ${res.status} ${text.substring(0,200)}`);
  return JSON.parse(text); // { url: '...' }
}

async function patchClient(id, fields) {
  const res = await fetch(`${BASE_URL}/api/admin/clients/${id}`, {
    method: 'PATCH',
    headers: { ...authHeaders(), 'Content-Type': 'application/json' },
    body: JSON.stringify(fields),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`Patch failed: ${res.status} ${text.substring(0,200)}`);
  return JSON.parse(text);
}

async function main() {
  // List actual logo files (skip .htm and non-image files)
  const files = readdirSync(LOGOS_DIR).filter(f => {
    const ext = extname(f).toLowerCase();
    return ['.webp', '.png', '.jpg', '.jpeg', '.svg'].includes(ext);
  });

  console.log(`\n🖼️  Uploading ${files.length} client logos to ${BASE_URL}\n`);

  let ok = 0;
  for (let i = 0; i < files.length; i++) {
    const filename = files[i];
    const meta = CLIENTS[filename];
    if (!meta) {
      console.log(`  ⚠️  No metadata for "${filename}" — using filename as name`);
    }
    const name       = meta?.name ?? basename(filename, extname(filename));
    const website    = meta?.website ?? '';
    const sort_order = meta?.sort_order ?? (i + 1);
    const filePath   = join(LOGOS_DIR, filename);

    process.stdout.write(`  ⏳  ${name}… `);
    try {
      // 1. Create the client record
      const client = await createClient(name, website, sort_order);

      // 2. Upload the logo file
      const { url } = await uploadLogo(client.id, filePath, filename);

      // 3. Patch the client with the R2 URL
      await patchClient(client.id, { logo_url: url });

      console.log(`✅  ${url.substring(0, 70)}`);
      ok++;
    } catch (err) {
      console.log(`❌  ${err.message}`);
    }
    await new Promise(r => setTimeout(r, 600));
  }

  console.log(`\n  ${ok}/${files.length} client logos uploaded.\n`);
}

main().catch(err => { console.error(err); process.exit(1); });
