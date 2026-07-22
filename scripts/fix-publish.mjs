/**
 * fix-publish.mjs
 *
 * The 22 work posts were already created as drafts.
 * This script patches each one to: published=true, published_at=now,
 * and fetches a Microlink screenshot for the featured_image_url.
 *
 * Usage (PowerShell):
 *   [System.Environment]::SetEnvironmentVariable("SESSION_COOKIE", "<jwt>")
 *   [System.Environment]::SetEnvironmentVariable("BASE_URL", "https://ezeroandone.io")
 *   node scripts/fix-publish.mjs
 */

import { writeFileSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));
const SESSION  = process.env.SESSION_COOKIE ?? '';
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';

if (!SESSION) {
  console.error('Set SESSION_COOKIE env var first.');
  process.exit(1);
}

// IDs from the previous failed run, paired with their source URLs for screenshots
const POSTS = [
  { title: 'Dotun Taylor',          id: 'eee0dc48e9eab5e5f5dc5dc8d2f4dde6', url: 'http://dotuntaylor.com/' },
  { title: 'Ojis Travels',          id: '9579686ee44611af04c48d5620a52a92',  url: 'https://ojistravels.com/' },
  { title: 'Risyn AI',              id: 'b0e0f0a09fc79521d798db279977f988',  url: 'https://www.risyn.ai/' },
  { title: 'Mitkeda',               id: '96b223fb68281cf6e2789af643b5595d',  url: 'https://www.mitkeda.com/' },
  { title: 'Hizon LLC',             id: '077fea43aa8513e64743d402954680c3',  url: 'https://www.hizonllc.com/' },
  { title: 'TTP Nigeria',           id: 'd3bee13c10722a1af3af3b2d59a49b8e',  url: 'https://ttp.ng/' },
  { title: 'Unicorn HR Consulting', id: '4e9ee33bfff29ead6e90fec96e69566e',  url: 'https://unicornhrconsulting.com/' },
  { title: 'Unicorn Digital',       id: '004625f7f073ebb47c492b1b07905117',  url: 'https://unicorndigital.ng/' },
  { title: 'Joshua Ronatus',        id: '4699573bc7899afc578798c19fd1680f',  url: 'http://joshuaronatus.com/' },
  { title: 'Tobe Asikoko',          id: '84e9402234c792a4bdec06bad93d7214',  url: 'https://tobeasikoko.com/' },
  { title: 'C-Suite Brand Partners',id: '6133ffeb53bab5ea7ac5298be248c50d',  url: 'https://csuitebrandpartners.co.uk/' },
  { title: 'KU8 Nigeria',           id: '272f856d2401c4be974874cd1e76bf82',  url: 'https://ku8.org.ng/' },
  { title: 'NITPCS Nigeria',        id: '9b79efaabbd89baed14a4b0889ef87b8',  url: 'https://nitpcs.org.ng/' },
  { title: 'Lincoln Adighije',      id: '64450390e004f311595188a6bae321f5',  url: 'https://www.lincolnadighije.com/' },
  { title: 'Assurance by Jummy',    id: 'f5ed10099926d97beadbcea984b42740',  url: 'https://assurancebyjummy.com.ng/' },
  { title: 'OAJ Energy',            id: 'b20a685d5aae31ced310cf5b195656cb',  url: 'https://oajenergy.com.ng/' },
  { title: 'Sola by Adesola',       id: '0f9a971fc3ed59b6b438ff7876d273d1',  url: 'https://www.solabyadesola.com.ng/' },
  { title: 'OptimaMind',            id: 'c7d3f888a4b0d77a610b99a8a80dd365',  url: 'https://www.optimamind.com.ng/' },
  { title: 'Ibrahim Energy',        id: '9c942e2ee55dd4b75f5e74d24dad9e74',  url: 'https://www.ibrahimenergy.com.ng/' },
  { title: 'Kazeem Digitals',       id: '53cbbe2f2530ef77c600a01fc6201126',  url: 'https://www.kazeemdigitals.com.ng/' },
  { title: 'Bammy John',            id: 'f8508bd5bdb31098a2139226152ed38e',  url: 'https://www.bammyjohn.com.ng/' },
  { title: 'FormSend',              id: '7a6b5603755352d3ed9605aae96f07f8',  url: 'https://formsend.ezeroandone.io/' },
];

async function getScreenshot(url) {
  try {
    const api = `https://api.microlink.io/?url=${encodeURIComponent(url)}&screenshot=true&meta=false&embed=screenshot.url`;
    const res = await fetch(api, { headers: { Accept: 'application/json' } });
    if (!res.ok) return null;
    const json = await res.json();
    return json?.data?.screenshot?.url ?? null;
  } catch {
    return null;
  }
}

async function patchPost(id, fields) {
  const res = await fetch(`${BASE_URL}/api/admin/content/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', Cookie: `session=${SESSION}` },
    body: JSON.stringify(fields),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`${res.status}: ${text.substring(0, 200)}`);
  return JSON.parse(text);
}

async function main() {
  console.log(`\n🔧  Publishing ${POSTS.length} drafts at ${BASE_URL}\n`);

  const results = [];
  const now = Math.floor(Date.now() / 1000);

  for (const post of POSTS) {
    process.stdout.write(`  ⏳  ${post.title}… `);
    try {
      // Try to get screenshot
      const screenshotUrl = await getScreenshot(post.url);
      if (screenshotUrl) process.stdout.write('screenshot ✓, ');
      else process.stdout.write('no screenshot, ');

      const fields = { published: true, published_at: now };
      if (screenshotUrl) fields.featured_image_url = screenshotUrl;

      await patchPost(post.id, fields);
      console.log('published ✅');
      results.push({ title: post.title, ok: true });
    } catch (err) {
      console.log(`❌  ${err.message}`);
      results.push({ title: post.title, ok: false, error: err.message });
    }

    // Throttle: 1s between requests
    await new Promise(r => setTimeout(r, 1000));
  }

  const ok = results.filter(r => r.ok).length;
  console.log(`\n  ${ok}/${POSTS.length} published.\n`);

  if (ok < POSTS.length) {
    console.log('Failed:');
    results.filter(r => !r.ok).forEach(r => console.log(`  ❌  ${r.title}: ${r.error}`));
  }
}

main().catch(err => { console.error(err); process.exit(1); });
