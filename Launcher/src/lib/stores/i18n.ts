import { writable, derived } from 'svelte/store';
import type { Translations } from '../i18n/types';

export type Language = 'en' | 'es' | 'fr' | 'de' | 'pt' | 'zh' | 'ja';

export const languages: { code: Language; name: string; nativeName: string }[] = [
  { code: 'en', name: 'English', nativeName: 'English' },
  { code: 'es', name: 'Spanish', nativeName: 'Español' },
  { code: 'fr', name: 'French', nativeName: 'Français' },
  { code: 'de', name: 'German', nativeName: 'Deutsch' },
  { code: 'pt', name: 'Portuguese', nativeName: 'Português' },
  { code: 'zh', name: 'Chinese', nativeName: '中文' },
  { code: 'ja', name: 'Japanese', nativeName: '日本語' },
];

// Lazy load translations
async function loadTranslation(lang: Language): Promise<Translations> {
  switch (lang) {
    case 'es': return (await import('../i18n/es')).es;
    case 'fr': return (await import('../i18n/fr')).fr;
    case 'de': return (await import('../i18n/de')).de;
    case 'pt': return (await import('../i18n/pt')).pt;
    case 'zh': return (await import('../i18n/zh')).zh;
    case 'ja': return (await import('../i18n/ja')).ja;
    default: return (await import('../i18n/en')).en;
  }
}

// Cache for loaded translations
const translationCache: Partial<Record<Language, Translations>> = {};

function getDefaultLanguage(): Language {
  if (typeof window === 'undefined') return 'en';
  
  // Check saved preference
  const saved = localStorage.getItem('chrono-language') as Language;
  if (saved && languages.find(l => l.code === saved)) return saved;
  
  // Check browser language
  const browserLang = navigator.language.split('-')[0] as Language;
  if (languages.find(l => l.code === browserLang)) return browserLang;
  
  return 'en';
}

function createLanguageStore() {
  const defaultLang = getDefaultLanguage();
  const { subscribe, set } = writable<Language>(defaultLang);

  return {
    subscribe,
    set: (lang: Language) => {
      if (typeof window !== 'undefined') {
        localStorage.setItem('chrono-language', lang);
      }
      set(lang);
    }
  };
}

export const language = createLanguageStore();

// Store for the current translations
const currentTranslations = writable<Translations | null>(null);

// Initialize translations
async function initTranslations(lang: Language) {
  if (!translationCache[lang]) {
    translationCache[lang] = await loadTranslation(lang);
  }
  currentTranslations.set(translationCache[lang]!);
}

// Subscribe to language changes and load translations
language.subscribe(async (lang) => {
  await initTranslations(lang);
});

// Initialize on startup
if (typeof window !== 'undefined') {
  initTranslations(getDefaultLanguage());
}

// Derived store that provides the translation function
export const t = derived(currentTranslations, ($trans) => {
  return (key: string, params?: Record<string, string | number>): string => {
    if (!$trans) return key;
    
    const keys = key.split('.');
    let value: any = $trans;
    
    for (const k of keys) {
      value = value?.[k];
      if (value === undefined) break;
    }
    
    if (typeof value !== 'string') return key;
    
    // Replace parameters
    if (params) {
      Object.entries(params).forEach(([param, val]) => {
        value = value.replace(new RegExp(`{${param}}`, 'g'), String(val));
      });
    }
    
    return value;
  };
});
