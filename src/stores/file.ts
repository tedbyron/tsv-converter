import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { writable } from 'svelte/store'

export const filePath = writable<string | undefined>()
export const fileError = writable<string | undefined>()

export const ogOutputFileName = writable<string | undefined>()
export const outputFileName = writable<string | undefined>()

// watch the filePath for filesystem events; see the `watch` tauri command
filePath.subscribe((inputPath) => {
  if (inputPath === undefined) return

  invoke('watch', { path: inputPath }).catch(console.error)
  invoke('output_name', { path: inputPath })
    .then((path) => {
      ogOutputFileName.set(path as string)
      outputFileName.set(path as string)
    })
    .catch(console.error)
})

// listen for filesystem modify/remove events and unset filePath (we can't get detailed
// cross-platform event information)
listen('fs-change', () => {
  filePath.set(undefined)
  fileError.set('The active file was modified/removed')
}).catch(console.error)
