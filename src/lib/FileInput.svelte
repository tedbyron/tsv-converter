<script lang="ts">
  import { video } from '$stores/video'

  let videoInput: HTMLInputElement
  let files: FileList | undefined

  $: if (files !== undefined && files.length > 0) {
    video.set(files[0])
  }
</script>

<button
  class="hover-focus p-3 border-2 border-white hover:border-sky-300 focus-visible:border-sky-300 rounded-lg"
>
  <label for="video-input" on:click={() => videoInput.click()} class="hover:cursor-pointer">
    Select a video
  </label>
  <input
    type="file"
    name="video-input"
    accept=".mp4, .mov, .mpg, .mpeg, .avi, .gif"
    aria-hidden
    bind:this={videoInput}
    bind:files
    class="hidden invisible"
  />
</button>
