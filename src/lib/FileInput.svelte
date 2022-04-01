<script lang="ts" context="module">
  import type { OpenDialogOptions, DialogFilter } from '@tauri-apps/api/dialog'

  const videoFilter: DialogFilter = {
    name: 'Videos',
    extensions: ['mp4', 'mov', 'mpg', 'mpeg', 'avi', 'gif']
  }

  const openDialogOptions: OpenDialogOptions = {
    filters: [videoFilter]
  }
</script>

<script lang="ts">
  import { open } from '@tauri-apps/api/dialog'
  import { convertFileSrc } from '@tauri-apps/api/tauri'
  import { video } from '$stores'

  let className = ''

  const openDialog = async (): Promise<void> => {
    try {
      let selection = await open(openDialogOptions)

      if (selection === null || selection === undefined) return
      if (Array.isArray(selection)) throw new Error('Only one video file may be selected.')

      video.set(convertFileSrc(selection))
    } catch (error: unknown) {
      console.error(error)
    }
  }

  export { className as class }
</script>

<button on:click={openDialog} class="button hover-focus {className}"> Select a video </button>
