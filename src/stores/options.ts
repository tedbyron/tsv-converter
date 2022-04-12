import { derived, writable } from 'svelte/store'

/** Video scaling options */
export enum Crop {
  Letterbox = 'Letterbox (Contain)',
  Zoom = 'Zoom (Cover)',
  Fill = 'Fill (Scale)'
}

// Corresponds to the `Options` type in `src-tauri/src/command.rs`.
export interface Options {
  path: string
  outputName: string
  scale: string

  frameRate: string
  videoFrameBytes: number

  sampleBitDepth: number
  sampleRate: string
  audioFrameBytes: number
}

/** Video duration in seconds. */
export const duration = writable(NaN)

// Video
export const frameRate = 30
export const width = 96
export const height = 64
export const videoFrameBytes = 2 * width * height

// Audio
export const sampleBitDepth = 10
export const sampleCountPerFrame = 2 * 512
export const sampleRate = frameRate * sampleCountPerFrame
export const audioFrameBytes = 2 * sampleCountPerFrame

export const crop = writable(Crop.Letterbox)

export const totalFrames = derived([duration], ([$duration]) => {
  return $duration * frameRate
})
export const scale = derived([crop], ([$crop]) => {
  switch ($crop) {
    case Crop.Letterbox:
      return `scale=${width}:${height}`
    case Crop.Zoom:
      return `scale=${width}:${height}:force_original_aspect_ratio=increase,crop=${width}:${height}`
    case Crop.Fill:
      return `scale=${width}:${height}:force_original_aspect_ratio=decrease,pad=${width}:${height}:(ow-iw)/2:(oh-ih)/2`
  }
})
