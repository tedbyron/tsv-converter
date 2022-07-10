declare module 'tailwindcss/nesting/index.js' {
  import type { Plugin, Processor } from 'postcss'
  export = () => Plugin | Processor
}
