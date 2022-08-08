const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,svelte}'],
  theme: {
    extend: {
      fontFamily: {
        sans: ['"Inter"', ...defaultTheme.fontFamily.sans]
        // serif: ['', ...defaultTheme.fontFamily.serif]
      },
      colors: {
        'tc-blue': '#2DAAE1', // TinyCircuits brand blue
        'tc-orange': '#F79122', // TinyCircuits brand orange
      },
    },
    container: {
      center: true
    }
  },
  plugins: [require('@tailwindcss/line-clamp')]
}
