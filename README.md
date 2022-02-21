<div align="center">
  <h1><code>tsv-converter</code></h1>
  <p><strong>TinyCircuits TinyScreen Video (TSV) converter.</strong></p>
</div>

# Build

## macOS

```sh
brew install ffmpeg pkg-config
```

## Linux (Debian-based)

```sh
apt install -y clang libavcodec-dev libavformat-dev libavutil-dev pkg-config
```

## Windows

- Install [Build Tools for Visual Studio](https://visualstudio.microsoft.com/downloads) and ensure
  that LLVM's `bin` is in your `PATH` or the `LIBCLANG_PATH` environment variable is set.
- Install [ffmpeg](https://ffmpeg.org/download.html) and set the `FFMPEG_DIR` environment variable.
