import type { Handle } from '@sveltejs/kit'

// disable SSR for all pages
export const handle: Handle = async ({ event, resolve }) => {
  return await resolve(event, { ssr: false })
}
