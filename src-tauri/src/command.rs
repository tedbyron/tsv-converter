//! Tauri commands.

use std::io::{self, Read};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use notify::{Config, EventKind, RecursiveMode, Watcher};
use time::OffsetDateTime;

// Corresponds to the `Metadata` type in `src/lib/FileStatTable.svelte`.
#[derive(serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    name: Option<String>,
    mimes: Vec<String>,
    len: Option<u64>,
    #[serde(with = "time::serde::timestamp::option")]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    modified: Option<OffsetDateTime>,
}

#[tauri::command]
pub async fn metadata(path: String) -> Metadata {
    let path = Path::new(&path);

    let name = path.file_name().map(|s| s.to_string_lossy().to_string());

    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();

    let (mut len, mut created, mut modified) = (None, None, None);
    if let Ok(meta) = path.metadata() {
        len = Some(meta.len());
        created = meta.created().ok().map(OffsetDateTime::from);
        modified = meta.modified().ok().map(OffsetDateTime::from);
    }

    Metadata {
        name,
        mimes,
        len,
        created,
        modified,
    }
}

#[tauri::command]
pub async fn watch(path: String, window: tauri::Window) {
    let path = PathBuf::from(path);
    let (tx, mut rx) = tauri::async_runtime::channel(1);

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(_) | EventKind::Remove(_) => {
                        tx.blocking_send("fs-change").unwrap()
                    }
                    _ => (),
                },
                Err(err) => eprintln!("Path watch error: {err:?}"),
            };
        })
        .unwrap();
    watcher.configure(Config::PreciseEvents(true)).unwrap();
    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    while let Some(msg) = rx.recv().await {
        window.emit(msg, ()).unwrap();
        watcher.unwatch(&path).unwrap();
        break;
    }
}

// Corresponds to the `Options` type in `src/stores/options.ts`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    path: String,
    duration: u64,
    total_frames: u64,
    scale: String,

    // Video
    frame_rate: String,
    video_frame_bytes: u16,

    // Audio
    sample_rate: String,
    audio_frame_bytes: u16,
}

#[tauri::command]
pub async fn convert(options: Options) {
    let path = Path::new(&options.path);
    let file_stem = match limit_file_stem(path) {
        Some(stem) => stem,
        None => "out",
    };
    let output_path = path.with_file_name(file_stem).with_extension("tsv");
    let ffmpeg_path = sidecar_path("ffmpeg");
    let timer = Instant::now();

    #[rustfmt::skip]
    let child = Command::new(&ffmpeg_path)
        .args([
            "-i", &options.path,
            "-f", "image2pipe",
            "-r", &options.frame_rate,
            "-vf", &options.scale,
            "-vcodec", "rawvideo",
            "-pix_fmt", "bgr565be",
            "-f", "rawvideo",
            "-",
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut video_stdout = child.stdout.unwrap();
    let mut frame = vec![0_u8; usize::from(options.video_frame_bytes)];

    #[rustfmt::skip]
    let child = Command::new(&ffmpeg_path)
        .args([
            "-i", &options.path,
            "-f", "s16le",
            "-acodec", "pcm_s16le",
            "-ar", &options.sample_rate,
            "-ac", "1",
            "-"
        ])
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut audio_stdout = child.stdout.unwrap();
    let mut sample = vec![0_u8; usize::from(options.audio_frame_bytes)];

    // TODO: parallelize these loops and hopefully zip them
    while let Ok(_) = video_stdout.read_exact(&mut frame) {
        // write frame to output
    }

    let elapsed = timer.elapsed();
}

/// File name must be < 50 bytes, so the file stem is truncated to a valid unicode character
/// boundary if necessary.
fn limit_file_stem(path: &Path) -> Option<&str> {
    let stem = path.file_stem()?.to_str()?;
    if stem.len() < 47 {
        return Some(stem);
    }

    let mut idx = 47;
    while !stem.is_char_boundary(idx) {
        idx -= 1;
    }

    Some(&stem[..idx])
}

/// Find a sidecar executable's path.
fn sidecar_path(name: &str) -> PathBuf {
    let path = tauri::utils::platform::current_exe()
        .unwrap()
        .with_file_name(name);

    #[cfg(windows)]
    return path.with_extension("exe");
    #[cfg(not(windows))]
    return path;
}
