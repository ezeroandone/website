/**
 * Theme store unit tests — task 22.5
 * Validates: Requirements 16.2, 16.3
 */
import { describe, it, expect, beforeEach, vi } from 'vitest';

// ── Mocks must be set up before importing the module under test ────────────
// Mock localStorage in jsdom
const localStorageMock: Record<string, string> = {};
Object.defineProperty(globalThis, 'localStorage', {
  value: {
    getItem: (key: string) => localStorageMock[key] ?? null,
    setItem: (key: string, val: string) => { localStorageMock[key] = val; },
    removeItem: (key: string) => { delete localStorageMock[key]; },
    clear: () => { for (const k in localStorageMock) delete localStorageMock[k]; }
  },
  writable: true
});

// ── Tests ──────────────────────────────────────────────────────────────────

describe('theme store', () => {
  beforeEach(() => {
    // Reset localStorage and html attribute before each test
    localStorage.clear();
    document.documentElement.removeAttribute('data-theme');
    // Re-import freshly to pick up the cleared localStorage
    vi.resetModules();
  });

  it('defaults to dark when no stored preference exists', async () => {
    const { theme } = await import('./theme.js');
    let currentTheme = '';
    const unsub = theme.subscribe((t) => { currentTheme = t; });
    expect(currentTheme).toBe('dark');
    unsub();
  });

  it('restores stored preference from localStorage', async () => {
    localStorage.setItem('ezo-theme', 'light');
    const { theme } = await import('./theme.js');
    let currentTheme = '';
    const unsub = theme.subscribe((t) => { currentTheme = t; });
    expect(currentTheme).toBe('light');
    unsub();
  });

  it('persists theme change to localStorage', async () => {
    const { theme } = await import('./theme.js');
    theme.set('light');
    expect(localStorage.getItem('ezo-theme')).toBe('light');
  });

  it('sets data-theme attribute on document.documentElement when theme changes', async () => {
    const { theme } = await import('./theme.js');
    theme.set('light');
    expect(document.documentElement.getAttribute('data-theme')).toBe('light');
    theme.set('dark');
    expect(document.documentElement.getAttribute('data-theme')).toBe('dark');
  });

  it('toggleTheme switches from dark to light', async () => {
    const { theme, toggleTheme } = await import('./theme.js');
    // ensure starting point is dark
    theme.set('dark');
    toggleTheme();
    let currentTheme = '';
    const unsub = theme.subscribe((t) => { currentTheme = t; });
    expect(currentTheme).toBe('light');
    unsub();
  });

  it('toggleTheme switches from light to dark', async () => {
    const { theme, toggleTheme } = await import('./theme.js');
    theme.set('light');
    toggleTheme();
    let currentTheme = '';
    const unsub = theme.subscribe((t) => { currentTheme = t; });
    expect(currentTheme).toBe('dark');
    unsub();
  });
});
