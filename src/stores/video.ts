import { writable } from 'svelte/store'

export const videoPath = writable<string | undefined>()
export const videoAssetUrl = writable<string | undefined>()
