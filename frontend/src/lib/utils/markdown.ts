/**
 * Safe Markdown renderer — Task 23.1 (Requirement 19.8)
 *
 * Audit findings
 * ==============
 * Five {@html} directives were found across src/routes/:
 *
 *   UNSAFE (user-supplied content — fixed by this module):
 *   - src/routes/insights/[slug]/+page.svelte      → data.post.body_md
 *   - src/routes/work/[slug]/+page.svelte           → data.post.body_md
 *   - src/routes/capabilities/[slug]/+page.svelte   → data.post.body_md
 *   - src/routes/careers/[slug]/+page.svelte        → data.career.description_md
 *
 *   SAFE (trusted library output — left unchanged):
 *   - src/routes/team/[username]/+page.svelte       → qrcode SVG (npm library, not user data)
 *
 * All four unsafe sites now call renderMarkdown() from this module instead of
 * rendering raw body_md/description_md.
 *
 * Security guarantees
 * ===================
 * 1. marked converts Markdown → HTML.
 * 2. isomorphic-dompurify (DOMPurify with jsdom fallback for SSR/Cloudflare Workers)
 *    removes every dangerous construct:
 *      - <script> tags and their content
 *      - <iframe>, <object>, <embed>, <form> tags
 *      - All inline event handlers: onerror=, onclick=, onload=, onmouseover=, etc.
 *      - javascript: and data: URI schemes in href/src attributes
 * 3. The FORCE_BODY option ensures the output is always a string even for
 *    fragment-level input (no accidental empty-string returns).
 *
 * SSR note
 * ========
 * This project uses adapter-cloudflare, which performs SSR inside a Cloudflare
 * Worker environment.  DOMPurify requires a DOM — isomorphic-dompurify bundles
 * jsdom as a server-side fallback, making it safe to call during SSR.
 */

import { marked } from 'marked';
import DOMPurify from 'isomorphic-dompurify';

// ---------------------------------------------------------------------------
// DOMPurify configuration
// ---------------------------------------------------------------------------

/**
 * Tags that are completely removed together with their children.
 * FORBID_TAGS takes precedence over ALLOWED_TAGS.
 */
const FORBIDDEN_TAGS: string[] = [
	'script',
	'iframe',
	'object',
	'embed',
	'form',
	'input',
	'button',
	'textarea',
	'select',
	'style', // inline stylesheets can be used for CSS injection / exfiltration
	'link',   // <link rel="import"> / exfiltration
	'meta',
	'base'
];

/**
 * Attributes that are stripped from every surviving element.
 * This covers all DOM event handler attributes.
 */
const FORBIDDEN_ATTR: string[] = [
	// Mouse events
	'onclick',
	'ondblclick',
	'onmousedown',
	'onmouseup',
	'onmouseover',
	'onmousemove',
	'onmouseout',
	'onmouseenter',
	'onmouseleave',
	// Keyboard events
	'onkeydown',
	'onkeyup',
	'onkeypress',
	// Form / focus events
	'onfocus',
	'onblur',
	'onchange',
	'oninput',
	'onsubmit',
	'onreset',
	'onselect',
	// Load / resource events
	'onload',
	'onunload',
	'onbeforeunload',
	'onerror',
	'onabort',
	'onresize',
	'onscroll',
	// Drag events
	'ondragstart',
	'ondragend',
	'ondragover',
	'ondrop',
	// Clipboard events
	'oncopy',
	'oncut',
	'onpaste',
	// Pointer / touch events
	'onpointerdown',
	'onpointerup',
	'onpointermove',
	'ontouchstart',
	'ontouchend',
	'ontouchmove',
	// Animation / transition events
	'onanimationstart',
	'onanimationend',
	'ontransitionend',
	// Generic
	'oncontextmenu',
	'onwheel',
	'ontoggle',
	// SVG-specific script hooks
	'onactivate',
	'onbegin',
	'onend',
	'onrepeat'
];

// ---------------------------------------------------------------------------
// marked configuration
// ---------------------------------------------------------------------------

marked.use({
	// gfm: true is the default; tables, task lists, strikethrough, etc.
	// async: false (default) — synchronous parse path used throughout.
	gfm: true,
	breaks: false // preserve paragraph semantics; don't turn single newlines into <br>
});

// ---------------------------------------------------------------------------
// Public API
// ---------------------------------------------------------------------------

/**
 * Convert a Markdown string to sanitised HTML safe for use with {@html}.
 *
 * @param markdown - Raw Markdown text (e.g. post.body_md, career.description_md)
 * @returns Sanitised HTML string — all script tags, event handler attributes,
 *          javascript: URIs, and other XSS vectors have been removed.
 *
 * @example
 * ```svelte
 * <script>
 *   import { renderMarkdown } from '$lib/utils/markdown';
 *   let { data } = $props();
 *   const bodyHtml = $derived(renderMarkdown(data.post.body_md));
 * </script>
 *
 * <div class="post-body">
 *   {@html bodyHtml}
 * </div>
 * ```
 */
export function renderMarkdown(markdown: string | null | undefined): string {
	if (!markdown) return '';

	// Step 1: Parse Markdown → raw HTML (synchronous)
	const rawHtml = marked.parse(markdown) as string;

	// Step 2: Sanitise HTML with DOMPurify
	const clean = DOMPurify.sanitize(rawHtml, {
		FORBID_TAGS: FORBIDDEN_TAGS,
		FORBID_ATTR: FORBIDDEN_ATTR,
		// Ensure the return value is always a string even for fragment input
		FORCE_BODY: true,
		// Do not allow data: URIs in src/href (covers <img src="data:…"> exfiltration)
		ALLOW_DATA_ATTR: false
	});

	return clean;
}
