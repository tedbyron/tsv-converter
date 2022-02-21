//! Utility functions.

use std::path::{Path, PathBuf};

use anyhow::{Error, Result};
use ffmpeg_next::software::scaling;
use ffmpeg_next::{codec, format, media};
use native_dialog::FileDialog;

use crate::app::Crop;
use crate::SUPPORTED_EXTS;

/// Output video constants.
mod output {
    pub(super) const WIDTH: u32 = 96;
    pub(super) const HEIGHT: u32 = 64;
    // pub(super) const FRAMERATE: u32 = 30;
    // pub(super) const AUDIO_SAMPLE_BIT_DEPTH: u32 = 10;
    // pub(super) const AUDIO_SAMPLE_COUNT_PER_FRAME: u32 = 2 * 512;
    // pub(super) const AUDIO_SAMPLE_RATE: u32 = FRAMERATE * AUDIO_SAMPLE_COUNT_PER_FRAME;
}

/// Preview frame constants.
mod preview {
    use super::output;

    pub(super) const WIDTH: u32 = output::WIDTH * 2;
    pub(super) const HEIGHT: u32 = output::HEIGHT * 2;
}

/// Open a file dialog, prompting for video files.
pub fn select_file() -> Result<Option<PathBuf>> {
    FileDialog::new()
        .add_filter("Video files", SUPPORTED_EXTS)
        .show_open_single_file()
        .map_err(Error::new)
}

/// Load a preview frame of the video with the current settings applied.
pub fn preview_frame<P>(path: P, _: Crop) -> Result<()>
where
    P: AsRef<Path>,
{
    let input_ctx = format::input(&path)?;
    let input = input_ctx
        .streams()
        .best(media::Type::Video)
        .ok_or(ffmpeg_next::Error::StreamNotFound)?;
    let decoder = codec::context::Context::from_parameters(input.parameters())?
        .decoder()
        .video()?;
    let _scaler = scaling::Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        format::Pixel::RGB24,
        preview::WIDTH,
        preview::HEIGHT,
        scaling::Flags::BILINEAR,
    )?;

    // let mut frame = frame::Video::empty();
    // while decoder.

    Ok(())
}
