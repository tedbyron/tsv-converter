<script lang="ts">
  import { filePath, ogOutputFileName, outputFileName } from '$stores/file'
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
  import { invoke } from '@tauri-apps/api'

  const valid = true

  const convert = async (): Promise<void> => {
    if ($filePath === undefined || $outputFileName === undefined) return

    const options: Options = {
      path: $filePath,
      outputName: $outputFileName,
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

<form on:submit|preventDefault={convert} class="flex w-full flex-col items-start space-y-2">
  <!-- crop radio group -->
  <fieldset class="form-fieldset group">
    <legend class="form-legend">Crop</legend>

    <ul class="px-2 pt-1 pb-2">
      {#each Object.values(Crop) as opt}
        <li>
          <label
            class="flex items-center rounded-md px-2 py-1 transition-colors hover:bg-gray-200 focus-visible:bg-gray-200"
          >
            <input
              type="radio"
              name="crop"
              checked={$crop === opt}
              on:change={() => {
                $crop = opt
              }}
              class="mr-1"
            />
            {opt}
          </label>
        </li>
      {/each}
    </ul>
  </fieldset>

  <!-- TODO: background color for letterbox crop
    <fieldset>
      <legend>Background Color</legend>

      <input type="text" value="#000000" maxlength="7" />
    </fieldset>
  -->

  <!-- output file name -->
  <fieldset class="form-fieldset group w-full">
    <legend class="form-legend group-invalid:border-orange-300">Output name</legend>

    <div class="flex w-full space-x-2 px-3 pt-1 pb-2">
      <input
        name="output-name"
        required
        autocorrect="off"
        autocomplete="off"
        spellcheck="false"
        minlength="1"
        maxlength="46"
        pattern={'\\p{ASCII}+'}
        bind:value={$outputFileName}
        class="grow rounded-md px-1"
      />
      <button
        type="button"
        disabled={$outputFileName === $ogOutputFileName}
        on:click={() => {
          $outputFileName = $ogOutputFileName
        }}
      >
        Reset
      </button>
    </div>
  </fieldset>

  <button disabled={!valid} class="button hover-focus">Convert</button>
</form>
