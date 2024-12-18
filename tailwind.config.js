import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./internal/server/views/*.html",
    "./public/*.{html,ts}"
  ],
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
      'black'
    ],
  },
}

