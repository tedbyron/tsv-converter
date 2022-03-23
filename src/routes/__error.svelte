<script lang="ts" context="module">
  import type { ErrorLoad } from '@sveltejs/kit'

  export const load: ErrorLoad = ({ status, error }) => {
    return {
      props: {
        error: `${status}: ${error?.message}`
      }
    }
  }
</script>

<script lang="ts">
  export let error: string
</script>

<section class="container flex flex-col gap-6 mt-12 mb-6 text-center">
  <h1 class="text-5xl lg:text-6xl">&#x1fae4;</h1>
  <p class="font-2xl font-serif">{error}</p>
</section>
