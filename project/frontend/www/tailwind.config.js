/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}",
    "./www/src/**/*.{rs,html}",
  ],
  theme: {
    extend: {
      fontFamily: {
        Poppins: ['Poppins, sans-serif'],
      },
      container: {
        center: true,
        padding: '1rem',
      },
    },
  },
  plugins: [],

  corePlugins : {
    preflight: false
  }
};
