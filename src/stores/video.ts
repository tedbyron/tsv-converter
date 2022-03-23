import { writable } from 'svelte/store'

export const video = writable<File | undefined>()
