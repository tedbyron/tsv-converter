<script lang="ts" context="module">
  export type VideoMetadata = Readonly<{
    duration?: number
    videoWidth?: number
    videoHeight?: number
  }>
</script>

<script lang="ts">
  import { fade } from 'svelte/transition'
  import { fileSize, secondsToHHMMSS } from '$lib/util'
  import type { Metadata } from '$stores/video'

  let className = ''
  export { className as class }
  export let videoMetadata: VideoMetadata | undefined
  export let metadata: Metadata | undefined
</script>

{#if metadata}
  <div
    transition:fade={{ duration: 1000 }}
    class="border-2 border-stone-600 rounded-lg {className}"
  >
    <div class="bg-stone-700 rounded-t-md text-center">
      {#if metadata.name !== undefined}
        <code aria-label="File Name">{metadata.name}</code>
      {:else}
        <code>Video</code>
      {/if}
    </div>

    <div class="grid grid-cols-[auto_1fr] justify-items-start px-3 py-2 gap-x-3 gap-y-1">
      {#if metadata.mimes.length > 0}
        <span>Type</span>
        <code class="px-1 bg-stone-700 rounded-md">{metadata.mimes.join(', ')}</code>
      {/if}

      {#if videoMetadata !== undefined}
        {#if videoMetadata.duration && !isNaN(videoMetadata.duration)}
          <span>Duration</span>
          <span>{secondsToHHMMSS(videoMetadata.duration)}</span>
        {/if}

        {#if videoMetadata.videoWidth !== undefined && videoMetadata.videoHeight !== undefined}
          <span>Dimensions</span>
          <span>{`${videoMetadata.videoWidth}x${videoMetadata.videoHeight}`}</span>
        {/if}
      {/if}

      {#if metadata.len !== undefined}
        <span>Size</span>
        <span>{fileSize(metadata.len)}</span>
      {/if}

      {#if metadata.modified !== undefined}
        <span>Modified</span>
        <span>{new Date(metadata.modified * 1000).toLocaleString()}</span>
      {/if}

      {#if metadata.created !== undefined}
        <span>Created</span>
        <span>{new Date(metadata.created * 1000).toLocaleString()}</span>
      {/if}
    </div>
  </div>
{/if}
