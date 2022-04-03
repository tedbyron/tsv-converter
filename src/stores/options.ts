import { writable } from 'svelte/store'

export enum Crop {
  Letterbox = 'Letterbox (Contain)',
  Zoom = 'Zoom (Cover)',
  Fill = 'Fill (Scale)'
}

export const width = writable(96)
export const height = writable(64)
export const frameRate = writable(30)
export const audioSampleBitDepth = writable(10)
export const audioSampleCountPerFrame = writable(2 * 512)
export const audioSampleRate = writable(30 * 2 * 512)
export const crop = writable(Crop.Letterbox)
