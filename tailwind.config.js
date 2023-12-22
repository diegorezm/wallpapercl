/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      extend: {
        colors: {
          "foreground": 'var(--foreground)',
          "red-color": 'var(--red-color)',
          "purple-color": 'var(--purple-color)',
          "pink-color": 'var(--pink-color)',
          "base": "var(--base)",
          "crust": "var(--crust)",
          "gradient-bg": "var(--gradient-bg)"
        }
      },
    },
  },
  plugins: [],
};
