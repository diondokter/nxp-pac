//! nxp-pac generator
//!
//! This is used to generate the PAC. This applies the patches to each chip which has been enabled. For
//! some chips this may also include fetching metadata.
//!
//! ## Running
//!
//! Running the generate is done using `cargo run -p generator`. Additionally to only generate a single part,
//! you can specify the name of the part. For example to generate only `MIMXRT1011`:
//!
//! ```text
//! cargo run -p generator -- MIMXRT1011
//! ```

mod metadata;
mod metapac;
mod pac;

use std::{
    env,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use tracing::{debug, error, info, level_filters::LevelFilter};
use tracing_subscriber::EnvFilter;

struct Feature {
    /// The name of the chip to generate.
    chip: &'static str,

    /// Metadata file for this chip.
    metadata: &'static str,

    /// Cores to generate. If the chip has a single core, then this is the same as the
    /// [`name`](Feature::name) of the chip.
    cores: &'static [&'static str],

    /// When true, the source of the chip won't be the SVD anymore, but the manually curated peripherals
    metapac: bool,
}

/// Parts (and cores) to generate.
#[rustfmt::skip]
const GENERATE: &[Feature] = &[
    Feature { chip: "MIMXRT1011", metadata: "MIMXRT1011", cores: &["MIMXRT1011"], metapac: false },
    Feature { chip: "MIMXRT1062", metadata: "MIMXRT106x", cores: &["MIMXRT1062"], metapac: false },
    Feature { chip: "MIMXRT1064", metadata: "MIMXRT106x", cores: &["MIMXRT1064"], metapac: false },
    Feature { chip: "MIMXRT685S", metadata: "", cores: &["MIMXRT685S_cm33"], metapac: false },
    Feature { chip: "LPC55S16", metadata: "LPC55S16", cores: &["LPC55S16"], metapac: false },
    Feature { chip: "LPC55S69", metadata: "LPC55S6x", cores: &["LPC55S69_cm33_core0", "LPC55S69_cm33_core1"], metapac: false },
    Feature { chip: "MCXN947", metadata: "", cores: &["MCXN947_cm33_core0", "MCXN947_cm33_core1"], metapac: false },
    Feature { chip: "MCXA256", metadata: "MCXA2xx", cores: &["MCXA256"], metapac: true },
    Feature { chip: "MCXA577", metadata: "MCXA5xx", cores: &["MCXA577"], metapac: true },
];

fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_writer(std::io::stderr)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .init();

    let current = env::current_dir()?;

    let mut args = env::args();

    let generate_chips: Vec<&Feature> = if args.len() > 1 {
        let chip = args.nth(1).context("unreachable")?;

        let feature = GENERATE
            .iter()
            .find(|feature| feature.chip == chip)
            .context("selected chip does not exist in generate list")?;

        vec![feature]
    } else {
        GENERATE.iter().collect()
    };

    // Export the metapac peripherals
    metapac::export_meta_peripherals(&current)?;

    // Generating code for SVDs can take a moment (RT11xx can generate a million lines of code)
    // so it is best to run multiple cores in parallel.
    let outputs: Vec<anyhow::Result<()>> = generate_chips
        .par_iter()
        .map(|&feature| generate_chip(&current, feature))
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

    info!("Formatting code");
    Command::new("cargo")
        .arg("fmt")
        .current_dir(current)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Formatting pac code with `cargo fmt`")?;

    Ok(())
}

fn generate_chip(current_dir: &Path, feature: &Feature) -> anyhow::Result<()> {
    let chip_src_dir = current_dir
        .join("data")
        .join("mcux-soc-svd")
        .join(feature.chip);
    let metadata_dir = current_dir.join("data").join("metadata");
    let pac_dir = current_dir.join("nxp-pac");

    for core in feature.cores {
        let svd = chip_src_dir.join(core).with_extension("xml");
        debug!("svd path: {:?}", svd);
        let chips_dir = pac_dir.join("src").join("chips");

        if feature.metapac {
            if feature.metadata.is_empty() {
                bail!("Metadata should not be empty when using metapac");
            }

            let metadata = metadata::generate_core(
                &chips_dir,
                &svd,
                &metadata_dir.join(feature.metadata).with_extension("json"),
                core,
            )
            .context("Generating metadata")?;

            if feature.metapac {
                metapac::assemble_metapac(current_dir, core, metadata)
                    .context(format!("Assembling metapac for {core}"))?
            }
        } else {
            let transforms_dir = current_dir.join("data").join("transforms");
            debug!("transforms path: {:?}", transforms_dir);

            info!("Generating {}/{}", feature.chip, core);
            pac::generate_peripherals(&svd, &metadata_dir, core, &transforms_dir)
                .context("Generating peripherals")?;

            pac::generate_core(&svd, &chips_dir, &transforms_dir, core)
                .context("Generating PAC")?;
        }
    }

    Ok(())
}

fn rustfmt(path: &Path) -> anyhow::Result<()> {
    let output = Command::new("rustfmt")
        .arg(path.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error during rustfmt: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    Ok(())
}
