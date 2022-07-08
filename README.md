<div align="center">
  <h1><code>tsv-converter</code></h1>
  <p><strong>TinyCircuits TinyScreen Video (TSV) converter.</strong></p>
</div>

# Features

- Video preview (source resolution, TinyScreen aspect ratio)

FIXME:

- webkit2gtk video issue: https://github.com/tauri-apps/tauri/issues/3725
- `asset://` protocol does not release memory:
  - https://github.com/tauri-apps/tauri/issues/2952
  - https://github.com/MicrosoftEdge/WebView2Feedback/issues/2171

TODO:

- Crop cover (zoom) gravity (and extent?)
- Crop contain (letterbox) background color
- Drag and drop file
- ffmpeg log
- keyboard shortcuts

# Development

1. Follow the [tauri prerequisites documentation](https://tauri.studio/docs/getting-started/prerequisites).
   Use node LTS gallium (v16) and enable yarn with corepack.

   ```sh
   nvm install --lts=gallium # or install node v16 manually
   nvm use --lts=gallium

   corepack enable yarn
   ```

2. Install dependencies.

   ```sh
   yarn
   ```

3. Run the frontend and the app in dev mode.

   ```sh
   yarn dev
   # in another terminal:
   yarn tauri dev
   ```

## Build

```sh
yarn tauri build
```
