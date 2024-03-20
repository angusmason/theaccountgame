/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ['./src/**/*.{html,rs}', 'index.html'],
  theme: {
    extend: {
      fontFamily: {
        sans: "RedHatDisplay",
      }
    },
  },
  plugins: [
    require('@tailwindcss/typography'),
    require('@tailwindcss/forms'),
  ],
};
