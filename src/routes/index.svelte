<script lang="ts">
  import { fade } from 'svelte/transition'
  import { video } from '$stores'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileStatTable.svelte'
  import EditForm from '$lib/EditForm.svelte'

  let videoElement: HTMLVideoElement | undefined
  let videoMetadata: VideoMetadata | undefined

  const setDuration = (): void => {
    if (videoElement !== undefined) {
      videoMetadata = {
        duration: videoElement.duration,
        videoWidth: videoElement.videoWidth,
        videoHeight: videoElement.videoHeight
      }
    }
  }
</script>

<svelte:head>
  <title>TSV Converter</title>
  <meta property="og:title" content="TSV Converter" />
</svelte:head>

<section class="container relative flex flex-col items-center gap-6">
  <FileInput />

  {#if $video}
    <!-- svelte-ignore component-name-lowercase a11y-media-has-caption -->
    <video
      src={$video}
      autoplay
      loop
      muted
      controls
      on:loadedmetadata={setDuration}
      bind:this={videoElement}
      transition:fade={{ delay: 100, duration: 300 }}
      class="max-w-md max-h-64"
    />

    <div class="flex gap-6">
      <FileStatTable path={$video} {videoMetadata} />
      <EditForm />
    </div>
  {/if}
</section>
