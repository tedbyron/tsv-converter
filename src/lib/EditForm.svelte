<script lang="ts">
  import octicons from '@primer/octicons'
  import { ogOutputFileName, outputFileName } from '$stores'
  import { Crop, crop } from '$stores/options'

  const iterationsIcon = octicons.iterations.toSVG({
    'aria-label': 'Reset',
    fill: 'currentColor'
  })
</script>

<form on:submit|preventDefault={() => {}} class="space-y-2">
  <!-- Output file name -->
  <fieldset>
    <legend> Output name </legend>

    <input
      type="text"
      name="output-name"
      autocorrect="off"
      autocomplete="off"
      spellcheck="false"
      minlength="1"
      maxlength="46"
      pattern={`\p{ASCII}+`}
      bind:value={$outputFileName}
    />
    <button
      type="button"
      on:click={() => {
        $outputFileName = $ogOutputFileName
      }}
      class="button hover-focus"
    >
      {@html iterationsIcon}
    </button>
  </fieldset>

  <!-- Crop radio group -->
  <fieldset
    class="group p-2 pt-1 border-2 border-stone-600 focus-within:border-sky-300 focus-within:bg-stone-800 rounded-lg"
  >
    <legend
      class="ml-3 px-2 py-1 border-2 border-stone-600 group-focus-within:border-sky-300 group-focus-within:bg-stone-800 rounded-md"
    >
      Crop
    </legend>

    <ul>
      {#each Object.values(Crop) as opt}
        <li>
          <label
            class="transition-colors px-3 py-1 rounded-md hover:bg-stone-800 focus-visible:bg-stone-800"
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

  <button class="button hover-focus">Convert</button>
</form>
