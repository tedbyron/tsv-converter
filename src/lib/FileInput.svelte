<script lang="ts">
  import { video } from '$stores'

  let videoInput: HTMLInputElement
  let files: FileList | undefined
  let className = ''

  $: if (files !== undefined && files[0] !== undefined) {
    video.set(files[0])
  }

  export { className as class }
</script>

<button on:click={() => videoInput.click()} class="button hover-focus {className}">
  Select a video
  <input
    type="file"
    name="video-input"
    accept=".mp4, .mov, .mpg, .mpeg, .avi, .gif"
    aria-hidden
    bind:files
    bind:this={videoInput}
    class="hidden"
  />
</button>
