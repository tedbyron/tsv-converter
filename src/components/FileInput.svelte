<script lang="ts">
  import type { DialogFilter, OpenDialogOptions } from '@tauri-apps/api/dialog'
  import { open } from '@tauri-apps/api/dialog'
  import Loading from '~icons/tabler/loader-2'
  import { onMount } from 'svelte'

  import { ffprobe } from '$lib/fileUtils'
  import { inputError, inputPath } from '$stores/file'

  onMount(() => {
    if (document.activeElement instanceof HTMLElement) {
      document.activeElement.blur()
    }
  })

  const videoFilter: DialogFilter = {
    name: 'Videos',
    extensions: ['mp4', 'mov', 'mpg', 'mpeg', 'avi', 'gif']
  }

  const openDialogOptions: OpenDialogOptions = {
    filters: [videoFilter]
  }

  let className = ''
  export { className as class }
  let loading = false

  const openFileDialog = async (): Promise<void> => {
    loading = true
    const originalPath = $inputPath
    const path = await open(openDialogOptions)

    if (path === null) {
      loading = false
      return
    }
    if (Array.isArray(path)) {
      $inputError = 'Please select only one file'
      loading = false
      return
    }

    if (await ffprobe(path)) {
      $inputError = undefined
      $inputPath = path
    } else {
      $inputPath = undefined
      $inputError = "Couldn't read the file's metadata"
    }

    // keep loading icon when transitioning views
    if (
      (originalPath !== undefined && $inputPath !== undefined) ||
      (originalPath === undefined && $inputPath === undefined)
    ) {
      loading = false
    }
  }
</script>

<button
  type="button"
  disabled={loading}
  on:click={openFileDialog}
  class="button-primary button relative {className}"
>
  <Loading
    aria-label="loading"
    class="absolute top-[calc(50%-.75rem)] left-[calc(50%-.75rem)] h-6 w-6 {loading
      ? 'animate-spin'
      : 'hidden'}"
  />
  <div />
  <span class:invisible={loading}>Select a video</span>
</button>
