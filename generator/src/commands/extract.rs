use anyhow::Result;
use clap::Parser;
use std::{env, path::PathBuf};
use tracing::*;

use crate::{ChipDescription, commands::select_chip};

/// Extract all metadata information from the SVD and apply any transforms.
#[derive(Parser)]
pub struct Extract {
    /// Chip name to extract the SVD data for.
    #[clap(required = true)]
    pub chip: String,
    /// Do not run the configured transforms.
    #[clap(long)]
    pub skip_transforms: bool,
    /// Output directory of the metadata.
    #[clap(short, long, default_value = "./data/metadata/peripherals/raw")]
    pub output: PathBuf,
}

/// Command to extract all metadata information from a single SVD.
///
/// Applies any transforms automagically.
pub fn extract(extract: Extract) -> Result<()> {
    extract_chip(
        select_chip(&extract.chip)?,
        extract.output,
        extract.skip_transforms,
    )
}

fn extract_chip(
    chip_description: &ChipDescription,
    output: PathBuf,
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

        crate::metadata::extract_peripherals(
            &svd,
            core,
            transforms_dir.as_deref(),
            &output.join(core),
        )?;
    }

    Ok(())
}
