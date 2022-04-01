<div align="center">
  <h1><code>tsv-converter</code></h1>
  <p><strong>TinyCircuits TinyScreen Video (TSV) converter.</strong></p>
</div>

TODO:

- https://github.com/tauri-apps/tauri/issues/3725

# Dev

1. Follow the [tauri prerequisites documentation](https://tauri.studio/docs/getting-started/prerequisites).
   Use node LTS gallium (v16) and enable corepack for yarn.

   ```sh
   nvm install --lts=gallium # or install node v16 manually
   nvm use --lts=gallium

   corepack enable
   ```

2. Install dependencies and generate the `.svelte-kit` dir with the `dev` script.

   ```sh
   yarn
   yarn dev
   ```

3. Verify setup.

   ```sh
   yarn tauri info
   ```

4. Run the svelte frontend and the app in dev mode.

   ```sh
   yarn dev
   # new terminal tab/window
   yarn tauri dev
   ```
