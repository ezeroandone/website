<script lang="ts">
  import type { PageData } from './$types';
  import GlassCard from '$lib/components/GlassCard.svelte';
  import { renderMarkdown } from '$lib/utils/markdown';

  let { data }: { data: PageData } = $props();

  const descriptionHtml = $derived(renderMarkdown(data.career.description_md));

  // ── Brand copy — lifecycle state messages ────────────────────────────────
  const LIFECYCLE_COPY = {
    applied:    'Submission Received. Your technical parameters are undergoing initial integrity parsing.',
    probation:  'Onboarding Phase Active. Assigned internal domain credential provisioning initialized.',
    confirmed:  'Tenure Confirmed. Full administrative rights, project keys, and organizational permissions unlocked.',
  } as const;

  // ── Sample job posting — shown when no description_md is authored ────────
  const SAMPLE_JOB = {
    role: 'Full-Stack Edge Engineer',
    department: 'Core Infrastructure',
    type: 'Full-Time' as const,
    intro: `We are looking for a Full-Stack Edge Engineer who operates at the boundary between
system design and implementation — someone who can architect a cryptographic auth pipeline in
the morning and ship a responsive, accessible UI component in the afternoon. This role exists
at the convergence of typed systems, reactive interfaces, and atomic data primitives.`,
    responsibilities: [
      'Design and implement Cloudflare Workers in Rust compiled to WebAssembly, handling auth flows, content APIs, and file storage pipelines.',
      'Build reactive SvelteKit frontends with server-side rendering, typed API contracts, and progressive enhancement.',
      'Own D1 schema design — writing migrations, defining constraint sets, and optimising query paths for sub-10 ms response targets.',
      'Integrate external APIs (payment gateways, travel inventory systems, corporate registries) behind versioned, fault-tolerant adapter layers.',
      'Participate in architecture reviews, contributing structured proposals backed by first-principles reasoning rather than convention.',
      'Maintain security posture across the stack — cryptographic session management, input validation, MIME-type enforcement, and presigned URL scoping.',
    ],
    requirements: [
      'Proficiency in at least one typed systems language (Rust, TypeScript, Go) with demonstrable production experience.',
      'Strong command of reactive frontend styling — CSS custom properties, responsive grid systems, animation performance, and accessibility compliance.',
      'Experience with atomic database configurations: schema migrations, foreign key constraints, index strategy, and transactional integrity.',
      'Familiarity with edge computing paradigms — Workers, serverless functions, CDN cache models, and distributed state management.',
      'Comfort operating in an environment where architecture is derived from fundamentals, not frameworks.',
    ],
    niceToHave: [
      'Prior experience compiling Rust or C to WebAssembly for production deployment.',
      'Contributions to open-source cryptographic or systems libraries.',
      'Understanding of JWT internals, HMAC signature schemes, and session cookie security attributes.',
    ],
  } as const;

  const hasDescription = data.career.description_md && data.career.description_md.trim().length > 0;

  // ── Application form state ────────────────────────────────────────────────
  type FormStatus = 'idle' | 'submitting' | 'success' | 'error';
  let formStatus = $state<FormStatus>('idle');
  let formError = $state<string>('');
  let applicationId = $state<string>('');

  // Form fields
  let applicantName = $state('');
  let applicantEmail = $state('');
  let coverLetter = $state('');

  async function handleSubmitApplication(e: Event) {
    e.preventDefault();
    formStatus = 'submitting';
    formError = '';

    try {
      const res = await fetch(`/api/careers/${data.career.slug}/apply`, {
        method: 'POST',
        credentials: 'include',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          applicantName,
          applicantEmail,
          coverLetter,
        }),
      });

      if (!res.ok) {
        const body = await res.text();
        throw new Error(body || `Error ${res.status}`);
      }

      const result = await res.json();
      applicationId = result.id ?? result.application_id ?? '';
      formStatus = 'success';
    } catch (err) {
      formError = err instanceof Error ? err.message : 'Submission failed. Please try again.';
      formStatus = 'error';
    }
  }

  // ── Document upload state ─────────────────────────────────────────────────
  const MAX_FILES = 3;
  const MAX_SIZE_BYTES = 10 * 1024 * 1024; // 10 MB
  const ALLOWED_TYPES = [
    'application/pdf',
    'application/msword',
    'application/vnd.openxmlformats-officedocument.wordprocessingml.document',
  ];
  const ALLOWED_LABEL = 'PDF, DOC, DOCX';

  let uploadFiles = $state<File[]>([]);
  let uploadStatus = $state<'idle' | 'uploading' | 'done' | 'error'>('idle');
  let uploadError = $state('');
  let uploadedCount = $state(0);

  function handleFileChange(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    uploadError = '';
    const selected = Array.from(input.files ?? []);

    const invalid = selected.filter(
      (f) => !ALLOWED_TYPES.includes(f.type) || f.size > MAX_SIZE_BYTES
    );

    if (invalid.length > 0) {
      uploadError = `Some files are invalid. Only ${ALLOWED_LABEL} up to 10 MB each are accepted.`;
      input.value = '';
      return;
    }

    if (selected.length > MAX_FILES) {
      uploadError = `You can upload at most ${MAX_FILES} documents.`;
      input.value = '';
      return;
    }

    uploadFiles = selected;
  }

  async function handleUploadDocuments(e: Event) {
    e.preventDefault();
    if (!applicationId || uploadFiles.length === 0) return;

    uploadStatus = 'uploading';
    uploadError = '';
    uploadedCount = 0;

    for (const file of uploadFiles) {
      const formData = new FormData();
      formData.append('file', file);

      const res = await fetch(`/api/careers/${data.career.slug}/apply/documents`, {
        method: 'POST',
        credentials: 'include',
        body: formData,
        // NOTE: the Worker matches the application via the session or via a
        // query param; pass the application_id so the Worker can associate it.
        headers: {
          'X-Application-Id': applicationId,
        },
      });

      if (!res.ok) {
        const body = await res.text();
        uploadError = body || `Upload failed for "${file.name}"`;
        uploadStatus = 'error';
        return;
      }
      uploadedCount += 1;
    }

    uploadStatus = 'done';
  }

  function formatDate(unixSeconds: number): string {
    return new Date(unixSeconds * 1000).toLocaleDateString('en-US', {
      year: 'numeric',
      month: 'long',
      day: 'numeric',
    });
  }
