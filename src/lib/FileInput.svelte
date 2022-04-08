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
  import { isVideo } from '$lib/ffmpeg'
  import LoadingIcon from '$lib/assets/LoadingIcon.svelte'

  let className = ''
  export { className as class }

  let videoOk: boolean
  let loading = true

  const openDialog = async (): Promise<void> => {
    try {
      loading = true
      let selection = await open(openDialogOptions)

      if (selection === null || selection === undefined) return
      if (Array.isArray(selection)) throw new Error('Only one video file may be selected.')

      videoOk = await isVideo(selection)
      if (videoOk) {
        $videoPath = selection
      } else {
        $videoPath = undefined
      }
    } catch (error: unknown) {
      console.error(error)
    } finally {
      loading = false
    }
  }
</script>

<button
  type="button"
  disabled={loading}
  on:click={openDialog}
  class="button hover-focus {className}"
>
  <span>Select a video</span>
  {#if loading}
    <LoadingIcon class="inline w-5 h-5" />
  {/if}
</button>

{#if videoOk === false}
  <span>Hello</span>
{/if}
