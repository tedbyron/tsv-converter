import { sveltekit } from '@sveltejs/kit/vite'
import icons from 'unplugin-icons/vite'
import { defineConfig } from 'vite'

// postcss plugins
import autoprefixer from 'autoprefixer'
import cssnano from 'cssnano'
import advancedPreset from 'cssnano-preset-advanced'
import tailwindcss from 'tailwindcss'
import nesting from 'tailwindcss/nesting/index.js'

export default defineConfig(({ mode }) => ({
  clearScreen: false,
  envPrefix: 'TSV_CONVERTER_',
  build: {
    cssCodeSplit: false
  },
  plugins: [
    sveltekit(),
    icons({
      autoInstall: true,
      compiler: 'svelte',
      scale: 1.25
    })
  ],
  css: {
    postcss: {
      plugins: [
        nesting(),
        tailwindcss() as any,
        ...(mode === 'production'
          ? [
              cssnano({
                preset: advancedPreset({
                  autoprefixer: { add: true },
                  convertValues: { length: true },
                  discardComments: { removeAll: true }
                })
              })
            ]
          : [autoprefixer()]) // cssnano-preset-advanced has autoprefixer built-in
      ]
    }
  }
}))