</script>

<svelte:head>
  <title>{data.career.title} — Active Operations — eZeroAndOne</title>
  <meta
    name="description"
    content="{data.career.title} — {data.career.type} position in {data.career.department} at eZeroAndOne. Join the core team building lasting systems."
  />
</svelte:head>

<main class="career-detail-page">
  <a href="/careers" class="back-link">← Active Operations</a>

  <!-- Job detail card -->
  <GlassCard accentColor="blue" class="job-card">
    <div class="job-header">
      <div class="job-badges">
        <span class="badge badge-type">{data.career.type}</span>
        {#if data.career.department}
          <span class="badge badge-dept">{data.career.department}</span>
        {/if}
      </div>
      <h1 class="job-title">{data.career.title}</h1>
      <p class="job-date">Posted {formatDate(data.career.created_at)}</p>
    </div>

    {#if hasDescription}
      <div class="job-body">
        {@html descriptionHtml}
      </div>
    {:else}
      <!-- Sample Full-Stack Edge Engineer posting -->
      <div class="job-body sample-job">
        <p class="job-intro">{SAMPLE_JOB.intro}</p>

        <h2>Responsibilities</h2>
        <ul>
          {#each SAMPLE_JOB.responsibilities as item}
            <li>{item}</li>
          {/each}
        </ul>

        <h2>Requirements</h2>
        <ul>
          {#each SAMPLE_JOB.requirements as item}
            <li>{item}</li>
          {/each}
        </ul>

        <h2>Nice to Have</h2>
        <ul>
          {#each SAMPLE_JOB.niceToHave as item}
            <li>{item}</li>
          {/each}
        </ul>
      </div>
    {/if}
  </GlassCard>

  <!-- Application form -->
  {#if formStatus === 'success'}
    <section class="section-spacing">
      <GlassCard accentColor="green">
        <div class="success-message">
          <span class="success-icon" aria-hidden="true">✓</span>
          <div>
            <h2>Application Transmitted.</h2>
            <p>{LIFECYCLE_COPY.applied}</p>
            <p class="success-detail">
              A confirmation will be dispatched to <strong>{applicantEmail}</strong> once
              initial parsing is complete.
            </p>
          </div>
        </div>
      </GlassCard>

      <!-- Optional document upload section, shown after a successful application -->
      <div class="section-spacing">
        <GlassCard accentColor="yellow">
          <div class="upload-section">
            <h2>Attach Supporting Documents</h2>
            <p class="upload-hint">
              Transmit your CV, portfolio, or certification records ({ALLOWED_LABEL}, max 10 MB
              each, up to {MAX_FILES} files). Documents are stored in a private, access-controlled
              vault and are only accessible to authorized administrators.
            </p>

            {#if uploadStatus === 'done'}
              <p class="upload-done" role="status">
                ✓ {uploadedCount} document{uploadedCount !== 1 ? 's' : ''} uploaded successfully.
              </p>
            {:else}
              <form onsubmit={handleUploadDocuments} novalidate>
                <label class="file-label" for="doc-upload">
                  Choose files
                  <input
                    id="doc-upload"
                    type="file"
                    accept=".pdf,.doc,.docx,application/pdf,application/msword,application/vnd.openxmlformats-officedocument.wordprocessingml.document"
                    multiple
                    onchange={handleFileChange}
                    class="file-input"
                    aria-describedby="file-hint"
                  />
                </label>
                <p id="file-hint" class="upload-hint">
                  {#if uploadFiles.length > 0}
                    {uploadFiles.length} file{uploadFiles.length !== 1 ? 's' : ''} selected:
                    {uploadFiles.map((f) => f.name).join(', ')}
                  {:else}
                    No files selected
                  {/if}
                </p>

                {#if uploadError}
                  <p class="field-error" role="alert">{uploadError}</p>
                {/if}

                <button
                  type="submit"
                  class="btn btn-secondary"
                  disabled={uploadFiles.length === 0 || uploadStatus === 'uploading'}
                  aria-disabled={uploadFiles.length === 0 || uploadStatus === 'uploading'}
                >
                  {uploadStatus === 'uploading' ? 'Uploading…' : 'Upload documents'}
                </button>
              </form>
            {/if}
          </div>
        </GlassCard>
      </div>
    </section>
  {:else}
    <!-- Main application form -->
    <section class="section-spacing">
      <GlassCard accentColor="blue">
        <div class="form-section">
          <h2>Initialize Your Application</h2>

          <form onsubmit={handleSubmitApplication} novalidate>
            <!-- Applicant name -->
            <div class="form-field">
              <label for="applicant-name">Full name <span class="required" aria-hidden="true">*</span></label>
              <input
                id="applicant-name"
                type="text"
                bind:value={applicantName}
                required
                placeholder="Jane Smith"
                autocomplete="name"
                class="form-input"
                aria-required="true"
              />
            </div>

            <!-- Applicant email -->
            <div class="form-field">
              <label for="applicant-email">Email address <span class="required" aria-hidden="true">*</span></label>
              <input
                id="applicant-email"
                type="email"
                bind:value={applicantEmail}
                required
                placeholder="jane@example.com"
                autocomplete="email"
                class="form-input"
                aria-required="true"
              />
            </div>

            <!-- Cover letter -->
            <div class="form-field">
              <label for="cover-letter">Cover letter <span class="required" aria-hidden="true">*</span></label>
              <textarea
                id="cover-letter"
                bind:value={coverLetter}
                required
                rows={8}
                placeholder="Tell us why you're interested in this role and what makes you a great fit…"
                class="form-input form-textarea"
                aria-required="true"
              ></textarea>
            </div>

            {#if formStatus === 'error' && formError}
              <p class="field-error" role="alert">{formError}</p>
            {/if}

            <button
              type="submit"
              class="btn btn-primary"
              disabled={formStatus === 'submitting'}
              aria-disabled={formStatus === 'submitting'}
            >
              {formStatus === 'submitting' ? 'Submitting…' : 'Submit application'}
            </button>
          </form>
        </div>
      </GlassCard>
    </section>
  {/if}
</main>

<style>
  .career-detail-page {
    max-width: 800px;
    margin: 0 auto;
    padding: 2rem 1rem;
  }

  .back-link {
    display: inline-block;
    margin-bottom: 2rem;
    font-size: 0.875rem;
    color: var(--accent-blue);
    text-decoration: none;
    transition: color 0.2s ease;
  }

  .back-link:hover {
    color: var(--accent-green);
    text-decoration: underline;
  }

  .section-spacing {
    margin-top: 2rem;
  }

  /* Job header */
  .job-header {
    padding: 2rem 2rem 1.5rem;
    border-bottom: 1px solid var(--glass-border);
  }

  .job-badges {
    display: flex;
    gap: 0.5rem;
    margin-bottom: 0.75rem;
    flex-wrap: wrap;
  }

  .badge {
    font-size: 0.7rem;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.05em;
    padding: 0.2rem 0.6rem;
    border-radius: 999px;
    border: 1px solid currentColor;
  }

  .badge-type { color: var(--accent-blue); }
  .badge-dept { color: var(--color-text-muted); border-color: var(--color-border); }

  .job-title {
    font-size: 2rem;
    font-weight: 700;
    margin: 0 0 0.5rem 0;
    color: var(--color-text-primary);
  }

  .job-date {
    font-size: 0.875rem;
    color: var(--color-text-muted);
    margin: 0;
  }

  .job-body {
    padding: 2rem;
    font-size: 1rem;
    line-height: 1.8;
    color: var(--color-text-secondary);
  }

  .job-body :global(h2) {
    font-size: 1.5rem;
    color: var(--color-text-primary);
    margin-top: 2rem;
    margin-bottom: 0.75rem;
  }

  .job-body :global(h3) {
    font-size: 1.25rem;
    color: var(--color-text-primary);
    margin-top: 1.5rem;
    margin-bottom: 0.5rem;
  }

  .job-body :global(ul),
  .job-body :global(ol) {
    padding-left: 1.5rem;
    margin-bottom: 1rem;
  }

  .job-body :global(li) {
    margin-bottom: 0.4rem;
  }

  .job-body :global(p) {
    margin-bottom: 1rem;
  }

  /* Sample job posting */
  .job-intro {
    font-size: 1rem;
    line-height: 1.8;
    color: var(--color-text-secondary);
    border-left: 3px solid var(--accent-blue);
    padding-left: 1.25rem;
    margin-bottom: 2rem;
    font-style: italic;
    white-space: pre-line;
  }

  .sample-job h2 {
    font-size: 0.75rem;
    font-weight: 700;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: var(--accent-blue);
    margin-top: 2rem;
    margin-bottom: 0.75rem;
  }

  .sample-job ul {
    padding-left: 1.25rem;
    margin: 0 0 1rem;
  }

  .sample-job li {
    font-size: 0.9rem;
    line-height: 1.7;
    color: var(--color-text-secondary);
    margin-bottom: 0.5rem;
  }

  .success-detail {
    font-size: 0.875rem;
    color: var(--color-text-secondary);
    margin-top: 0.5rem;
  }

  /* Success */
  .success-message {
    display: flex;
    align-items: flex-start;
    gap: 1.25rem;
    padding: 2rem;
  }

  .success-icon {
    font-size: 2rem;
    color: var(--accent-green);
    flex-shrink: 0;
  }

  .success-message h2 {
    font-size: 1.5rem;
    margin: 0 0 0.5rem 0;
    color: var(--color-text-primary);
  }

  .success-message p {
    margin: 0;
    color: var(--color-text-secondary);
    line-height: 1.6;
  }

  /* Application form */
  .form-section {
    padding: 2rem;
  }

  .form-section h2 {
    font-size: 1.5rem;
    font-weight: 600;
    margin: 0 0 1.5rem 0;
    color: var(--color-text-primary);
  }

  form {
    display: flex;
    flex-direction: column;
    gap: 1.25rem;
  }

  .form-field {
    display: flex;
    flex-direction: column;
    gap: 0.4rem;
  }

  label {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text-secondary);
  }

  .required {
    color: var(--accent-red);
    margin-left: 0.1rem;
  }

  .form-input {
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-strong);
    border-radius: 8px;
    padding: 0.6rem 0.9rem;
    font-size: 1rem;
    color: var(--color-text-primary);
    outline: none;
    transition: border-color 0.2s ease, box-shadow 0.2s ease;
    font-family: inherit;
    width: 100%;
  }

  .form-input:focus {
    border-color: var(--accent-blue);
    box-shadow: 0 0 0 3px rgba(0, 212, 255, 0.15);
  }

  .form-textarea {
    resize: vertical;
    min-height: 160px;
  }

  .field-error {
    font-size: 0.875rem;
    color: var(--accent-red);
    margin: 0;
  }

  /* Buttons */
  .btn {
    padding: 0.75rem 1.75rem;
    border-radius: 8px;
    font-size: 1rem;
    font-weight: 600;
    cursor: pointer;
    border: none;
    transition: opacity 0.2s ease, transform 0.1s ease;
    align-self: flex-start;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn:not(:disabled):active {
    transform: scale(0.98);
  }

  .btn-primary {
    background: var(--accent-blue);
    color: #fff;
  }

  .btn-primary:not(:disabled):hover {
    opacity: 0.9;
  }

  .btn-secondary {
    background: var(--color-bg-elevated);
    color: var(--accent-yellow);
    border: 1px solid var(--accent-yellow);
  }

  .btn-secondary:not(:disabled):hover {
    background: var(--accent-yellow);
    color: #000;
  }

  /* Document upload */
  .upload-section {
    padding: 2rem;
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }

  .upload-section h2 {
    font-size: 1.25rem;
    font-weight: 600;
    margin: 0;
    color: var(--color-text-primary);
  }

  .upload-hint {
    font-size: 0.875rem;
    color: var(--color-text-muted);
    margin: 0;
    line-height: 1.5;
  }

  .file-label {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    padding: 0.6rem 1.25rem;
    background: var(--color-bg-elevated);
    border: 1px solid var(--color-border-strong);
    border-radius: 8px;
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--color-text-secondary);
    cursor: pointer;
    transition: border-color 0.2s ease;
    align-self: flex-start;
  }

  .file-label:hover {
    border-color: var(--accent-yellow);
    color: var(--accent-yellow);
  }

  .file-input {
    position: absolute;
    width: 1px;
    height: 1px;
    padding: 0;
    margin: -1px;
    overflow: hidden;
    clip: rect(0, 0, 0, 0);
    white-space: nowrap;
    border: 0;
  }

  .upload-done {
    font-size: 0.9rem;
    color: var(--accent-green);
    font-weight: 500;
  }
</style>
