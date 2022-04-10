import { derived, writable } from 'svelte/store'

export enum Crop {
  Letterbox = 'Letterbox (Contain)',
  Zoom = 'Zoom (Cover)',
  Fill = 'Fill (Scale)'
}

/** Video duration in seconds. */
export const duration = writable(NaN)
export const frameRate = 30
export const videoWidth = 96
export const videoHeight = 64
export const videoBitDepth = 16
export const videoFrameBytes = (2 * videoWidth * videoHeight) / (videoBitDepth / 16)

export const audio = true
export const audioSampleBitDepth = 10
export const audioSampleCountPerFrame = 2 * 512
export const audioSampleRate = frameRate * audioSampleCountPerFrame
export const audioFrameBytes = 2 * audioSampleCountPerFrame

export const crop = writable(Crop.Letterbox)

export const totalFrames = derived([duration], ([$duration]) => {
  return $duration * frameRate
})
export const scale = derived([crop], ([$crop]) => {
  switch ($crop) {
    case Crop.Letterbox:
      return `scale=${videoWidth}:${videoHeight}`
    case Crop.Zoom:
      return `scale=${videoWidth}:${videoHeight}:force_original_aspect_ratio=increase,crop=${videoWidth}:${videoHeight}`
    case Crop.Fill:
      return `scale=${videoWidth}:${videoHeight}:force_original_aspect_ratio=decrease,pad=${videoWidth}:${videoHeight}:(ow-iw)/2:(oh-ih)/2`
  }
})
