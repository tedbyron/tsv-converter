import path from 'path'

import adapter from '@sveltejs/adapter-static'
import preprocess from 'svelte-preprocess'

/** @type {import('@sveltejs/kit').Config} */
const config = {
  preprocess: preprocess(),
  kit: {
    adapter: adapter({
      fallback: path.resolve('src', 'routes', 'index.html')
    }),
    prerender: {
      default: false
    },
    vite: {
      envPrefix: 'TSV_CONVERTER_',
      resolve: {
        alias: {
          $routes: path.resolve('src', 'routes'),
          $stores: path.resolve('src', 'stores')
        }
      }
    }
  }
}

export default config
