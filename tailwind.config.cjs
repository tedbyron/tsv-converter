// @ts-nocheck

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,svelte,js,ts}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Poppins"', ...defaultTheme.fontFamily.sans],
        serif: ['"Press Start 2P"', ...defaultTheme.fontFamily.serif]
      }
    },
    container: {
      center: true
    }
  }
}