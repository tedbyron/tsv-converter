//! Load a video from a `PathBuf`.

use std::fs::File;
use std::path::Path;

use anyhow::Result;

/// Load a video from a `PathBuf`, returning a thumbnail image.
pub fn load_vid(path: &Path) -> Result<()> {
    let file = File::open(path)?;

    Ok(())
}
