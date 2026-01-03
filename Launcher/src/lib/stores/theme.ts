import { writable } from 'svelte/store';

export type Theme = 'light' | 'dark' | 'system';

function createThemeStore() {
  // Load saved theme or default to system
  const savedTheme = typeof window !== 'undefined' 
    ? (localStorage.getItem('chrono-theme') as Theme) || 'system'
    : 'system';
  
  const { subscribe, set, update } = writable<Theme>(savedTheme);

  return {
    subscribe,
    set: (value: Theme) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('chrono-theme', value);
      }
      set(value);
      applyTheme(value);
    },
    toggle: () => {
      update(current => {
        const next = current === 'light' ? 'dark' : 'light';
        if (typeof window !== 'undefined') {
          localStorage.setItem('chrono-theme', next);
        }
        applyTheme(next);
        return next;
      });
    },
    init: () => {
      applyTheme(savedTheme);
      
      // Listen for system preference changes
      if (typeof window !== 'undefined') {
        window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
          const currentTheme = localStorage.getItem('chrono-theme') as Theme;
          if (currentTheme === 'system') {
            applyTheme('system');
          }
        });
      }
    }
  };
}

function applyTheme(theme: Theme) {
  if (typeof window === 'undefined') return;
  
  const root = document.documentElement;
  const isDark = theme === 'dark' || 
    (theme === 'system' && window.matchMedia('(prefers-color-scheme: dark)').matches);
  
  if (isDark) {
    root.classList.add('dark');
  } else {
    root.classList.remove('dark');
  }
}

export const theme = createThemeStore();

// Derived store for actual theme (resolves 'system' to actual value)
import { derived } from 'svelte/store';

export const resolvedTheme = derived(theme, ($theme) => {
  if ($theme === 'system' && typeof window !== 'undefined') {
    return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
  }
  return $theme === 'system' ? 'light' : $theme;
});
