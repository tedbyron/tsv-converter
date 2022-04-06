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
  import { videoPath } from '$stores/video'

  let className = ''
  export { className as class }

  const openDialog = async (): Promise<void> => {
    try {
      let selection = await open(openDialogOptions)

      if (selection === null || selection === undefined) return
      if (Array.isArray(selection)) throw new Error('Only one video file may be selected.')

      $videoPath = selection
    } catch (error: unknown) {
      console.error(error)
    }
  }
</script>

<button type="button" on:click={openDialog} class="button hover-focus {className}">
  Select a video
</button>
