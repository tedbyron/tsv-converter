<script lang="ts">
  import { filePath, ogOutputFileName, outputFileName } from '$stores'
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
  import octicons from '@primer/octicons'
  import { invoke } from '@tauri-apps/api'

  const valid = true
  let nameElement: HTMLInputElement
  let resetClicked = false

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

  $: syncIcon = octicons.sync.toSVG({
    'aria-label': 'Reset',
    fill: 'currentColor',
    class: resetClicked ? 'animate-spin-cc' : ''
  })
</script>

<form on:submit|preventDefault={convert} class="flex w-full flex-col items-start space-y-2">
  <!-- crop radio group -->
  <fieldset class="form-fieldset group">
    <legend class="form-legend">Crop</legend>

    <ul class="space-y-1 px-2 pt-1 pb-2">
      {#each Object.values(Crop) as opt}
        <li>
          <label
            class="rounded-md px-2 py-1 transition-colors hover:bg-stone-800
            focus-visible:bg-stone-800"
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
  <fieldset
    on:click={() => {
      // TODO: fix this garbage
      if (document.activeElement !== nameElement) nameElement.focus()
    }}
    class="form-fieldset group w-full invalid:border-orange-300"
  >
    <legend class="form-legend group-invalid:border-orange-300">Output name</legend>

    <div class="flex w-full space-x-2 px-3 pt-1 pb-2">
      <input
        type="text"
        name="output-name"
        required
        autocorrect="off"
        autocomplete="off"
        spellcheck="false"
        minlength="1"
        maxlength="46"
        pattern={'\\p{ASCII}+'}
        bind:this={nameElement}
        bind:value={$outputFileName}
        class="grow rounded-md px-1 group-focus-within:bg-stone-800"
      />
      <button
        type="button"
        disabled={$outputFileName === $ogOutputFileName}
        on:click={() => {
          resetClicked = true
          setTimeout(() => {
            resetClicked = false
          }, 600)
          $outputFileName = $ogOutputFileName
        }}
        class="hover-focus rounded-lg border-2 border-stone-600 p-2 hover:border-sky-300
        focus-visible:border-sky-300 disabled:cursor-default disabled:border-stone-600
        disabled:text-stone-600 hover:disabled:border-stone-600 hover:disabled:bg-stone-900
        hover:disabled:text-stone-600 group-focus-within:hover:disabled:bg-stone-800"
      >
        {@html syncIcon}
      </button>
    </div>
  </fieldset>

  <button disabled={!valid} class="button hover-focus">Convert</button>
</form>
