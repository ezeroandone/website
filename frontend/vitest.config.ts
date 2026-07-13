/**
 * Separate Vitest config for unit tests.
 *
 * Our test files are pure TypeScript (no .svelte imports) so we don't need
 * the svelte plugin at all for the test runner. The svelte plugin v6 is
 * incompatible with Vite 5 (bundled inside Vitest 2.1.9) because it uses
 * the Vite 6 `server.environments` API.
 */
import { defineConfig } from 'vitest/config';
import { resolve } from 'path';

export default defineConfig({
  resolve: {
    alias: {
      '$lib': resolve(__dirname, 'src/lib'),
      '$app/environment': resolve(__dirname, 'src/lib/test-mocks/app-environment.ts'),
      '$app/stores': resolve(__dirname, 'src/lib/test-mocks/app-stores.ts'),
      // Resolve svelte/store without the svelte plugin (which needs Vite 6)
      'svelte/store': resolve(__dirname, 'node_modules/svelte/src/store/index-client.js')
    }
  },
  test: {
    include: ['src/**/*.{test,spec}.{js,ts}'],
    environment: 'jsdom',
    globals: true,
    setupFiles: []
  }
});
