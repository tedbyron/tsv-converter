<script lang="ts">
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'

  import EditForm from '$components/EditForm.svelte'
  import FileInput from '$components/FileInput.svelte'
  import FileStatTable from '$components/FileMetadata.svelte'
  import { fileError, filePath } from '$stores/file'
  import { crop, Crop } from '$stores/options'

  const videoObjectFit = {
    [Crop.Contain]: 'object-contain',
    [Crop.Cover]: 'object-cover',
    [Crop.Fill]: 'object-fill'
  }

  let duration: number
  let videoWidth: number
  let videoHeight: number
</script>

<svelte:head>
  <title>TSV Converter</title>
</svelte:head>

<section class="h-full p-2">
  {#if $filePath === undefined}
    <div
      in:fade={{ delay: 300, duration: 300 }}
      out:fade={{ duration: 300 }}
      class="flex h-full items-center justify-center"
    >
      <FileInput />
      {#if $fileError !== undefined}
        <p>
          {$fileError}
        </p>
      {/if}
    </div>
  {:else}
    <div
      in:fade={{ delay: 300, duration: 300 }}
      out:fade={{ duration: 300 }}
      class="flex h-full space-x-2"
    >
      <!-- left group -->
      <div class="w-1/2 space-y-2">
        <div
          class="flex aspect-[3/2] items-center justify-center rounded-md border border-white
          bg-gray-200"
        >
          <!-- svelte-ignore a11y-media-has-caption -->
          <video
            src={convertFileSrc($filePath)}
            controls
            loop
            bind:duration
            bind:videoWidth
            bind:videoHeight
            class="h-full w-full rounded-md {videoObjectFit[$crop]}"
          />
        </div>

        <FileStatTable {duration} {videoWidth} {videoHeight} path={$filePath} />
      </div>

      <!-- right group -->
      <div class="w-1/2 space-y-2">
        <FileInput />
        <EditForm />
      </div>
    </div>
  {/if}
</section>
