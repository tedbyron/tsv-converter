<script lang="ts" context="module">
  type Metadata = Readonly<{
    name: string
    mimes: string[]
    len: number
    created?: string | number | Date
    modified?: string | number | Date
  }>

  export type VideoMetadata = Readonly<{
    duration?: number
    videoWidth?: number
    videoHeight?: number
  }>
</script>

<script lang="ts">
  import { invoke } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'
  import { fileSize } from '$lib/util'

  export let path: string
  export let videoMetadata: VideoMetadata | undefined
</script>

{#await invoke('file_metadata', { path })}
  <span>Loading</span>
{:then metadata}
  <table
    transition:fade={{ delay: 100, duration: 300 }}
    class="table-auto border-separate text-left"
  >
    <thead>
      <tr>
        <th
          aria-label="File Name"
          colspan="2"
          class="bg-stone-700 border-2 border-b-0 border-white rounded-t-lg text-center"
        >
          <code>{metadata.name}</code>
        </th>
      </tr>
    </thead>

    <tbody>
      <tr>
        <th scope="row">Type</th>
        <td>
          <code class="p-1 bg-stone-700 rounded-md">{JSON.stringify(metadata.mimes)}</code>
        </td>
      </tr>

      {#if videoMetadata}
        {#if videoMetadata.duration && !isNaN(videoMetadata.duration)}
          <tr>
            <th scope="row">Duration</th>
            <td>{videoMetadata.duration.toFixed(2)}s</td>
          </tr>
        {/if}

        {#if videoMetadata.videoWidth && videoMetadata.videoHeight}
          <tr>
            <th scope="row">Dimensions</th>
            <td>{`${videoMetadata.videoWidth}x${videoMetadata.videoHeight}`}</td>
          </tr>
        {/if}
      {/if}

      <tr>
        <th scope="row">Size</th>
        <td>{fileSize(metadata.len)}</td>
      </tr>

      {#if metadata.created}
        <tr>
          <th scope="row">Created</th>
          <td>{new Date(metadata.created).toLocaleString()}</td>
        </tr>
      {/if}

      {#if metadata.modified}
        <tr>
          <th scope="row">Modified</th>
          <td>{new Date(metadata.modified).toLocaleString()}</td>
        </tr>
      {/if}
    </tbody>
  </table>
{:catch err}
  <span>{err}</span>
{/await}
