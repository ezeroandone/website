import { writable } from 'svelte/store';
import { browser } from '$app/environment';

type Theme = 'dark' | 'light';

const stored = browser
  ? (localStorage.getItem('ezo-theme') as Theme) ?? 'dark'
  : 'dark';

export const theme = writable<Theme>(stored);

theme.subscribe((val) => {
  if (browser) {
    localStorage.setItem('ezo-theme', val);
    document.documentElement.setAttribute('data-theme', val);
  }
});

export function toggleTheme(): void {
  theme.update((t) => (t === 'dark' ? 'light' : 'dark'));
}
