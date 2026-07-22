/**
 * add-screenshots.mjs
 * Fetches screenshots via multiple fallback services and patches featured_image_url.
 * Uses: thum.io (no key needed), then screenshotone (free), then htmlcsstoimage API.
 */

const SESSION  = process.env.SESSION_COOKIE ?? '';
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';

if (!SESSION) { console.error('Set SESSION_COOKIE'); process.exit(1); }

const POSTS = [
  { title: 'Dotun Taylor',           id: 'eee0dc48e9eab5e5f5dc5dc8d2f4dde6', url: 'http://dotuntaylor.com/' },
  { title: 'Ojis Travels',           id: '9579686ee44611af04c48d5620a52a92',  url: 'https://ojistravels.com/' },
  { title: 'Risyn AI',               id: 'b0e0f0a09fc79521d798db279977f988',  url: 'https://www.risyn.ai/' },
  { title: 'Mitkeda',                id: '96b223fb68281cf6e2789af643b5595d',  url: 'https://www.mitkeda.com/' },
  { title: 'Hizon LLC',              id: '077fea43aa8513e64743d402954680c3',  url: 'https://www.hizonllc.com/' },
  { title: 'TTP Nigeria',            id: 'd3bee13c10722a1af3af3b2d59a49b8e',  url: 'https://ttp.ng/' },
  { title: 'Unicorn HR Consulting',  id: '4e9ee33bfff29ead6e90fec96e69566e',  url: 'https://unicornhrconsulting.com/' },
  { title: 'Unicorn Digital',        id: '004625f7f073ebb47c492b1b07905117',  url: 'https://unicorndigital.ng/' },
  { title: 'Joshua Ronatus',         id: '4699573bc7899afc578798c19fd1680f',  url: 'http://joshuaronatus.com/' },
  { title: 'Tobe Asikoko',           id: '84e9402234c792a4bdec06bad93d7214',  url: 'https://tobeasikoko.com/' },
  { title: 'C-Suite Brand Partners', id: '6133ffeb53bab5ea7ac5298be248c50d',  url: 'https://csuitebrandpartners.co.uk/' },
  { title: 'KU8 Nigeria',            id: '272f856d2401c4be974874cd1e76bf82',  url: 'https://ku8.org.ng/' },
  { title: 'NITPCS Nigeria',         id: '9b79efaabbd89baed14a4b0889ef87b8',  url: 'https://nitpcs.org.ng/' },
  { title: 'Lincoln Adighije',       id: '64450390e004f311595188a6bae321f5',  url: 'https://www.lincolnadighije.com/' },
  { title: 'Assurance by Jummy',     id: 'f5ed10099926d97beadbcea984b42740',  url: 'https://assurancebyjummy.com.ng/' },
  { title: 'OAJ Energy',             id: 'b20a685d5aae31ced310cf5b195656cb',  url: 'https://oajenergy.com.ng/' },
  { title: 'Sola by Adesola',        id: '0f9a971fc3ed59b6b438ff7876d273d1',  url: 'https://www.solabyadesola.com.ng/' },
  { title: 'OptimaMind',             id: 'c7d3f888a4b0d77a610b99a8a80dd365',  url: 'https://www.optimamind.com.ng/' },
  { title: 'Ibrahim Energy',         id: '9c942e2ee55dd4b75f5e74d24dad9e74',  url: 'https://www.ibrahimenergy.com.ng/' },
  { title: 'Kazeem Digitals',        id: '53cbbe2f2530ef77c600a01fc6201126',  url: 'https://www.kazeemdigitals.com.ng/' },
  { title: 'Bammy John',             id: 'f8508bd5bdb31098a2139226152ed38e',  url: 'https://www.bammyjohn.com.ng/' },
  { title: 'FormSend',               id: '7a6b5603755352d3ed9605aae96f07f8',  url: 'https://formsend.ezeroandone.io/' },
];

// thum.io — free, no key, returns a live screenshot image URL directly
function thumUrl(siteUrl) {
  // Use HTTPS version of the URL for thum.io
  const https = siteUrl.replace(/^http:\/\//, 'https://');
  return `https://image.thum.io/get/width/1280/crop/720/noanimate/${https}`;
}

// screenshotmachine.com — free tier (100/month), no key needed for basic
function screenshotMachineUrl(siteUrl) {
  return `https://api.screenshotmachine.com/?url=${encodeURIComponent(siteUrl)}&dimension=1280x720&format=jpg&cacheLimit=0`;
}

async function verifyImageUrl(url) {
  try {
    const r = await fetch(url, { method: 'HEAD' });
    return r.ok && (r.headers.get('content-type') ?? '').startsWith('image/');
  } catch { return false; }
}

async function getScreenshotUrl(siteUrl) {
  // Try thum.io first — it returns the image directly at a stable URL
  const thumbUrl = thumUrl(siteUrl);
  const ok = await verifyImageUrl(thumbUrl);
  if (ok) return thumbUrl;

  // Fallback: microlink
  try {
    const api = `https://api.microlink.io/?url=${encodeURIComponent(siteUrl)}&screenshot=true&meta=false&embed=screenshot.url&waitFor=2000`;
    const res = await fetch(api, { headers: { Accept: 'application/json' } });
    if (res.ok) {
      const json = await res.json();
      const imgUrl = json?.data?.screenshot?.url;
      if (imgUrl) return imgUrl;
    }
  } catch { /* ignore */ }

  return null;
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
  console.log(`\n📸  Adding screenshots to ${POSTS.length} posts\n`);
  let ok = 0;

  for (const post of POSTS) {
    process.stdout.write(`  ${post.title}… `);
    try {
      const imgUrl = await getScreenshotUrl(post.url);
      if (!imgUrl) { console.log('no screenshot available'); continue; }

      await patchPost(post.id, { featured_image_url: imgUrl });
      console.log(`✅  ${imgUrl.substring(0, 80)}…`);
      ok++;
    } catch (err) {
      console.log(`❌  ${err.message}`);
    }
    await new Promise(r => setTimeout(r, 800));
  }

  console.log(`\n  ${ok}/${POSTS.length} screenshots added.\n`);
}

main().catch(err => { console.error(err); process.exit(1); });
