<script lang="ts">
  import LoadingIcon from '$components/LoadingIcon.svelte'
  import { ffprobe } from '$lib/fileUtils'
  import { fileError, filePath } from '$stores'
  import type { DialogFilter, OpenDialogOptions } from '@tauri-apps/api/dialog'
  import { open } from '@tauri-apps/api/dialog'

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

  const openDialog = async (): Promise<void> => {
    const ogFilePath = $filePath

    try {
      loading = true
      const selection = await open(openDialogOptions)

      if (selection === null || selection === undefined) return
      if (Array.isArray(selection)) throw new Error('Only one video file may be selected.')

      if (await ffprobe(selection)) {
        $fileError = undefined
        $filePath = selection
      } else {
        $filePath = undefined
        $fileError = "Couldn't read the file metadata \u{1f626}"
      }
    } catch (error: unknown) {
      console.error(error)
    } finally {
      // keep loading icon when transitioning views
      if (
        (ogFilePath !== undefined && $filePath !== undefined) ||
        (ogFilePath === undefined && $filePath === undefined)
      ) {
        loading = false
      }
    }
  }
</script>

<button
  type="button"
  disabled={loading}
  on:click={openDialog}
  class="button hover-focus relative min-h-[52px] min-w-[135px] {className}"
>
  {#if loading}
    <LoadingIcon
      class="absolute top-[var(--loading-offset)] left-[var(--loading-offset)] h-5 w-5"
    />
  {:else}
    <span>Select a video</span>
  {/if}
</button>
