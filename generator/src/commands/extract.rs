use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

/// Extract all metadata information from the SVD and apply any transforms.
#[derive(Parser)]
pub struct Extract {
    /// One of the encoded chip names.
    #[clap(required = true)]
    pub chip: String,
    /// Output directory of the metadata.
    #[clap(short, long, default_value = "./data/metadata/raw")]
    pub output: PathBuf,
}

/// Command to extract all metadata information from a single SVD.
///
/// Applies any transforms automagically.
pub fn extract(extract: Extract) -> Result<()> {
    todo!()
}
