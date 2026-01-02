import type { Config } from 'tailwindcss';
import forms from '@tailwindcss/forms';

const config: Config = {
  darkMode: 'class',
  content: [
    './src/**/*.{html,js,svelte,ts}',
    './index.html'
  ],
  theme: {
    extend: {},
  },
  plugins: [
    forms
  ],
};

export default config;
