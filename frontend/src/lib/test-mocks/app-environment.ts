/**
 * Mock for $app/environment in the Vitest environment.
 * In jsdom, `browser` should be true so the theme store exercises the
 * localStorage/document paths.
 */
export const browser = true;
export const building = false;
export const dev = true;
export const version = 'test';
