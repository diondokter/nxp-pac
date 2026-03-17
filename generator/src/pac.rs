use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use temp_dir::TempDir;

use crate::rustfmt;

pub fn generate_core(
    svd: &Path,
    chip_dir: &Path,
    transforms_dir: &Path,
    core: &str,
) -> anyhow::Result<()> {
    if !fs::exists(&svd)? {
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

    let output = Command::new("chiptool")
        .arg("generate")
        .arg("--svd")
        .arg(svd.canonicalize()?)
        .arg("--transform")
        .arg(transform.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .current_dir(temp.path())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error generating {core}:\nSTDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let lib_temp = temp.path().join("lib.rs");
    rustfmt(&lib_temp).context("Formatting lib.rs")?;

    let device_x = temp.path().join("device.x");
    let output_dir = chip_dir.join(core.to_lowercase());
    fs::create_dir_all(&output_dir)?;
    fs::copy(&device_x, output_dir.join("device.x"))?;

    // Remove #![no_std] attribute, as this is not lib.rs
    let mut pac = fs::read_to_string(&lib_temp)?;
    pac = pac.replace("#![no_std]\n", "");
    fs::write(&lib_temp, pac)?;

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
            fs::remove_file(file.path())?;
        }
    }

    let output = Command::new("chiptool")
        .arg("extract-all")
        .arg("--svd")
        .arg(svd.canonicalize()?)
        .arg("--output")
        .arg(raw_peripherals_dir.canonicalize()?)
        .arg("--transform")
        .arg(transform.canonicalize()?)
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()?;

    if !output.status.success() {
        bail!(
            "Error generating peripheral yamls for {core}:\nSTDERR:\n{}",
            String::from_utf8_lossy(&output.stderr)
        );
    }

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
