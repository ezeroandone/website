<script lang="ts">
  import { onMount } from 'svelte';
  import GlassCard from '$lib/components/GlassCard.svelte';
  import NeuralBrain from '$lib/components/NeuralBrain.svelte';
  import type { PageData } from './$types';

  let { data }: { data: PageData } = $props();

  // ── Service pillars (4 core verticals shown on landing) ────────────
  const pillars = [
    {
      icon: 'layers',
      title: 'Digital Architecture',
      body: "Custom web apps, APIs, and enterprise platforms engineered on Cloudflare's edge — sub-100ms globally.",
      href: '/capabilities#software-engineering',
      accent: '#00c2ff',
    },
    {
      icon: 'security',
      title: 'Cybersecurity & Compliance',
      body: "Penetration testing, NDPR/ISO 27001 audits, and managed SOC operations — protecting what you've built.",
      href: '/capabilities#cybersecurity',
      accent: '#ff3366',
    },
    {
      icon: 'router',
      title: 'Infrastructure & Hardware',
      body: 'Fault-tolerant networks, server procurement, smart building systems, and renewable energy installations.',
      href: '/capabilities#infrastructure',
      accent: '#00e676',
    },
    {
      icon: 'insights',
      title: 'IT Strategy & Growth',
      body: 'CTO advisory, digital marketing, brand identity, and managed IT support — aligning technology to revenue.',
      href: '/capabilities#consulting',
      accent: '#ffd600',
    },
  ];

  // ── GSAP scroll-triggered animations ──────────────────────────────
  onMount(async () => {
    try {
      const { gsap } = await import('gsap');
      const { ScrollTrigger } = await import('gsap/ScrollTrigger');
      gsap.registerPlugin(ScrollTrigger);

      // Hero left column stagger entrance
      gsap.fromTo('.hero-eyebrow',
        { opacity: 0, y: 20 },
        { opacity: 1, y: 0, duration: 0.7, ease: 'power3.out', delay: 0.1 }
      );
      gsap.fromTo('.hero-h1',
        { opacity: 0, y: 40 },
        { opacity: 1, y: 0, duration: 0.9, ease: 'power3.out', delay: 0.25 }
      );
      gsap.fromTo('.hero-sub',
        { opacity: 0, y: 24 },
        { opacity: 1, y: 0, duration: 0.8, ease: 'power3.out', delay: 0.45 }
      );
      gsap.fromTo('.hero-ctas',
        { opacity: 0, y: 20 },
        { opacity: 1, y: 0, duration: 0.7, ease: 'power3.out', delay: 0.6 }
      );
      gsap.fromTo('.hero-badge',
        { opacity: 0, scale: 0.9 },
        { opacity: 1, scale: 1, duration: 0.6, ease: 'back.out(1.5)', delay: 0.8 }
      );
      gsap.fromTo('.hero-right',
        { opacity: 0, x: 40 },
        { opacity: 1, x: 0, duration: 1.1, ease: 'power3.out', delay: 0.3 }
      );

      // Parallax on hero brain
      gsap.to('.hero-right', {
        scrollTrigger: { trigger: '.hero', scrub: 1.5 },
        y: -80,
        ease: 'none',
      });

      // Philosophy parallax
      gsap.fromTo('.phil-left',
        { opacity: 0, x: -40 },
        {
          opacity: 1, x: 0, duration: 0.9, ease: 'power3.out',
          scrollTrigger: { trigger: '.philosophy', start: 'top 75%' }
        }
      );
      gsap.fromTo('.phil-right',
        { opacity: 0, x: 40 },
        {
          opacity: 1, x: 0, duration: 0.9, ease: 'power3.out', delay: 0.15,
          scrollTrigger: { trigger: '.philosophy', start: 'top 75%' }
        }
      );

      // Pillar cards stagger
      gsap.fromTo('.pillar-card',
        { opacity: 0, y: 50 },
        {
          opacity: 1, y: 0, stagger: 0.12, duration: 0.7, ease: 'power3.out',
          scrollTrigger: { trigger: '.pillars-grid', start: 'top 80%' }
        }
      );

      // Work cards stagger
      gsap.fromTo('.work-card',
        { opacity: 0, y: 40, scale: 0.97 },
        {
          opacity: 1, y: 0, scale: 1, stagger: 0.1, duration: 0.65, ease: 'power3.out',
          scrollTrigger: { trigger: '.cards-grid--work', start: 'top 82%' }
        }
      );

      // Insight cards stagger
      gsap.fromTo('.insight-card',
        { opacity: 0, y: 40, scale: 0.97 },
        {
          opacity: 1, y: 0, scale: 1, stagger: 0.1, duration: 0.65, ease: 'power3.out',
          scrollTrigger: { trigger: '.cards-grid--insights', start: 'top 82%' }
        }
      );

      // Section headings slide up
      gsap.utils.toArray<HTMLElement>('.section-h2').forEach(el => {
        gsap.fromTo(el,
          { opacity: 0, y: 30 },
          {
            opacity: 1, y: 0, duration: 0.8, ease: 'power3.out',
            scrollTrigger: { trigger: el, start: 'top 85%' }
          }
        );
      });

      // Stats / intake section parallax background
      gsap.to('.intake', {
        scrollTrigger: { trigger: '.intake', scrub: 2 },
        backgroundPositionY: '30%',
        ease: 'none',
      });

      // Client logos auto-scroll animation
      const track = document.querySelector('.clients-track') as HTMLElement;
      if (track) {
        gsap.to(track, {
          xPercent: -50,
          duration: 28,
          ease: 'none',
          repeat: -1,
          modifiers: { xPercent: gsap.utils.wrap(-50, 0) }
        });
      }

    } catch (e) {
      // GSAP not available (SSR or missing module) — graceful degradation
      console.warn('GSAP animations unavailable:', e);
    }
  });
