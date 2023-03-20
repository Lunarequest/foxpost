/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    '../templates/*.html.tera',
    '../static/**/*.js'
  ],
  theme: {
  },
  plugins: [require('@catppuccin/tailwindcss')({
    defaultFlavor: 'mocha'
  }),
  require('@tailwindcss/forms')
  ],
}
