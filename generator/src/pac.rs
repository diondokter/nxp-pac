use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use chiptool::commands::{GenShared, extract_all::ExtractAll, generate::Generate};
use temp_dir::TempDir;

use crate::rustfmt;

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
    rustfmt(&lib_temp).context("Formatting lib.rs")?;

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

    fs::rename(output_dir.join("lib.rs"), output_dir.join("mod.rs"))?;

    Ok(())
}

pub fn generate_peripherals(
    svd: &Path,
    metadata_dir: &Path,
    core: &str,
    transforms_dir: &Path,
) -> Result<(), anyhow::Error> {
    use std::fmt::Write;

    let transform = transforms_dir
        .join(core.to_lowercase())
        .with_extension("yaml");

    if !fs::exists(&transform).context("checking transform existance")? {
        bail!(
            "transform for core \"{}\" does not exist?",
            core.to_lowercase()
        );
    }

    let raw_peripherals_dir = metadata_dir.join("peripherals/raw").join(core);
    if !fs::exists(&raw_peripherals_dir).context("checking raw_peripherals_dir existance")? {
        fs::create_dir(&raw_peripherals_dir).context("creating raw_peripherals_dir")?;
    }
    for file in fs::read_dir(&raw_peripherals_dir).context("reading raw peripherals dir")? {
        let file = file?;
        if file.file_name().to_string_lossy() != ".gitignore" {
            fs::remove_file(file.path())
                .with_context(|| format!("removing {}", file.path().display()))?;
        }
    }

    chiptool::commands::extract_all::extract_all(ExtractAll {
        svd: svd.canonicalize()?,
        output: raw_peripherals_dir.canonicalize()?,
        transform: Some(vec![transform.canonicalize()?]),
    })
    .with_context(|| format!("Error generating peripheral yamls for {core}"))?;

    let svd_contents = fs::read_to_string(svd).context("Read SVD")?;
    let svd = svd_parser::parse(&svd_contents).context("Parse SVD")?;

    let nvic_priority_bits = svd
        .cpu
        .map(|cpu| cpu.nvic_priority_bits)
        .unwrap_or_default();

    let mut interrupts = Vec::new();

    for peripheral in svd.peripherals.iter() {
        for interrupt in peripheral.interrupt.iter() {
            // Rust uses fully capitalized interrupt names for singletons.
            interrupts.push((interrupt.name.clone().to_uppercase(), interrupt.value));
        }
    }

    interrupts.sort_unstable_by_key(|(_, val)| *val);
    interrupts.dedup();

    let mut interrupts_json = String::new();
    writeln!(
        &mut interrupts_json,
        "{{\n  \"nvic_prio_bits\": {nvic_priority_bits},\n  \"interrupts\": {{"
    )?;
    for (i, (name, num)) in interrupts.iter().enumerate() {
        writeln!(
            &mut interrupts_json,
            "    \"{name}\": {num}{}",
            if i != interrupts.len() - 1 { "," } else { "" }
        )?;
    }
    writeln!(&mut interrupts_json, "  }}\n}}")?;

    fs::write(
        raw_peripherals_dir.join("_interrupts.json"),
        interrupts_json.as_bytes(),
    )
    .context("writing _interrupts.json")?;

    let peripheral_addresses = svd
        .peripherals
        .iter()
        .map(|p| (&p.name, p.base_address))
        .collect::<Vec<_>>();
    let mut addresses_json = String::new();
    writeln!(&mut addresses_json, "{{")?;
    for (i, (name, address)) in peripheral_addresses.iter().enumerate() {
        writeln!(
            &mut addresses_json,
            "  \"{name}\": \"{address:#010X}\"{}",
            if i != peripheral_addresses.len() - 1 {
                ","
            } else {
                ""
            }
        )?;
    }
    writeln!(&mut addresses_json, "}}")?;
    fs::write(
        raw_peripherals_dir.join("_addresses.json"),
        addresses_json.as_bytes(),
    )
    .context("writing _addresses.json")?;

    Ok(())
}
