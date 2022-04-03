import { writable } from 'svelte/store'

export const videoPath = writable<string | undefined>()
