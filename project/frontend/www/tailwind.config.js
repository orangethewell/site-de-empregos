/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/*.rs",
    "./src/**/*.rs",
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
