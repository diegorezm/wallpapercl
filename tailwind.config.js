import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: ["internal/web/**/*.tsx", "public/**/*.html"],
  theme: {
    extend: {},
  },
  plugins: [
    daisyui
  ],
  daisyui: {
    themes: [
      'light',
      'dark',
      'night',
    ],
  },
}

