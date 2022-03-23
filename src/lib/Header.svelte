<script lang="ts">
  import octicons from '@primer/octicons'

  const markGithubIcon = octicons['mark-github'].toSVG({
    'aria-label': 'View on GitHub',
    width: '24',
    fill: 'currentColor'
  })
</script>

<header class="container my-6">
  <nav class="grid grid-cols-1 grid-rows-2 lg:grid-cols-3 lg:grid-rows-1">
    <!-- Placeholder div to center in grid -->
    <div class="hidden lg:block" />

    <!-- Site heading -->
    <div class="flex justify-center items-center">
      <h1 class="text-2xl xl:text-3xl 2xl:text-4xl font-serif text-center">TSV Converter</h1>
    </div>

    <!-- Links -->
    <div class="lg:justify-self-end flex justify-center items-center gap-3">
      <a
        href="https://github.com/tedbyron/tsv-converter"
        title="View on GitHub"
        class="hover-focus flex items-center gap-2 rounded-lg p-3"
      >
        {@html markGithubIcon}
        tedbyron/tsv-converter
      </a>
    </div>
  </nav>
</header>
