/**
 * populate-work.mjs
 *
 * Populates work posts via the eZeroAndOne admin API.
 * Run locally while `wrangler pages dev` is active, or point BASE_URL at production.
 *
 * Usage:
 *   node scripts/populate-work.mjs
 *
 * Requires:
 *   - SESSION_COOKIE env var set to your admin session cookie value
 *   - (optional) BASE_URL env var (defaults to http://localhost:8788)
 *
 * Screenshots are fetched via api.microlink.io (free, no key needed).
 */

import { readFileSync, writeFileSync, existsSync } from 'fs';
import { join, dirname } from 'path';
import { fileURLToPath } from 'url';

const __dirname = dirname(fileURLToPath(import.meta.url));

const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';
const SESSION  = process.env.SESSION_COOKIE ?? '';

if (!SESSION) {
  console.error('❌  Set SESSION_COOKIE env var to your admin session cookie value.');
  console.error('    Example: SESSION_COOKIE="session=eyJ..." node scripts/populate-work.mjs');
  process.exit(1);
}

// ---------------------------------------------------------------------------
// Work post data
// ---------------------------------------------------------------------------

const WORKS = [
  // ── WordPress / Branding
  {
    url: 'http://dotuntaylor.com/',
    title: 'Dotun Taylor',
    summary: 'Personal brand website for Dotun Taylor — thought leader, speaker, and executive coach. Clean editorial design with a focus on content and credibility.',
    category: 'Branding & Web Design',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, CSS',
    tags: 'personal brand, speaking, coaching',
    body_md: `## Overview

Dotun Taylor required a digital presence that communicated authority, warmth, and expertise. The brief called for an editorial-quality website that would serve as a hub for speaking engagements, publications, and executive coaching services.

## Approach

The design centres on typography-first layouts with generous white space, drawing the visitor's attention to content rather than decoration. A custom WordPress theme built on Elementor provides the client full editorial autonomy over page content.

## Outcome

A polished personal brand platform that has since served as the foundation for booking engagements and positioning Dotun Taylor as a recognised voice in leadership and professional development.`,
    branded: true,
  },
  {
    url: 'https://ojistravels.com/',
    title: 'Ojis Travels',
    summary: 'Travel agency website for Ojis Travels — showcasing curated tour packages, destination guides, and seamless booking enquiry flow.',
    category: 'Branding & Web Design',
    project_type: 'WordPress Website',
    technologies: 'WordPress, WooCommerce, Divi',
    tags: 'travel, tourism, booking, agency',
    body_md: `## Overview

Ojis Travels needed a digital storefront that communicated wanderlust and trustworthiness in equal measure — a site that would convert visitors into enquiries and repeat bookings.

## Approach

The visual direction leans into destination photography, with a card-based layout surfacing tour packages prominently above the fold. A WooCommerce-backed enquiry system captures lead data while keeping the user journey frictionless.

## Outcome

Increased inbound enquiries from organic search traffic, with repeat clients citing the clarity of the packages page as the primary driver of their decision to book.`,
    branded: true,
  },
  {
    url: 'https://www.risyn.ai/',
    title: 'Risyn AI',
    summary: 'B2B SaaS landing page for Risyn AI — an artificial intelligence platform. Designed to communicate technical credibility and drive demo sign-ups.',
    category: 'Branding & Web Design',
    project_type: 'Web App / Landing Page',
    technologies: 'React, TailwindCSS, Framer Motion',
    tags: 'AI, SaaS, B2B, product, landing page',
    body_md: `## Overview

Risyn AI is a B2B artificial intelligence platform targeting enterprise clients. The brief was a conversion-focused landing page that would communicate technical depth without alienating non-technical decision-makers.

## Approach

We built a React-based single-page application with scroll-driven animations via Framer Motion. The information architecture follows a problem-solution-proof structure, supported by data visualisation components that demonstrate the platform's capabilities without requiring a demo.

## Outcome

The launch page drove a measurable uplift in demo sign-up rates and was cited by the founding team as the primary asset used during investor and partner conversations.`,
    branded: true,
  },
  {
    url: 'https://www.mitkeda.com/',
    title: 'Mitkeda',
    summary: 'E-commerce and brand identity for Mitkeda — a fashion and lifestyle brand. Complete digital presence from logo to storefront.',
    category: 'Branding & Web Design',
    project_type: 'WordPress Website',
    technologies: 'WordPress, WooCommerce, Custom Theme',
    tags: 'fashion, e-commerce, brand identity, lifestyle',
    body_md: `## Overview

Mitkeda is a fashion and lifestyle brand that required both a brand identity from scratch and a fully functional e-commerce storefront to match.

## Approach

Brand identity work encompassed logo design, colour palette, typography system, and brand guidelines. The storefront was built on WordPress with WooCommerce, with a custom theme developed to express the brand's visual language across product listings, editorial content, and checkout.

## Outcome

A cohesive digital brand presence with a storefront capable of handling product catalogue management, inventory, and payment processing independently.`,
    branded: true,
  },

  // ── WordPress only
  {
    url: 'https://www.hizonllc.com/',
    title: 'Hizon LLC',
    summary: 'Corporate website for Hizon LLC — a US-based professional services firm. Clean, professional design built to support client acquisition.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, SEO',
    tags: 'corporate, professional services, USA',
    body_md: `## Overview

Hizon LLC required a professional corporate website to support business development and client acquisition in the US market. The brief prioritised credibility, clarity, and mobile performance.

## Approach

A clean, uncluttered design using Elementor with a custom page structure optimised for service-industry lead generation. On-page SEO was implemented from the ground up, targeting high-intent keywords for the firm's primary service lines.

## Outcome

A well-performing corporate digital presence that communicates professionalism and supports the firm's business development pipeline.`,
  },
  {
    url: 'https://ttp.ng/',
    title: 'TTP Nigeria',
    summary: 'WordPress website for TTP Nigeria — a consulting and professional development firm. Structured for service clarity and lead generation.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'consulting, Nigeria, professional development',
    body_md: `## Overview

TTP Nigeria is a consulting firm providing professional development and capacity-building services. The website needed to clearly communicate service offerings and drive enquiry conversions.

## Approach

Built on WordPress with Elementor, the site architecture maps directly to the client's service taxonomy. Contact and enquiry forms are integrated at multiple points in the user journey to capture inbound leads efficiently.

## Outcome

A clean, functional digital presence that serves as the primary inbound channel for the firm's consulting pipeline.`,
  },
  {
    url: 'https://unicornhrconsulting.com/',
    title: 'Unicorn HR Consulting',
    summary: 'HR consulting firm website with a strong service-focus and credibility-building content architecture.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, WPForms',
    tags: 'HR, consulting, talent, recruitment',
    body_md: `## Overview

Unicorn HR Consulting provides human resources advisory and outsourced HR services. The website required a design language that communicated both professional authority and approachability.

## Approach

The information architecture surfaces service definitions prominently, supported by case study references and team credibility signals. Lead capture is integrated throughout via WPForms, with routing to the appropriate service enquiry handler.

## Outcome

A functional, professional website serving as the primary channel for client acquisition and service communication.`,
  },
  {
    url: 'https://unicorndigital.ng/',
    title: 'Unicorn Digital',
    summary: 'Digital agency website for Unicorn Digital Nigeria — showcasing services, case studies, and team capabilities.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'digital agency, Nigeria, marketing',
    body_md: `## Overview

Unicorn Digital is a Nigerian digital agency requiring a portfolio-driven website that would effectively communicate their service range and attract B2B clients.

## Approach

A service-first layout that positions the agency's capabilities alongside real client outcomes. Portfolio cards and case study teasers support credibility, while clear CTAs drive enquiries to the appropriate service team.

## Outcome

A professional agency website positioned to compete effectively in the Nigerian digital services market.`,
  },
  {
    url: 'http://joshuaronatus.com/',
    title: 'Joshua Ronatus',
    summary: 'Personal brand website for Joshua Ronatus — showcasing professional services, thought leadership, and portfolio work.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'personal brand, portfolio, professional',
    body_md: `## Overview

Joshua Ronatus required a personal brand website to serve as a central hub for professional services, thought leadership content, and portfolio work.

## Approach

A single-page-style WordPress build using Elementor, structured to communicate personal brand values and route visitors to the most relevant service or content area.

## Outcome

A clean, effective personal brand digital presence supporting ongoing professional development and client acquisition.`,
  },
  {
    url: 'https://tobeasikoko.com/',
    title: 'Tobe Asikoko',
    summary: 'Personal brand and portfolio website for Tobe Asikoko — creative professional and entrepreneur.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'personal brand, creative, entrepreneur',
    body_md: `## Overview

Tobe Asikoko needed a personal brand website that reflected a creative and entrepreneurial identity while maintaining a professional, client-ready presentation.

## Approach

The design emphasises visual personality through considered use of colour, typography, and imagery. Built on WordPress with Elementor for ongoing editorial flexibility.

## Outcome

A distinctive personal brand digital presence that stands apart from template-driven personal sites.`,
  },
  {
    url: 'https://csuitebrandpartners.co.uk/',
    title: 'C-Suite Brand Partners',
    summary: 'Executive brand consultancy website for C-Suite Brand Partners — a UK-based firm advising senior leaders on personal brand strategy.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, SEO',
    tags: 'executive branding, UK, consulting, C-suite',
    body_md: `## Overview

C-Suite Brand Partners advises senior executives and board-level professionals on personal brand strategy and positioning. The website required a design that communicated premium authority and discretion.

## Approach

A restrained, editorial design language that avoids visual noise in favour of substance. Copy and layout work together to communicate premium positioning. UK-targeted SEO implemented across all key service pages.

## Outcome

A premium brand consulting website that successfully positions the firm at the C-suite end of the personal branding market.`,
  },
  {
    url: 'https://ku8.org.ng/',
    title: 'KU8 Nigeria',
    summary: 'Non-profit organisation website for KU8 — a Nigerian youth and community development organisation.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'non-profit, NGO, Nigeria, community, youth',
    body_md: `## Overview

KU8 is a Nigerian non-profit organisation focused on youth empowerment and community development. The website needed to communicate mission, impact, and pathways for donors and volunteers to get involved.

## Approach

A mission-forward design that leads with impact stories and programme information. Volunteer and donation pathways are integrated prominently, with clear calls to action at every stage of the user journey.

## Outcome

A functional non-profit digital presence that effectively communicates the organisation's mission and supports community engagement.`,
  },
  {
    url: 'https://nitpcs.org.ng/',
    title: 'NITPCS Nigeria',
    summary: 'Professional council website for the Nigerian Institute of Transport and Procurement Consultants and Suppliers.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, WPForms',
    tags: 'professional council, Nigeria, transport, procurement',
    body_md: `## Overview

NITPCS is a Nigerian professional council requiring a formal institutional website to support membership registration, communicate council activities, and serve as an authoritative reference for the industry.

## Approach

A structured institutional design language appropriate for a professional council. Member registration forms, event announcements, and council governance documentation are all integrated into a clear information architecture.

## Outcome

A functional institutional website serving the council's membership and public communication needs.`,
  },
  {
    url: 'https://www.lincolnadighije.com/',
    title: 'Lincoln Adighije',
    summary: 'Personal brand website for Lincoln Adighije — lawyer, author, and public intellectual.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'personal brand, law, author, thought leadership',
    body_md: `## Overview

Lincoln Adighije is a lawyer, author, and public intellectual requiring a personal brand website that communicated expertise across multiple disciplines — law, writing, and public commentary.

## Approach

A content-led design that foregrounds publications, speaking engagements, and professional biography. The layout supports ongoing editorial updates via a WordPress CMS, allowing the client to publish commentary and announcements independently.

## Outcome

A professional personal brand platform that effectively communicates Lincoln Adighije's multi-disciplinary expertise.`,
  },
  {
    url: 'https://assurancebyjummy.com.ng/',
    title: 'Assurance by Jummy',
    summary: 'E-commerce website for Assurance by Jummy — a Nigerian beauty and wellness brand.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, WooCommerce, Elementor',
    tags: 'beauty, wellness, e-commerce, Nigeria',
    body_md: `## Overview

Assurance by Jummy is a beauty and wellness brand requiring a consumer e-commerce storefront that would communicate brand identity and support direct-to-consumer product sales.

## Approach

A WooCommerce-based storefront built on WordPress with a custom Elementor theme designed to express the brand's feminine, premium aesthetic. Product photography integration and a streamlined checkout flow support conversion.

## Outcome

A functional beauty e-commerce platform enabling the brand to sell direct-to-consumer and manage its product catalogue independently.`,
  },
  {
    url: 'https://oajenergy.com.ng/',
    title: 'OAJ Energy',
    summary: 'Corporate website for OAJ Energy — a Nigerian energy sector company. Professional design for a regulated industry context.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, SEO',
    tags: 'energy, oil and gas, Nigeria, corporate',
    body_md: `## Overview

OAJ Energy operates in Nigeria's energy sector and required a corporate website that communicated stability, compliance, and capability to industry partners and regulators.

## Approach

A formal, authoritative design language appropriate for the regulated energy sector. Company profile, service offerings, and regulatory compliance credentials are presented clearly. On-page SEO targets industry-specific search terms.

## Outcome

A professional corporate digital presence positioned for the Nigerian energy sector.`,
  },
  {
    url: 'https://www.solabyadesola.com.ng/',
    title: 'Sola by Adesola',
    summary: 'Fashion brand website for Sola by Adesola — a Nigerian fashion designer showcasing collections and enabling custom order enquiries.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, WooCommerce, Elementor',
    tags: 'fashion, Nigeria, designer, collections',
    body_md: `## Overview

Sola by Adesola is a Nigerian fashion brand requiring a digital showroom to showcase collections and enable custom order enquiries from local and diaspora clients.

## Approach

A visually led design that prioritises photography of the designer's work. WooCommerce handles product catalogue management, while a custom enquiry flow routes bespoke order requests to the design team.

## Outcome

A fashion-forward digital presence enabling the brand to reach a wider audience and handle inbound custom orders efficiently.`,
  },
  {
    url: 'https://www.optimamind.com.ng/',
    title: 'OptimaMind',
    summary: 'Consultancy website for OptimaMind — a Nigerian management and organisational development consultancy.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, WPForms',
    tags: 'consulting, management, organisational development, Nigeria',
    body_md: `## Overview

OptimaMind is a management and organisational development consultancy requiring a website that would communicate expertise and attract corporate and institutional clients.

## Approach

A clean, professional design structured around the consultancy's service taxonomy. Credibility signals — including client logos and service descriptions — are positioned prominently throughout the site.

## Outcome

A professional consulting website serving as the primary channel for OptimaMind's new business pipeline.`,
  },
  {
    url: 'https://www.ibrahimenergy.com.ng/',
    title: 'Ibrahim Energy',
    summary: 'Corporate website for Ibrahim Energy — a Nigerian energy company. Formal design for an industry context requiring regulatory credibility.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor, SEO',
    tags: 'energy, Nigeria, corporate, oil and gas',
    body_md: `## Overview

Ibrahim Energy required a corporate digital presence that demonstrated regulatory credibility and communicated the company's capabilities to partners, clients, and industry bodies.

## Approach

A formal, structured design built on WordPress with Elementor. Company profile, capabilities, and compliance documentation are well signposted. SEO targets energy sector keywords in the Nigerian market.

## Outcome

A credible corporate website serving the company's partner communication and new business objectives.`,
  },
  {
    url: 'https://www.kazeemdigitals.com.ng/',
    title: 'Kazeem Digitals',
    summary: 'Digital marketing agency website for Kazeem Digitals — a Nigerian agency offering social media, SEO, and digital advertising services.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'digital marketing, agency, Nigeria, SEO, social media',
    body_md: `## Overview

Kazeem Digitals is a Nigerian digital marketing agency requiring a web presence that would serve as a lead generation asset for the agency's own services — effectively demonstrating their digital capabilities to potential clients.

## Approach

A service-focused design that surfaces core offerings prominently while positioning the agency through social proof and clear service definitions. The site itself serves as a demonstration of the agency's digital execution capabilities.

## Outcome

A professional agency website that functions as a credible lead generation platform for the Kazeem Digitals team.`,
  },
  {
    url: 'https://www.bammyjohn.com.ng/',
    title: 'Bammy John',
    summary: 'Personal brand and business website for Bammy John — entrepreneur and digital professional.',
    category: 'Web Design & Development',
    project_type: 'WordPress Website',
    technologies: 'WordPress, Elementor',
    tags: 'personal brand, entrepreneur, Nigeria',
    body_md: `## Overview

Bammy John required a personal brand website to support business development and professional positioning as an entrepreneur and digital professional in the Nigerian market.

## Approach

A clear, professional personal brand site built on WordPress with Elementor. Service offerings, professional biography, and contact pathways are all well integrated.

## Outcome

A functional personal brand digital presence supporting Bammy John's professional and business objectives.`,
  },
  {
    url: 'https://formsend.ezeroandone.io/',
    title: 'FormSend',
    summary: 'Internal product — a Cloudflare Workers-powered form endpoint service enabling no-backend form submissions for static sites.',
    category: 'Engineering & Infrastructure',
    project_type: 'Web App',
    technologies: 'Cloudflare Workers, TypeScript, D1, Email Routing',
    tags: 'product, form endpoint, Cloudflare Workers, serverless',
    body_md: `## Overview

FormSend is an internal eZeroAndOne product: a form-to-email endpoint service built entirely on Cloudflare Workers. It enables static site owners to receive form submissions without running a backend server.

## The Problem

Static sites built on Jamstack architecture commonly need a contact or enquiry form but lack a server-side component to handle form submissions. Third-party form services add cost, data compliance concerns, and vendor dependency.

## Architecture

FormSend is a Cloudflare Worker that receives POST requests from HTML forms, validates submission data, stores it in D1, and routes notification emails via Cloudflare Email Routing. There is no origin server — the entire service runs at the edge.

Key components:
- **Edge handler**: TypeScript Cloudflare Worker receiving and validating form data
- **Persistent storage**: D1 SQLite database storing submission records with TTL-based retention
- **Email delivery**: Cloudflare Email Routing forwards formatted submission emails to configured recipients
- **Spam protection**: Honeypot field detection and rate limiting per origin domain

## Outcome

A zero-cost (within Cloudflare Free Tier), zero-maintenance form endpoint service deployed to 300+ edge locations globally. Used across multiple eZeroAndOne client projects as the default form backend.`,
  },
];

