import { derived, writable } from 'svelte/store'

/** Video scaling options. Values are CSS `object-fit` with alternate terminology in parentheses. */
export enum Crop {
  Contain = 'Contain (Letterbox)',
  Cover = 'Cover (Zoom)',
  Fill = 'Fill (Stretch)'
}

/** Video conversion options. */
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

// video
export const frameRate = 30
export const width = 96
export const height = 64
export const videoFrameBytes = 2 * width * height

// audio
export const sampleBitDepth = 10
export const sampleCountPerFrame = 2 * 512
export const sampleRate = frameRate * sampleCountPerFrame
export const audioFrameBytes = 2 * sampleCountPerFrame

export const crop = writable(Crop.Contain)

export const totalFrames = derived([duration], ([$duration]) => {
  return $duration * frameRate
})

export const scale = derived([crop], ([$crop]) => {
  switch ($crop) {
    case Crop.Contain:
      return `scale=${width}:${height}`
    case Crop.Cover:
      return `scale=${width}:${height}:force_original_aspect_ratio=increase,crop=${width}:${height}`
    case Crop.Fill:
      return `scale=${width}:${height}:force_original_aspect_ratio=decrease,pad=${width}:${height}:(ow-iw)/2:(oh-ih)/2`
  }
})
