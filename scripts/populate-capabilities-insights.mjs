/**
 * populate-capabilities-insights.mjs
 *
 * 1. Creates 18 capability posts (6 categories × 3 services each)
 * 2. Creates 12 SEO-optimised insight posts
 * 3. Attaches AI-generated cover images via Unsplash Source (free, no key)
 *
 * Usage:
 *   [System.Environment]::SetEnvironmentVariable("SESSION_COOKIE","<jwt>")
 *   [System.Environment]::SetEnvironmentVariable("BASE_URL","https://ezeroandone.io")
 *   node scripts/populate-capabilities-insights.mjs
 */

const SESSION  = process.env.SESSION_COOKIE ?? '';
const BASE_URL = process.env.BASE_URL ?? 'https://ezeroandone.io';
if (!SESSION) { console.error('Set SESSION_COOKIE'); process.exit(1); }

const NOW = Math.floor(Date.now() / 1000);

// ---------------------------------------------------------------------------
// Unsplash Source — free, no key, stable CDN image by keyword
// ---------------------------------------------------------------------------
const img = (query, w = 1280, h = 720) =>
  `https://source.unsplash.com/${w}x${h}/?${encodeURIComponent(query)}`;


// ---------------------------------------------------------------------------
// SERVICE TAXONOMY  (6 categories, each with 3 services = 18 capability posts)
// Landing page shows 6 category cards; /capabilities shows all 18 individual cards
// ---------------------------------------------------------------------------