</script>

<svelte:head>
  <title>eZeroAndOne — We Deconstruct Complexity. You Create Possibilities.</title>
  <meta name="description" content="eZeroAndOne builds scalable digital products from first principles — software engineering, cybersecurity, IT infrastructure, and renewable energy installations for Nigerian businesses." />
  <link href="https://fonts.googleapis.com/css2?family=Inter+Tight:wght@400;700;800;900&family=Outfit:wght@300;400;500;600&display=swap" rel="stylesheet" />
</svelte:head>

<!-- ══ HERO — two-column split ═══════════════════════════════════════ -->
<section class="hero" aria-labelledby="hero-h1">
  <div class="inner hero-inner">

    <!-- Left: copy + CTAs -->
    <div class="hero-left">
      <p class="hero-eyebrow">
        <span class="eyebrow-dot" aria-hidden="true"></span>
        Cloudflare-native · First principles engineering
      </p>

      <h1 id="hero-h1" class="hero-h1">
        We Deconstruct<br />
        Complexity.<br />
        <span class="hero-h1-accent">You Create<br />Possibilities.</span>
      </h1>

      <p class="hero-sub">
        Every breakthrough digital platform begins with a single electron — a zero or a one.
        We reduce complex business problems to their simplest form, then build scalable,
        secure systems that compound in value over time.
      </p>

      <div class="hero-ctas">
        <a href="/capabilities" class="btn btn-primary hero-cta-primary">
          Initialize Project
          <span aria-hidden="true">→</span>
        </a>
        <a href="#what-we-do" class="btn btn-secondary hero-cta-secondary">
          What We Do
        </a>
      </div>

      {#if data.openRoles > 0}
        <a href="/careers" class="hero-badge">
          <span class="live-dot" aria-hidden="true"></span>
          {data.openRoles} active {data.openRoles === 1 ? 'role' : 'roles'} open
          <span aria-hidden="true">›</span>
        </a>
      {/if}

      <!-- Trust signals -->
      <div class="hero-trust">
        <span class="trust-item">
          <span class="trust-icon" aria-hidden="true">✓</span>
          NDPR Compliant
        </span>
        <span class="trust-sep" aria-hidden="true">·</span>
        <span class="trust-item">
          <span class="trust-icon" aria-hidden="true">✓</span>
          ISO 27001 Aligned
        </span>
        <span class="trust-sep" aria-hidden="true">·</span>
        <span class="trust-item">
          <span class="trust-icon" aria-hidden="true">✓</span>
          Edge-Native Architecture
        </span>
      </div>
    </div>

    <!-- Right: interactive neural brain -->
    <div class="hero-right" aria-hidden="true">
      <NeuralBrain />
    </div>

  </div>
</section>

<!-- ══ CLIENT LOGOS — scrolling ticker ════════════════════════════ -->
{#if data.clients.length > 0}
<section class="clients-bar" aria-label="Brands we have worked with">
  <div class="inner clients-inner">
    <p class="clients-label">Trusted by</p>
    <div class="clients-track-wrap">
      <div class="clients-track">
        {#each [...data.clients, ...data.clients] as client}
          {#if client.website_url}
            <a href={client.website_url} target="_blank" rel="noopener noreferrer"
               class="client-logo-link" title={client.name} aria-label={client.name}>
              <img src={client.logo_url} alt={client.name} class="client-logo" loading="lazy" height="36" />
            </a>
          {:else}
            <div class="client-logo-link" title={client.name}>
              <img src={client.logo_url} alt={client.name} class="client-logo" loading="lazy" height="36" />
            </div>
          {/if}
        {/each}
      </div>
    </div>
  </div>
</section>
{/if}

<!-- ══ PHILOSOPHY ════════════════════════════════════════════════ -->
<section id="what-we-do" class="philosophy" aria-labelledby="phil-h2">
  <div class="inner phil-inner">
    <div class="phil-grid">
      <div class="phil-left">
        <span class="section-eyebrow">// Baseline Ideation</span>
        <h2 id="phil-h2" class="phil-h2">
          From First Principles<br />to Infinite Solutions.
        </h2>
      </div>
      <div class="phil-right">
        <p>
          We don't stack workarounds on fragile foundations. Real transformation starts at the
          base layer — the absolute technical foundation — and compounds upward.
        </p>
        <p>
          By decomposing complex enterprise problems into simple, modular primitives, we eliminate
          technical debt before it accumulates. Every system we build is designed to remain
          maintainable, scalable, and secure five years from deployment.
        </p>
        <a href="/capabilities" class="phil-link">
          Explore our methodology
          <span aria-hidden="true">→</span>
        </a>
      </div>
    </div>
  </div>
</section>

<!-- ══ FOUR CORE VERTICALS ════════════════════════════════════════ -->
<section class="pillars" aria-labelledby="pillars-h2">
  <div class="inner">
    <div class="section-header">
      <div>
        <span class="section-eyebrow">// Scope of Operations</span>
        <h2 id="pillars-h2" class="section-h2">Four core verticals.<br />One integrated partner.</h2>
      </div>
      <a href="/capabilities" class="section-more">
        All 18 services <span aria-hidden="true">→</span>
      </a>
    </div>
    <div class="pillars-grid">
      {#each pillars as pillar}
        <a href={pillar.href} class="pillar-card" style="--pillar-accent:{pillar.accent}">
          <span class="pillar-icon material-symbols-outlined" aria-hidden="true">{pillar.icon}</span>
          <h3 class="pillar-title">{pillar.title}</h3>
          <p class="pillar-body">{pillar.body}</p>
          <span class="pillar-arrow" aria-hidden="true">→</span>
        </a>
      {/each}
    </div>
  </div>
</section>

<!-- ══ WORK PREVIEW ═══════════════════════════════════════════════ -->
{#if data.work.length > 0}
<section class="content-section" aria-labelledby="work-h2">
  <div class="inner">
    <div class="section-header">
      <div>
        <p class="section-eyebrow">
          <span class="material-symbols-outlined" aria-hidden="true">workspace_premium</span>
          Built Legacies
        </p>
        <h2 id="work-h2" class="section-h2">Proven engineering in production.</h2>
      </div>
      <a href="/work" class="section-more">
        All {data.work.length}+ projects <span aria-hidden="true">→</span>
      </a>
    </div>
    <div class="cards-grid cards-grid--work">
      {#each data.work.slice(0, 6) as post}
        <GlassCard accentColor="blue">
          <a href="/work/{post.slug}" class="card-link work-card">
            {#if post.featured_image_url}
              <div class="card-thumb">
                <img src={post.featured_image_url} alt={post.title} loading="lazy" />
                <div class="card-thumb-overlay"></div>
              </div>
            {:else}
              <span class="material-symbols-outlined card-type-icon" aria-hidden="true">folder_open</span>
            {/if}
            <div class="card-body">
              {#if post.category}<span class="card-tag">{post.category}</span>{/if}
              <h3>{post.title}</h3>
              <p>{post.summary}</p>
            </div>
            <span class="card-cta">View case study <span aria-hidden="true">→</span></span>
          </a>
        </GlassCard>
      {/each}
    </div>
  </div>
</section>
{/if}

<!-- ══ INSIGHTS PREVIEW ═══════════════════════════════════════════ -->
{#if data.insights.length > 0}
<section class="content-section content-section--alt" aria-labelledby="ins-h2">
  <div class="inner">
    <div class="section-header">
      <div>
        <p class="section-eyebrow">
          <span class="material-symbols-outlined" aria-hidden="true">terminal</span>
          Technical Insights
        </p>
        <h2 id="ins-h2" class="section-h2">Deep dives from the engine room.</h2>
      </div>
      <a href="/insights" class="section-more">
        All insights <span aria-hidden="true">→</span>
      </a>
    </div>
    <div class="cards-grid cards-grid--insights">
      {#each data.insights as post}
        <GlassCard accentColor="green">
          <a href="/insights/{post.slug}" class="card-link insight-card">
            <div class="card-body">
              {#if post.category}<span class="card-tag card-tag--green">{post.category}</span>{/if}
              <span class="material-symbols-outlined card-type-icon" aria-hidden="true">article</span>
              <h3>{post.title}</h3>
              <p>{post.summary}</p>
              {#if post.author}
                <span class="card-author">
                  <span class="material-symbols-outlined" aria-hidden="true">person</span>
                  {post.author.name}
                </span>
              {/if}
            </div>
            <span class="card-cta card-cta--green">Read insight <span aria-hidden="true">→</span></span>
          </a>
        </GlassCard>
      {/each}
    </div>
  </div>
</section>
{/if}

<!-- ══ CONVERSION INTAKE ══════════════════════════════════════════ -->
<section class="intake" aria-labelledby="intake-h2">
  <div class="intake-bg" aria-hidden="true"></div>
  <div class="inner">
    <div class="intake-grid">
      <div class="intake-left">
        <span class="section-eyebrow">// Project Initialisation</span>
        <h2 id="intake-h2" class="intake-h2">
          Let's Build<br />Something That<br />Lasts.
        </h2>
        <p class="intake-sub">
          Got a complex problem, a legacy system to replace, or a product to launch?
          Tell us about it. We respond within 24 hours.
        </p>
        <div class="intake-proof">
          <div class="proof-item">
            <span class="proof-num">22+</span>
            <span class="proof-label">Projects delivered</span>
          </div>
          <div class="proof-item">
            <span class="proof-num">100%</span>
            <span class="proof-label">Client retention</span>
          </div>
          <div class="proof-item">
            <span class="proof-num">24h</span>
            <span class="proof-label">Response time</span>
          </div>
        </div>
      </div>
      <div class="intake-form-wrap">
        <form class="intake-form" method="POST" action="/contact" novalidate>
          <div class="form-row">
            <div class="form-field">
              <label for="contact-name">Your name</label>
              <input id="contact-name" type="text" name="name" placeholder="Jane Smith" autocomplete="name" required />
            </div>
            <div class="form-field">
              <label for="contact-email">Work email</label>
              <input id="contact-email" type="email" name="email" placeholder="jane@company.com" autocomplete="email" required />
            </div>
          </div>
          <div class="form-field">
            <label for="contact-service">What do you need?</label>
            <select id="contact-service" name="service">
              <option value="">Select a service area</option>
              <option>Software Development</option>
              <option>Cybersecurity & Compliance</option>
              <option>IT Infrastructure & Hardware</option>
              <option>Digital Marketing & Branding</option>
              <option>Smart Systems & Renewable Energy</option>
              <option>IT Strategy & Consulting</option>
              <option>Other</option>
            </select>
          </div>
          <div class="form-field">
            <label for="contact-message">Describe the problem</label>
            <textarea id="contact-message" name="message" rows="4"
              placeholder="What challenge are you solving? What systems are involved? What does success look like?"
              required></textarea>
          </div>
          <button type="submit" class="btn btn-primary intake-submit">
            Send Message <span aria-hidden="true">→</span>
          </button>
        </form>
      </div>
    </div>
  </div>
</section>

<style>
  /* ── Shared tokens ─────────────────────────────────── */
  .section-eyebrow {
    display: inline-flex; align-items: center; gap: 0.4rem;
    font-family: var(--font-heading); font-size: 0.68rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent-blue-hi);
    margin-bottom: 0.75rem;
  }
  .section-header {
    display: flex; justify-content: space-between; align-items: flex-end;
    margin-bottom: 3rem; gap: 2rem; flex-wrap: wrap;
  }
  .section-h2 {
    font-size: clamp(1.75rem, 3vw, 2.5rem); font-weight: 800;
    color: #fff; margin: 0; line-height: 1.1; letter-spacing: -0.03em;
  }
  .section-more {
    display: inline-flex; align-items: center; gap: 0.4rem;
    font-family: var(--font-heading); font-size: 0.8rem; font-weight: 700;
    color: var(--accent-blue-hi); text-decoration: none; white-space: nowrap;
    transition: gap 0.2s ease;
  }
  .section-more:hover { gap: 0.7rem; text-decoration: none; }

  /* ── HERO ──────────────────────────────────────────── */
  .hero {
    min-height: 100svh;
    display: flex; align-items: center;
    padding: 5rem 0 3rem;
    overflow: hidden;
    position: relative;
  }

  /* Subtle radial glow behind brain */
  .hero::after {
    content: '';
    position: absolute; inset: 0; pointer-events: none;
    background: radial-gradient(ellipse 60% 70% at 75% 50%, rgba(0,82,255,0.07) 0%, transparent 65%);
  }

  .hero-inner {
    display: grid;
    grid-template-columns: 1fr 1fr;
    gap: 4rem;
    align-items: center;
  }

  .hero-left {
    display: flex; flex-direction: column; gap: 0;
    opacity: 0; /* GSAP animates this in */
  }

  .hero-eyebrow {
    display: inline-flex; align-items: center; gap: 0.5rem;
    font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent-blue-hi);
    margin-bottom: 1.5rem;
  }

  .eyebrow-dot {
    width: 6px; height: 6px; border-radius: 50%; background: var(--accent-blue-hi);
    animation: pulse-dot 2s ease-in-out infinite;
  }

  @keyframes pulse-dot { 0%,100%{opacity:1;transform:scale(1)} 50%{opacity:0.4;transform:scale(0.7)} }

  .hero-h1 {
    font-size: clamp(2.8rem, 5.5vw, 5.5rem); font-weight: 900;
    letter-spacing: -0.055em; line-height: 0.95; color: #fff;
    margin: 0 0 1.75rem;
  }

  .hero-h1-accent {
    background: linear-gradient(135deg, var(--accent-blue) 0%, var(--accent-blue-hi) 100%);
    -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
    display: block;
  }

  .hero-sub {
    font-family: var(--font-body); font-size: clamp(0.95rem, 1.5vw, 1.1rem);
    line-height: 1.8; color: var(--text-secondary);
    max-width: 480px; margin: 0 0 2.5rem;
  }

  .hero-ctas {
    display: flex; gap: 1rem; flex-wrap: wrap; margin-bottom: 1.75rem; opacity: 0;
  }

  .hero-cta-primary {
    background: var(--accent-blue); color: #fff;
    padding: 0.85rem 2rem; font-size: 0.85rem;
    letter-spacing: 0.06em; font-weight: 700;
    position: relative; overflow: hidden;
    transition: all 0.3s var(--ease-out-expo);
  }

  .hero-cta-primary::after {
    content: ''; position: absolute; inset: 0;
    background: linear-gradient(135deg, rgba(255,255,255,0.1) 0%, transparent 60%);
    opacity: 0; transition: opacity 0.3s;
  }

  .hero-cta-primary:hover { transform: translateY(-2px); box-shadow: 0 0 28px rgba(0,82,255,0.5); }
  .hero-cta-primary:hover::after { opacity: 1; }

  .hero-cta-secondary {
    border: 1px solid rgba(255,255,255,0.18); color: rgba(255,255,255,0.8);
    padding: 0.85rem 1.75rem; font-size: 0.85rem; letter-spacing: 0.06em;
    transition: all 0.3s var(--ease-out-expo);
  }

  .hero-cta-secondary:hover { border-color: #fff; color: #fff; transform: translateY(-2px); }

  .hero-badge {
    display: inline-flex; align-items: center; gap: 0.5rem;
    font-family: var(--font-body); font-size: 0.78rem; font-weight: 500;
    color: var(--accent-green); border: 1px solid rgba(0,230,118,0.2);
    border-radius: 100px; padding: 0.3rem 0.85rem;
    text-decoration: none; margin-bottom: 1.5rem;
    transition: var(--transition-std); opacity: 0;
  }

  .hero-badge:hover { background: rgba(0,230,118,0.06); text-decoration: none; }

  .live-dot {
    width: 6px; height: 6px; border-radius: 50%; background: var(--accent-green);
    animation: pulse-dot 2s ease-in-out infinite; flex-shrink: 0;
  }

  .hero-trust {
    display: flex; align-items: center; gap: 0.75rem; flex-wrap: wrap;
    font-family: var(--font-body); font-size: 0.72rem;
    color: rgba(255,255,255,0.35);
  }

  .trust-item { display: flex; align-items: center; gap: 0.3rem; }
  .trust-icon { color: var(--accent-green); font-size: 0.75rem; }
  .trust-sep  { color: rgba(255,255,255,0.2); }

  .hero-right {
    height: 580px; position: relative; opacity: 0;
  }

  /* ── CLIENT LOGOS ──────────────────────────────────── */
  .clients-bar {
    padding: 2rem 0; overflow: hidden;
    border-top: 1px solid rgba(255,255,255,0.05);
    border-bottom: 1px solid rgba(255,255,255,0.05);
    background: rgba(255,255,255,0.015);
  }

  .clients-inner { display: flex; align-items: center; gap: 2rem; }

  .clients-label {
    font-family: var(--font-heading); font-size: 0.62rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.12em; color: rgba(255,255,255,0.18);
    white-space: nowrap; flex-shrink: 0;
  }

  .clients-track-wrap {
    overflow: hidden; flex: 1;
    mask-image: linear-gradient(to right, transparent, black 6%, black 94%, transparent);
    -webkit-mask-image: linear-gradient(to right, transparent, black 6%, black 94%, transparent);
  }

  .clients-track {
    display: flex; align-items: center; gap: 3.5rem; width: max-content;
  }

  .client-logo-link {
    display: flex; align-items: center; flex-shrink: 0;
    opacity: 0.4; transition: opacity 0.25s; text-decoration: none;
  }

  .client-logo-link:hover { opacity: 0.8; }

  .client-logo {
    height: 32px; width: auto; max-width: 120px; object-fit: contain;
    filter: brightness(0) invert(1);
  }

  /* ── PHILOSOPHY ────────────────────────────────────── */
  .philosophy {
    padding: 8rem 0;
    background: #000;
    border-top: 1px solid rgba(255,255,255,0.06);
  }

  .phil-inner { max-width: 1100px; }

  .phil-grid {
    display: grid; grid-template-columns: 5fr 7fr;
    gap: 6rem; align-items: center;
  }

  .phil-h2 {
    font-size: clamp(2rem, 4vw, 3.25rem); font-weight: 800;
    letter-spacing: -0.04em; color: #fff; margin: 0;
  }

  .phil-right {
    border-left: 1px solid rgba(255,255,255,0.08);
    padding-left: 4rem; display: flex; flex-direction: column; gap: 1.5rem;
  }

  .phil-right p {
    font-family: var(--font-body); font-size: 1.05rem; line-height: 1.8;
    color: var(--text-secondary); margin: 0;
  }

  .phil-link {
    display: inline-flex; align-items: center; gap: 0.4rem;
    font-family: var(--font-heading); font-size: 0.875rem; font-weight: 700;
    color: var(--accent-blue-hi); text-decoration: none;
    transition: gap 0.2s ease;
  }

  .phil-link:hover { gap: 0.7rem; text-decoration: none; }

  /* ── PILLARS ───────────────────────────────────────── */
  .pillars { padding: 8rem 0; background: #030308; }

  .pillars-grid {
    display: grid; grid-template-columns: repeat(4, 1fr); gap: 1.25rem;
  }

  .pillar-card {
    display: flex; flex-direction: column; gap: 1rem;
    padding: 2rem; text-decoration: none; color: inherit;
    background: rgba(255,255,255,0.03);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 16px;
    transition: all 0.35s var(--ease-out-expo);
    position: relative; overflow: hidden;
    opacity: 0; /* GSAP animates in */
  }

  .pillar-card::before {
    content: ''; position: absolute; inset: 0; border-radius: 16px;
    background: radial-gradient(ellipse 80% 60% at 50% 0%, color-mix(in srgb, var(--pillar-accent) 8%, transparent) 0%, transparent 100%);
    opacity: 0; transition: opacity 0.35s;
  }

  .pillar-card:hover {
    border-color: color-mix(in srgb, var(--pillar-accent) 40%, transparent);
    transform: translateY(-6px);
    box-shadow: 0 16px 40px -12px color-mix(in srgb, var(--pillar-accent) 25%, transparent);
    text-decoration: none; color: inherit;
  }

  .pillar-card:hover::before { opacity: 1; }

  .pillar-icon {
    font-size: 2rem; color: var(--pillar-accent); position: relative; z-index: 1;
    transition: transform 0.3s var(--ease-out-expo);
  }

  .pillar-card:hover .pillar-icon { transform: scale(1.15) rotate(-4deg); }

  .pillar-title {
    font-family: var(--font-heading); font-size: 1rem; font-weight: 700;
    color: #fff; letter-spacing: -0.02em; margin: 0; position: relative; z-index: 1;
  }

  .pillar-body {
    font-family: var(--font-body); font-size: 0.85rem; line-height: 1.7;
    color: var(--text-secondary); margin: 0; flex: 1; position: relative; z-index: 1;
  }

  .pillar-arrow {
    font-size: 1.1rem; color: var(--pillar-accent); margin-top: auto;
    opacity: 0; transform: translateX(-6px);
    transition: all 0.25s ease; position: relative; z-index: 1;
  }

  .pillar-card:hover .pillar-arrow { opacity: 1; transform: translateX(0); }

  /* ── CONTENT SECTIONS ──────────────────────────────── */
  .content-section { padding: 7rem 0; background: #000; }
  .content-section--alt { background: #030308; }

  .cards-grid {
    display: grid; grid-template-columns: repeat(auto-fill, minmax(300px, 1fr)); gap: 1.5rem;
  }

  .card-link {
    display: flex; flex-direction: column; gap: 0.75rem;
    padding: 1.5rem; text-decoration: none; color: inherit; height: 100%;
    position: relative;
  }

  .card-thumb {
    position: relative; border-radius: 10px; overflow: hidden;
    aspect-ratio: 16/9; margin: -1.5rem -1.5rem 1rem;
  }

  .card-thumb img { width: 100%; height: 100%; object-fit: cover; display: block; transition: transform 0.4s ease; }
  .card-link:hover .card-thumb img { transform: scale(1.04); }
  .card-thumb-overlay {
    position: absolute; inset: 0;
    background: linear-gradient(to bottom, transparent 40%, rgba(0,0,0,0.6) 100%);
  }

  .card-type-icon { font-size: 22px; color: var(--accent-blue-hi); }
  .card-body { display: flex; flex-direction: column; gap: 0.5rem; flex: 1; }
  .card-tag {
    display: inline-block; font-family: var(--font-heading); font-size: 0.62rem;
    font-weight: 700; text-transform: uppercase; letter-spacing: 0.08em;
    color: var(--accent-blue-hi);
    background: rgba(0,194,255,0.08); border: 1px solid rgba(0,194,255,0.2);
    border-radius: 4px; padding: 2px 7px; align-self: flex-start;
  }
  .card-tag--green { color: var(--accent-green); background: rgba(0,230,118,0.08); border-color: rgba(0,230,118,0.2); }
  .card-body h3 {
    font-family: var(--font-heading); font-size: 1rem; font-weight: 700;
    letter-spacing: -0.02em; color: #fff; margin: 0;
  }
  .card-body p {
    font-family: var(--font-body); font-size: 0.83rem; line-height: 1.65;
    color: var(--text-secondary); margin: 0; flex: 1;
    display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
  }
  .card-author {
    display: flex; align-items: center; gap: 0.3rem;
    font-size: 0.72rem; color: var(--text-muted); margin-top: auto; padding-top: 0.5rem;
    border-top: 1px solid rgba(255,255,255,0.06);
  }
  .card-cta {
    font-family: var(--font-heading); font-size: 0.72rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.08em; color: var(--accent-blue-hi);
    margin-top: auto; padding-top: 0.75rem;
    border-top: 1px solid rgba(255,255,255,0.07);
    opacity: 0; transform: translateY(4px);
    transition: opacity 0.25s, transform 0.25s;
  }
  .card-cta--green { color: var(--accent-green); }
  .card-link:hover .card-cta { opacity: 1; transform: translateY(0); }

  /* ── INTAKE ────────────────────────────────────────── */
  .intake {
    padding: 8rem 0; background: #000;
    position: relative; overflow: hidden;
    border-top: 1px solid rgba(255,255,255,0.06);
  }

  .intake-bg {
    position: absolute; inset: 0; pointer-events: none;
    background:
      radial-gradient(ellipse 80% 50% at 20% 80%, rgba(0,82,255,0.06) 0%, transparent 60%),
      radial-gradient(ellipse 60% 60% at 80% 20%, rgba(0,194,255,0.04) 0%, transparent 60%);
  }

  .intake-grid {
    display: grid; grid-template-columns: 5fr 7fr; gap: 6rem; align-items: start;
    position: relative; z-index: 1;
  }

  .intake-h2 {
    font-size: clamp(2.25rem, 4.5vw, 3.75rem); font-weight: 900;
    letter-spacing: -0.05em; color: #fff; line-height: 0.95; margin: 0.75rem 0 1.5rem;
  }

  .intake-sub {
    font-family: var(--font-body); font-size: 1rem; line-height: 1.8;
    color: var(--text-secondary); margin: 0 0 2.5rem;
  }

  .intake-proof { display: flex; gap: 2rem; flex-wrap: wrap; }
  .proof-item { display: flex; flex-direction: column; gap: 2px; }
  .proof-num {
    font-family: var(--font-heading); font-size: 2rem; font-weight: 900;
    letter-spacing: -0.04em; color: #fff;
    background: linear-gradient(135deg, var(--accent-blue), var(--accent-blue-hi));
    -webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
  }
  .proof-label {
    font-family: var(--font-body); font-size: 0.72rem; color: rgba(255,255,255,0.4); white-space: nowrap;
  }

  .intake-form-wrap {
    background: rgba(255,255,255,0.025);
    border: 1px solid rgba(255,255,255,0.08);
    border-radius: 20px; padding: 2.5rem;
    backdrop-filter: blur(8px);
  }

  .intake-form { display: flex; flex-direction: column; gap: 1.25rem; }
  .form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 1.25rem; }
  .form-field { display: flex; flex-direction: column; gap: 0.4rem; }
  .form-field label {
    font-family: var(--font-heading); font-size: 0.62rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.1em; color: rgba(255,255,255,0.45);
  }
  .intake-form input,
  .intake-form select,
  .intake-form textarea {
    background: rgba(255,255,255,0.04);
    border: 1px solid rgba(255,255,255,0.1);
    border-radius: 10px; color: #fff;
    font-family: var(--font-body); font-size: 0.9rem;
    padding: 0.8rem 1rem; transition: border-color 0.2s, box-shadow 0.2s;
    width: 100%; box-sizing: border-box;
  }
  .intake-form input:focus,
  .intake-form select:focus,
  .intake-form textarea:focus {
    border-color: var(--accent-blue); outline: none;
    box-shadow: 0 0 0 3px rgba(0,82,255,0.15);
  }
  .intake-submit {
    background: var(--accent-blue); color: #fff;
    padding: 0.9rem 2.25rem; border-radius: 10px;
    font-size: 0.85rem; letter-spacing: 0.06em;
    align-self: flex-start; transition: all 0.3s var(--ease-out-expo);
    border: none; cursor: pointer;
  }
  .intake-submit:hover { background: #003dd4; transform: translateY(-2px); box-shadow: 0 0 24px rgba(0,82,255,0.45); }

  /* ── RESPONSIVE ────────────────────────────────────── */
  @media (max-width: 1100px) {
    .hero-inner      { grid-template-columns: 1fr; gap: 2rem; }
    .hero-right      { height: 380px; }
    .pillars-grid    { grid-template-columns: 1fr 1fr; }
    .phil-grid       { grid-template-columns: 1fr; gap: 3rem; }
    .phil-right      { border-left: none; padding-left: 0; border-top: 1px solid rgba(255,255,255,0.08); padding-top: 2rem; }
    .intake-grid     { grid-template-columns: 1fr; gap: 3rem; }
  }

  @media (max-width: 640px) {
    .hero { padding: 3rem 0 2rem; }
    .hero-h1 { font-size: 2.5rem; }
    .hero-right { height: 300px; }
    .pillars-grid { grid-template-columns: 1fr; }
    .hero-ctas    { flex-direction: column; align-items: flex-start; }
    .form-row     { grid-template-columns: 1fr; }
    .intake-form-wrap { padding: 1.5rem; }
    .intake-proof { gap: 1.25rem; }
  }
</style>
