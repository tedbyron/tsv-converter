<script lang="ts">
  import { fileSize, secondsToHHMMSS } from '$lib/fileUtils'
  import { invoke } from '@tauri-apps/api'

  type Metadata = Readonly<{
    name?: string
    mimes: string[]
    len?: number
    created?: number
    modified?: number
  }>

  export let path: string
  export let duration: number
  export let videoWidth: number
  export let videoHeight: number

  let metadata: Metadata | undefined

  $: invoke('metadata', { path })
    .then((res) => {
      metadata = res as Metadata
    })
    .catch(console.error)
</script>

{#if metadata}
  <div class="h-[var(--h-metadata)] w-[var(--w-video)] rounded-lg border">
    <div class="rounded-t-md bg-gray-200 text-center">
      {#if metadata.name !== undefined}
        <code aria-label="File Name">{metadata.name}</code>
      {:else}
        <code>Video</code>
      {/if}
    </div>

    <div class="grid grid-cols-[auto_1fr] justify-items-start gap-x-3 gap-y-1 px-3 py-2">
      {#if metadata.mimes.length > 0}
        <span>Type</span>
        <code class="rounded-md bg-gray-200 px-1">{metadata.mimes.join(', ')}</code>
      {/if}

      {#if duration}
        <span>Duration</span>
        <span>{secondsToHHMMSS(duration)}</span>
      {/if}

      {#if videoWidth && videoHeight}
        <span>Dimensions</span>
        <span>{`${videoWidth}x${videoHeight}`}</span>
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
