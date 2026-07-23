/**
 * patch-images.mjs — patches featured_image_url on capabilities + insights
 * Uses curated Unsplash photo IDs (stable, no API key needed)
 * Run: node scripts/patch-images.mjs
 */

const SESSION  = process.env.SESSION_COOKIE ?? '';
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';
if (!SESSION) { console.error('Set SESSION_COOKIE'); process.exit(1); }

// Stable Unsplash photos by their ID (unsplash.com/photos/<id>)
// Direct CDN format: https://images.unsplash.com/photo-<id>?w=1280&q=80
const u = id => `https://images.unsplash.com/photo-${id}?w=1280&q=80&auto=format&fit=crop`;

const IMAGES = {
  // Capabilities — matched by title substring
  'Web Application Development':       u('1555066931-bf19f8fd1085'), // code on screen
  'Mobile App Development':            u('1512941937669-90a1b58e7e9c'), // smartphone dev
  'Desktop & Enterprise Software':     u('1461749280684-dccba630e2f6'), // code monitor
  'Cloud Architecture & DevOps':       u('1451187580459-43490279c0fa'), // cloud servers
  'Network Design & Management':       u('1558494949-ef010cbdcc31'), // network cables
  'Hardware Procurement & Installation': u('1518770660439-4636190af475'), // circuit board
  'Cybersecurity Assessment & Hardening': u('1550751827-4bd374c3f58b'), // security lock
  'IT Auditing & Compliance Advisory': u('1507003211169-0a1dd7228f2d'), // audit checklist
  'Managed Security Operations':       u('1563986768609-322da13575f3'), // SOC monitors
  'Digital Marketing & SEO':           u('1432888498266-38ffec3eaf0a'), // analytics dashboard
  'Brand Identity & Web Design':       u('1561070791-2526d30994b5'), // design tools
  'E-Commerce Development':            u('1556742049-0cfed4f6a45d'), // shopping online
  'Smart Home & Building Automation':  u('1558618666-fcd25c85cd64'), // smart home
  'Renewable Energy Installation':     u('1509391366360-2e959784a276'), // solar panels
  'IoT & Industrial Automation':       u('1573164713988-8665fc963095'), // IoT sensors
  'IT Strategy & Consulting':          u('1454165804606-c3d57bc86b40'), // boardroom strategy
  'IT Training & Staff Development':   u('1531482615713-2afd69097998'), // training session
  'Help Desk & Managed IT Support':    u('1486312338219-ce68d2c6f44d'), // help desk
  // Insights
  'Why Nigerian Businesses Are Losing Money': u('1573164713988-8665fc963095'),
  'Complete Guide to Solar Power':            u('1509391366360-2e959784a276'),
  'NDPR Compliance':                          u('1550751827-4bd374c3f58b'),
  'Native App and a Web App':                 u('1512941937669-90a1b58e7e9c'),
  'Penetration Testing':                      u('1563986768609-322da13575f3'),
  'IT Infrastructure for Nigerian Startups':  u('1451187580459-43490279c0fa'),
  'Website That Actually Ranks on Google':    u('1432888498266-38ffec3eaf0a'),
  'Smart Office Technology':                  u('1558618666-fcd25c85cd64'),
  'Cloud vs On-Premise':                      u('1451187580459-43490279c0fa'),
  'How Much Does a Website Cost':             u('1561070791-2526d30994b5'),
  'IT Managed Services vs In-House IT':       u('1486312338219-ce68d2c6f44d'),
  'Right ERP for a Nigerian Business':        u('1454165804606-c3d57bc86b40'),
};

async function patchPost(id, fields) {
  const res = await fetch(`${BASE_URL}/api/admin/content/${id}`, {
    method: 'PATCH',
    headers: { 'Content-Type': 'application/json', Cookie: `session=${SESSION}` },
    body: JSON.stringify(fields),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`${res.status}: ${text.substring(0,150)}`);
}

async function main() {
  // Fetch all posts
  const res = await fetch(`${BASE_URL}/api/admin/content`, {
    headers: { Cookie: `session=${SESSION}` }
  });
  const posts = await res.json();

  console.log(`\n📸  Patching images on ${posts.length} posts\n`);
  let ok = 0;

  for (const post of posts) {
    if (post.featured_image_url) continue; // already has one

    // Match by title prefix
    const match = Object.keys(IMAGES).find(k =>
      post.title.toLowerCase().includes(k.toLowerCase().substring(0, 20))
    );
    if (!match) continue;

    process.stdout.write(`  ${post.title.substring(0, 55)}… `);
    try {
      await patchPost(post.id, { featured_image_url: IMAGES[match] });
      console.log('✅');
      ok++;
    } catch (err) {
      console.log(`❌  ${err.message}`);
    }
    await new Promise(r => setTimeout(r, 400));
  }

  console.log(`\n  ${ok} images patched.\n`);
}

main().catch(err => { console.error(err); process.exit(1); });
