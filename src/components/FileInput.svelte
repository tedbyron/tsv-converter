<script lang="ts">
  import type { DialogFilter, OpenDialogOptions } from '@tauri-apps/api/dialog'
  import { open } from '@tauri-apps/api/dialog'
  import Loading from '~icons/tabler/loader-2'

  import { ffprobe } from '$lib/fileUtils'
  import { fileError, filePath } from '$stores/file'

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
    const ogFilePath = $filePath

    try {
      const path = await open(openDialogOptions)

      if (path === null || path === undefined) return
      if (Array.isArray(path)) throw new Error('Only one video file may be selected.')

      if (await ffprobe(path)) {
        $fileError = undefined
        $filePath = path
      } else {
        $filePath = undefined
        $fileError = "Couldn't read the file's metadata"
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
