<script lang="ts">
  import octicons from '@primer/octicons'
  import { ogOutputFileName, outputFileName } from '$stores'
  import { Crop, crop } from '$stores/options'

  let valid = true
  let nameElement: HTMLInputElement
  let resetClicked = false

  $: syncIcon = octicons.sync.toSVG({
    'aria-label': 'Reset',
    fill: 'currentColor',
    class: resetClicked ? 'animate-spin-cc' : ''
  })
</script>

<form on:submit|preventDefault={() => {}} class="w-full flex flex-col items-start space-y-2">
  <!-- Crop radio group -->
  <fieldset class="form-fieldset group">
    <legend class="form-legend">Crop</legend>

    <ul class="space-y-1 px-2 pt-1 pb-2">
      {#each Object.values(Crop) as opt}
        <li>
          <label
            class="transition-colors px-2 py-1 rounded-md hover:bg-stone-800
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

  <!-- Output file name -->
  <fieldset
    on:click={() => nameElement.focus()}
    class="form-fieldset group w-full group-invalid:border-orange-300"
  >
    <legend class="form-legend group-invalid:border-orange-300">Output name</legend>

    <div class="flex space-x-2 w-full px-3 pt-1 pb-2">
      <input
        type="text"
        name="output-name"
        required
        autocorrect="off"
        autocomplete="off"
        spellcheck="false"
        minlength="1"
        maxlength="46"
        pattern={`\p{ASCII}+`}
        bind:this={nameElement}
        bind:value={$outputFileName}
        class="grow px-1 group-focus-within:bg-stone-800 rounded-md"
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
        class="p-2 border-2 border-stone-600 rounded-lg hover:border-sky-300
hover:disabled:border-stone-600 focus-visible:border-sky-300 disabled:cursor-default
disabled:border-stone-600 hover:disabled:bg-stone-900 group-focus-within:hover:disabled:bg-stone-800 disabled:text-stone-600
hover:disabled:text-stone-600 hover-focus"
      >
        {@html syncIcon}
      </button>
    </div>
  </fieldset>

  <button disabled={!valid} class="button hover-focus">Convert</button>
</form>
