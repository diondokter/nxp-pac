use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use chiptool::commands::{GenShared, generate::Generate};
use temp_dir::TempDir;

/// Generates nxp-pac Rust code for non-metapac PACs.
pub fn generate_core(
    svd: &Path,
    chip_dir: &Path,
    transforms_dir: &Path,
    core: &str,
) -> anyhow::Result<()> {
    if !fs::exists(svd)? {
        bail!(
            "SVD file for {} does not exist. help: did you clone submodules?",
            core
        );
    }

    let transform = transforms_dir
        .join(core.to_lowercase())
        .with_extension("yaml");

    if !fs::exists(&transform)? {
        bail!(
            "transform for core \"{}\" does not exist?",
            core.to_lowercase()
        );
    }

    let temp = TempDir::new()
        .context("Creating temp dir")?
        .dont_delete_on_drop();

    chiptool::commands::generate::generate(Generate {
        svd: svd.canonicalize()?,
        transform: vec![transform.canonicalize()?],
        gen_shared: GenShared {
            common_module: None,
            defmt_feature: "defmt".to_string(),
            no_defmt: false,
            yes_defmt: false,
            skip_no_std: true,
        },
        debug_ir_output: None,
        output: Some(temp.path().to_path_buf()),
    })
    .context(format!("Error generating {core}"))?;

    let lib_temp = temp.path().join("lib.rs");

    let device_x = temp.path().join("device.x");
    let output_dir = chip_dir.join(core.to_lowercase());
    fs::create_dir_all(&output_dir)?;
    fs::copy(&device_x, output_dir.join("device.x"))?;

    Command::new("form")
        .arg("-i")
        .arg(lib_temp.canonicalize()?)
        .arg("-o")
        .arg(output_dir.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .context("Running the `form` command. Make sure to have it installed: https://crates.io/crates/form")?;

    let mod_path = output_dir.join("mod.rs");
    fs::rename(output_dir.join("lib.rs"), &mod_path)?;
    crate::util::rustfmt(&mod_path)?;

    Ok(())
}