const CAPABILITIES = [

  // ── CATEGORY 1: Software Engineering ────────────────────────────────────
  {
    title: 'Web Application Development',
    summary: 'Custom web apps engineered on modern edge runtimes — SvelteKit, React, Next.js — deployed globally on Cloudflare Pages with sub-100 ms TTFB.',
    category: 'Software Engineering',
    material_icon: 'web',
    tags: 'web app, SvelteKit, React, Cloudflare, full-stack',
    body_md: `## Service Overview

We build production-grade web applications from first principles, optimised for performance, security, and long-term maintainability. Every project is architected with clear separation of concerns, typed APIs, and zero-downtime deployment pipelines.

### Stack
- **Frontend**: SvelteKit / React / Next.js — SSR + edge rendering
- **Backend**: Cloudflare Workers (Rust or TypeScript), Node.js, Django
- **Data**: Cloudflare D1 (SQLite at edge), PostgreSQL, Supabase, PlanetScale
- **Hosting**: Cloudflare Pages, Vercel, AWS Amplify

### Typical Deliverables
- MVP web application (4–12 weeks)
- Enterprise platform migration
- Progressive Web App (PWA) conversion
- Performance audit and re-architecture`,
  },
  {
    title: 'Mobile App Development',
    summary: 'iOS and Android applications built with React Native and Flutter — single codebase, native performance, seamless backend integration.',
    category: 'Software Engineering',
    material_icon: 'phone_android',
    tags: 'mobile app, React Native, Flutter, iOS, Android',
    body_md: `## Service Overview

We deliver cross-platform mobile applications that feel native on both iOS and Android. Our mobile builds are tightly integrated with your existing API layer and designed for App Store / Play Store approval from the first submission.

### Stack
- **Cross-platform**: React Native, Expo, Flutter
- **Native modules**: Swift (iOS), Kotlin (Android) where performance demands
- **State management**: Redux Toolkit, Zustand, Riverpod
- **Backend integration**: REST, GraphQL, WebSockets

### Typical Deliverables
- Consumer app (8–16 weeks)
- Enterprise field-ops application
- E-commerce mobile storefront
- Push notification and offline sync systems`,
  },
  {
    title: 'Desktop & Enterprise Software',
    summary: 'Electron and Tauri desktop applications for Windows, macOS, and Linux — purpose-built internal tools that replace legacy enterprise software.',
    category: 'Software Engineering',
    material_icon: 'computer',
    tags: 'desktop app, Electron, Tauri, enterprise software, internal tools',
    body_md: `## Service Overview

Many enterprise workflows remain locked in outdated desktop software. We build modern replacements using Tauri (Rust + web frontend) for minimal footprint or Electron for rapid delivery — both with auto-update pipelines and enterprise MDM compatibility.

### Stack
- **Framework**: Tauri (Rust), Electron, .NET MAUI
- **UI**: React, SvelteKit, WPF
- **Distribution**: MSIX, DMG, AppImage, auto-update via Squirrel / Tauri Updater
- **Security**: Code signing, sandboxed file access, Windows Hello integration

### Typical Deliverables
- Internal operations dashboard
- Offline-first data collection tool
- Legacy system replacement`,
  },

  // ── CATEGORY 2: Infrastructure & Cloud ──────────────────────────────────
  {
    title: 'Cloud Architecture & DevOps',
    summary: 'Multi-cloud infrastructure designed for resilience and cost efficiency — Cloudflare, AWS, GCP, Azure — with full CI/CD and Infrastructure-as-Code.',
    category: 'Infrastructure & Cloud',
    material_icon: 'cloud_sync',
    tags: 'cloud, AWS, Cloudflare, DevOps, CI/CD, Infrastructure as Code',
    body_md: `## Service Overview

We design and operate cloud infrastructure that scales with your business — from startup MVP to enterprise multi-region deployments. Every environment is version-controlled, monitored, and reproducible.

### Services
- Cloud architecture design (AWS, GCP, Azure, Cloudflare)
- Infrastructure-as-Code (Terraform, Pulumi, CDK)
- CI/CD pipeline setup (GitHub Actions, GitLab CI, CircleCI)
- Container orchestration (Kubernetes, ECS, Docker Swarm)
- Cost optimisation and FinOps reporting`,
  },
  {
    title: 'Network Design & Management',
    summary: 'Enterprise-grade LAN/WAN design, managed routing, SD-WAN, and secure remote access — from structured cabling to zero-trust network segmentation.',
    category: 'Infrastructure & Cloud',
    material_icon: 'router',
    tags: 'networking, LAN, WAN, SD-WAN, zero trust, Cisco, Mikrotik',
    body_md: `## Service Overview

Reliable connectivity is the foundation of every digital operation. We design, install, and manage enterprise networks that eliminate single points of failure and enforce least-privilege access at every layer.

### Services
- LAN/WAN design and structured cabling
- Router and firewall configuration (Cisco, Mikrotik, Fortinet, pfSense)
- SD-WAN deployment and management
- VPN and zero-trust remote access (Cloudflare Access, WireGuard)
- Network monitoring and alerting (Zabbix, PRTG, Grafana)`,
  },
  {
    title: 'Hardware Procurement & Installation',
    summary: 'End-to-end hardware sourcing, configuration, and installation — servers, workstations, network equipment, access control, and CCTV systems.',
    category: 'Infrastructure & Cloud',
    material_icon: 'memory',
    tags: 'hardware, servers, procurement, installation, CCTV, access control',
    body_md: `## Service Overview

We handle the full hardware lifecycle from specification and procurement through to rack-and-stack installation, configuration, and handover. Vendor-agnostic sourcing ensures you get the right hardware at the best available price.

### Services
- Server procurement and rack installation (Dell, HP, Lenovo, SuperMicro)
- Workstation and laptop fleet deployment
- Biometric and RFID access control installation
- IP CCTV and NVR system design and installation
- UPS and power conditioning
- Spare-parts inventory management`,
  },

  // ── CATEGORY 3: Cybersecurity ────────────────────────────────────────────
  {
    title: 'Cybersecurity Assessment & Hardening',
    summary: 'Vulnerability assessments, penetration testing, and security hardening for web applications, networks, and cloud environments — aligned to ISO 27001 and NDPR.',
    category: 'Cybersecurity',
    material_icon: 'security',
    tags: 'cybersecurity, penetration testing, vulnerability assessment, ISO 27001, NDPR',
    body_md: `## Service Overview

We identify security gaps before attackers do. Our assessments combine automated scanning with expert manual testing, producing clear remediation roadmaps prioritised by risk.

### Services
- Web application penetration testing (OWASP Top 10)
- Network vulnerability assessment and port scanning
- Cloud security posture review (AWS, GCP, Cloudflare)
- Social engineering and phishing simulation
- Security hardening (CIS Benchmarks, STIG)
- ISO 27001 gap analysis and remediation roadmap`,
  },
  {
    title: 'IT Auditing & Compliance Advisory',
    summary: 'Independent IT audits aligned to COBIT, ISO 27001, NDPR, and SOC 2 — helping organisations meet regulatory obligations and board-level governance requirements.',
    category: 'Cybersecurity',
    material_icon: 'fact_check',
    tags: 'IT audit, compliance, COBIT, ISO 27001, SOC 2, NDPR, governance',
    body_md: `## Service Overview

Regulatory compliance is not a one-time exercise. We conduct rigorous IT audits that evaluate controls, identify gaps, and produce evidence-backed reports suitable for regulatory submission and board review.

### Services
- IT general controls audit (ITGC)
- Data protection impact assessment (DPIA) — NDPR / GDPR
- SOC 2 Type I/II readiness assessment
- Business continuity and disaster recovery review
- Third-party vendor risk assessment
- Audit committee reporting and management letter`,
  },
  {
    title: 'Managed Security Operations',
    summary: '24/7 threat monitoring, incident response, and SIEM management — continuous visibility across your endpoints, network, and cloud workloads.',
    category: 'Cybersecurity',
    material_icon: 'shield',
    tags: 'SOC, SIEM, threat detection, incident response, endpoint security, MDR',
    body_md: `## Service Overview

Threats don't keep business hours. Our managed security operations provide continuous monitoring across your digital estate with defined SLAs for detection, triage, and response.

### Services
- Security Information and Event Management (SIEM) deployment and tuning
- Endpoint Detection and Response (EDR) — CrowdStrike, SentinelOne, Microsoft Defender
- 24/7 alert triage and incident response
- Threat intelligence integration
- Forensic investigation and root-cause analysis
- Monthly threat briefing and security posture reporting`,
  },

  // ── CATEGORY 4: Digital & Marketing ─────────────────────────────────────
  {
    title: 'Digital Marketing & SEO',
    summary: 'Data-driven digital marketing — technical SEO, paid media, content strategy, and conversion rate optimisation to grow organic traffic and reduce CAC.',
    category: 'Digital & Marketing',
    material_icon: 'trending_up',
    tags: 'SEO, digital marketing, paid media, Google Ads, content strategy, CRO',
    body_md: `## Service Overview

We grow digital revenue through a combination of technical excellence and strategic content. Every campaign is tied to measurable business outcomes, not vanity metrics.

### Services
- Technical SEO audit and implementation (Core Web Vitals, schema, sitemaps)
- Keyword research and content strategy
- Google Ads and Meta Ads campaign management
- Email marketing automation (Mailchimp, SendGrid, Klaviyo)
- Conversion rate optimisation (A/B testing, heatmaps)
- Analytics setup (GA4, Mixpanel, Plausible)`,
  },
  {
    title: 'Brand Identity & Web Design',
    summary: 'Brand strategy, visual identity, and website design — from logo and style guides to full design systems and Figma-to-code production.',
    category: 'Digital & Marketing',
    material_icon: 'palette',
    tags: 'brand identity, logo design, web design, Figma, UI/UX, design system',
    body_md: `## Service Overview

A strong brand is your most durable competitive asset. We build brand identities that communicate authority at every touchpoint — from your first investor deck to your highest-traffic landing page.

### Services
- Brand strategy and positioning workshop
- Logo design and visual identity system
- Typography, colour palette, and brand guidelines
- Website design (Figma wireframes → production code)
- UI/UX design for web and mobile applications
- Design system component library`,
  },
  {
    title: 'E-Commerce Development',
    summary: 'High-converting online stores on WooCommerce, Shopify, and custom headless stacks — integrated with payment gateways, inventory systems, and fulfilment APIs.',
    category: 'Digital & Marketing',
    material_icon: 'storefront',
    tags: 'e-commerce, WooCommerce, Shopify, online store, payment gateway, headless',
    body_md: `## Service Overview

We build e-commerce platforms engineered for conversion — not just catalogues. Every store is optimised for page speed, mobile UX, and seamless checkout across Paystack, Flutterwave, Stripe, and bank transfer flows.

### Services
- WooCommerce build and customisation
- Shopify store setup and theme development
- Headless e-commerce (Next.js + Shopify Storefront API)
- Payment gateway integration (Paystack, Flutterwave, Stripe)
- Inventory and ERP integration (Odoo, SAP, Sage)
- Marketplace integration (Jumia, Konga, Amazon)`,
  },

  // ── CATEGORY 5: Smart Systems & Energy ──────────────────────────────────
  {
    title: 'Smart Home & Building Automation',
    summary: 'Intelligent building automation using KNX, Zigbee, and Z-Wave — lighting, climate, security, and AV control unified into a single intuitive interface.',
    category: 'Smart Systems & Energy',
    material_icon: 'home_work',
    tags: 'smart home, building automation, KNX, Zigbee, Home Assistant, IoT',
    body_md: `## Service Overview

We engineer smart environments that respond intelligently to occupancy, time, and user preference — reducing energy consumption while dramatically improving comfort and security.

### Services
- Smart lighting control (Philips Hue, Lutron, KNX)
- HVAC and climate automation
- Smart lock, intercom, and access control integration
- Motorised blinds and AV system control
- Home Assistant / Google Home / Amazon Alexa integration
- Remote monitoring and scheduled automation rules`,
  },
  {
    title: 'Renewable Energy Installation',
    summary: 'Commercial and residential solar PV systems, battery storage, and energy monitoring — engineered for maximum yield and ROI in the Nigerian energy environment.',
    category: 'Smart Systems & Energy',
    material_icon: 'wb_sunny',
    tags: 'solar, renewable energy, battery storage, inverter, MPPT, energy monitoring',
    body_md: `## Service Overview

Load-shedding and grid unreliability are operational realities in Nigeria. We design and install solar-plus-storage systems that provide guaranteed uptime for homes, offices, and industrial facilities.

### Services
- Solar PV system design (on-grid, off-grid, hybrid)
- Battery storage system installation (lithium, AGM, gel)
- Inverter and charge controller sizing and installation
- Energy monitoring dashboard (real-time yield vs consumption)
- Existing generator integration and automatic switchover
- Annual maintenance and performance audit`,
  },
  {
    title: 'IoT & Industrial Automation',
    summary: 'Custom IoT sensor networks, SCADA systems, and industrial automation solutions — real-time data collection from physical assets to cloud dashboards.',
    category: 'Smart Systems & Energy',
    material_icon: 'sensors',
    tags: 'IoT, SCADA, industrial automation, sensors, MQTT, PLC, edge computing',
    body_md: `## Service Overview

We connect physical infrastructure to digital intelligence — from factory floors to agricultural sensors — providing real-time visibility and automated control over critical operational assets.

### Services
- IoT sensor network design and deployment (temperature, humidity, vibration, flow)
- MQTT broker setup and data pipeline (HiveMQ, EMQX, AWS IoT Core)
- PLC programming and SCADA system configuration
- Edge computing nodes (Raspberry Pi, Industrial PCs)
- Cloud dashboard and alerting (Grafana, InfluxDB, Tableau)
- Predictive maintenance models`,
  },

  // ── CATEGORY 6: IT Consulting & Training ────────────────────────────────
  {
    title: 'IT Strategy & Consulting',
    summary: 'Board-level technology advisory — digital transformation roadmaps, IT budget optimisation, vendor selection, and technology governance frameworks.',
    category: 'IT Consulting & Training',
    material_icon: 'insights',
    tags: 'IT consulting, digital transformation, IT strategy, CTO advisory, vendor management',
    body_md: `## Service Overview

Technology decisions made at the strategic level determine outcomes for years. We provide independent advisory that bridges the gap between business leadership and technical execution.

### Services
- Digital transformation strategy and roadmap
- IT budget review and cost optimisation
- Technology vendor selection and RFP management
- IT governance framework design (COBIT, ITIL)
- Virtual CTO / fractional CTO service
- Technology due diligence for investment and M&A`,
  },
  {
    title: 'IT Training & Staff Development',
    summary: 'Structured IT training programmes for organisations — from end-user digital skills to developer upskilling and cybersecurity awareness campaigns.',
    category: 'IT Consulting & Training',
    material_icon: 'school',
    tags: 'IT training, cybersecurity awareness, developer training, digital skills, corporate training',
    body_md: `## Service Overview

Your technology infrastructure is only as strong as the people operating it. We design and deliver training programmes that build genuine capability — not just compliance checkboxes.

### Services
- End-user digital literacy and productivity training
- Cybersecurity awareness programme (phishing, social engineering)
- Developer upskilling (modern web, cloud, security)
- IT department capacity building
- Custom LMS setup and content development
- Training needs analysis and skills gap assessment`,
  },
  {
    title: 'Help Desk & Managed IT Support',
    summary: 'Outsourced IT support for businesses — tiered help desk, device management, software licensing, and proactive maintenance with defined SLA response times.',
    category: 'IT Consulting & Training',
    material_icon: 'support_agent',
    tags: 'managed IT, help desk, IT support, MSP, device management, SLA',
    body_md: `## Service Overview

Reliable day-to-day IT operations are the bedrock of business continuity. Our managed IT support model gives you a dedicated technical team without the overhead of a full in-house department.

### Services
- Tier 1/2/3 help desk (phone, email, remote access)
- Endpoint device management (Windows, macOS, mobile MDM)
- Software licensing and patch management
- Backup monitoring and recovery testing
- Proactive maintenance and health checks
- Monthly IT operations report and SLA review`,
  },
];

