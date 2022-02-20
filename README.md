<div align="center">
  <h1><code>tsv-converter</code></h1>
  <p><strong>TinyCircuits TinyScreen Video (TSV) converter.</strong></p>
</div>

# Build

## *nix

- `clang`, `pkg-config`, and `ffmpeg` are required.

  - macOS

    ```sh
    brew install pkg-config ffmpeg
    ```

  - Debian-based

    ```sh
    apt install clang pkg-config ffmpeg libavcodec-dev libavformat-dev libavutil-dev
    ```

## Windows

- Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads) and ensure
  that LLVM's `bin` is in your `PATH` or the `LIBCLANG_PATH` environment variable is set.
- Install [ffmpeg](https://ffmpeg.org/download.html) and set the `FFMPEG_DIR` environment variable.
