/** @type {import('tailwindcss').Config} */
module.exports = {
  darkMode: 'class',
  content: [
    '../templates/*.html.tera',
    '../static/**/*.js'
  ],
  plugins: [
    require('@tailwindcss/forms')
  ]
}
