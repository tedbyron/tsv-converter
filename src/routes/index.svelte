<script lang="ts">
  import { onMount } from 'svelte'
  import { fade } from 'svelte/transition'
  import { video } from '$stores'
  import FileInput from '$lib/FileInput.svelte'
  import FileStatTable, { type VideoMetadata } from '$lib/FileStatTable.svelte'
  import EditForm from '$lib/EditForm.svelte'

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

<section class="container relative flex flex-col items-center gap-6">
  <FileInput />

  {#if $video}
    <!-- svelte-ignore component-name-lowercase a11y-media-has-caption -->
    <video
      {src}
      autoplay
      loop
      muted
      controls
      on:loadedmetadata={setDuration}
      bind:this={videoElement}
      transition:fade={{ delay: 100, duration: 300 }}
      class="max-w-md max-h-64"
    >
      Your browser doesn't support embedded videos.
    </video>

    <div class="flex gap-6">
      <FileStatTable file={$video} {metadata} />
      <EditForm />
    </div>
  {/if}
</section>
