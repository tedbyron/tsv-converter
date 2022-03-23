<script lang="ts">
  import { browser } from '$app/env'
  import { video } from '$stores/video'
  import FileInput from '$lib/FileInput.svelte'
  import { fileSize } from '$lib/util'
</script>

<svelte:head>
  <title>TSV Converter</title>
  <meta property="og:title" content="TSV Converter" />
</svelte:head>

<section
  class="container absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2 flex flex-col items-center"
>
  {#if $video === undefined}
    <FileInput />
  {:else}
    <FileInput />
    <p>File: {$video.name}</p>
    <p>Size: {fileSize($video.size)}</p>
    <p>Type: {$video.type}</p>
    {#if browser && navigator.language}
      <p>Last Modified: {new Date($video.lastModified).toLocaleString(navigator.language)}</p>
    {/if}
  {/if}
</section>
