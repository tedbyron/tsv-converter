<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'
  import { Crop, crop } from '$stores/options'
  import { videoPath } from '$stores/video'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileStatTable.svelte'
  import EditForm from '$lib/EditForm.svelte'

  let videoElement: HTMLVideoElement
  let videoMetadata: VideoMetadata | undefined
  let objectFit: 'object-contain' | 'object-cover' | 'object-fill' = 'object-contain'

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
  {#if $videoPath === undefined}
    <FileInput class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
  {:else}
    <div in:fade={{ duration: 1000 }} out:fade={{ duration: 300 }} class="h-full flex space-x-2">
      <div class="space-y-2">
        <!-- svelte-ignore component-name-lowercase a11y-media-has-caption -->
        <div
          class="flex justify-center items-center w-[var(--w-video)] h-[var(--h-video)] bg-black rounded-lg border border-transparent"
        >
          <video
            src={convertFileSrc($videoPath)}
            controls
            on:loadedmetadata={getVideoMetadata}
            bind:this={videoElement}
            class="block w-full h-full rounded-md {objectFit}"
          />
        </div>

        <FileStatTable
          {videoMetadata}
          path={$videoPath}
          class="w-[var(--w-video)] h-[var(--h-metadata)]"
        />
      </div>

      <div class="flex flex-col space-y-2 items-start max-h-[var(--h-edit)] overflow-y-scroll">
        <FileInput />
        <EditForm />
      </div>
    </div>
  {/if}
</section>
