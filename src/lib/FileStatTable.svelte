<script lang="ts" context="module">
  export interface VideoMetadata {
    readonly duration?: number
    readonly videoWidth?: number
    readonly videoHeight?: number
  }
</script>

<script lang="ts">
  import { browser } from '$app/env'
  import { fileSize } from '$lib/util'

  export let file: File
  export let metadata: VideoMetadata

  const { duration, videoWidth, videoHeight } = metadata
</script>

<table class="table-auto border-collapse rounded-lg text-left">
  <thead>
    <tr>
      <th aria-label="File Name" colspan="2" class="bg-stone-900 rounded-t-lg">
        <code>{file.name}</code>code
      </th>
    </tr>
  </thead>

  <tbody class="bg-stone-800">
    <tr>
      <th scope="row">Type</th>
      <td>
        <code class="p-1 bg-stone-700 rounded-md">{file.type}</code>
      </td>
    </tr>

    {#if duration && !isNaN(duration)}
      <tr>
        <th scope="row">Duration</th>
        <td>{duration.toFixed(2)}s</td>
      </tr>
    {/if}

    {#if videoWidth && videoHeight}
      <tr>
        <th scope="row">Dimensions</th>
        <td>{`${videoWidth}x${videoHeight}`}</td>
      </tr>
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
