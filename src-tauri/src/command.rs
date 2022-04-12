//! Tauri commands.

use std::io::Read;
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
pub fn metadata(path: String) -> Metadata {
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
                        tx.blocking_send(event.kind).unwrap()
                    }
                    _ => (),
                },
                Err(err) => eprintln!("Path watch error: {err:?}"),
            };
        })
        .unwrap();
    watcher.configure(Config::PreciseEvents(true)).unwrap();
    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    while let Some(event_kind) = rx.recv().await {
        window.emit("fs-change", event_kind).unwrap();
        watcher.unwatch(&path).unwrap();
        break;
    }
}

/// Output file name with extension must be C char[] < 50 bytes.
fn limit_file_stem(path: &Path) -> Option<&str> {
    let stem = path.file_stem()?.to_str()?;

    if stem.is_ascii() {
        if stem.len() < 47 {
            Some(stem)
        } else {
            Some(&stem[..47])
        }
    } else {
        None
    }
}

#[tauri::command]
pub fn output_name(path: String) -> String {
    let path = Path::new(&path);
    match limit_file_stem(&path) {
        Some(stem) => stem.to_string(),
        None => "out".to_string(),
    }
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

// Corresponds to the `Options` type in `src/stores/options.ts`.
#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Options {
    path: String,
    output_name: String,
    scale: String,

    // Video
    frame_rate: String,
    video_frame_bytes: usize,

    // Audio
    sample_bit_depth: u8,
    sample_rate: String,
    audio_frame_bytes: usize,
}

#[tauri::command]
pub async fn convert(options: Options) {
    let path = Path::new(&options.path);
    let output_path = path
        .with_file_name(&options.output_name)
        .with_extension("tsv");
    let ffmpeg_path = sidecar_path("ffmpeg");
    let timer = Instant::now();

    #[rustfmt::skip]
    let mut video_cmd = Command::new(&ffmpeg_path)
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
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut video_stdout = video_cmd.stdout.take().unwrap();
    let mut video_frame = vec![0; options.video_frame_bytes];

    #[rustfmt::skip]
    let mut audio_cmd = Command::new(&ffmpeg_path)
        .args([
            "-i", &options.path,
            "-f", "s16le",
            "-acodec", "pcm_s16le",
            "-ar", &options.sample_rate,
            "-ac", "1",
            "-"
        ])
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::null())
        .spawn()
        .unwrap();
    let mut audio_stdout = audio_cmd.stdout.take().unwrap();
    let mut audio_frame = vec![0; options.audio_frame_bytes];

    // TODO: parallelize these loops
    while let Ok(_) = video_stdout.read_exact(&mut video_frame) {
        // Write video here.

        if let Ok(_) = audio_stdout.read_exact(&mut audio_frame) {
            for i in 0..options.audio_frame_bytes / 2 {
                let sample = (0x8000
                    + ((audio_frame[i * 2 + 1] as i32) << 8 | (audio_frame[i * 2] as i32))
                    >> (16 - (options.sample_bit_depth as i32)))
                    & (0xFFFF >> (16 - (options.sample_bit_depth as i32)));

                audio_frame[i * 2] = (sample & 0xFF) as u8;
                audio_frame[i * 2 + 1] = (sample >> 8) as u8;
            }
        } else {
            audio_frame.fill(0);
        }

        // Write audio here.
    }

    let elapsed = timer.elapsed();
    dbg!(elapsed);

    // No more stdout, just wait for command to finish.
    video_cmd.wait().unwrap();
    audio_cmd.wait().unwrap();
}
