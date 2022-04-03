<script lang="ts">
  import { fade } from 'svelte/transition'
  import { Crop, crop } from '$stores/options'
  import { videoAssetUrl, videoPath } from '$stores/video'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileStatTable.svelte'
  import EditForm from '$lib/EditForm.svelte'

  let videoElement: HTMLVideoElement
  let videoMetadata: VideoMetadata | undefined
  let objectFit: 'object-contain' | 'object-cover' | 'object-fill'

  $: switch ($crop) {
    case Crop.Letterbox:
      objectFit = 'object-contain'
      break
    case Crop.Zoom:
      objectFit = 'object-cover'
      break
    case Crop.Fill:
      objectFit = 'object-fill'
      break
  }

  const getVideoMetadata = (): void => {
    videoMetadata = {
      duration: videoElement.duration,
      videoWidth: videoElement.videoWidth,
      videoHeight: videoElement.videoHeight
    }
  }
</script>

<svelte:head>
  <title>TSV Converter</title>
</svelte:head>

<section class="relative h-full p-2">
  {#if $videoAssetUrl === undefined || $videoPath === undefined}
    <FileInput class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
  {:else}
    <div class="h-full grid grid-cols-2 gap-2">
      <!-- svelte-ignore component-name-lowercase a11y-media-has-caption -->
      <div
        class="flex justify-center items-center w-[calc(50vw-.75rem)] h-[calc((50vw-.75rem)*2/3)] bg-black rounded-lg border border-transparent"
      >
        <video
          src={$videoAssetUrl}
          controls
          on:loadedmetadata={getVideoMetadata}
          bind:this={videoElement}
          transition:fade={{ delay: 100, duration: 300 }}
          class="block w-full h-full rounded-md {objectFit}"
        />
      </div>

      <div class="row-start-[span_2] space-y-2">
        <FileInput />
        <EditForm />
      </div>

      <FileStatTable path={$videoPath} {videoMetadata} class="self-start" />
    </div>
  {/if}
</section>