// ---------------------------------------------------------------------------
// Screenshot helper via Microlink API
// ---------------------------------------------------------------------------

async function getScreenshot(url) {
  try {
    const apiUrl = `https://api.microlink.io/?url=${encodeURIComponent(url)}&screenshot=true&meta=false&embed=screenshot.url`;
    const res = await fetch(apiUrl, { headers: { Accept: 'application/json' } });
    if (!res.ok) return null;
    const json = await res.json();
    return json?.data?.screenshot?.url ?? null;
  } catch {
    return null;
  }
}

// ---------------------------------------------------------------------------
// Admin API helpers
// ---------------------------------------------------------------------------

function authHeaders() {
  return {
    'Content-Type': 'application/json',
    Cookie: `session=${SESSION}`,
  };
}

async function createPost(work) {
  const slug = work.title
    .toLowerCase()
    .replace(/[^a-z0-9\s-]/g, '')
    .replace(/\s+/g, '-')
    .replace(/-+/g, '-');

  const body = {
    type: 'work',
    title: work.title,
    summary: work.summary,
    body_md: work.body_md ?? '',
    category: work.category,
    project_type: work.project_type,
    technologies: work.technologies,
    tags: work.tags,
    featured_image_url: '',
  };

  const res = await fetch(`${BASE_URL}/api/admin/content`, {
    method: 'POST',
    headers: authHeaders(),
    body: JSON.stringify(body),
  });

  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Create failed for "${work.title}": ${res.status} ${text}`);
  }

  return await res.json();
}

async function patchPost(id, fields) {
  const res = await fetch(`${BASE_URL}/api/admin/content/${id}`, {
    method: 'PATCH',
    headers: authHeaders(),
    body: JSON.stringify(fields),
  });
  if (!res.ok) {
    const text = await res.text();
    throw new Error(`Patch failed for ${id}: ${res.status} ${text}`);
  }
  return await res.json();
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function main() {
  console.log(`\n🚀  Populating ${WORKS.length} work posts at ${BASE_URL}\n`);

  const results = [];
  const STATE_FILE = join(__dirname, '.populate-state.json');
  const state = existsSync(STATE_FILE)
    ? JSON.parse(readFileSync(STATE_FILE, 'utf8'))
    : {};

  for (const work of WORKS) {
    const key = work.title;

    if (state[key]?.done) {
      console.log(`  ✅  ${work.title} — already done, skipping`);
      results.push(state[key]);
      continue;
    }

    process.stdout.write(`  ⏳  ${work.title}… `);

    try {
      // 1. Create the post
      const post = await createPost(work);
      process.stdout.write('created');

      // 2. Get screenshot
      let screenshotUrl = null;
      try {
        screenshotUrl = await getScreenshot(work.url);
        if (screenshotUrl) process.stdout.write(', screenshot');
        else process.stdout.write(', no screenshot');
      } catch {
        process.stdout.write(', screenshot failed');
      }

      // 3. Patch with screenshot + publish
      const publishedAt = Math.floor(Date.now() / 1000);
      const patches = {
        published: true,
        published_at: publishedAt,
      };
      if (screenshotUrl) patches.featured_image_url = screenshotUrl;

      await patchPost(post.id, patches);
      process.stdout.write(', published');

      const result = { done: true, id: post.id, slug: post.slug, url: work.url, screenshotUrl };
      state[key] = result;
      results.push(result);
      writeFileSync(STATE_FILE, JSON.stringify(state, null, 2));

      console.log(` ✅`);

      // Rate-limit: 1.5s between posts to avoid D1 write pressure
      await new Promise(r => setTimeout(r, 1500));

    } catch (err) {
      console.log(` ❌  ${err.message}`);
      state[key] = { done: false, error: err.message };
      writeFileSync(STATE_FILE, JSON.stringify(state, null, 2));
    }
  }

  console.log('\n📋  Summary:');
  results.forEach(r => {
    if (r.done) console.log(`  ✅  /work/${r.slug} — ${r.url}`);
    else console.log(`  ❌  ${r.error}`);
  });

  const done = results.filter(r => r.done).length;
  console.log(`\n  ${done}/${WORKS.length} posts created and published.\n`);
}

main().catch(err => { console.error(err); process.exit(1); });
