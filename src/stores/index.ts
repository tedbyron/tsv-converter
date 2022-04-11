import { invoke } from '@tauri-apps/api'
import { listen } from '@tauri-apps/api/event'
import { writable } from 'svelte/store'

export const filePath = writable<string | undefined>()
export const fileError = writable<string | undefined>()

// Watch the filePath for filesystem events; see the `watch` function in `src-tauri/src/command.rs`.
filePath.subscribe((self) => {
  if (self === undefined) return
  invoke('watch', { path: self }).catch(console.error)
})

// Listen for filesystem modify/remove events and unset filePath (we can't get detailed
// cross-platform event information).
listen('fs-change', () => {
  filePath.set(undefined)
  fileError.set('The active file was modified/removed 😦')
}).catch(console.error)
