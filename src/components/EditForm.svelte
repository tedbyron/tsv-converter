<script lang="ts">
  import { invoke } from '@tauri-apps/api'

  import { inputName, inputPath, outputName } from '$stores/file'
  import {
    audioFrameBytes,
    Crop,
    crop,
    frameRate,
    sampleBitDepth,
    sampleRate,
    scale,
    videoFrameBytes,
    type Options
  } from '$stores/options'

  const valid = true

  const convert = async (): Promise<void> => {
    if ($inputPath === undefined || $outputName === undefined) return

    const options: Options = {
      path: $inputPath,
      outputName: $outputName,
      scale: $scale,

      frameRate: frameRate.toString(),
      videoFrameBytes,

      sampleBitDepth,
      sampleRate: sampleRate.toString(),
      audioFrameBytes
    }

    await invoke('convert', { options })
  }
</script>

<form on:submit|preventDefault={convert} class="flex flex-col items-start space-y-2">
  <!-- crop radio group -->
  <fieldset class="form-fieldset flex flex-col items-start">
    <legend class="form-legend">Crop</legend>

    {#each Object.values(Crop) as opt}
      <label
        class="flex items-center rounded-md px-2 py-0.5 hover:bg-zinc-200 dark:hover:bg-zinc-700"
      >
        <input
          type="radio"
          name="crop"
          checked={$crop === opt}
          on:change={() => {
            $crop = opt
          }}
          class="mr-2"
        />
        <span>{opt}</span>
      </label>
    {/each}
  </fieldset>

  <!-- TODO: background color for letterbox crop
    <fieldset>
      <legend>Background Color</legend>

      <input type="text" value="#000000" maxlength="7" />
    </fieldset>
  -->

  <!-- output file name -->
  <fieldset class="form-fieldset">
    <legend class="form-legend">Output Name</legend>

    <div class="flex items-center space-x-2">
      <input
        name="output-name"
        required
        autocorrect="off"
        autocomplete="off"
        spellcheck="false"
        minlength="1"
        maxlength="46"
        pattern="[\w\.-]+"
        bind:value={$outputName}
        class="grow"
      />
      <button
        type="button"
        disabled={$outputName === $inputName}
        on:click={() => {
          $outputName = $inputName
        }}
        class="button"
      >
        Reset
      </button>
    </div>
  </fieldset>

  <!-- convert button -->
  <button disabled={!valid} class="button button-primary">Convert</button>
</form>
