<script lang="ts" context="module">
  import { Crop } from '$stores/options'

  const objectFit = {
    [Crop.Letterbox]: 'object-contain',
    [Crop.Zoom]: 'object-cover',
    [Crop.Fill]: 'object-fill'
  }
</script>

<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'
  import { crop } from '$stores/options'
  import { videoPath } from '$stores/video'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileMetadata.svelte'
  import EditForm from '$lib/EditForm.svelte'

  let videoElement: HTMLVideoElement
  let videoMetadata: VideoMetadata | undefined

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
        <div
          class="flex justify-center items-center w-[var(--w-video)] h-[var(--h-video)] bg-black rounded-lg border border-transparent"
        >
          <!-- svelte-ignore a11y-media-has-caption -->
          <video
            src={convertFileSrc($videoPath)}
            controls
            on:loadedmetadata={getVideoMetadata}
            bind:this={videoElement}
            class="block w-full h-full rounded-md {objectFit[$crop]}"
          />
        </div>

        <FileStatTable {videoMetadata} path={$videoPath} />
      </div>

      <div class="flex flex-col space-y-2 items-start max-h-[var(--h-edit)] overflow-y-scroll">
        <FileInput />
        <EditForm path={$videoPath} />
      </div>
    </div>
  {/if}
</section>
