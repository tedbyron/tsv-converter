<script lang="ts" context="module">
  // Corresponds to the `Metadata` struct in `src-tauri/src/command.rs`.
  type Metadata = Readonly<{
    name?: string
    mimes: string[]
    len?: number
    created?: number
    modified?: number
  }>
</script>

<script lang="ts">
  import { invoke } from '@tauri-apps/api'
  import { fileSize, secondsToHHMMSS } from '$lib/util'

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
  <div class="w-[var(--w-video)] h-[var(--h-metadata)] border-2 border-stone-600 rounded-lg">
    <div class="bg-stone-800 rounded-t-md text-center">
      {#if metadata.name !== undefined}
        <code aria-label="File Name">{metadata.name}</code>
      {:else}
        <code>Video</code>
      {/if}
    </div>

    <div class="grid grid-cols-[auto_1fr] justify-items-start px-3 py-2 gap-x-3 gap-y-1">
      {#if metadata.mimes.length > 0}
        <span>Type</span>
        <code class="px-1 bg-stone-800 rounded-md">{metadata.mimes.join(', ')}</code>
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
