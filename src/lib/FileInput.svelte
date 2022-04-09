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
  import { videoPath, ffprobeError } from '$stores/video'
  import { isVideo } from '$lib/ffmpeg'
  import LoadingIcon from '$lib/assets/LoadingIcon.svelte'

  let className = ''
  export { className as class }

  let loading = false

  const openDialog = async (): Promise<void> => {
    const ogVideoPath = $videoPath

    try {
      loading = true
      let selection = await open(openDialogOptions)

      if (selection === null || selection === undefined) return
      if (Array.isArray(selection)) throw new Error('Only one video file may be selected.')

      let ffprobeOk = await isVideo(selection)
      if (ffprobeOk) {
        $ffprobeError = undefined
        $videoPath = selection
      } else {
        $videoPath = undefined
        $ffprobeError = "Couldn't read the file metadata \u{1f626}"
      }
    } catch (error: unknown) {
      console.error(error)
    } finally {
      // Keep loading icon when transitioning views.
      if (
        (ogVideoPath !== undefined && $videoPath !== undefined) ||
        (ogVideoPath === undefined && $videoPath === undefined)
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
  class="relative button hover-focus min-w-[135px] min-h-[52px] {className}"
>
  {#if loading}
    <LoadingIcon
      class="absolute top-[var(--loading-offset)] left-[var(--loading-offset)] w-5 h-5"
    />
  {:else}
    <span>Select a video</span>
  {/if}
</button>
