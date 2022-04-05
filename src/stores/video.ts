import { emit, listen, type Event } from '@tauri-apps/api/event'
import { writable } from 'svelte/store'

export type Metadata = Readonly<{
  name?: string
  mimes: string[]
  len?: number
  created?: number
  modified?: number
}>

export const videoPath = writable<string | undefined>()
export const metadata = writable<Metadata | undefined>()

// Emit the video path to the backend on change.
videoPath.subscribe((self) => {
  if (self === undefined) return
  emit('video-path', self).catch(console.error)
})

// Listen for metadata updates from the backend.
export const unlisten = listen('video-metadata', (event: Event<Metadata>) => {
  metadata.set(event.payload)
})