// ---------------------------------------------------------------------------
// INSIGHT POSTS  (12 SEO-optimised articles)
// Target: Nigeria IT market + global SME / enterprise searches
// ---------------------------------------------------------------------------

const INSIGHTS = [
  {
    title: 'Why Nigerian Businesses Are Losing Money to Downtime (And How to Fix It)',
    summary: 'Unplanned IT downtime costs Nigerian SMEs an average of ₦2.1 million per hour. This guide breaks down the root causes and the infrastructure decisions that eliminate them.',
    category: 'Infrastructure & Cloud',
    tags: 'IT downtime, Nigeria, SME, business continuity, infrastructure',
    featured_image_query: 'server room, data center, Nigeria, technology',
    body_md: `Every hour your systems are down, you are paying: in lost sales, idle staff, and damaged client relationships. Yet most Nigerian businesses treat downtime as inevitable rather than engineered-out.

## The True Cost of Downtime

Gartner estimates the average cost of IT downtime at $5,600 per minute globally. In a Nigerian SME context — where staff costs are lower but revenue concentration is higher — the figure translates differently but the pain is identical.

The three most common causes of downtime in Nigerian businesses are:
1. **Power interruptions** — a generator that starts 90 seconds after NEPA goes is 90 seconds of crashed databases
2. **Single points of failure** — one router, one ISP, one server with no failover
3. **Deferred maintenance** — equipment running past rated lifecycle without replacement

## The Architecture That Eliminates It

A resilient stack for a Nigerian SME does not require enterprise budgets:

- **Instant power failover**: APC or Eaton UPS with at least 10 minutes of runtime, combined with a clean generator handover
- **Dual-ISP load balancing**: A Mikrotik or pfSense router configured to failover to a mobile LTE connection in under 30 seconds
- **Cloud-first applications**: Moving critical apps to Cloudflare Workers / SaaS products means your application layer survives local power events entirely
- **Automated backups**: Scheduled offsite backups to an S3-compatible bucket (Cloudflare R2 has no egress fees) with weekly restore testing

## What a Resilience Audit Looks Like

A proper business continuity review identifies your Recovery Time Objective (RTO) and Recovery Point Objective (RPO) for every critical system, then maps your current architecture against those targets. The gap between the two is your risk exposure.

eZeroAndOne conducts resilience audits for Nigerian businesses of all sizes. [Contact us](/capabilities) to discuss your environment.`,
  },
  {
    title: 'The Complete Guide to Solar Power for Nigerian Businesses in 2025',
    summary: 'Everything Nigerian business owners need to know before investing in a solar installation — system sizing, inverter selection, battery chemistry, and realistic ROI calculations.',
    category: 'Smart Systems & Energy',
    tags: 'solar power Nigeria, renewable energy, inverter, battery backup, ROI',
    featured_image_query: 'solar panels, Nigeria, rooftop, energy',
    body_md: `NEPA (PHCN, DisCo — whatever name the grid goes by this year) averages 12 hours of supply per day in Lagos and less in most other states. For businesses, this is not inconvenience — it is a direct operational tax. Solar-plus-storage eliminates that tax permanently.

## Sizing Your System Correctly

The most common mistake is undersizing. Start with your actual load:

1. List every device that must run during a grid outage
2. Find its wattage (label on the back, or a Kill-A-Watt meter)
3. Multiply by your required runtime (hours per day)
4. Add 25% buffer for efficiency losses and aging batteries

**Example**: A small office with 10 computers (150W each), lighting (200W), and a server room (800W) needs:
- Total load: 2,700W
- 8 hours runtime: 21.6 kWh per day
- System size: 6 kW solar array + 20 kWh battery bank (lithium-iron phosphate recommended)

## Battery Chemistry: Lead-Acid vs Lithium

| | Flooded Lead-Acid | AGM | Lithium (LFP) |
|---|---|---|---|
| Cost | ₦ | ₦₦ | ₦₦₦₦ |
| Cycle life | 300–500 | 500–800 | 3,000–6,000 |
| Depth of discharge | 50% | 50–60% | 80–90% |
| Maintenance | Monthly | Quarterly | None |

For a business, the correct answer is almost always lithium iron phosphate (LFP). The higher upfront cost is recovered within 3–4 years through cycle life alone.

## ROI Calculation

A well-sized solar system for a Nigerian office building typically breaks even in 3–4 years when replacing generator fuel costs. With grid incentives improving and panel prices down 40% since 2020, the economics have never been better.

[Request a solar consultation from eZeroAndOne →](/capabilities)`,
  },
  {
    title: 'NDPR Compliance Checklist: What Nigerian Companies Must Do Before December',
    summary: 'The Nigeria Data Protection Regulation carries fines up to 2% of annual gross revenue. This practical checklist covers every control your organisation needs to implement.',
    category: 'Cybersecurity',
    tags: 'NDPR compliance, Nigeria data protection, NITDA, data privacy, cybersecurity',
    featured_image_query: 'cybersecurity, data protection, compliance, digital security',
    body_md: `The Nigeria Data Protection Regulation (NDPR) — now codified under the Nigeria Data Protection Act 2023 — is no longer aspirational. NITDA has issued fines, and enforcement is accelerating.

## What NDPR Requires

Every organisation that processes personal data of Nigerian citizens must:

1. **Appoint a Data Protection Officer (DPO)** — or engage a licensed Data Protection Compliance Organisation (DPCO)
2. **Conduct a Data Protection Impact Assessment (DPIA)** for high-risk processing activities
3. **Maintain a Data Processing Agreement** with every third-party vendor that handles personal data
4. **Implement appropriate technical and organisational measures** — encryption, access controls, audit logs
5. **Establish a data breach notification procedure** — 72 hours to NITDA, without undue delay

## The Practical Checklist

**Governance**
- [ ] DPO appointed and registered with NITDA
- [ ] Privacy policy published and up to date
- [ ] Records of processing activities (ROPA) maintained
- [ ] Staff data protection training completed (annual minimum)

**Technical Controls**
- [ ] Personal data encrypted at rest and in transit (AES-256, TLS 1.3)
- [ ] Access controls enforcing least privilege
- [ ] Multi-factor authentication on all systems processing personal data
- [ ] Audit logs retained for minimum 12 months
- [ ] Data retention schedules defined and enforced

**Vendor Management**
- [ ] Data Processing Agreements signed with all third-party processors
- [ ] Vendor security assessments conducted
- [ ] Cross-border transfer controls (adequacy or contractual safeguards)

**Incident Response**
- [ ] Breach detection and notification procedure documented
- [ ] Incident response retainer or on-call team identified
- [ ] Tabletop exercise conducted within the last 12 months

Non-compliance fines start at ₦10 million and escalate to 2% of annual gross revenue. eZeroAndOne provides NDPR compliance assessments and remediation programmes. [Contact us →](/capabilities)`,
  },
  {
    title: 'How to Choose Between a Native App and a Web App for Your Business',
    summary: 'Native app vs web app vs progressive web app — a clear framework for making the right technology decision based on your budget, timeline, and user needs.',
    category: 'Software Engineering',
    tags: 'native app vs web app, mobile app development, PWA, React Native, technology decision',
    featured_image_query: 'mobile app development, smartphone, code, UI',
    body_md: `The most expensive technical decision a business makes is often choosing the wrong platform before a single line of code is written. Here is a clear framework for getting it right.

## The Three Options

### Native App
Built separately for iOS (Swift) and Android (Kotlin/Java). Best performance and access to all device APIs. Highest cost — you are essentially building two products.

**Choose native when**: You need camera, Bluetooth, NFC, health sensors, or AR at the deepest level, and you have budget for two codebases.

### Cross-Platform App (React Native / Flutter)
One codebase targeting both platforms. 80–95% of native performance for most use cases. Significantly lower development cost than pure native.

**Choose cross-platform when**: You need a mobile app and want to reduce cost without sacrificing the App Store / Play Store presence your users expect.

### Progressive Web App (PWA)
A web application that installs on the home screen, works offline, and sends push notifications. No app store required. Indexed by Google.

**Choose PWA when**: Your users are primarily on mobile web, you want Google discoverability, or you cannot afford the App Store approval overhead.

## Decision Framework

| | Native | Cross-Platform | PWA |
|---|---|---|---|
| Cost | ₦₦₦₦ | ₦₦₦ | ₦₦ |
| Performance | 100% | 85–95% | 70–85% |
| Device API access | Full | Most | Limited |
| App Store presence | Yes | Yes | No |
| Google indexable | No | No | Yes |
| Time to market | Longest | Medium | Fastest |

For most Nigerian SMEs building their first mobile product, React Native is the pragmatic choice — one team, one codebase, two stores.

[Discuss your app with eZeroAndOne →](/capabilities)`,
  },
  {
    title: 'What Is Penetration Testing and Does Your Nigerian Business Need It?',
    summary: 'Penetration testing is not just for banks. This guide explains what a pen test actually involves, what it costs, and which Nigerian businesses are legally required to conduct one.',
    category: 'Cybersecurity',
    tags: 'penetration testing Nigeria, cybersecurity assessment, NDPR, vulnerability testing, ethical hacking',
    featured_image_query: 'ethical hacking, cybersecurity, network security, terminal',
    body_md: `Penetration testing — simulated cyberattacks conducted by authorised security professionals — is one of the most misunderstood services in the Nigerian IT market. Most business owners either think it is only for banks or that they cannot afford it. Both assumptions are wrong.

## What a Penetration Test Actually Is

A penetration test is a structured attempt to compromise your systems using the same techniques a real attacker would use — before a real attacker does.

A typical engagement has three phases:

1. **Reconnaissance** — gathering publicly available information about your domain, IP addresses, employee LinkedIn profiles, and technology stack
2. **Exploitation** — attempting to exploit discovered vulnerabilities to gain unauthorised access
3. **Reporting** — a detailed report of every finding, its risk rating, and a step-by-step remediation guide

## Who Needs a Pen Test in Nigeria

**Legally required**:
- Any company that stores or processes personal data under NDPR (virtually every formal business)
- PCI DSS-compliant merchants processing card payments
- CBN-regulated fintechs and financial institutions

**Strongly recommended**:
- Any company with a public-facing web application
- SaaS businesses and software companies
- Healthcare organisations storing patient records
- Companies with remote workforces

## What It Costs

A professional web application penetration test for a Nigerian SME typically ranges from ₦800,000 to ₦3,000,000 depending on scope. The cost of a breach — including NDPR fines, client notification, remediation, and reputational damage — is multiples of that.

[Request a penetration testing quote →](/capabilities)`,
  },
  {
    title: 'The 2025 Guide to IT Infrastructure for Nigerian Startups',
    summary: 'What cloud services, hardware, and security tools does a Nigerian startup actually need in 2025? A practical, budget-conscious guide from zero to production-ready.',
    category: 'IT Consulting & Training',
    tags: 'IT infrastructure Nigeria, startup tech stack, cloud services, Nigerian startup, SME technology',
    featured_image_query: 'startup office, technology, team, Nigeria, computers',
    body_md: `Most Nigerian startup founders either overspend on hardware they do not need or underspend on security they cannot afford to skip. This guide draws the correct line.

## Phase 1: Pre-Product (1–5 people)

At this stage, you need zero hardware. Everything should be cloud-native:

- **Work devices**: Each founder's personal laptop — no company hardware yet
- **Collaboration**: Google Workspace (₦1,100/user/month) — email, Docs, Meet, Drive
- **Version control**: GitHub (free for private repos up to 3 collaborators)
- **Cloud hosting**: Cloudflare Workers + Pages (free tier covers early-stage apps)
- **Password management**: 1Password Teams or Bitwarden (mandatory from day one)
- **Total monthly spend**: ~₦5,500 for 5 people

## Phase 2: Early Team (5–20 people)

As you bring on employees, a few things become non-negotiable:

- **Device management**: Microsoft Intune or Jamf to enforce disk encryption and remote wipe
- **MFA everywhere**: Enforce hardware TOTP or passkeys — SMS 2FA is not acceptable
- **Dedicated internet**: A business-class ISP with an SLA, plus a mobile failover router
- **Backup**: Daily automated backups to Cloudflare R2 or Backblaze B2 — test monthly

## Phase 3: Growth (20–100 people)

This is when infrastructure becomes a strategic function:

- **Dedicated IT support**: Either hire a junior sysadmin or engage a managed IT provider
- **Endpoint detection**: Deploy an EDR tool (Microsoft Defender for Business is ₦400/device/month)
- **Network segmentation**: VLAN separation of guest WiFi, staff, and production systems
- **NDPR compliance**: Conduct your first DPIA and appoint a DPO

[Talk to eZeroAndOne about your startup's IT needs →](/capabilities)`,
  },
  {
    title: 'How to Build a Website That Actually Ranks on Google in Nigeria',
    summary: 'Technical SEO for Nigerian businesses — the exact factors Google uses to rank Nigerian websites, and a practical 90-day roadmap to the first page.',
    category: 'Digital & Marketing',
    tags: 'SEO Nigeria, Google ranking Nigeria, technical SEO, local SEO, website optimisation',
    featured_image_query: 'SEO, Google search, digital marketing, analytics, website',
    body_md: `Most Nigerian business websites are invisible on Google. Not because the content is bad, but because the technical foundation is broken. Here is what actually drives rankings in the Nigerian search market.

## Why Most Nigerian Sites Fail on Google

The three most common technical SEO failures we find in Nigerian website audits:

1. **Page speed below 50 on mobile** — Google uses Core Web Vitals as a ranking signal. Sites hosted on cheap shared hosting in the US or EU serving Nigerian users often score below 30.
2. **No HTTPS** — Google has explicitly deprioritised HTTP sites since 2014. Yet a significant portion of Nigerian SME websites are still on HTTP.
3. **No structured data** — Local businesses with Google Business Profile but no schema markup miss out on rich results (star ratings, opening hours, phone click-to-call).

## The Technical SEO Checklist

**Speed (most impactful)**
- [ ] Serve from a CDN with Nigerian PoPs (Cloudflare has Johannesburg; closest to Lagos)
- [ ] Images in WebP format, lazy-loaded, with explicit dimensions
- [ ] Core Web Vitals: LCP < 2.5s, FID < 100ms, CLS < 0.1
- [ ] Remove render-blocking scripts

**On-page**
- [ ] Title tags 50–60 characters, containing primary keyword
- [ ] Meta descriptions 150–160 characters with a CTA
- [ ] H1 containing primary keyword — one per page
- [ ] Schema markup: LocalBusiness, Service, FAQ, Article as appropriate

**Local SEO**
- [ ] Google Business Profile claimed and complete
- [ ] NAP (Name, Address, Phone) consistent across all citations
- [ ] Local keyword targeting: "web design Lagos", "IT company Abuja"
- [ ] Customer reviews actively requested and responded to

## The 90-Day Roadmap

- **Month 1**: Technical audit, fix speed issues, implement schema, submit sitemap
- **Month 2**: Content creation — 4 blog posts per month targeting long-tail keywords
- **Month 3**: Link building — local directory submissions, press releases, partner exchanges

[eZeroAndOne builds websites engineered for Google from the ground up →](/capabilities)`,
  },
  {
    title: 'Smart Office Technology: What to Install and What to Skip',
    summary: 'A practical guide to smart office installations for Nigerian businesses — what actually saves money and improves productivity versus what is just expensive novelty.',
    category: 'Smart Systems & Energy',
    tags: 'smart office, building automation, smart lighting, IoT Nigeria, office technology',
    featured_image_query: 'smart office, building automation, modern office, lighting control',
    body_md: `The smart building market is full of vendors selling solutions in search of a problem. This guide separates the investments that deliver measurable ROI from the gadgets that impress in demos and gather dust in production.

## High ROI: Install These

**Smart lighting control**
Occupancy-based lighting that turns off automatically when a room is empty typically reduces lighting energy by 35–45%. In a Nigerian office with 12+ hours of generator runtime per day, this directly reduces diesel costs.

**Access control (smart locks and biometric entry)**
Replaces physical keys (which cannot be revoked), provides an audit trail of who entered which space and when, and integrates with CCTV for incident investigation. For any office with 10+ staff, the security and administrative benefits justify the cost within 12 months.

**Energy monitoring**
A sub-metered energy monitoring system (Shelly EM, Emporia, or custom CT-clamp installations) gives you real-time visibility of exactly what is consuming power. Most businesses find 15–20% waste they can eliminate on day one of monitoring.

**IP CCTV with cloud backup**
Local DVR/NVR is a liability — it gets stolen alongside the laptops it was supposed to protect. IP cameras with direct cloud upload (Hikvision with Hik-Connect, or ONVIF cameras with a Synology NAS + cloud sync) ensure footage is preserved even if hardware is taken.

## Low ROI: Proceed with Caution

**Voice assistants in offices**: High distraction-to-productivity ratio. The exceptions are reception desks and meeting room controls.

**Smart coffee machines and kitchen appliances**: Interesting in a product demo, irrelevant in an operational budget.

**Automated blinds without solar panels**: The ROI case only closes when combined with a solar installation that benefits from reduced thermal gain.

[Design your smart office with eZeroAndOne →](/capabilities)`,
  },
  {
    title: 'Cloud vs On-Premise: The Right Answer for Nigerian Businesses in 2025',
    summary: 'The cloud vs on-premise debate has a clear answer for most Nigerian businesses in 2025 — but the exceptions matter. A practical framework for the decision.',
    category: 'Infrastructure & Cloud',
    tags: 'cloud vs on-premise Nigeria, cloud migration, hybrid cloud, AWS Nigeria, IT infrastructure',
    featured_image_query: 'cloud computing, data center, server room, technology infrastructure',
    body_md: `In most markets, this debate was settled years ago in cloud's favour. In Nigeria, the calculation is more nuanced — but the conclusion is the same for most businesses.

## Why Cloud Wins for Nigerian SMEs

**No capital expenditure**: A server that costs ₦8 million to procure is ₦8 million you cannot invest in your product, people, or market. Cloud replaces CapEx with OpEx.

**Power independence**: Your on-premise server needs your generator running. Your cloud workloads run on AWS Johannesburg data centres with N+2 power redundancy regardless of what NEPA does.

**Internet dependency is already assumed**: Every modern Nigerian business already depends on internet connectivity for email, collaboration, and banking. Adding cloud applications to that dependency stack has zero incremental risk.

**Security is better**: An AWS or Cloudflare data centre has more physical and cyber security than any server room in a Nigerian office. Your server room does not.

## When On-Premise Makes Sense

**Regulatory data residency requirements**: CBN and NITDA have specific requirements for some categories of financial data. If your regulator requires data to remain in Nigeria and your cloud provider has no Nigerian region, you may need a hybrid approach.

**Ultra-low latency applications**: Latency-sensitive applications (real-time trading, industrial control systems) may require on-premise edge compute.

**Very high data volumes with infrequent access**: Cold archive storage at scale can still be cheaper on-premise for large enterprises.

## The Hybrid Answer

For most Nigerian companies above 50 staff, the right answer is hybrid: cloud-first for all new applications and productivity workloads, on-premise edge compute only where latency or regulatory requirements specifically demand it.

[eZeroAndOne designs cloud architecture for Nigerian businesses →](/capabilities)`,
  },
  {
    title: 'How Much Does a Website Cost in Nigeria? A Transparent Breakdown',
    summary: 'The honest answer to the most-asked question in Nigerian digital services — what websites actually cost, why prices vary so dramatically, and how to evaluate a quote.',
    category: 'Digital & Marketing',
    tags: 'website cost Nigeria, web design pricing, web development Nigeria, how much website Lagos',
    featured_image_query: 'web design, website development, pricing, digital agency',
    body_md: `"How much does a website cost?" is the most-asked question in Nigerian digital services, and it receives the most dishonest answers. Here is the transparent version.

## The Three Categories of Website

### Brochure / Marketing Website
A 5–10 page informational site: home, about, services, blog, contact. Built on WordPress or Webflow.

- **₦150,000 – ₦500,000**: Freelancer or small agency using templates
- **₦500,000 – ₦2,000,000**: Professional agency with custom design
- **₦2,000,000+**: Enterprise-grade with bespoke design system and CMS training

### E-Commerce Website
A WooCommerce, Shopify, or custom store with product catalogue, cart, and payment gateway.

- **₦300,000 – ₦800,000**: Template-based WooCommerce build
- **₦800,000 – ₦3,000,000**: Custom-designed e-commerce with Paystack/Flutterwave
- **₦3,000,000+**: Headless or custom platform with inventory integration

### Web Application
A functional product — bookings, portals, dashboards, SaaS.

- **₦1,500,000 – ₦5,000,000**: MVP application with core features
- **₦5,000,000 – ₦20,000,000**: Full-featured enterprise application
- **₦20,000,000+**: Complex platform with multiple user roles, APIs, and integrations

## What the Price Actually Buys You

The difference between ₦150,000 and ₦1,500,000 for the same brief is not profit margin — it is:

- Custom design vs template
- Performance optimisation vs default WordPress
- SEO-ready architecture vs no metadata
- Ongoing support vs none
- Code ownership vs opaque builders

## How to Evaluate a Quote

1. Is the design custom or a template? (Ask to see the template library)
2. Who hosts it and what are the ongoing costs?
3. Do you own the code and domain?
4. Is there a maintenance retainer and what does it include?
5. What is the handover process?

[Get a transparent quote from eZeroAndOne →](/capabilities)`,
  },
  {
    title: 'The Business Case for IT Managed Services vs In-House IT in Nigeria',
    summary: 'Should your Nigerian business hire an in-house IT team or outsource to a managed service provider? A total-cost-of-ownership comparison with real Nigerian salary data.',
    category: 'IT Consulting & Training',
    tags: 'managed IT services Nigeria, MSP Nigeria, IT outsourcing, in-house IT, IT support cost',
    featured_image_query: 'IT support team, managed services, help desk, technology support',
    body_md: `Most Nigerian businesses above 20 staff face the same question: hire an IT person or outsource to an IT company? Here is the actual maths.

## The True Cost of In-House IT

A single IT support officer in Lagos in 2025 commands:

- **Salary**: ₦1,500,000 – ₦3,600,000 per annum (mid-range: ₦2,400,000)
- **Employer pension contribution**: ₦192,000 (8% minimum)
- **Health insurance**: ₦120,000 – ₦240,000
- **Annual leave (15 days)**: 4.1% of salary = ₦98,400
- **Training and certification**: ₦150,000 – ₦400,000 per year
- **Total annual cost**: ₦2,960,400 – ₦4,332,000

And one person cannot:
- Provide 24/7 support
- Cover all technology domains (network, security, applications, hardware)
- Be available during sick leave or annual leave
- Provide specialist expertise in security, cloud, or automation

## What Managed IT Actually Costs

A comprehensive managed IT support contract for a 20–50 seat Nigerian business typically costs ₦800,000 – ₦2,000,000 per annum, depending on scope and SLA.

For that, you receive:
- A team of engineers across all disciplines
- Defined response SLAs (1-hour critical, 4-hour standard)
- 24/7 monitoring and alerting
- Regular proactive maintenance
- Access to specialist expertise (security, cloud, compliance)
- No HR overhead

## When In-House Makes Sense

- You have more than 200 staff and your IT complexity justifies a full IT department
- You process highly sensitive data with strict access controls requiring on-site presence
- You have dedicated development work requiring full-time engineering resource

For the majority of Nigerian businesses under 200 staff, managed IT is the economically rational choice.

[Get a managed IT proposal from eZeroAndOne →](/capabilities)`,
  },
  {
    title: 'Choosing the Right ERP for a Nigerian Business: A Practical Guide',
    summary: 'Odoo, SAP, Microsoft Dynamics, or Sage — which ERP makes sense for a growing Nigerian company? A no-jargon comparison focused on total cost, implementation time, and local support.',
    category: 'IT Consulting & Training',
    tags: 'ERP Nigeria, Odoo Nigeria, SAP Nigeria, Microsoft Dynamics, enterprise software, business management system',
    featured_image_query: 'ERP software, business management, enterprise, dashboard, analytics',
    body_md: `ERP selection is one of the highest-stakes technology decisions a growing Nigerian business makes. A wrong choice costs years and millions to undo. This guide simplifies it.

## The Shortlist

### Odoo
The most pragmatic choice for Nigerian SMEs. Open-source core with a modular SaaS option. Covers CRM, accounting (IFRS-compliant), inventory, HR, manufacturing, e-commerce, and project management in one platform.

- **Pricing**: Free community edition; Enterprise from $7.25/user/month
- **Implementation time**: 8–24 weeks for full deployment
- **Nigerian consultants**: Growing ecosystem; eZeroAndOne is an Odoo implementation partner
- **Best for**: Manufacturing, distribution, professional services — 10 to 500 users

### SAP Business One / S/4HANA
The gold standard for enterprise. Comprehensive, deeply integrated, and enormously expensive to implement correctly.

- **Pricing**: ₦5M–₦50M+ implementation; significant annual licensing
- **Implementation time**: 6–24 months
- **Best for**: Subsidiaries of multinationals that require SAP group reporting; companies above 500 staff

### Microsoft Dynamics 365
Strong integration with the Microsoft ecosystem (Office 365, Teams, Azure). Mid-market positioning with enterprise ambition.

- **Pricing**: From $65/user/month (Business Central); $95–$210/user/month (full Dynamics 365)
- **Best for**: Companies already standardised on Microsoft; financial services; 50–1,000 users

### Sage 50 / Sage 200
Popular with Nigerian accounting departments. Excellent for finance; limited for broader business operations.

- **Best for**: Finance-first deployments; companies that want accounting software rather than a true ERP

## The Decision Framework

1. **What problem are you solving first?** — Inventory chaos? Finance? HR? Start there.
2. **What is your real budget?** — Include implementation, training, data migration, and 3 years of licensing.
3. **Who supports it locally?** — A system with no local implementors is a risk.
4. **Can it grow with you?** — Odoo's module library covers most scenarios; you will not outgrow it easily.

[Talk to eZeroAndOne about ERP selection and implementation →](/capabilities)`,
  },
];

