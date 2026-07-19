<script lang="ts">
	import type { PageData } from './$types';
	import FloatingHeader from '$lib/components/FloatingHeader.svelte';
	import GlassCard from '$lib/components/GlassCard.svelte';

	let { data }: { data: PageData } = $props();

	const navLinks = [
		{ href: '/capabilities', label: 'Capabilities' },
		{ href: '/work',         label: 'Work'         },
		{ href: '/insights',     label: 'Insights'     },
		{ href: '/careers',      label: 'Careers'      },
		{ href: '/team',         label: 'Team'         },
	];

	const pillars = [
		{
			icon: 'layers',
			title: 'Digital Architecture',
			body: 'High-speed web applications, custom enterprise platforms, and secure API gateways built on lightweight edge runtimes.'
		},
		{
			icon: 'insights',
			title: 'Strategic IT Consultancy',
			body: 'First-principles technology auditing, infrastructure optimisation, and rigorous corporate cybersecurity compliance frameworks.'
		},
		{
			icon: 'router',
			title: 'Hardware & Networks',
			body: 'Fault-tolerant network mapping, managed routing, data centre installations, and secure physical access control terminals.'
		},
		{
			icon: 'wb_sunny',
			title: 'Renewable Energy',
			body: 'Commercial solar PV arrays, industrial smart battery matrices, and telemetry-driven energy efficiency automation loops.'
		},
	];
</script>

<svelte:head>
	<title>eZeroAndOne — We Deconstruct Complexity. You Create the Possibilities.</title>
	<meta name="description" content="eZeroAndOne builds Cloudflare-native products from first principles. Full-stack architecture, API integration, and edge infrastructure." />
</svelte:head>

