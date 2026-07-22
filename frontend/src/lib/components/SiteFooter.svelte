<script lang="ts">
  // Site-wide footer — shared across all public pages.
</script>

<footer class="site-footer" aria-label="Site footer">
  <div class="inner footer-grid">

    <!-- Col 1: Identity -->
    <div class="footer-col footer-col--brand">
      <a href="/" class="footer-logo" aria-label="eZeroAndOne.io">
        eZeroAndOne.io
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

  <div class="footer-bottom">
    <div class="inner footer-bottom-inner">
      <p>eZeroAndOne.io &copy; 2026. All rights reserved.</p>
      <p class="footer-bottom-mono">Strict cryptographic access control enforced.</p>
    </div>
  </div>
</footer>

<style>
  .site-footer {
    background: var(--bg-base, #050508);
    border-top: 1px solid rgba(255,255,255,0.05);
    padding-top: 6rem;
  }
  .footer-grid {
    display: grid; grid-template-columns: 1.4fr 1fr 1fr 1fr;
    gap: 3rem; padding-bottom: 4rem;
  }
  .footer-logo {
    font-family: var(--font-heading, 'Inter Tight', sans-serif); font-size: 1.5rem; font-weight: 800;
    letter-spacing: -0.04em; color: #ffffff; text-decoration: none; display: inline-block;
    margin-bottom: 1rem;
  }
  .footer-bio {
    font-family: var(--font-body); font-size: 0.875rem; line-height: 1.7;
    color: var(--text-secondary, rgba(255,255,255,0.5)); margin: 0 0 1.5rem;
  }
  .footer-social { display: flex; gap: 0.75rem; }
  .footer-social a {
    display: flex; align-items: center; justify-content: center;
    width: 36px; height: 36px; border: 1px solid rgba(255,255,255,0.08);
    color: var(--text-secondary, rgba(255,255,255,0.5)); text-decoration: none;
    transition: border-color 0.2s, color 0.2s;
  }
  .footer-social a:hover { border-color: var(--accent-blue-hi, #00d4ff); color: var(--accent-blue-hi, #00d4ff); }
  .footer-col-title {
    font-family: var(--font-heading); font-size: 0.65rem; font-weight: 700;
    text-transform: uppercase; letter-spacing: 0.12em; color: rgba(255,255,255,0.3);
    margin: 0 0 1.25rem;
  }
  .footer-col ul { list-style: none; margin: 0; padding: 0; display: flex; flex-direction: column; gap: 0.7rem; }
  .footer-col a {
    display: flex; align-items: center; gap: 0.5rem;
    font-family: var(--font-body); font-size: 0.875rem;
    color: var(--text-secondary, rgba(255,255,255,0.5));
    text-decoration: none; transition: color 0.2s;
  }
  .footer-col a:hover { color: #fff; text-decoration: none; }
  .footer-col a .material-symbols-outlined { font-size: 16px; opacity: 0.5; }
  .footer-cta-link--primary { color: var(--accent-blue-hi, #00d4ff) !important; font-weight: 600; }
  .footer-status-link { cursor: default; }
  .status-dot {
    display: inline-block; width: 7px; height: 7px; border-radius: 50%;
    background: var(--accent-green, #00ff88);
    animation: pulse 2s ease-in-out infinite;
  }
  @keyframes pulse { 0%,100%{opacity:1} 50%{opacity:0.35} }
  .footer-bottom { border-top: 1px solid rgba(255,255,255,0.06); padding: 1.25rem 0; }
  .footer-bottom-inner {
    display: flex; align-items: center; justify-content: space-between; flex-wrap: wrap; gap: 1rem;
  }
  .footer-bottom-inner p {
    font-family: var(--font-body); font-size: 0.75rem; color: rgba(255,255,255,0.3); margin: 0;
  }
  .footer-bottom-mono {
    font-family: var(--font-heading); font-size: 0.6rem !important;
    text-transform: uppercase; letter-spacing: 0.1em;
  }
  @media (max-width: 1100px) {
    .footer-grid { grid-template-columns: 1fr 1fr; gap: 2rem; }
  }
  @media (max-width: 640px) {
    .footer-grid { grid-template-columns: 1fr; }
    .footer-bottom-inner { flex-direction: column; align-items: flex-start; }
  }
</style>
