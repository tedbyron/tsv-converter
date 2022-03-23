<script lang="ts">
  import '@fontsource/poppins'
  import '@fontsource/press-start-2p'
  import { page } from '$app/stores'
  import Header from '$lib/Header.svelte'
  import '../app.css'
</script>

<svelte:head>
  <meta name="description" content="TinyCircuits TinyScreen Video (TSV) converter." />
  <meta name="url" content={$page.url.href} />
  <meta name="color-scheme" content="#0ea5e9" />
  <meta property="og:description" content="TinyCircuits TinyScreen Video (TSV) converter." />
  <meta property="og:url" content={$page.url.href} />
  <meta property="og:type" content="website" />
  <meta property="og:locale" content="en_US" />
</svelte:head>

<Header />
<slot />
