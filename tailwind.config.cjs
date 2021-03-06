const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Inter"', ...defaultTheme.fontFamily.sans]
        // serif: ['', ...defaultTheme.fontFamily.serif]
      }
    },
    container: {
      center: true
    }
  },
  plugins: [require('@tailwindcss/line-clamp')]
}
