<script lang="ts" context="module">
  export interface VideoMetadata {
    readonly duration?: number
    readonly videoWidth?: number
    readonly videoHeight?: number
  }
</script>

<script lang="ts">
  import { fade } from 'svelte/transition'
  import { browser } from '$app/env'
  import { fileSize } from '$lib/util'

  export let file: File
  export let metadata: VideoMetadata | undefined
</script>

<table
  transition:fade={{ delay: 100, duration: 300 }}
  class="table-auto border-collapse rounded-lg text-left"
>
  <thead>
    <tr>
      <th aria-label="File Name" colspan="2" class="bg-stone-700 rounded-t-lg text-center">
        <code>{file.name}</code>
      </th>
    </tr>
  </thead>

  <tbody class="bg-stone-800 rounded-b-lg">
    <tr>
      <th scope="row">Type</th>
      <td>
        <code class="p-1 bg-stone-700 rounded-md">{file.type}</code>
      </td>
    </tr>

    {#if metadata}
      {#if metadata.duration && !isNaN(metadata.duration)}
        <tr>
          <th scope="row">Duration</th>
          <td>{metadata.duration.toFixed(2)}s</td>
        </tr>
      {/if}

      {#if metadata.videoWidth && metadata.videoHeight}
        <tr>
          <th scope="row">Dimensions</th>
          <td>{`${metadata.videoWidth}x${metadata.videoHeight}`}</td>
        </tr>
      {/if}
    {/if}

    <tr>
      <th scope="row">Size</th>
      <td>{fileSize(file.size)}</td>
    </tr>

    {#if browser}
      <tr>
        <th scope="row">Modified</th>
        <td>{new Date(file.lastModified).toLocaleString()}</td>
      </tr>
    {/if}
  </tbody>
</table>
