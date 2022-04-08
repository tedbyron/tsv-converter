<script lang="ts">
  import { Crop, crop } from '$stores/options'
</script>

<form on:submit|preventDefault={() => {}} class="space-y-2">
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
              class="mr-1"
              checked={$crop === opt}
              on:change={() => {
                $crop = opt
              }}
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
