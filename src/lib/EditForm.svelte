<script lang="ts">
  import { fade } from 'svelte/transition'
  import { Crop, crop } from '$stores/options'

  const convert = (_event: Event): void => {}
</script>

<form
  on:submit|preventDefault={convert}
  transition:fade={{ delay: 100, duration: 300 }}
  class="space-y-2"
>
  <fieldset class="p-2 pt-1 border-2 border-white rounded-lg">
    <legend class="ml-3 px-2 py-1 border-2 border-white rounded-md">Crop</legend>

    <ul class="">
      {#each Object.values(Crop) as opt}
        <li>
          <label
            class="transition-colors px-3 py-1 rounded-md hover:bg-stone-700/50 focus-visible:bg-stone-700/50"
          >
            <input
              type="radio"
              name="crop"
              class="mr-1"
              checked={$crop === opt}
              on:change={() => crop.set(opt)}
            />
            {opt}
          </label>
        </li>
      {/each}
    </ul>
  </fieldset>

  <fieldset>
    <legend>Background Color</legend>

    <input type="text" value="#000000" maxlength="7" />
  </fieldset>

  <button class="button hover-focus">Convert</button>
</form>
