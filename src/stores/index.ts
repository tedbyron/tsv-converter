import { writable } from 'svelte/store'

export const crops = ['letterbox', 'zoom', 'stretch'] as const

export const video = writable<string | undefined>()
export const crop = writable<typeof crops[number]>('letterbox')
