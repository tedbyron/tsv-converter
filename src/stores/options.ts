import { derived, writable } from 'svelte/store'

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

export const scale = derived([width, height, crop], ([$width, $height, $crop]) => {
  switch ($crop) {
    case Crop.Letterbox:
      return `scale=${$width}:${$height}`
    case Crop.Zoom:
      return `scale=${$width}:${$height}:force_original_aspect_ratio=increase,crop=${$width}:${$height}`
    case Crop.Fill:
      return `scale=${$width}:${$height}:force_original_aspect_ratio=decrease,pad=${$width}:${$height}:(ow-iw)/2:(oh-ih)/2`
  }
})
