<script lang="ts" context="module">
  type Metadata = Readonly<{
    name?: string
    mimes: string[]
    len?: number
    created?: number
    modified?: number
  }>

  export type VideoMetadata = Readonly<{
    duration?: number
    videoWidth?: number
    videoHeight?: number
  }>
</script>

<script lang="ts">
  import { onMount } from 'svelte'
  import { invoke } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'
  import { fileSize, secondsToHHMMSS } from '$lib/util'

  let className = ''
  export { className as class }
  export let path: string
  export let videoMetadata: VideoMetadata | undefined

  let metadata: Metadata | undefined

  // Invoke the `file_metadata` rust command.
  onMount(async () => {
    metadata = await invoke('file_metadata', { path })
  })
</script>

{#if metadata}
  <div
    transition:fade={{ delay: 100, duration: 300 }}
    class="h-[var(--h-metadata)] border-2 border-stone-600 rounded-lg {className}"
  >
    <div class="bg-stone-700 rounded-t-md text-center">
      {#if metadata.name !== undefined}
        <code aria-label="File Name">{metadata.name}</code>
      {:else}
        <code>Video</code>
      {/if}
    </div>

    <div class="grid grid-cols-[auto_1fr] justify-items-start px-2 py-1 gap-3">
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