<!-- ══ NAVIGATION ════════════════════════════════════════════ -->
<FloatingHeader>
	<nav class="nav-inner">
		<a href="/" class="brand" aria-label="eZeroAndOne home">
			e<span class="brand-accent">0</span>&amp;<span class="brand-accent">1</span>
		</a>
		<ul class="nav-links" role="list">
			{#each navLinks as link}
				<li><a href={link.href}>{link.label}</a></li>
			{/each}
		</ul>
		<div class="nav-actions">
			<a href="/auth/login" class="btn btn-secondary nav-signin">
				<span class="material-symbols-outlined" aria-hidden="true">lock</span>
				Staff Portal
			</a>
			<a href="/capabilities" class="btn btn-primary nav-cta">
				Initialize Project
			</a>
		</div>
	</nav>
</FloatingHeader>

<!-- ══ SECTION 1: HERO ═══════════════════════════════════════ -->
<section class="hero section-constrained" aria-labelledby="hero-h1">
	<div class="inner hero-inner">
		<p class="eyebrow">
			<span class="material-symbols-outlined eyebrow-icon" aria-hidden="true">bolt</span>
			Cloudflare-native · First principles engineering
		</p>
		<h1 id="hero-h1" class="hero-h1">
			We Deconstruct Complexity.<br />
			<span class="hero-h1-accent">You Create Possibilities.</span>
		</h1>
		<p class="hero-sub">
			At the foundation of every groundbreaking digital platform is a stream of electrons, zeros,
			and ones. We break complex business challenges down into simple, modular parts to build
			scalable software ecosystems that stand the test of time.
		</p>
		<div class="hero-ctas">
			<a href="/capabilities" class="btn btn-primary">
				Initialize Project
			</a>
			<a href="#philosophy" class="btn btn-secondary">
				Our Philosophy
			</a>
		</div>
		{#if data.openRoles > 0}
			<a href="/careers" class="live-badge">
				<span class="live-dot" aria-hidden="true"></span>
				{data.openRoles} active {data.openRoles === 1 ? 'operation' : 'operations'} open
				<span class="material-symbols-outlined live-arrow" aria-hidden="true">arrow_forward</span>
			</a>
		{/if}
	</div>
</section>

<!-- ══ SECTION 2: CORE PHILOSOPHY ════════════════════════════ -->
<section id="philosophy" class="philosophy section-constrained" aria-labelledby="phil-h2">
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
					We don't believe in quick fixes or fragile code layers. True digital transformation
					requires going back to the basics — the absolute foundation.
				</p>
				<p>
					By treating complex enterprise problems as a modular collection of simple blocks, we
					eliminate technical debt before it even begins. We build flexible architectures that
					unlock the spectrum of infinite possibilities lying between 0 and 1.
				</p>
				<a href="/capabilities" class="phil-link">
					Explore our methodology
					<span class="material-symbols-outlined" aria-hidden="true">arrow_forward</span>
				</a>
			</div>
		</div>
	</div>
</section>

<!-- ══ SECTION 3: FOUR CORE VERTICALS ═══════════════════════ -->
<section class="pillars section-constrained" aria-labelledby="pillars-h2">
	<div class="inner">
		<div class="section-header">
			<div>
				<span class="section-eyebrow">// Scope of Operations</span>
				<h2 id="pillars-h2" class="section-h2">Structural Core Verticals</h2>
			</div>
			<a href="/capabilities" class="section-more">
				All capabilities
				<span class="material-symbols-outlined" aria-hidden="true">arrow_forward</span>
			</a>
		</div>
		<div class="pillars-grid">
			{#each pillars as pillar}
				<div class="interactive-card pillar-card">
					<span class="material-symbols-outlined pillar-icon" aria-hidden="true">{pillar.icon}</span>
					<h3 class="pillar-title">{pillar.title}</h3>
					<p class="pillar-body">{pillar.body}</p>
				</div>
			{/each}
		</div>
	</div>
</section>

<!-- ══ WORK PREVIEW ═══════════════════════════════════════════ -->
{#if data.work.length > 0}
<section class="content-section section-constrained" aria-labelledby="work-h2">
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
				All projects <span class="material-symbols-outlined" aria-hidden="true">arrow_forward</span>
			</a>
		</div>
		<div class="cards-grid">
			{#each data.work as post}
				<GlassCard accentColor="blue">
					<a href="/work/{post.slug}" class="card-link">
						<span class="material-symbols-outlined card-type-icon" aria-hidden="true">folder_open</span>
						<h3>{post.title}</h3>
						<p>{post.summary}</p>
					</a>
				</GlassCard>
			{/each}
		</div>
	</div>
</section>
{/if}

<!-- ══ INSIGHTS PREVIEW ═══════════════════════════════════════ -->
{#if data.insights.length > 0}
<section class="content-section section-constrained" aria-labelledby="ins-h2">
	<div class="inner">
		<div class="section-header">
			<div>
				<p class="section-eyebrow">
					<span class="material-symbols-outlined" aria-hidden="true">terminal</span>
					Technical Musings
				</p>
				<h2 id="ins-h2" class="section-h2">Deep dives from the engine room.</h2>
			</div>
			<a href="/insights" class="section-more">
				All insights <span class="material-symbols-outlined" aria-hidden="true">arrow_forward</span>
			</a>
		</div>
		<div class="cards-grid">
			{#each data.insights as post}
				<GlassCard accentColor="blue">
					<a href="/insights/{post.slug}" class="card-link">
						<span class="material-symbols-outlined card-type-icon" aria-hidden="true">article</span>
						<h3>{post.title}</h3>
						<p>{post.summary}</p>
						{#if post.author}
							<span class="card-author">
								<span class="material-symbols-outlined" aria-hidden="true">person</span>
								{post.author.name}
							</span>
						{/if}
					</a>
				</GlassCard>
			{/each}
		</div>
	</div>
</section>
{/if}

<!-- ══ SECTION 4: CONVERSION INTAKE ══════════════════════════ -->
<section class="intake section-constrained" aria-labelledby="intake-h2">
	<div class="inner">
		<div class="intake-grid">
			<div class="intake-left">
				<span class="section-eyebrow">// Project Initialisation</span>
				<h2 id="intake-h2" class="intake-h2">
					Let's Build Something<br />That Lasts.
				</h2>
				<p class="intake-sub">
					Ready to turn your complex ideas into clean, executable, long-term code?
					Let's start from the foundation and build your digital legacy together.
				</p>
			</div>
			<div class="intake-form-wrap">
				<form class="intake-form" method="POST" action="/contact">
					<div class="form-row">
						<div class="form-field">
							<label for="contact-name">Name</label>
							<input id="contact-name" type="text" name="name" placeholder="Jane Smith" autocomplete="name" />
						</div>
						<div class="form-field">
							<label for="contact-email">Corporate Email</label>
							<input id="contact-email" type="email" name="email" placeholder="jane@company.com" autocomplete="email" />
						</div>
					</div>
					<div class="form-field">
						<label for="contact-message">The complex problem you need solved</label>
						<textarea id="contact-message" name="message" rows="4" placeholder="Describe your infrastructure challenge, product vision, or technical bottleneck…"></textarea>
					</div>
					<button type="submit" class="btn btn-primary intake-submit">
						Initialize Project
					</button>
				</form>
			</div>
		</div>
	</div>
</section>

<!-- ══ FOOTER ════════════════════════════════════════════════ -->
<footer class="site-footer" aria-label="Site footer">
	<div class="inner footer-grid">

		<!-- Col 1: Identity -->
		<div class="footer-col footer-col--brand">
			<a href="/" class="footer-logo" aria-label="eZeroAndOne">
				e<span>0</span>&amp;<span>1</span>
			</a>
			<p class="footer-bio">Engineered from the baseline.<br />Built for legacy.</p>
			<div class="footer-social">
				<a href="https://github.com" aria-label="GitHub" rel="noopener noreferrer" target="_blank">
					<span class="material-symbols-outlined">code</span>
				</a>
				<a href="https://linkedin.com" aria-label="LinkedIn" rel="noopener noreferrer" target="_blank">
					<span class="material-symbols-outlined">corporate_fare</span>
				</a>
			</div>
		</div>

		<!-- Col 2: Core Pillars -->
		<nav class="footer-col" aria-label="Core pillar links">
			<h3 class="footer-col-title">Core Pillars</h3>
			<ul role="list">
				<li><a href="/capabilities"><span class="material-symbols-outlined" aria-hidden="true">deployed_code</span>Capabilities</a></li>
				<li><a href="/work"><span class="material-symbols-outlined" aria-hidden="true">workspace_premium</span>Case Studies</a></li>
				<li><a href="/insights"><span class="material-symbols-outlined" aria-hidden="true">terminal</span>Technical Insights</a></li>
				<li><a href="/careers"><span class="material-symbols-outlined" aria-hidden="true">radio_button_checked</span>Active Careers</a></li>
			</ul>
		</nav>

		<!-- Col 3: Trust Nodes -->
		<nav class="footer-col" aria-label="Security and trust links">
			<h3 class="footer-col-title">Trust Nodes</h3>
			<ul role="list">
				<li><a href="/team"><span class="material-symbols-outlined" aria-hidden="true">groups</span>The Engineers</a></li>
				<li><a href="/verify"><span class="material-symbols-outlined" aria-hidden="true">qr_code_scanner</span>Identity Verification</a></li>
				<li><a href="/security"><span class="material-symbols-outlined" aria-hidden="true">shield</span>Security Docs</a></li>
				<li><a href="/status" class="footer-status-link">
					<span class="status-dot" aria-hidden="true"></span>Edge Status Operational
				</a></li>
			</ul>
		</nav>

		<!-- Col 4: Access Gateways -->
		<div class="footer-col">
			<h3 class="footer-col-title">Access Gateways</h3>
			<ul role="list">
				<li>
					<a href="/auth/login" class="footer-cta-link">
						<span class="material-symbols-outlined" aria-hidden="true">lock</span>
						Secure Staff Portal
					</a>
				</li>
				<li>
					<a href="mailto:hello@ezeroandone.io" class="footer-cta-link">
						<span class="material-symbols-outlined" aria-hidden="true">mail</span>
						hello@ezeroandone.io
					</a>
				</li>
				<li>
					<a href="/capabilities" class="footer-cta-link footer-cta-link--primary">
						<span class="material-symbols-outlined" aria-hidden="true">rocket_launch</span>
						Initialize a Project
					</a>
				</li>
			</ul>
		</div>

	</div>

	<!-- Bottom strip -->
	<div class="footer-bottom">
		<div class="inner footer-bottom-inner">
			<p>eZeroAndOne &copy; 2026. All rights reserved.</p>
			<p class="footer-bottom-mono">Strict cryptographic access control enforced.</p>
		</div>
	</div>
</footer>

<style>
	/* ── Nav ──────────────────────────────────────────── */
	.nav-inner {
		display: flex; align-items: center; gap: 2rem;
		max-width: 1200px; margin: 0 auto; width: 100%;
	}
	.brand {
		font-family: var(--font-heading); font-size: 1.25rem; font-weight: 800;
		color: #fff; text-decoration: none; letter-spacing: -0.04em; flex-shrink: 0;
	}
	.brand-accent { color: var(--accent-blue-hi); }
	.nav-links {
		display: flex; list-style: none; margin: 0; padding: 0; gap: 1.75rem; flex: 1;
	}
	.nav-links a {
		font-family: var(--font-body); color: rgba(255,255,255,0.5); font-size: 0.9rem;
		font-weight: 500; text-decoration: none; letter-spacing: 0.02em;
		transition: color 0.2s ease;
	}
	.nav-links a:hover, .nav-links a[aria-current="page"] { color: #fff; text-decoration: none; }
	.nav-actions { display: flex; align-items: center; gap: 0.75rem; flex-shrink: 0; }
	.nav-signin { padding: 0.45rem 1rem; font-size: 0.75rem; }
	.nav-cta    { padding: 0.45rem 1.25rem; font-size: 0.75rem; }

	/* ── Shared section tokens ───────────────────────── */
	.section-constrained { padding: 8rem 0; }
	.section-header {
		display: flex; justify-content: space-between; align-items: flex-end;
		margin-bottom: 3rem; gap: 2rem; flex-wrap: wrap;
	}
	.section-eyebrow {
		display: block; font-family: var(--font-heading); font-size: 0.7rem;
		font-weight: 700; text-transform: uppercase; letter-spacing: 0.12em;
		color: var(--accent-blue); margin-bottom: 0.75rem;
	}
	.section-h2 { font-size: clamp(1.5rem, 2.75vw, 2.25rem); font-weight: 800; color: #fff; margin: 0; }
	.section-more {
		display: inline-flex; align-items: center; gap: 0.3rem;
		font-family: var(--font-heading); font-size: 0.8rem; font-weight: 600;
		color: var(--accent-blue-hi); text-decoration: none; white-space: nowrap;
		transition: gap 0.2s ease;
	}
	.section-more:hover { gap: 0.6rem; text-decoration: none; }

	/* ── Hero ─────────────────────────────────────────── */
	.hero { text-align: center; min-height: 100svh; height: auto; overflow: visible; padding: 6rem 2rem; }
	.hero-inner { max-width: 860px; }
	.eyebrow {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em; color: var(--accent-blue-hi);
		margin: 0 0 1.5rem; border: 1px solid rgba(0, 194, 255, 0.25);
		border-radius: 100px; padding: 0.3rem 0.9rem;
	}
	.eyebrow-icon { font-size: 14px; }
	.hero-h1 {
		font-size: clamp(3rem, 8vw, 6.5rem); font-weight: 800; letter-spacing: -0.05em;
		line-height: 1.0; color: #fff; margin: 0 0 1.75rem;
	}
	.hero-h1-accent {
		background: linear-gradient(90deg, var(--accent-blue) 0%, var(--accent-blue-hi) 100%);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
	}
	.hero-sub {
		font-family: var(--font-body); font-size: clamp(1rem, 2vw, 1.2rem); line-height: 1.75;
		color: var(--text-secondary); max-width: 640px; margin: 0 auto 2.5rem;
	}
	.hero-ctas { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; margin-bottom: 2rem; }
	.live-badge {
		display: inline-flex; align-items: center; gap: 0.5rem;
		font-family: var(--font-body); font-size: 0.82rem; font-weight: 500;
		color: var(--accent-green); border: 1px solid rgba(0, 230, 118, 0.25);
		border-radius: 100px; padding: 0.3rem 0.9rem; text-decoration: none;
		transition: var(--transition-std);
	}
	.live-badge:hover { background: rgba(0,230,118,0.06); text-decoration: none; }
	.live-dot {
		width: 7px; height: 7px; border-radius: 50%; background: var(--accent-green);
		animation: pulse 2s ease-in-out infinite;
	}
	@keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.35} }
	.live-arrow { font-size: 14px; }

	/* ── Philosophy ───────────────────────────────────── */
	.philosophy { background: var(--bg-surface); }
	.phil-inner { max-width: 1100px; }
	.phil-grid {
		display: grid; grid-template-columns: 5fr 7fr; gap: 6rem; align-items: center;
		border-bottom: 1px solid rgba(255,255,255,0.05); padding-bottom: 0;
	}
	.phil-h2 {
		font-size: clamp(2rem, 4.5vw, 3.5rem); font-weight: 800;
		letter-spacing: -0.03em; color: #fff; margin: 0;
	}
	.phil-right {
		border-left: 1px solid rgba(255,255,255,0.08); padding-left: 4rem;
		display: flex; flex-direction: column; gap: 1.5rem;
	}
	.phil-right p {
		font-family: var(--font-body); font-size: 1.05rem; line-height: 1.8;
		color: var(--text-secondary); margin: 0;
	}
	.phil-link {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.875rem; font-weight: 700;
		color: var(--accent-blue-hi); text-decoration: none; transition: gap 0.2s ease;
	}
	.phil-link:hover { gap: 0.7rem; text-decoration: none; }

	/* ── Four pillars ─────────────────────────────────── */
	.pillars { background: var(--bg-base); }
	.pillars-grid { display: grid; grid-template-columns: repeat(4, 1fr); gap: 1.5rem; }
	.pillar-card { display: flex; flex-direction: column; gap: 1rem; height: 100%; }
	.pillar-icon { font-size: 28px; color: var(--accent-blue); }
	.pillar-title {
		font-family: var(--font-heading); font-size: 1.05rem; font-weight: 700;
		color: #fff; letter-spacing: -0.02em; margin: 0;
	}
	.pillar-body {
		font-family: var(--font-body); font-size: 0.875rem; line-height: 1.7;
		color: var(--text-secondary); margin: 0; flex: 1;
	}

	/* ── Content card sections ────────────────────────── */
	.content-section { background: var(--bg-surface); }
	.cards-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px, 1fr)); gap: 1.5rem; }
	.card-link {
		display: flex; flex-direction: column; gap: 0.75rem;
		padding: 2rem; text-decoration: none; color: inherit; height: 100%;
	}
	.card-type-icon { font-size: 22px; color: var(--accent-blue-hi); }
	.card-link h3 {
		font-family: var(--font-heading); font-size: 1.05rem; font-weight: 700;
		letter-spacing: -0.02em; color: #fff; margin: 0;
	}
	.card-link p {
		font-family: var(--font-body); font-size: 0.875rem; line-height: 1.65;
		color: var(--text-secondary); margin: 0; flex: 1;
		display: -webkit-box; -webkit-line-clamp: 3; -webkit-box-orient: vertical; overflow: hidden;
	}
	.card-author {
		display: flex; align-items: center; gap: 0.3rem;
		font-size: 0.75rem; color: var(--text-muted); margin-top: auto;
	}

	/* ── Intake (contact) section ─────────────────────── */
	.intake { background: var(--bg-base); }
	.intake-grid {
		display: grid; grid-template-columns: 5fr 7fr; gap: 6rem; align-items: center;
	}
	.intake-h2 {
		font-size: clamp(2rem, 4vw, 3.25rem); font-weight: 800;
		letter-spacing: -0.03em; color: #fff; margin: 0 0 1.25rem;
	}
	.intake-sub {
		font-family: var(--font-body); font-size: 1rem; line-height: 1.8;
		color: var(--text-secondary); margin: 0;
	}
	.intake-form-wrap {
		background: #050505; border: 1px solid rgba(255,255,255,0.05);
		padding: 3rem;
	}
	.intake-form { display: flex; flex-direction: column; gap: 1.5rem; }
	.form-row { display: grid; grid-template-columns: 1fr 1fr; gap: 1.5rem; }
	.form-field { display: flex; flex-direction: column; gap: 0.5rem; }
	.form-field label {
		font-family: var(--font-heading); font-size: 0.65rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.1em; color: #fff;
	}
	.intake-submit { align-self: flex-start; }

	/* ── Footer ───────────────────────────────────────── */
	.site-footer {
		background: var(--bg-base); border-top: 1px solid rgba(255, 255, 255, 0.05);
		padding-top: 6rem;
	}
	.footer-grid {
		display: grid; grid-template-columns: 1.4fr 1fr 1fr 1fr;
		gap: 3rem; padding-bottom: 4rem;
	}
	.footer-logo {
		font-family: var(--font-heading); font-size: 1.5rem; font-weight: 800;
		letter-spacing: -0.04em; color: #fff; text-decoration: none; display: inline-block;
		margin-bottom: 1rem;
	}
	.footer-logo span { color: var(--accent-blue-hi); }
	.footer-bio {
		font-family: var(--font-body); font-size: 0.875rem; line-height: 1.7;
		color: var(--text-secondary); margin: 0 0 1.5rem;
	}
	.footer-social { display: flex; gap: 0.75rem; }
	.footer-social a {
		display: flex; align-items: center; justify-content: center;
		width: 36px; height: 36px; border: 1px solid var(--glass-border);
		color: var(--text-secondary); text-decoration: none; transition: var(--transition-std);
	}
	.footer-social a:hover { border-color: var(--accent-blue); color: var(--accent-blue-hi); }
	.footer-col-title {
		font-family: var(--font-heading); font-size: 0.65rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em; color: var(--text-muted);
		margin: 0 0 1.25rem;
	}
	.footer-col ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.7rem; }
	.footer-col a {
		display: flex; align-items: center; gap: 0.5rem;
		font-family: var(--font-body); font-size: 0.875rem; color: var(--text-secondary);
		text-decoration: none; transition: color 0.2s ease;
	}
	.footer-col a:hover { color: #fff; text-decoration: none; }
	.footer-col a .material-symbols-outlined { font-size: 16px; color: var(--text-muted); }
	.footer-cta-link--primary { color: var(--accent-blue-hi) !important; font-weight: 600; }
	.footer-status-link { cursor: default; }
	.status-dot {
		display: inline-block; width: 7px; height: 7px; border-radius: 50%;
		background: var(--accent-green); animation: pulse 2s ease-in-out infinite;
	}
	.footer-bottom { border-top: 1px solid var(--glass-border); padding: 1.25rem 0; }
	.footer-bottom-inner {
		display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem;
	}
	.footer-bottom-inner p {
		font-family: var(--font-body); font-size: 0.75rem; color: var(--text-muted); margin: 0;
	}
	.footer-bottom-mono {
		font-family: var(--font-heading); font-size: 0.6rem !important;
		text-transform: uppercase; letter-spacing: 0.1em;
	}

	/* ── Responsive ───────────────────────────────────── */
	@media (max-width: 1100px) {
		.pillars-grid { grid-template-columns: 1fr 1fr; }
		.footer-grid  { grid-template-columns: 1fr 1fr; gap: 2rem; }
		.phil-grid    { grid-template-columns: 1fr; gap: 3rem; }
		.phil-right   { border-left: none; padding-left: 0; border-top: 1px solid rgba(255,255,255,0.08); padding-top: 2rem; }
		.intake-grid  { grid-template-columns: 1fr; gap: 3rem; }
	}
	@media (max-width: 640px) {
		.nav-links            { display: none; }
		.nav-actions .nav-signin { display: none; }
		.pillars-grid         { grid-template-columns: 1fr; }
		.footer-grid          { grid-template-columns: 1fr; }
		.footer-bottom-inner  { flex-direction: column; align-items: flex-start; }
		.hero-ctas            { flex-direction: column; align-items: center; }
		.form-row             { grid-template-columns: 1fr; }
		.intake-form-wrap     { padding: 1.75rem; }
	}
</style>
