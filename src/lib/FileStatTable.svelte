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
  <table
    transition:fade={{ delay: 100, duration: 300 }}
    class="table-auto border-separate text-left {className}"
  >
    <thead>
      <tr>
        <th
          aria-label="File Name"
          colspan="2"
          class="bg-stone-700 border-2 border-b-0 border-stone-600 rounded-t-lg text-center"
        >
          {#if metadata.name !== undefined}
            <code>{metadata.name}</code>
          {:else}
            <code>Video</code>
          {/if}
        </th>
      </tr>
    </thead>

    <tbody>
      {#if metadata.mimes.length > 0}
        <tr>
          <th scope="row">Type</th>
          <td>
            <code class="p-1 bg-stone-700 rounded-md">{metadata.mimes.join(', ')}</code>
          </td>
        </tr>
      {/if}

      {#if videoMetadata !== undefined}
        {#if videoMetadata.duration && !isNaN(videoMetadata.duration)}
          <tr>
            <th scope="row">Duration</th>
            <td>{secondsToHHMMSS(videoMetadata.duration)}</td>
          </tr>
        {/if}

        {#if videoMetadata.videoWidth !== undefined && videoMetadata.videoHeight !== undefined}
          <tr>
            <th scope="row">Dimensions</th>
            <td>{`${videoMetadata.videoWidth}x${videoMetadata.videoHeight}`}</td>
          </tr>
        {/if}
      {/if}

      {#if metadata.len !== undefined}
        <tr>
          <th scope="row">Size</th>
          <td>{fileSize(metadata.len)}</td>
        </tr>
      {/if}

      {#if metadata.modified !== undefined}
        <tr>
          <th scope="row">Modified</th>
          <td>{new Date(metadata.modified * 1000).toLocaleString()}</td>
        </tr>
      {/if}

      {#if metadata.created !== undefined}
        <tr>
          <th scope="row">Created</th>
          <td>{new Date(metadata.created * 1000).toLocaleString()}</td>
        </tr>
      {/if}
    </tbody>
  </table>
{/if}
