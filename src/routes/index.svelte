<script lang="ts">
  import { onMount } from 'svelte'
  import { video } from '$stores/video'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileStatTable.svelte'

  let videoElement: HTMLVideoElement | undefined
  let src: string | undefined
  let metadata: VideoMetadata | undefined

  // Avoid memory leaks from creating too many URLs.
  onMount(() => {
    const unsubscribe = video.subscribe((self) => {
      if (src !== undefined) {
        URL.revokeObjectURL(src)
      }

      if (self !== undefined) {
        src = URL.createObjectURL(self)
      }
    })

    return () => {
      unsubscribe()

      if (src !== undefined) {
        URL.revokeObjectURL(src)
      }
    }
  })

  const setDuration = (): void => {
    if (videoElement !== undefined) {
      metadata = {
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

<section class="container relative flex flex-col items-center">
  <FileInput />

  {#if $video !== undefined}
    <!-- svelte-ignore component-name-lowercase a11y-media-has-caption -->
    <video
      {src}
      autoplay
      loop
      muted
      controls
      on:loadedmetadata={setDuration}
      bind:this={videoElement}
      class="max-w-md max-h-md"
    >
      Your browser doesn't support embedded videos.
    </video>

    {#if metadata}
      <FileStatTable file={$video} {metadata} />
    {/if}
  {/if}
</section>
