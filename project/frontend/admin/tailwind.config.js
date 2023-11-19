/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./index.html",
    "./src/**/*.{rs,html}",
    "./frontend/src/**/*.{rs,html}",
  ],
  theme: {
    extend: {
      fontFamily: {
        Poppins: ['Poppins, sans-serif'],
      },
      keyframes: {
        slideIn: {
          '0%': { transform: 'translateY(-10px)' },
          '100%': { transform: 'translateY(0px)' },
        }
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
