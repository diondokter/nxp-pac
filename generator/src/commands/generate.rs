use std::{env, path::Path};

use anyhow::{Context, bail};
use clap::Parser;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::*;

use crate::{CHIPS, ChipDescription};

/// Generate the nxp-pac Rust code for a single chip or all the chips.
///
/// Will apply any transforms defined for the chips that are pre-metapac.
#[derive(Parser)]
pub struct Generate {
    /// Select a single chip, or all chips if not provided.
    pub chip: Option<String>,
}

/// Command to generate the nxp-pac Rust code for one or all of the chips.
pub fn generate(args: Generate) -> anyhow::Result<()> {
    let chips = if let Some(chip_name) = args.chip {
        vec![crate::commands::select_chip(&chip_name)?]
    } else {
        CHIPS.iter().collect()
    };

    let current = env::current_dir()?;

    // Export the metapac peripherals
    crate::metapac::generate_meta_peripherals(&current)?;

    // Generating code for SVDs can take a moment (RT11xx can generate a million lines of code)
    // so it is best to run multiple cores in parallel.
    let outputs: Vec<anyhow::Result<()>> = chips
        .par_iter()
        .map(|feature| generate_chip(&current, feature))
        .collect();

    let mut error = false;

    for output in outputs {
        if let Err(e) = output {
            error |= true;
            error!("Error for output {e:?}");
        }
    }

    if error {
        bail!("Failed to generate chips {:?}", error);
    }

    Ok(())
}

/// Generate (unformatted) code for a single chip.
fn generate_chip(current_dir: &Path, feature: &ChipDescription) -> anyhow::Result<()> {
    let chip_src_dir = current_dir
        .join("data")
        .join("mcux-soc-svd")
        .join(feature.chip);
    let metadata_dir = current_dir.join("data").join("metadata");
    let pac_dir = current_dir.join("nxp-pac");

    for core in feature.cores {
        let chips_dir = pac_dir.join("src").join("chips");

        if feature.metapac {
            let Some(metadata_file) = feature.metadata else {
                bail!("Metadata should not be empty when using metapac");
            };

            let metadata = crate::metadata::generate(
                &chips_dir,
                &metadata_dir.join(metadata_file).with_extension("json"),
                core,
            )
            .context("Generating metadata")?;

            crate::metapac::generate_core(current_dir, core, metadata)
                .context(format!("Assembling metapac for {core}"))?
        } else {
            let svd = chip_src_dir.join(core).with_extension("xml");
            debug!("svd path: {:?}", svd);
            let transforms_dir = current_dir.join("data").join("transforms");
            debug!("transforms path: {:?}", transforms_dir);

            info!("Generating {}/{}", feature.chip, core);
            crate::pac::generate_core(&svd, &chips_dir, &transforms_dir, core)
                .context("Generating PAC")?;
        }
    }

    Ok(())
}
