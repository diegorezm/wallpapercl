import daisyui from 'daisyui';

/** @type {import('tailwindcss').Config} */
export default {
  content: ["./internal/views/**/*.{templ,go}"],
  theme: {
    extend: {},
  },
  plugins: [
    daisyui
  ],
}

