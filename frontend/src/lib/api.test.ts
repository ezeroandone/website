/**
 * fast-check property tests for frontend API deserialisation — task 22.6
 *
 * Validates: Requirements 20.4
 *
 * For arbitrary objects matching the TypeScript interfaces, serialise to JSON
 * and parse back; assert no fields are dropped or type-coerced.
 */
import { describe, it, expect } from 'vitest';
import fc from 'fast-check';
import type { Post, StaffPublicProfile, Career, IdentityResponse } from './api.js';

// ── Arbitraries ─────────────────────────────────────────────────────────────

const arbStaffPublicProfile = (): fc.Arbitrary<StaffPublicProfile> =>
  fc.record({
    username: fc.string({ minLength: 1, maxLength: 64 }),
    name: fc.string({ minLength: 1, maxLength: 128 }),
    job_title: fc.string({ minLength: 0, maxLength: 128 }),
    bio: fc.string({ minLength: 0, maxLength: 1024 }),
    avatar_url: fc.string({ minLength: 0, maxLength: 512 })
  });

const arbPost = (): fc.Arbitrary<Post> =>
  fc.record({
    id: fc.hexaString({ minLength: 32, maxLength: 32 }),
    type: fc.constantFrom<'insight' | 'work' | 'capability'>('insight', 'work', 'capability'),
    slug: fc.stringMatching(/^[a-z0-9-]{1,80}$/),
    title: fc.string({ minLength: 1, maxLength: 256 }),
    summary: fc.string({ minLength: 0, maxLength: 512 }),
    body_md: fc.string({ minLength: 0, maxLength: 4096 }),
    author_id: fc.option(fc.hexaString({ minLength: 32, maxLength: 32 }), { nil: null }),
    published_at: fc.option(fc.integer({ min: 0, max: 9999999999 }), { nil: null }),
    updated_at: fc.integer({ min: 0, max: 9999999999 }),
    published: fc.boolean()
  });

const arbCareer = (): fc.Arbitrary<Career> =>
  fc.record({
    id: fc.hexaString({ minLength: 32, maxLength: 32 }),
    slug: fc.stringMatching(/^[a-z0-9-]{1,80}$/),
    title: fc.string({ minLength: 1, maxLength: 256 }),
    description_md: fc.string({ minLength: 0, maxLength: 4096 }),
    department: fc.string({ minLength: 0, maxLength: 128 }),
    type: fc.constantFrom<'Full-Time' | 'Part-Time' | 'Contract' | 'Internship'>(
      'Full-Time', 'Part-Time', 'Contract', 'Internship'
    ),
    active: fc.boolean(),
    created_at: fc.integer({ min: 0, max: 9999999999 })
  });

const arbIdentityResponse = (): fc.Arbitrary<IdentityResponse> =>
  fc.record({
    name: fc.string({ minLength: 1, maxLength: 128 }),
    photo_url: fc.string({ minLength: 0, maxLength: 512 }),
    identity_status: fc.string({ minLength: 1, maxLength: 64 }),
    clearance_level: fc.integer({ min: 1, max: 4 }),
    verified_at: fc.integer({ min: 0, max: 9999999999 })
  });

// ── Helper ───────────────────────────────────────────────────────────────────

function jsonRoundTrip<T>(val: T): T {
  return JSON.parse(JSON.stringify(val)) as T;
}

// ── Tests ────────────────────────────────────────────────────────────────────

describe('API deserialisation property tests', () => {
  it('StaffPublicProfile: JSON round-trip preserves all fields', () => {
    fc.assert(
      fc.property(arbStaffPublicProfile(), (profile) => {
        const rt = jsonRoundTrip(profile);
        expect(rt.username).toBe(profile.username);
        expect(rt.name).toBe(profile.name);
        expect(rt.job_title).toBe(profile.job_title);
        expect(rt.bio).toBe(profile.bio);
        expect(rt.avatar_url).toBe(profile.avatar_url);
        // no extra fields
        expect(Object.keys(rt)).toEqual(Object.keys(profile));
      }),
      { numRuns: 200 }
    );
  });

  it('Post: JSON round-trip preserves all fields and types', () => {
    fc.assert(
      fc.property(arbPost(), (post) => {
        const rt = jsonRoundTrip(post);
        expect(rt.id).toBe(post.id);
        expect(rt.type).toBe(post.type);
        expect(rt.slug).toBe(post.slug);
        expect(rt.title).toBe(post.title);
        expect(rt.summary).toBe(post.summary);
        expect(rt.body_md).toBe(post.body_md);
        expect(rt.author_id).toBe(post.author_id);
        // Timestamps stay as numbers (Requirement 20.3)
        if (post.published_at !== null) {
          expect(typeof rt.published_at).toBe('number');
        }
        expect(typeof rt.updated_at).toBe('number');
        expect(rt.published).toBe(post.published);
      }),
      { numRuns: 200 }
    );
  });

  it('Post: timestamp fields serialise as JSON numbers, not strings', () => {
    fc.assert(
      fc.property(
        fc.integer({ min: 0, max: 9999999999 }),
        (ts) => {
          const post: Partial<Post> = { updated_at: ts };
          const json = JSON.stringify(post);
          const parsed = JSON.parse(json) as Partial<Post>;
          expect(typeof parsed.updated_at).toBe('number');
          // Must not be coerced to a string by JSON round-trip
          expect(parsed.updated_at).toBe(ts);
        }
      ),
      { numRuns: 500 }
    );
  });

  it('Career: JSON round-trip preserves all fields and types', () => {
    fc.assert(
      fc.property(arbCareer(), (career) => {
        const rt = jsonRoundTrip(career);
        expect(rt.id).toBe(career.id);
        expect(rt.slug).toBe(career.slug);
        expect(rt.title).toBe(career.title);
        expect(rt.description_md).toBe(career.description_md);
        expect(rt.department).toBe(career.department);
        expect(rt.type).toBe(career.type);
        expect(rt.active).toBe(career.active);
        expect(typeof rt.created_at).toBe('number');
        expect(rt.created_at).toBe(career.created_at);
      }),
      { numRuns: 200 }
    );
  });

  it('IdentityResponse: JSON round-trip preserves all fields', () => {
    fc.assert(
      fc.property(arbIdentityResponse(), (resp) => {
        const rt = jsonRoundTrip(resp);
        expect(rt.name).toBe(resp.name);
        expect(rt.photo_url).toBe(resp.photo_url);
        expect(rt.identity_status).toBe(resp.identity_status);
        expect(rt.clearance_level).toBe(resp.clearance_level);
        expect(typeof rt.verified_at).toBe('number');
        expect(rt.verified_at).toBe(resp.verified_at);
      }),
      { numRuns: 200 }
    );
  });
});
