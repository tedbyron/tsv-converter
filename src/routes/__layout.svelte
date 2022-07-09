<script lang="ts">
  import { page } from '$app/stores'
  import '@fontsource/inter'
  import '../app.css'
</script>

<svelte:head>
  <meta name="description" content="TinyCircuits TinyScreen Video (TSV) converter." />
  <meta name="url" content={$page.url.href} />
  <meta name="color-scheme" content="#0ea5e9" />
</svelte:head>

<slot />
