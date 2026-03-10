use std::{
    fs,
    path::Path,
    process::{Command, Stdio},
};

use anyhow::{Context, bail};
use proc_macro2::{Ident, Literal, Span, TokenStream};
use quote::quote;
use rayon::iter::{ParallelBridge, ParallelIterator};

use crate::metadata::{Interrupts, Metadata};

pub fn export_meta_peripherals(current: &Path) -> anyhow::Result<()> {
    let pac_peri_dir = current.join("nxp-pac/src/meta_peripherals");
    let yaml_peri_dir = current.join("data/metadata/peripherals");

    if !pac_peri_dir.exists() {
        fs::create_dir_all(&pac_peri_dir)?;
    }

    for file in read_dir_all::read_dir_all(&pac_peri_dir)? {
        let file = file?;
        // Erasing is for niceness and is not required. If it doesn't work that's fine
        let _ = fs::remove_file(file.path());
    }

    read_dir_all::read_dir_all(&yaml_peri_dir)?
        .par_bridge()
        .try_for_each(|entry| {
            let entry = entry?;
            let entry_path = entry.path().to_path_buf();

            let relative_entry_path = entry_path.strip_prefix(&yaml_peri_dir)?;
            if relative_entry_path.starts_with("raw") || entry_path.is_dir() {
                return Ok(());
            }

            tracing::info!("{}", relative_entry_path.display());

            let mut output_path = pac_peri_dir.join(relative_entry_path);
            output_path.set_extension("rs");
            fs::create_dir_all(
                output_path
                    .parent()
                    .context("Getting parent of output path")?,
            )?;

            let output = Command::new("chiptool")
                .arg("gen-block")
                .arg("--input")
                .arg(
                    entry
                        .path()
                        .canonicalize()
                        .context("Canonicalizing entry path")?,
                )
                .arg("--output")
                .arg(&output_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            if !output.status.success() {
                bail!(
                    "Error generating block {}:\nSTDERR:\n{}",
                    relative_entry_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            let output = Command::new("rustfmt")
                .arg("--edition")
                .arg("2024")
                .arg(output_path)
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .output()?;

            if !output.status.success() {
                bail!(
                    "Error formatting block {}:\nSTDERR:\n{}",
                    relative_entry_path.display(),
                    String::from_utf8_lossy(&output.stderr)
                );
            }

            Ok(())
        })?;

    Ok(())
}

pub fn assemble_metapac(
    current: &Path,
    core: &str,
    metadata: &Metadata,
    interrupts: &Interrupts,
) -> anyhow::Result<()> {
    let chip_dir = current.join("nxp-pac/src/chips").join(core);
    if !chip_dir.exists() {
        fs::create_dir_all(&chip_dir).context("creating chip dir")?;
    }

    export_device_x(&chip_dir, interrupts).context("exporting device.x")?;
    export_mod_rs(&chip_dir, interrupts).context("exporting mod.rs")?;
    // TODO: export _vectors.rs

    Ok(())
}

fn export_device_x(chip_dir: &Path, interrupts: &Interrupts) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    for (name, _) in interrupts {
        writeln!(&mut contents, "PROVIDE({name} = DefaultHandler);")?;
    }

    fs::write(chip_dir.join("device.x"), contents.as_bytes())
        .context("writing contents to file")?;

    Ok(())
}

fn export_mod_rs(chip_dir: &Path, interrupts: &Interrupts) -> anyhow::Result<()> {
    use std::fmt::Write;

    let mut contents = String::new();

    writeln!(&mut contents, "{}", create_interrupts(interrupts))?;

    // TODO: Create instances for all peripherals
    // TODO: Create mods with a path pointing into meta_peripherals

    fs::write(chip_dir.join("mod.rs"), contents.as_bytes()).context("writing contents to file")?;

    Ok(())
}

fn create_interrupts(interrupts: &Interrupts) -> TokenStream {
    let interrupt_tokens = interrupts.iter().map(|(name, val)| {
        let val_lit = Literal::u32_unsuffixed(*val);
        let name_ident = Ident::new(name, Span::call_site());
        let doc = format!("{val} - {name}");

        quote! {
            #[doc = #doc]
            #name_ident = #val_lit
        }
    });

    quote! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        #[cfg_attr(feature = "defmt", derive(defmt::Format))]
        pub enum Interrupt {
            #(#interrupt_tokens),*
        }

        unsafe impl cortex_m::interrupt::InterruptNumber for Interrupt {
            #[inline(always)]
            fn number(self) -> u16 {
                self as u16
            }
        }

        // TODO: Enable
        // #[cfg(feature = "rt")]
        // mod _vectors;
    }
}
