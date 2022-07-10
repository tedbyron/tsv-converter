//! Tauri commands.

use std::fs::OpenOptions;
use std::io::{BufWriter, Read, Write};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
#[cfg(debug_assertions)]
use std::time::Instant;

use notify::{Config, EventKind, RecursiveMode, Watcher};
use tauri::async_runtime;
use time::OffsetDateTime;

/// File metadata.
#[derive(serde::Serialize)]
pub struct Metadata {
    name: Option<String>,
    mimes: Vec<String>,
    len: Option<u64>,
    #[serde(with = "time::serde::timestamp::option")]
    created: Option<OffsetDateTime>,
    #[serde(with = "time::serde::timestamp::option")]
    modified: Option<OffsetDateTime>,
}

/// Video conversion options.
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

/// Get file metadata from a path.
#[tauri::command]
pub fn metadata(path: String) -> Metadata {
    let path = Path::new(&path);
    let name = path.file_name().map(|s| s.to_string_lossy().to_string());
    let mimes = mime_guess::from_path(path)
        .iter_raw()
        .map(String::from)
        .collect();
    let mut len = None;
    let mut created = None;
    let mut modified = None;

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

/// Watches a file path for modify/remove events, and forwards the event to the frontend.
#[tauri::command]
pub async fn watch(path: String, window: tauri::Window) {
    let path = PathBuf::from(path);
    let (tx, mut rx) = async_runtime::channel(1);

    let mut watcher =
        notify::recommended_watcher(move |res: Result<notify::Event, notify::Error>| {
            match res {
                Ok(event) => match event.kind {
                    EventKind::Modify(_) | EventKind::Remove(_) => {
                        tx.blocking_send(event.kind).unwrap();
                    }
                    _ => (),
                },
                Err(err) => eprintln!("Path watch error: {err:?}"),
            };
        })
        .unwrap();
    watcher.configure(Config::PreciseEvents(true)).unwrap();
    watcher.watch(&path, RecursiveMode::NonRecursive).unwrap();

    if let Some(event_kind) = rx.recv().await {
        window.emit("fs-change", event_kind).unwrap();
        watcher.unwatch(&path).unwrap();
    }
}

/// Calculate a possible output file stem from a file path.
#[tauri::command]
pub fn output_name(path: String) -> String {
    let path = Path::new(&path);
    match limit_file_stem(path) {
        Some(stem) => stem,
        None => "out".to_string(),
    }
}

/// Limit a file stem to 46 bytes of ASCII (Output file name with `.tsv` extension must be a C
/// `char[]` < 50 bytes).
fn limit_file_stem(path: &Path) -> Option<String> {
    let stem = path
        .file_stem()?
        .to_str()?
        .chars()
        .filter(|c| match c {
            // TODO: is this all the supported characters?
            c if c.is_ascii_alphanumeric() => true,
            '_' | '-' | '.' => true,
            _ => false,
        })
        .take(46)
        .collect::<String>();
    if stem.is_empty() {
        None
    } else {
        Some(stem)
    }
}

/// Find a sidecar executable's path.
fn sidecar_path(name: &str) -> PathBuf {
    let path = tauri::utils::platform::current_exe()
        .unwrap()
        .with_file_name(name);
    if cfg!(windows) {
        path.with_extension("exe")
    } else {
        path
    }
}

/// Convert the source video to something displayable on the TinyScreen.
#[tauri::command]
pub fn convert(options: Options) {
    let path = Path::new(&options.path);
    let output_path = path
        .with_file_name(&options.output_name)
        .with_extension("tsv");
    let output_file = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .open(&output_path)
        .unwrap();
    let mut writer = BufWriter::with_capacity(
        options.video_frame_bytes.max(options.audio_frame_bytes),
        output_file,
    );
    let ffmpeg_path = sidecar_path("ffmpeg");
    #[cfg(debug_assertions)]
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

    while video_stdout.read_exact(&mut video_frame).is_ok() {
        writer.write_all(&video_frame).unwrap();

        if audio_stdout.read_exact(&mut audio_frame).is_ok() {
            for i in 0..options.audio_frame_bytes / 2 {
                let sample = ((0x8000
                    + (u32::from(audio_frame[i * 2 + 1]) << 8 | u32::from(audio_frame[i * 2])))
                    >> (16 - u32::from(options.sample_bit_depth)))
                    & (0xFFFF >> (16 - u32::from(options.sample_bit_depth)));

                audio_frame[i * 2] = (sample & 0xFF) as u8;
                audio_frame[i * 2 + 1] = (sample >> 8) as u8;
            }
        } else {
            audio_frame.fill(0);
        }

        writer.write_all(&audio_frame).unwrap();
    }

    video_cmd.wait().unwrap();
    audio_cmd.wait().unwrap();

    #[cfg(debug_assertions)]
    {
        let conversion = timer.elapsed();
        dbg!(conversion);
    }

    writer.flush().unwrap();
}
