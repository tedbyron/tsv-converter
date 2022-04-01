<script lang="ts" context="module">
  export interface VideoMetadata {
    readonly duration?: number
    readonly videoWidth?: number
    readonly videoHeight?: number
  }
</script>

<script lang="ts">
  import { fade } from 'svelte/transition'
  import { fileSize } from '$lib/util'

  export let file: File
  export let videoMetadata: VideoMetadata | undefined
</script>

<table transition:fade={{ delay: 100, duration: 300 }} class="table-auto border-separate text-left">
  <thead>
    <tr>
      <th
        aria-label="File Name"
        colspan="2"
        class="bg-stone-700 border-2 border-b-0 border-white rounded-t-lg text-center"
      >
        <code>{file.name}</code>
      </th>
    </tr>
  </thead>

  <tbody>
    <tr>
      <th scope="row">Type</th>
      <td>
        <code class="p-1 bg-stone-700 rounded-md">{file.type}</code>
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
      <td>{fileSize(file.size)}</td>
    </tr>

    <tr>
      <th scope="row">Modified</th>
      <td>{new Date(file.lastModified).toLocaleString()}</td>
    </tr>
  </tbody>
</table>
