<script lang="ts">
  import { fade } from 'svelte/transition'
  import { crop, crops } from '$stores'

  const convert = (_event: Event): void => {}
</script>

<form
  on:submit|preventDefault={convert}
  transition:fade={{ delay: 100, duration: 300 }}
  class="space-y-3"
>
  <fieldset class="p-3 border-2 border-white rounded-lg">
    <legend class="ml-3 px-2 py-1 border-2 border-white rounded-md">Crop</legend>

    <ul class="space-y-2">
      {#each crops as opt}
        <li>
          <label
            class="transition-colors px-3 py-1 rounded-md hover:bg-stone-700/50 focus-visible:bg-stone-700/50 capitalize"
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

  <button class="button hover-focus">Convert</button>
</form>
