import { sveltekit } from '@sveltejs/kit/vite'
import autoprefixer from 'autoprefixer'
import cssnano from 'cssnano'
import advancedPreset from 'cssnano-preset-advanced'
import type { Plugin } from 'postcss'
import tailwindcss from 'tailwindcss'
import icons from 'unplugin-icons/vite'
import { defineConfig } from 'vite'

export default defineConfig(({ mode }) => ({
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
        tailwindcss() as Plugin,
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
          : [autoprefixer()])
      ]
    }
  },
  clearScreen: false
}))
