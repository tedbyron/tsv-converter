import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { writable } from 'svelte/store'

export const videoPath = writable<string | undefined>()
export const ffprobeError = writable<string | undefined>()

listen('path-change', () => {
  videoPath.set(undefined)
}).catch(console.error)

videoPath.subscribe((self) => {
  if (self === undefined) return
  invoke('watch', { path: self }).catch(console.error)
})
