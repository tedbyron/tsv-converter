<script lang="ts">
  import EditForm from '$components/EditForm.svelte'
  import FileInput from '$components/FileInput.svelte'
  import FileStatTable from '$components/FileMetadata.svelte'
  import { fileError, filePath } from '$stores'
  import { crop, Crop } from '$stores/options'
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { fade } from 'svelte/transition'

  const objectFit = {
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
      class="relative h-full"
    >
      <FileInput class="absolute top-1/2 left-1/2 -translate-x-1/2 -translate-y-1/2" />
      {#if $fileError !== undefined}
        <p class="absolute top-[60%] left-1/2 -translate-x-1/2 -translate-y-1/2">
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
      <div class="space-y-2">
        <div
          class="flex h-[var(--h-video)] w-[var(--w-video)] items-center justify-center rounded-lg
          border border-transparent bg-black"
        >
          <!-- svelte-ignore a11y-media-has-caption -->
          <video
            src={convertFileSrc($filePath)}
            controls
            bind:duration
            bind:videoWidth
            bind:videoHeight
            class="block h-full w-full rounded-md {objectFit[$crop]}"
          />
        </div>

        <FileStatTable {duration} {videoWidth} {videoHeight} path={$filePath} />
      </div>

      <div
        class="flex max-h-[var(--h-edit)] w-full flex-col items-start space-y-2 overflow-y-scroll"
      >
        <FileInput />
        <EditForm />
      </div>
    </div>
  {/if}
</section>
