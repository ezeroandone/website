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

	const capabilities = [
		{
			icon: 'deployed_code',
			title: 'Full-Stack Web Architecture',
			body: 'High-speed user interfaces paired with ultra-low latency server layers built on modern, lightweight runtimes.',
		},
		{
			icon: 'hub',
			title: 'API & Ecosystem Integration',
			body: 'Secure, resilient gateway bridges connecting custom platforms to corporate database registries, travel inventories, and global financial networks.',
		},
		{
			icon: 'lan',
			title: 'Edge & Infrastructure Configuration',
			body: 'Hardened data routing pipelines, cryptographic access policies, and optimized global caching to guarantee maximum uptime.',
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
				<span class="icon" aria-hidden="true">lock</span>
				Staff Portal
			</a>
			<a href="/capabilities" class="btn btn-primary nav-cta">
				Initialize Project
			</a>
		</div>
	</nav>
</FloatingHeader>

<!-- ══ HERO — 100vh ══════════════════════════════════════════ -->
<section class="hero section-constrained" aria-labelledby="hero-h1">
	<div class="inner hero-inner">
		<p class="eyebrow">
			<span class="icon eyebrow-icon" aria-hidden="true">bolt</span>
			Cloudflare-native · First principles engineering
		</p>

		<h1 id="hero-h1" class="hero-h1">
			We Deconstruct Complexity.<br />
			<span class="hero-h1-accent">You Create the Possibilities.</span>
		</h1>

		<p class="hero-sub">
			At the foundation of every groundbreaking digital platform is a stream of electrons, zeros,
			and ones. We break complex business challenges down into simple, modular parts to build
			scalable software ecosystems that stand the test of time.
		</p>

		<div class="hero-ctas">
			<a href="/capabilities" class="btn btn-primary hero-cta-primary">
				<span class="icon" aria-hidden="true">rocket_launch</span>
				Initialize Project
			</a>
			<a href="/capabilities" class="btn btn-secondary hero-cta-secondary">
				<span class="icon" aria-hidden="true">menu_book</span>
				Our Capabilities
			</a>
		</div>

		{#if data.openRoles > 0}
			<a href="/careers" class="live-badge">
				<span class="live-dot" aria-hidden="true"></span>
				{data.openRoles} active {data.openRoles === 1 ? 'operation' : 'operations'} open
				<span class="icon live-arrow" aria-hidden="true">arrow_forward</span>
			</a>
		{/if}
	</div>
</section>

<!-- ══ PHILOSOPHY — 100vh ════════════════════════════════════ -->
<section class="philosophy section-constrained" aria-labelledby="phil-h2">
	<div class="inner phil-inner">
		<div class="phil-label">
			<span class="icon" aria-hidden="true">psychology</span>
			Core Philosophy
		</div>
		<h2 id="phil-h2" class="phil-h2">
			From First Principles<br />to Infinite Solutions.
		</h2>
		<div class="phil-columns">
			<p>
				We don't believe in quick fixes or fragile code. True digital transformation requires
				going back to the basics — the absolute foundation.
			</p>
			<p>
				By treating complex software problems as a collection of simple, accessible blocks, we
				eliminate technical debt before it starts. We build flexible architectures that unlock
				the spectrum of infinite possibilities lying between 0 and 1.
			</p>
		</div>
		<a href="/capabilities" class="phil-link">
			Explore our methodology
			<span class="icon" aria-hidden="true">arrow_forward</span>
		</a>
	</div>
</section>

<!-- ══ CAPABILITIES PREVIEW — 100vh ═════════════════════════ -->
<section class="cap-section section-constrained" aria-labelledby="cap-h2">
	<div class="inner">
		<div class="section-header">
			<div>
				<p class="section-eyebrow">
					<span class="icon" aria-hidden="true">deployed_code</span>
					Architectural Capabilities
				</p>
				<h2 id="cap-h2" class="section-h2">
					Modular solutions designed for<br />absolute performance at scale.
				</h2>
			</div>
			<a href="/capabilities" class="section-more">
				All capabilities
				<span class="icon" aria-hidden="true">arrow_forward</span>
			</a>
		</div>

		<div class="cap-grid">
			{#each capabilities as cap, i}
				<GlassCard accentColor="blue">
					<div class="cap-card">
						<span class="icon cap-icon" aria-hidden="true">{cap.icon}</span>
						<h3 class="cap-title">{cap.title}</h3>
						<p class="cap-body">{cap.body}</p>
						<a href="/capabilities" class="cap-more">
							Learn more
							<span class="icon" aria-hidden="true">chevron_right</span>
						</a>
					</div>
				</GlassCard>
			{/each}
		</div>
	</div>
</section>

<!-- ══ WORK PREVIEW — 100vh ══════════════════════════════════ -->
{#if data.work.length > 0}
<section class="content-section section-constrained" aria-labelledby="work-h2">
	<div class="inner">
		<div class="section-header">
			<div>
				<p class="section-eyebrow">
					<span class="icon" aria-hidden="true">workspace_premium</span>
					Built Legacies
				</p>
				<h2 id="work-h2" class="section-h2">Proven engineering in production.</h2>
			</div>
			<a href="/work" class="section-more">
				All projects <span class="icon" aria-hidden="true">arrow_forward</span>
			</a>
		</div>
		<div class="cards-grid">
			{#each data.work as post}
				<GlassCard accentColor="green">
					<a href="/work/{post.slug}" class="card-link">
						<span class="icon card-type-icon" aria-hidden="true">folder_open</span>
						<h3>{post.title}</h3>
						<p>{post.summary}</p>
					</a>
				</GlassCard>
			{/each}
		</div>
	</div>
</section>
{/if}

<!-- ══ INSIGHTS PREVIEW — 100vh ═════════════════════════════ -->
{#if data.insights.length > 0}
<section class="content-section section-constrained" aria-labelledby="ins-h2">
	<div class="inner">
		<div class="section-header">
			<div>
				<p class="section-eyebrow">
					<span class="icon" aria-hidden="true">terminal</span>
					Technical Musings
				</p>
				<h2 id="ins-h2" class="section-h2">Deep dives from the engine room.</h2>
			</div>
			<a href="/insights" class="section-more">
				All insights <span class="icon" aria-hidden="true">arrow_forward</span>
			</a>
		</div>
		<div class="cards-grid">
			{#each data.insights as post}
				<GlassCard accentColor="blue">
					<a href="/insights/{post.slug}" class="card-link">
						<span class="icon card-type-icon" aria-hidden="true">article</span>
						<h3>{post.title}</h3>
						<p>{post.summary}</p>
						{#if post.author}
							<span class="card-author">
								<span class="icon" aria-hidden="true">person</span>
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

<!-- ══ CAREERS CTA — 100vh ════════════════════════════════════ -->
<section class="cta-section section-constrained" aria-labelledby="cta-h2">
	<div class="inner cta-inner">
		<div class="cta-label">
			<span class="icon" aria-hidden="true">radio_button_checked</span>
			Active Operations
		</div>
		<h2 id="cta-h2" class="cta-h2">
			Join the core team<br />building lasting systems.
		</h2>
		<p class="cta-body">
			We're building at the edge of the network. If you're obsessed with performance,
			correctness, and shipping software engineered to stand the test of time — we want to talk.
		</p>
		<a href="/careers" class="btn btn-primary cta-btn">
			<span class="icon" aria-hidden="true">open_in_new</span>
			{#if data.openRoles > 0}
				View {data.openRoles} open {data.openRoles === 1 ? 'role' : 'roles'}
			{:else}
				See open roles
			{/if}
		</a>
	</div>
</section>

<!-- ══ FOOTER — 4-column ══════════════════════════════════════ -->
<footer class="site-footer">
	<div class="inner footer-grid">

		<!-- Col 1: Identity -->
		<div class="footer-col footer-col--brand">
			<a href="/" class="footer-logo" aria-label="eZeroAndOne">
				e<span>0</span>&amp;<span>1</span>
			</a>
			<p class="footer-bio">Engineered from the baseline.<br />Built for legacy.</p>
			<div class="footer-social">
				<a href="https://github.com" aria-label="GitHub" rel="noopener noreferrer" target="_blank">
					<span class="icon">code</span>
				</a>
				<a href="https://linkedin.com" aria-label="LinkedIn" rel="noopener noreferrer" target="_blank">
					<span class="icon">corporate_fare</span>
				</a>
			</div>
		</div>

		<!-- Col 2: Platforms -->
		<nav class="footer-col" aria-label="Platform links">
			<h3 class="footer-col-title">Platforms</h3>
			<ul role="list">
				<li><a href="/capabilities"><span class="icon" aria-hidden="true">deployed_code</span>Capabilities</a></li>
				<li><a href="/work"><span class="icon" aria-hidden="true">workspace_premium</span>Work</a></li>
				<li><a href="/insights"><span class="icon" aria-hidden="true">terminal</span>Insights</a></li>
				<li><a href="/careers"><span class="icon" aria-hidden="true">radio_button_checked</span>Careers</a></li>
				<li><a href="/team"><span class="icon" aria-hidden="true">groups</span>Team</a></li>
			</ul>
		</nav>

		<!-- Col 3: Security -->
		<nav class="footer-col" aria-label="Security and trust links">
			<h3 class="footer-col-title">Security &amp; Trust</h3>
			<ul role="list">
				<li><a href="/verify"><span class="icon" aria-hidden="true">qr_code_scanner</span>Verification Portal</a></li>
				<li><a href="/status"><span class="icon" aria-hidden="true">monitor_heart</span>Status Engine</a></li>
				<li><a href="/security"><span class="icon" aria-hidden="true">shield</span>Documentation Registry</a></li>
				<li><a href="/privacy"><span class="icon" aria-hidden="true">policy</span>Privacy Policy</a></li>
				<li><a href="/terms"><span class="icon" aria-hidden="true">gavel</span>Terms of Service</a></li>
			</ul>
		</nav>

		<!-- Col 4: Contact -->
		<div class="footer-col">
			<h3 class="footer-col-title">Contact &amp; Access</h3>
			<ul role="list">
				<li>
					<a href="/auth/login" class="footer-cta-link">
						<span class="icon" aria-hidden="true">lock</span>
						Staff Portal Login
					</a>
				</li>
				<li>
					<a href="mailto:hello@ezeroandone.io" class="footer-cta-link">
						<span class="icon" aria-hidden="true">mail</span>
						hello@ezeroandone.io
					</a>
				</li>
				<li>
					<a href="/capabilities" class="footer-cta-link footer-cta-link--primary">
						<span class="icon" aria-hidden="true">rocket_launch</span>
						Initialize a Project
					</a>
				</li>
			</ul>
		</div>

	</div>

	<!-- Bottom bar -->
	<div class="footer-bottom">
		<div class="inner footer-bottom-inner">
			<p>eZeroAndOne © 2026. All rights reserved. Strict cryptographic access control enforced.</p>
			<div class="footer-bottom-badges">
				<span class="badge-secure">
					<span class="icon" aria-hidden="true">verified_user</span>
					Cryptographic Access Control
				</span>
				<span class="badge-secure">
					<span class="icon" aria-hidden="true">cloud_done</span>
					Cloudflare Edge Infrastructure
				</span>
			</div>
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
	.nav-links a:hover,
	.nav-links a[aria-current="page"] { color: #fff; text-decoration: none; }
	.nav-actions { display: flex; align-items: center; gap: 0.75rem; flex-shrink: 0; }
	.nav-signin { padding: 0.45rem 1rem; font-size: 0.825rem; }
	.nav-cta    { padding: 0.45rem 1.25rem; font-size: 0.825rem; }

	/* ── Shared section structure ─────────────────────── */
	.section-constrained {
		padding: 8rem 0;
	}
	.section-header {
		display: flex; justify-content: space-between; align-items: flex-end;
		margin-bottom: 3rem; gap: 2rem; flex-wrap: wrap;
	}
	.section-eyebrow {
		display: flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em;
		color: var(--accent-blue-hi); margin: 0 0 0.75rem;
	}
	.section-h2 {
		font-size: clamp(1.5rem, 2.75vw, 2.25rem); font-weight: 800;
		color: #fff; margin: 0;
	}
	.section-more {
		display: inline-flex; align-items: center; gap: 0.3rem;
		font-family: var(--font-heading); font-size: 0.8rem; font-weight: 600;
		color: var(--accent-blue-hi); text-decoration: none; white-space: nowrap;
		transition: gap 0.2s ease;
	}
	.section-more:hover { gap: 0.6rem; text-decoration: none; }

	/* ── Hero ─────────────────────────────────────────── */
	.hero {
		text-align: center;
		/* Override section-constrained: use min-height so content never clips */
		min-height: 100svh;
		height: auto;
		overflow: visible;
		padding: 6rem 2rem;
	}
	.hero-inner { max-width: 860px; }
	.eyebrow {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em;
		color: var(--accent-blue-hi); margin: 0 0 1.5rem;
		border: 1px solid rgba(0, 194, 255, 0.25);
		border-radius: 100px; padding: 0.3rem 0.9rem;
	}
	.eyebrow-icon { font-size: 14px; }
	.hero-h1 {
		font-size: clamp(3rem, 8vw, 6.5rem); font-weight: 800;
		letter-spacing: -0.05em; line-height: 1.0;
		color: #fff; margin: 0 0 1.75rem;
	}
	.hero-h1-accent {
		background: linear-gradient(90deg, var(--accent-blue) 0%, var(--accent-blue-hi) 100%);
		-webkit-background-clip: text; -webkit-text-fill-color: transparent; background-clip: text;
	}
	.hero-sub {
		font-family: var(--font-body); font-size: clamp(1rem, 2vw, 1.2rem);
		line-height: 1.75; color: var(--text-secondary);
		max-width: 640px; margin: 0 auto 2.5rem;
	}
	.hero-ctas { display: flex; gap: 1rem; justify-content: center; flex-wrap: wrap; margin-bottom: 2rem; }
	.hero-cta-primary  { padding: 0.85rem 2.25rem; font-size: 0.95rem; }
	.hero-cta-secondary { padding: 0.85rem 2.25rem; font-size: 0.95rem; }
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
	.phil-inner { max-width: 900px; }
	.phil-label {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em;
		color: var(--accent-blue-hi); margin-bottom: 1.5rem;
	}
	.phil-h2 {
		font-size: clamp(2rem, 4.5vw, 3.5rem); font-weight: 800;
		letter-spacing: -0.03em; color: #fff; margin: 0 0 2.5rem;
	}
	.phil-columns {
		display: grid; grid-template-columns: 1fr 1fr; gap: 2.5rem; margin-bottom: 2.5rem;
	}
	.phil-columns p {
		font-family: var(--font-body); font-size: 1.05rem; line-height: 1.8; color: var(--text-secondary);
	}
	.phil-link {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.875rem; font-weight: 700;
		color: var(--accent-blue-hi); text-decoration: none;
		transition: gap 0.2s ease;
	}
	.phil-link:hover { gap: 0.7rem; text-decoration: none; }

	/* ── Capabilities grid ────────────────────────────── */
	.cap-grid { display: grid; grid-template-columns: repeat(3, 1fr); gap: 1.5rem; }
	.cap-card { padding: 2rem; display: flex; flex-direction: column; gap: 1rem; height: 100%; }
	.cap-icon {
		font-size: 28px; color: var(--accent-blue-hi);
		transition: transform 0.3s var(--ease-out-expo);
	}
	:global(.glass-card:hover) .cap-icon { transform: scale(1.15); }
	.cap-title {
		font-family: var(--font-heading); font-size: 1.1rem; font-weight: 700;
		color: #fff; letter-spacing: -0.02em; margin: 0;
	}
	.cap-body {
		font-family: var(--font-body); font-size: 0.9rem; line-height: 1.7;
		color: var(--text-secondary); margin: 0; flex: 1;
	}
	.cap-more {
		display: inline-flex; align-items: center; gap: 0.25rem;
		font-family: var(--font-heading); font-size: 0.78rem; font-weight: 700;
		color: var(--accent-blue-hi); text-decoration: none;
		transition: gap 0.2s ease;
	}
	.cap-more:hover { gap: 0.5rem; text-decoration: none; }

	/* ── Content cards ────────────────────────────────── */
	.content-section { background: var(--bg-surface); }
	.cards-grid { display: grid; grid-template-columns: repeat(auto-fill, minmax(280px,1fr)); gap: 1.5rem; }
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

	/* ── CTA section ──────────────────────────────────── */
	.cta-section { text-align: center; }
	.cta-inner { max-width: 680px; }
	.cta-label {
		display: inline-flex; align-items: center; gap: 0.4rem;
		font-family: var(--font-heading); font-size: 0.7rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.12em;
		color: var(--accent-green); margin-bottom: 1.5rem;
	}
	.cta-h2 {
		font-size: clamp(2rem, 4vw, 3.25rem); font-weight: 800;
		letter-spacing: -0.03em; color: #fff; margin: 0 0 1.5rem;
	}
	.cta-body {
		font-family: var(--font-body); font-size: 1.05rem; line-height: 1.8;
		color: var(--text-secondary); margin: 0 0 2.5rem;
	}
	.cta-btn { padding: 1rem 2.5rem; font-size: 1rem; }

	/* ── Footer ───────────────────────────────────────── */
	.site-footer {
		background: var(--bg-base); border-top: 1px solid rgba(255, 255, 255, 0.08);
		padding-top: 7rem;
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
		width: 36px; height: 36px; border-radius: 8px;
		border: 1px solid var(--glass-border); color: var(--text-secondary);
		text-decoration: none; transition: var(--transition-std);
	}
	.footer-social a:hover { border-color: var(--accent-blue); color: var(--accent-blue-hi); transform: scale(1.08); }
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
	.footer-col a .icon { font-size: 16px; color: var(--text-muted); }
	.footer-cta-link--primary {
		color: var(--accent-blue-hi) !important; font-weight: 600;
	}
	.footer-cta-link--primary .icon { color: var(--accent-blue-hi) !important; }

	/* Footer bottom bar */
	.footer-bottom {
		border-top: 1px solid var(--glass-border);
		padding: 1.25rem 0;
	}
	.footer-bottom-inner {
		display: flex; align-items: center; justify-content: space-between;
		flex-wrap: wrap; gap: 1rem;
	}
	.footer-bottom p {
		font-family: var(--font-body); font-size: 0.75rem; color: var(--text-muted); margin: 0;
	}
	.footer-bottom-badges { display: flex; gap: 1rem; flex-wrap: wrap; }
	.badge-secure {
		display: inline-flex; align-items: center; gap: 0.35rem;
		font-family: var(--font-heading); font-size: 0.62rem; font-weight: 700;
		text-transform: uppercase; letter-spacing: 0.08em;
		color: var(--text-muted); border: 1px solid var(--glass-border);
		border-radius: 100px; padding: 0.2rem 0.65rem;
	}
	.badge-secure .icon { font-size: 12px; }

	/* ── Responsive ───────────────────────────────────── */
	@media (max-width: 1024px) {
		.cap-grid { grid-template-columns: 1fr 1fr; }
		.footer-grid { grid-template-columns: 1fr 1fr; gap: 2rem; }
		.phil-columns { grid-template-columns: 1fr; }
	}
	@media (max-width: 640px) {
		.nav-links { display: none; }
		.nav-actions .nav-signin { display: none; }
		.cap-grid { grid-template-columns: 1fr; }
		.footer-grid { grid-template-columns: 1fr; }
		.footer-bottom-inner { flex-direction: column; align-items: flex-start; }
		.hero-ctas { flex-direction: column; align-items: center; }
	}
</style>