// ---------------------------------------------------------------------------
// API helpers
// ---------------------------------------------------------------------------

function headers() {
  return { 'Content-Type': 'application/json', Cookie: `session=${SESSION}` };
}

async function createPost(type, data) {
  const res = await fetch(`${BASE_URL}/api/admin/content`, {
    method: 'POST', headers: headers(),
    body: JSON.stringify({ type, ...data }),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`Create failed "${data.title}": ${res.status} ${text.substring(0,200)}`);
  return JSON.parse(text);
}

async function patchPost(id, fields) {
  const res = await fetch(`${BASE_URL}/api/admin/content/${id}`, {
    method: 'PATCH', headers: headers(),
    body: JSON.stringify(fields),
  });
  const text = await res.text();
  if (!res.ok) throw new Error(`Patch failed ${id}: ${res.status} ${text.substring(0,200)}`);
  return JSON.parse(text);
}

async function verifyImage(url) {
  try {
    const r = await fetch(url, { method: 'HEAD', redirect: 'follow' });
    return r.ok && (r.headers.get('content-type') ?? '').startsWith('image/');
  } catch { return false; }
}

async function getFeaturedImage(query) {
  const url = `https://source.unsplash.com/1280x720/?${encodeURIComponent(query)}`;
  const ok = await verifyImage(url);
  return ok ? url : null;
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

async function run(label, items, type, buildPayload) {
  console.log(`\n📦  ${label} (${items.length} items)\n`);
  let ok = 0;
  for (const item of items) {
    process.stdout.write(`  ⏳  ${item.title.substring(0,55)}… `);
    try {
      const post = await createPost(type, buildPayload(item));
      const imgUrl = await getFeaturedImage(item.featured_image_query ?? item.tags);
      await patchPost(post.id, {
        published: true,
        published_at: NOW,
        ...(imgUrl ? { featured_image_url: imgUrl } : {}),
      });
      console.log(imgUrl ? '✅  (image)' : '✅  (no image)');
      ok++;
    } catch (err) {
      console.log(`❌  ${err.message}`);
    }
    await new Promise(r => setTimeout(r, 1200));
  }
  console.log(`\n  ${ok}/${items.length} published.\n`);
}

async function main() {
  console.log(`\n🚀  Populating capabilities + insights at ${BASE_URL}`);

  await run('Capabilities', CAPABILITIES, 'capability', c => ({
    title: c.title,
    summary: c.summary,
    category: c.category,
    tags: c.tags,
    material_icon: c.material_icon,
    body_md: c.body_md,
    featured_image_query: c.tags,
  }));

  await run('Insights', INSIGHTS, 'insight', i => ({
    title: i.title,
    summary: i.summary,
    category: i.category,
    tags: i.tags,
    body_md: i.body_md,
    featured_image_query: i.featured_image_query,
  }));
}

main().catch(err => { console.error(err); process.exit(1); });
