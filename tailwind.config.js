/** @type {import('tailwindcss').Config} */
module.exports = {
  content: {
    relative: true,
    files: ["*.html", "**/**/*.rs"],
  },
  theme: {
    extend: {
      keyframes: {
        growout: {
          '0%': { transform: 'scale(0)' },
          '80%' : {transform: 'scale(1.1)'},
          '100%': { transform: 'scale(1)' },
        }
      }
    },
  },
  plugins: [],
}