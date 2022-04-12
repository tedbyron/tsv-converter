// @ts-nocheck

const defaultTheme = require('tailwindcss/defaultTheme')

module.exports = {
  content: ['./src/**/*.{html,svelte,js,ts}'],
  theme: {
    extend: {
      animation: {
        'spin-cc': 'spin-cc .6s ease-in-out infinite'
      },
      keyframes: {
        'spin-cc': {
          from: { transform: 'rotate(0deg)' },
          to: { transform: 'rotate(-360deg)' }
        }
      },
      fontFamily: {
        sans: ['"Inter"', ...defaultTheme.fontFamily.sans],
        serif: ['"Press Start 2P"', ...defaultTheme.fontFamily.serif]
      }
    },
    container: {
      center: true
    }
  }
}
