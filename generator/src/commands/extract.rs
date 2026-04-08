use anyhow::{Context, Result};
use clap::Parser;
use std::{env, path::PathBuf};
use tracing::*;

use crate::ChipDescription;

/// Extract all metadata information from the SVD and apply any transforms.
#[derive(Parser)]
pub struct Extract {
    /// One of the encoded chip names.
    #[clap(required = true)]
    pub chip: String,
    #[clap(long)]
    pub skip_transforms: bool,
    /// Output directory of the metadata.
    #[clap(short, long)]
    pub output: Option<PathBuf>,
}

/// Command to extract all metadata information from a single SVD.
///
/// Applies any transforms automagically.
pub fn extract(extract: Extract) -> Result<()> {
    let chip = crate::CHIPS
        .iter()
        .find(|chip_description| chip_description.chip == extract.chip)
        .context("selected chip does not exist in generate list")?;

    extract_chip(chip, extract.output, extract.skip_transforms)
}

fn extract_chip(
    chip_description: &ChipDescription,
    output: Option<PathBuf>,
    skip_transforms: bool,
) -> Result<()> {
    let current_dir = env::current_dir()?;

    let chip_src_dir = current_dir
        .join("data")
        .join("mcux-soc-svd")
        .join(chip_description.chip);

    for core in chip_description.cores {
        let svd = chip_src_dir.join(core).with_extension("xml");
        debug!("svd path: {:?}", svd);

        let transforms_dir = if skip_transforms {
            None
        } else {
            let transforms_dir = current_dir.join("data").join("transforms");
            debug!("transforms path: {:?}", transforms_dir);
            Some(transforms_dir)
        };

        let output_dir = output.clone().unwrap_or_else(|| {
            current_dir
                .join("data")
                .join("metadata")
                .join("peripherals")
                .join("raw")
                .join(core)
        });

        crate::metadata::extract_peripherals(
            &svd,
            core,
            transforms_dir.as_ref().map(|path| path.as_path()),
            &output_dir,
        )?;
    }

    Ok(())
}
