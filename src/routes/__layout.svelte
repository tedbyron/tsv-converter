<script lang="ts" context="module">
  import '@fontsource/inter'
  import '@fontsource/press-start-2p'
</script>

<script lang="ts">
  import { page } from '$app/stores'
</script>

<svelte:head>
  <meta name="description" content="TinyCircuits TinyScreen Video (TSV) converter." />
  <meta name="url" content={$page.url.href} />
  <meta name="color-scheme" content="#0ea5e9" />
</svelte:head>

<slot />

<style global>
  @import url('../app.css');
</style>
